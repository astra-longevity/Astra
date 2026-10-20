"""Split damage production (eta) from repair (beta) per organ: acute challenge vs mapped intervention.

Cross-sectional and repeated readouts only pin the balance eta - beta
(identifiability.md §6, §8). Two candidate routes, synthetic controls only:

- challenge: an acute insult adds 0.3 x Xc damage to one organ at 18 months
  (model.simulate challenge=), readout at +1, +2, +4, +8, +16, +26 weeks. Recovery speed
  is set by repair, so it should pin beta.
- mapped: a treated cohort whose eta on that organ is multiplied by 0.7 from
  18 months (as a sealed mechanistic mapping would), same post times plus 24
  and 30 months. Two variants: the multiplier is known (`mapped_known`) or is
  one more free parameter (`mapped_free`), as for a real class whose size of
  effect is not known in advance.

Per organ, parameters [eta.a, beta.a, xc.a, kappa, eps, c] (+ m), cross-seed
Fisher (seeds 11, 12) of readout mean and variance among survivors, assay noise
NOISE_REL x biological SD, n mice per time point. Reported: share of the
recoverable subspace on ratio (eta - beta), sum (eta + beta) and xc.
Raw organ state is the readout (ideal assay); realism is identifiability.md §8.

    python3 challenge.py [n_sim] > ../../results/challenge.json
"""

from __future__ import annotations

import json
import math
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))  # py/: model

import model  # noqa: E402
from identifiability import H, jacobi_eigen  # noqa: E402
from readouts import NOISE_REL, SE_OK  # noqa: E402

SEEDS = (11, 12)
T0 = 1.5  # challenge / treatment start, years (18 months)
WEEKS = (1, 2, 4, 8, 16, 26)
BASE_AGES = (1.0, 1.5, 2.0, 2.5)
DOSE = 0.3  # x Xc of the organ
MAP_MULT = 0.7  # eta multiplier of the mapped intervention
N_SWEEP = (10, 30, 100)
SHARED = ("kappa", "eps", "c")
ROUTES = ("baseline", "challenge", "mapped_known", "mapped_free")


def organs() -> list[str]:
    return [a for a in model.AXES if a not in model.NO_READOUT]


def _moments(ys: list[float]) -> tuple[float, float]:
    n = len(ys)
    m = sum(ys) / n if n else 0.0
    return m, (sum((y - m) ** 2 for y in ys) / (n - 1) if n > 1 else 0.0)


KAPPA = None  # sensitivity override of the repair half-saturation (prior 0.5)


def _p0() -> model.Params:
    p = model.Params()
    return model.replace(p, kappa=KAPPA) if KAPPA is not None else p


def cohort(kind: str, a: str, mult: dict, n_sim: int, seed: int) -> dict:
    """Readout (mean, var) of organ a per time point for one cohort."""
    p0 = _p0()
    post = tuple(T0 + w * 7 / 365.25 for w in WEEKS)
    ages = BASE_AGES if kind == "baseline" else post + ((2.0, 2.5) if kind == "mapped" else ())
    rec = {t: [] for t in ages}
    if kind == "challenge":
        # Fixed absolute dose (a set toxin amount), sized on the prior Xc: scaling it
        # with the fitted Xc would leak Xc information into the challenge.
        chal = {a: (T0, DOSE * p0.xc[a])}
        params = model._apply(p0, model.Perturbation(mult)) if mult else p0
        model.simulate(n_sim, seed, params=params, t_max=max(ages) + 0.05, record=rec, challenge=chal)
        return {t: _moments([x[a] for x in rec[t]]) for t in ages}
    else:
        m = dict(mult)
        if kind == "mapped":
            key = f"eta.{a}"
            m[key] = m.get(key, 1.0) * MAP_MULT * m.pop("m", 1.0)
            pert_base = {k: v for k, v in mult.items() if k != "m"}
            # Treatment starts at T0; before it the cohort follows the untreated params.
            params = model._apply(p0, model.Perturbation(pert_base)) if pert_base else p0
            treat = {key: MAP_MULT * mult.get("m", 1.0)}
            pert = model.Perturbation(treat, start_age=T0)
            model.simulate(n_sim, seed, params=params, perturbation=pert, t_max=max(ages) + 0.05, record=rec)
            return {t: _moments([x[a] for x in rec[t]]) for t in ages}
        params = model._apply(p0, model.Perturbation(m)) if m else p0
    model.simulate(n_sim, seed, params=params, t_max=max(ages) + 0.05, record=rec)
    return {t: _moments([x[a] for x in rec[t]]) for t in ages}


def names_for(a: str, free_m: bool) -> list[str]:
    return [f"eta.{a}", f"beta.{a}", f"xc.{a}", *SHARED] + (["m"] if free_m else [])


def derivs(kind: str, a: str, names: list[str], n_sim: int, seed: int) -> dict:
    base = cohort(kind, a, {}, n_sim, seed)
    d = []
    for nm in names:
        if nm == "m" and kind != "mapped":
            d.append({t: (0.0, 0.0) for t in base})
            continue
        up = cohort(kind, a, {nm: math.exp(H)}, n_sim, seed)
        dn = cohort(kind, a, {nm: math.exp(-H)}, n_sim, seed)
        d.append({t: tuple((up[t][i] - dn[t][i]) / (2 * H) for i in (0, 1)) for t in base})
    return {"m0": base, "d": d}


def fisher(d1: dict, d2: dict, n: float) -> list[list[float]]:
    k = len(d1["d"])
    f = [[0.0] * k for _ in range(k)]
    for t in d1["m0"]:
        v_bio = 0.5 * (d1["m0"][t][1] + d2["m0"][t][1])
        if v_bio <= 0:
            continue
        v = v_bio * (1 + NOISE_REL**2)
        for i_mom, w in ((0, n / v), (1, n / (2 * v * v))):
            u1 = [c[t][i_mom] for c in d1["d"]]
            u2 = [c[t][i_mom] for c in d2["d"]]
            for i in range(k):
                for j in range(k):
                    f[i][j] += w * 0.5 * (u1[i] * u2[j] + u2[i] * u1[j])
    return f


def _add(*ms):
    k = len(ms[0])
    return [[sum(m[i][j] for m in ms) for j in range(k)] for i in range(k)]


def shares(f: list[list[float]], names: list[str]) -> dict:
    lam, vecs = jacobi_eigen(f)
    thr = max(1 / SE_OK**2, 3 * max(0.0, -min(lam)))
    good = [v for l, v in zip(lam, vecs) if l > thr]
    ix = {n: i for i, n in enumerate(names)}
    e, b, x = names[0], names[1], names[2]

    def share(w: dict) -> float:
        nw = math.sqrt(sum(c * c for c in w.values()))
        return round(sum(sum(v[ix[k]] * c / nw for k, c in w.items()) ** 2 for v in good), 2)

    return {"n_recoverable": len(good), "ratio": share({e: 1, b: -1}), "sum": share({e: 1, b: 1}),
            "xc": share({x: 1})}


def study(a: str, n_sim: int) -> dict:
    full = names_for(a, True)
    dd = {k: [derivs(k, a, full, n_sim, s) for s in SEEDS] for k in ("baseline", "challenge", "mapped")}
    fm = {k: fisher(*dd[k], 1.0) for k in dd}

    def drop_m(f):
        return [row[:-1] for row in f[:-1]]

    out = {}
    for n in N_SWEEP:
        sc = {k: [[n * x for x in row] for row in f] for k, f in fm.items()}
        out[n] = {
            "baseline": shares(drop_m(sc["baseline"]), full[:-1]),
            "challenge": shares(drop_m(_add(sc["baseline"], sc["challenge"])), full[:-1]),
            "mapped_known": shares(drop_m(_add(sc["baseline"], sc["mapped"])), full[:-1]),
            "mapped_free": shares(_add(sc["baseline"], sc["mapped"]), full),
        }
    return out


GRID = ((None, 1.5), (None, 0.5), (2.0, 1.5), (8.0, 1.5))  # (kappa override, T0)


def main() -> None:
    global KAPPA, T0
    n_sim = int(sys.argv[1]) if len(sys.argv) > 1 else 1000
    runs = []
    for kappa, t0 in GRID:
        KAPPA, T0 = kappa, t0
        res = {a: study(a, n_sim) for a in organs()}
        passed = {r: sorted(a for a in res if res[a][N_SWEEP[0]][r]["sum"] >= 0.8) for r in ROUTES}
        runs.append({"kappa": kappa if kappa is not None else _p0().kappa, "t0": t0,
                     "split_at_n10": passed, "per_organ": res})
        print(f"kappa={kappa} t0={t0} done", file=sys.stderr, flush=True)
    json.dump({"n_sim": n_sim, "seeds": SEEDS, "weeks": WEEKS, "dose_xc": DOSE, "map_mult": MAP_MULT,
               "noise_rel": NOISE_REL, "n_sweep": N_SWEEP, "runs": runs}, sys.stdout, indent=1)


if __name__ == "__main__":
    main()
