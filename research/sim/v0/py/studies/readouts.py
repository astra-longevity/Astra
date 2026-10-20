"""Which axis readouts make R-Sim v0 parameters recoverable? (A28, G146)

Companion of identifiability.py. Synthetic only, no real data touched.

Setup:
- Survival: N_DATA control mice, monthly death bins (as identifiability.py).
- Readouts: cross-sectional measurement of one quantity in N_PER_AGE live
  mice at each age in AGES. A readout is a function of the axis state of a
  live mouse; assay noise has SD = NOISE_REL x the between-mouse SD at that
  age (placeholder). Information comes from the mean and the variance of the
  readout among survivors (Gaussian approximation):
      F_r = n * sum_a [ dmu_a dmu_a^T / V_a + 0.5 * dV_a dV_a^T / V_a^2 ]
  with V_a = between-mouse variance + assay variance.
- All 54 parameters (log scale). Derivatives by central differences with
  common random numbers; computed for two independent seeds and combined as
  symmetrised cross products, which removes the Monte-Carlo bias (see
  identifiability.py). The noise floor is |most negative eigenvalue|.
- A direction is recoverable when its eigenvalue > max(1/SE_OK^2, 3 x floor).
  A parameter is identified when >= 90 % of its unit vector lies in the
  recoverable subspace.

Run: python3 readouts.py [n_sim] > out.json   (about 6 min at 2000)
"""

from __future__ import annotations

import json
import math
import os
import pickle
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))  # py/: model, baseline

import model  # noqa: E402
from identifiability import H, N_DATA, SE_OK, T_MAX, _corr, bin_probs, full_names, jacobi_eigen  # noqa: E402

AGES = (1.0, 1.5, 2.0, 2.5)  # years: 12, 18, 24, 30 months
N_PER_AGE = 30
N_SWEEP = (10, 30, 100)
NOISE_REL = 0.5
IDENT_SHARE = 0.9
_DEFAULT = model.Params()


def _axis(a: str):
    return lambda x: x[a]


# name -> (function of state, axis or axes it reads, example assay)
READOUTS = {
    "grip": (_axis("A1_muscle"), "A1", "grip strength, rotarod"),
    "behaviour": (_axis("A3_brain"), "A3", "open field, Y-maze"),
    "echo": (_axis("A6_heart"), "A6", "echocardiography (EF, LV mass)"),
    "lung": (_axis("A8_lung"), "A8", "plethysmography"),
    "kidney": (_axis("A9_kidney"), "A9", "BUN, urine albumin"),
    "gtt": (_axis("A10_liver_metabolism"), "A10", "glucose tolerance, ALT"),
    "immune": (_axis("A11_immune"), "A11", "blood flow cytometry"),
    "cbc": (_axis("A12_blood"), "A12", "complete blood count"),
    "skin": (_axis("A13_skin_hair"), "A13", "coat/skin score"),
    "gut": (_axis("A14_gut"), "A14", "gut permeability"),
    "body_comp": (_axis("A15_fat"), "A15", "lean/fat mass (NMR, DEXA)"),
    "frailty": (lambda x: model.frailty(x, _DEFAULT), "A18 (all)", "clinical frailty index"),
    "clock": (lambda x: sum(x.values()) / len(x), "all (proxy)", "epigenetic clock"),
}


def _moments(states: list[dict]) -> dict:
    out = {}
    for name, (g, _, _) in READOUTS.items():
        ys = [g(x) for x in states]
        n = len(ys)
        m = sum(ys) / n if n else 0.0
        v = sum((y - m) ** 2 for y in ys) / (n - 1) if n > 1 else 0.0
        out[name] = (m, v)
    return out


def run(mult: dict, n_sim: int, seed: int) -> tuple[list[float], dict]:
    """Bin probabilities and, per age, readout (mean, variance) among survivors."""
    rec = {a: [] for a in AGES}
    pert = model.Perturbation(mult) if mult else None
    return summarise(model.simulate(n_sim, seed, perturbation=pert, t_max=T_MAX, record=rec), rec)


def summarise(lifespans: list[float], rec: dict) -> tuple[list[float], dict]:
    """Bin probabilities + per-age readout moments from one simulation (shard.py reuses it)."""
    return bin_probs(lifespans), {a: _moments(rec[a]) for a in AGES}


def derive(base: tuple, ups: list[tuple], downs: list[tuple]) -> dict:
    """Central differences from (p, m) summaries: base, then one up/down pair per parameter."""
    d = {"p0": base[0], "m0": base[1], "dp": [], "dm": []}
    for (pu, mu), (pd, md) in zip(ups, downs):
        d["dp"].append([(u - v) / (2 * H) for u, v in zip(pu, pd)])
        d["dm"].append({a: {r: tuple((mu[a][r][i] - md[a][r][i]) / (2 * H) for i in (0, 1)) for r in READOUTS}
                        for a in AGES})
    return d


def derivatives(names: list[str], n_sim: int, seed: int) -> dict:
    ups = [run({nm: math.exp(H)}, n_sim, seed) for nm in names]
    downs = [run({nm: math.exp(-H)}, n_sim, seed) for nm in names]
    return derive(run({}, n_sim, seed), ups, downs)


def _sym(u1: list[float], u2: list[float], w: float, f: list[list[float]]) -> None:
    m = len(u1)
    for i in range(m):
        for j in range(m):
            f[i][j] += w * 0.5 * (u1[i] * u2[j] + u2[i] * u1[j])


def survival_fisher(d1: dict, d2: dict) -> list[list[float]]:
    m = len(d1["dp"])
    f = [[0.0] * m for _ in range(m)]
    for k in range(len(d1["p0"])):
        p = 0.5 * (d1["p0"][k] + d2["p0"][k])
        if p > 0:
            _sym([c[k] for c in d1["dp"]], [c[k] for c in d2["dp"]], N_DATA / p, f)
    return f


def readout_fisher(d1: dict, d2: dict, r: str, n: int = 1) -> list[list[float]]:
    """Information from readout r with n mice per age."""
    m = len(d1["dm"])
    f = [[0.0] * m for _ in range(m)]
    for a in AGES:
        v_bio = 0.5 * (d1["m0"][a][r][1] + d2["m0"][a][r][1])
        if v_bio <= 0:
            continue
        v = v_bio * (1 + NOISE_REL**2)
        _sym([c[a][r][0] for c in d1["dm"]], [c[a][r][0] for c in d2["dm"]], n / v, f)
        _sym([c[a][r][1] for c in d1["dm"]], [c[a][r][1] for c in d2["dm"]], n / (2 * v * v), f)
    return f


def _add(*ms: list[list[float]], scale: tuple | None = None) -> list[list[float]]:
    scale = scale or (1.0,) * len(ms)
    m = len(ms[0])
    return [[sum(s * x[i][j] for s, x in zip(scale, ms)) for j in range(m)] for i in range(m)]


def assess(f: list[list[float]], names: list[str]) -> dict:
    lam, vecs = jacobi_eigen(f)
    floor = max(0.0, -min(lam))
    thr = max(1 / SE_OK**2, 3 * floor)
    good = [v for l, v in zip(lam, vecs) if l > thr]
    share = {n: sum(v[i] ** 2 for v in good) for i, n in enumerate(names)}
    ident = sorted(n for n, s in share.items() if s >= IDENT_SHARE)
    return {"n_recoverable": len(good), "noise_floor": round(floor, 1), "threshold": round(thr, 1),
            "n_identified": len(ident), "identified": ident}


def axis_shares(f: list[list[float]], names: list[str]) -> dict:
    """Per measured organ axis: share of three unit combinations inside the
    recoverable subspace (1 = pinned). ratio = eta - beta (damage/repair
    balance), sum = eta + beta (overall speed), xc = threshold."""
    lam, vecs = jacobi_eigen(f)
    thr = max(1 / SE_OK**2, 3 * max(0.0, -min(lam)))
    good = [v for l, v in zip(lam, vecs) if l > thr]
    ix = {n: i for i, n in enumerate(names)}

    def share(w: dict) -> float:
        nw = math.sqrt(sum(x * x for x in w.values()))
        return round(sum(sum(v[ix[k]] * x / nw for k, x in w.items()) ** 2 for v in good), 2)

    out = {}
    for a in model.AXES:
        if a in model.NO_READOUT:
            continue
        e, b, x = f"eta.{a}", f"beta.{a}", f"xc.{a}"
        out[a] = {"ratio": share({e: 1, b: -1}), "sum": share({e: 1, b: 1}), "xc": share({x: 1})}
    return out


def main() -> None:
    n_sim = int(sys.argv[1]) if len(sys.argv) > 1 else 2000
    names = full_names()
    cache = os.environ.get("READOUTS_CACHE")  # optional pickle of (n_sim, d1, d2) for re-analysis
    if cache and os.path.exists(cache):
        with open(cache, "rb") as fh:
            cached_n, d1, d2 = pickle.load(fh)
        assert cached_n == n_sim, "cache built with another n_sim"
    else:
        d1, d2 = derivatives(names, n_sim, 11), derivatives(names, n_sim, 12)
        if cache:
            with open(cache, "wb") as fh:
                pickle.dump((n_sim, d1, d2), fh)
    json.dump(analyse(d1, d2, n_sim, names), sys.stdout, indent=1)


def analyse(d1: dict, d2: dict, n_sim: int, names: list[str]) -> dict:
    """Full readout study from two seeds' derivatives (main and shard.py --gather)."""
    fs = survival_fisher(d1, d2)
    fr = {r: readout_fisher(d1, d2, r) for r in READOUTS}  # per mouse per age

    def with_(rs: list[str], n: int) -> list[list[float]]:
        return _add(fs, *(fr[r] for r in rs), scale=(1.0,) + (float(n),) * len(rs))

    def only(rs: list[str], n: int) -> list[list[float]]:
        # Readouts alone: the survival estimate's Monte-Carlo floor (~1e3) would
        # otherwise mask them. Independent data, so information adds; the
        # survival directions are reported separately.
        return _add(*(fr[r] for r in rs), scale=(float(n),) * len(rs))

    single = {r: assess(only([r], N_PER_AGE), names) for r in READOUTS}
    ranked = sorted(READOUTS, key=lambda r: (-single[r]["n_identified"], -single[r]["n_recoverable"]))
    greedy, chosen, left = [], [], list(READOUTS)
    while left:
        best = max(left, key=lambda r: (assess(only(chosen + [r], N_PER_AGE), names)["n_identified"],
                                        single[r]["n_recoverable"]))
        chosen.append(best)
        left.remove(best)
        res = assess(only(chosen, N_PER_AGE), names)
        greedy.append({"add": best, "n_identified": res["n_identified"], "n_recoverable": res["n_recoverable"]})
    sweep = {n: assess(only(list(READOUTS), n), names) for n in N_SWEEP}
    sweep_with_survival = {n: assess(with_(list(READOUTS), n), names) for n in N_SWEEP}
    seed_corr = {r: round(_corr([x for c in d1["dm"] for a in AGES for x in c[a][r]],
                                [x for c in d2["dm"] for a in AGES for x in c[a][r]]), 3) for r in READOUTS}
    out = {
        "n_data": N_DATA, "n_sim": n_sim, "ages": AGES, "n_per_age": N_PER_AGE, "noise_rel": NOISE_REL,
        "readouts": {r: {"reads": READOUTS[r][1], "assay": READOUTS[r][2]} for r in READOUTS},
        "survival_only": assess(fs, names),
        "single": {r: single[r] for r in ranked},
        "greedy": greedy,
        "all_readouts_by_n": sweep,
        "all_readouts_plus_survival_by_n": sweep_with_survival,
        "axis_shares_all_readouts": axis_shares(only(list(READOUTS), N_PER_AGE), names),
        "readout_seed_corr": seed_corr,
    }
    return out


if __name__ == "__main__":
    main()
