"""Repeated measures vs cross-sectional readouts, with assay realism (A28, G146).

Companion of readouts.py. Synthetic only, no real data touched.

Question: does measuring the SAME mouse at 12/18/24/30 months separate damage
production (eta) from repair (beta) per organ, which cross-sectional readouts
cannot (identifiability.md section 6)?

Designs, both scored on readouts alone (survival's Monte-Carlo floor masks
them at n_sim 2000, see readouts.py):
- cross: n different mice measured at each age (one block per age).
- long:  n mice enrolled at 12 months, each measured at every age it reaches.
  Mice are grouped by the last age reached (dropout pattern); each pattern is
  a multivariate Gaussian over the ages it covers. Weight = n x share of the
  enrolled mice in that pattern. The information in the dropout itself is
  survival information and is not counted here.
- A terminal assay (lung) is cross-sectional in both designs.

Gaussian information per block (mean m, covariance S incl. assay noise):
    F_ij = w [ dm_i' S^-1 dm_j + 0.5 tr(S^-1 dS_i S^-1 dS_j) ]
estimated with two seeds and symmetrised cross products (unbiased, as in
identifiability.py). Readouts with an unknown per-age offset (vaccine: a new
antigen at each age) keep only the covariance term.

Assay realism (A70 dossier, mouse/10-12, via A28): lung terminal; kidney =
GFR + albuminuria (two quantities, same mice); GTT and fat non-monotonic
(rise then fall), GTT per lean mass; vaccine needs a new antigen per age.
Curve shapes are placeholders. `ideal:` readouts = the organ state itself,
repeatable, for comparison.

Run: python3 longitudinal.py [n_sim] > out.json   (about 7 min at 2000)
"""

from __future__ import annotations

import json
import math
import os
import pickle
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))  # py/: model, baseline

import model  # noqa: E402
from identifiability import H, SE_OK, full_names, jacobi_eigen  # noqa: E402
from readouts import IDENT_SHARE

AGES = (1.0, 1.5, 2.0, 2.5)
N_PER_AGE = 30
N_SWEEP = (10, 30, 100)
NOISE_REL = 0.5
PEAK = 1.0  # placeholder state at which non-monotonic readouts peak
XC0 = model.Params().xc["A1_muscle"]


def _hump(x: float) -> float:
    """Rises to its maximum at x = PEAK, then falls (non-monotonic assay)."""
    return x * math.exp(1 - x / PEAK)


def _lean(x: dict) -> float:
    return 1 - 0.5 * min(1.0, x["A1_muscle"] / XC0)


def _one(a: str):
    return lambda x: [x[a]]


_DEFAULT = model.Params()
# name -> (state -> list of values, axis, assay, repeatable, mean informative)
REALISTIC = {
    "grip": (_one("A1_muscle"), "A1", "grip strength", True, True),
    "behaviour": (_one("A3_brain"), "A3", "open field, Y-maze", True, True),
    "echo": (_one("A6_heart"), "A6", "echocardiography", True, True),
    "lung": (_one("A8_lung"), "A8", "forced oscillation (terminal)", False, True),
    "kidney": (lambda x: [x["A9_kidney"], x["A9_kidney"] ** 2 / XC0], "A9", "transcutaneous GFR + albuminuria", True, True),
    "gtt": (lambda x: [_hump(x["A10_liver_metabolism"]) / _lean(x)], "A10", "GTT per lean mass (non-monotonic)", True, True),
    "vaccine": (_one("A11_immune"), "A11", "antibody titre, new antigen per age", True, False),
    "cbc": (_one("A12_blood"), "A12", "complete blood count", True, True),
    "skin": (_one("A13_skin_hair"), "A13", "coat/skin score", True, True),
    "gut": (_one("A14_gut"), "A14", "gut permeability", True, True),
    "body_comp": (lambda x: [_hump(x["A15_fat"])], "A15", "fat mass by NMR (non-monotonic)", True, True),
    "frailty": (lambda x: [model.frailty(x, _DEFAULT)], "A18", "clinical frailty index", True, True),
    "clock": (lambda x: [sum(x.values()) / len(x)], "all (proxy)", "epigenetic clock", True, True),
}
IDEAL = {f"ideal:{a}": (_one(a), a, "organ state itself", True, True) for a in model.AXES if a not in model.NO_READOUT}
ALL = {**REALISTIC, **IDEAL}


def _moments(vs: list[list[float]]):
    n = len(vs)
    if n < 2:
        return None
    k = len(vs[0])
    mu = [sum(v[i] for v in vs) / n for i in range(k)]
    cov = [[sum((v[i] - mu[i]) * (v[j] - mu[j]) for v in vs) / (n - 1) for j in range(k)] for i in range(k)]
    return [mu, cov]


def run(mult: dict, n_sim: int, seed: int) -> dict:
    track = {a: {} for a in AGES}
    pert = model.Perturbation(mult) if mult else None
    model.simulate(n_sim, seed, perturbation=pert, t_max=AGES[-1] + 0.1, track=track)
    return summarise(track)


def summarise(track: dict) -> dict:
    """Cross-sectional and per-dropout-pattern moments from one tracked simulation."""
    vals = {a: {i: {r: g(x) for r, (g, *_) in ALL.items()} for i, x in track[a].items()} for a in AGES}
    enrolled = max(1, len(track[AGES[0]]))
    out = {"w": {}, "cross": {r: {} for r in ALL}, "long": {r: {} for r in ALL}}
    for a in AGES:
        for r in ALL:
            out["cross"][r][a] = _moments([v[r] for v in vals[a].values()])
    for k in range(1, len(AGES) + 1):
        last = AGES[k - 1]
        ids = [i for i in track[last] if k == len(AGES) or i not in track[AGES[k]]]
        out["w"][k] = len(ids) / enrolled
        for r in ALL:
            out["long"][r][k] = _moments([[y for a in AGES[:k] for y in vals[a][i][r]] for i in ids])
    return out


def _diff(u, d):
    if u is None or d is None:
        return None
    if isinstance(u, dict):
        return {k: _diff(u[k], d[k]) for k in u}
    if isinstance(u, list):
        return [_diff(a, b) for a, b in zip(u, d)]
    return (u - d) / (2 * H)


def derive(base: dict, ups: list[dict], downs: list[dict]) -> dict:
    """Central differences from summaries: base, then one up/down pair per parameter."""
    dd = [_diff({"cross": u["cross"], "long": u["long"]}, {"cross": d["cross"], "long": d["long"]})
          for u, d in zip(ups, downs)]
    return {"m0": base, "dd": dd}


def derivatives(names: list[str], n_sim: int, seed: int) -> dict:
    ups = [run({nm: math.exp(H)}, n_sim, seed) for nm in names]
    downs = [run({nm: math.exp(-H)}, n_sim, seed) for nm in names]
    return derive(run({}, n_sim, seed), ups, downs)


def _inv(a: list[list[float]]) -> list[list[float]]:
    n = len(a)
    m = [row[:] + [float(i == j) for j in range(n)] for i, row in enumerate(a)]
    for c in range(n):
        p = max(range(c, n), key=lambda r: abs(m[r][c]))
        m[c], m[p] = m[p], m[c]
        piv = m[c][c]
        m[c] = [x / piv for x in m[c]]
        for r in range(n):
            if r != c and m[r][c]:
                fac = m[r][c]
                m[r] = [x - fac * y for x, y in zip(m[r], m[c])]
    return [row[n:] for row in m]


def _mm(a, b):
    return [[sum(a[i][k] * b[k][j] for k in range(len(b))) for j in range(len(b[0]))] for i in range(len(a))]


def _block(f, weight: float, s_bio, d1: list, d2: list, use_mean: bool) -> None:
    if s_bio is None or any(x is None for x in d1 + d2):
        return
    k = len(s_bio)
    s = [[s_bio[i][j] * (1 + NOISE_REL**2 * (i == j)) for j in range(k)] for i in range(k)]
    if any(s[i][i] <= 0 for i in range(k)):
        return
    si = _inv(s)
    a1 = [[sum(si[p][q] * x[0][q] for q in range(k)) for p in range(k)] for x in d1]
    m1 = [_mm(si, x[1]) for x in d1]
    m2 = [_mm(si, x[1]) for x in d2]
    m = len(f)
    for i in range(m):
        for j in range(i, m):
            v = 0.0
            if use_mean:
                v += 0.5 * (sum(p * q for p, q in zip(a1[i], d2[j][0])) + sum(p * q for p, q in zip(a1[j], d2[i][0])))
            tr = sum(m1[i][p][q] * m2[j][q][p] + m1[j][p][q] * m2[i][q][p] for p in range(k) for q in range(k))
            v += 0.25 * tr
            f[i][j] += weight * v
            if i != j:
                f[j][i] += weight * v


def _avg(a, b):
    if a is None or b is None:
        return None
    return [[0.5 * (x + y) for x, y in zip(ra, rb)] for ra, rb in zip(a[1], b[1])]


def fisher(d1: dict, d2: dict, r: str, design: str, n: float) -> list[list[float]]:
    m = len(d1["dd"])
    f = [[0.0] * m for _ in range(m)]
    _, _, _, repeatable, use_mean = ALL[r]
    if design == "cross" or not repeatable:
        for a in AGES:
            s = _avg(d1["m0"]["cross"][r][a], d2["m0"]["cross"][r][a])
            _block(f, n, s, [x["cross"][r][a] for x in d1["dd"]], [x["cross"][r][a] for x in d2["dd"]], use_mean)
    else:
        for k in d1["m0"]["w"]:
            w = 0.5 * (d1["m0"]["w"][k] + d2["m0"]["w"][k])
            s = _avg(d1["m0"]["long"][r][k], d2["m0"]["long"][r][k])
            _block(f, n * w, s, [x["long"][r][k] for x in d1["dd"]], [x["long"][r][k] for x in d2["dd"]], use_mean)
    return f


def _subspace(f):
    lam, vecs = jacobi_eigen(f)
    floor = max(0.0, -min(lam))
    thr = max(1 / SE_OK**2, 3 * floor)
    return [v for l, v in zip(lam, vecs) if l > thr], floor


def assess(f, names: list[str]) -> dict:
    good, floor = _subspace(f)
    ix = {n: i for i, n in enumerate(names)}

    def share(w: dict) -> float:
        nw = math.sqrt(sum(x * x for x in w.values()))
        return round(sum(sum(v[ix[k]] * x / nw for k, x in w.items()) ** 2 for v in good), 2)

    ident = sorted(n for n in names if share({n: 1}) >= IDENT_SHARE)
    axes = {}
    for a in model.AXES:
        if a in model.NO_READOUT:
            continue
        e, b, x = f"eta.{a}", f"beta.{a}", f"xc.{a}"
        axes[a] = {"eta": share({e: 1}), "beta": share({b: 1}), "ratio": share({e: 1, b: -1}),
                   "sum": share({e: 1, b: 1}), "xc": share({x: 1})}
    return {"n_recoverable": len(good), "noise_floor": round(floor, 1), "n_identified": len(ident),
            "identified": ident, "axes": axes}


def _sum(ms, n: float):
    m = len(ms[0])
    return [[n * sum(x[i][j] for x in ms) for j in range(m)] for i in range(m)]


def main() -> None:
    n_sim = int(sys.argv[1]) if len(sys.argv) > 1 else 2000
    names = full_names()
    cache = os.environ.get("LONG_CACHE")
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
    """Full repeated-measures study from two seeds' derivatives (main and shard.py --gather)."""
    per = {(r, dz): fisher(d1, d2, r, dz, 1.0) for r in ALL for dz in ("cross", "long")}
    out = {"n_sim": n_sim, "ages": AGES, "n_per_age": N_PER_AGE, "noise_rel": NOISE_REL, "peak": PEAK,
           "pattern_weights": d1["m0"]["w"],
           "readouts": {r: {"axis": v[1], "assay": v[2], "repeatable": v[3], "mean_informative": v[4]}
                        for r, v in REALISTIC.items()}}
    for label, group in (("realistic", REALISTIC), ("ideal", IDEAL)):
        for dz in ("cross", "long"):
            key = f"{label}_{dz}"
            single = {r: assess(_sum([per[(r, dz)]], N_PER_AGE), names) for r in group}
            out[key] = {
                "single": {r: {k: v for k, v in s.items() if k != "axes"} for r, s in
                           sorted(single.items(), key=lambda kv: -kv[1]["n_recoverable"])},
                "all_by_n": {n: assess(_sum([per[(r, dz)] for r in group], n), names) for n in N_SWEEP},
            }
    return out


if __name__ == "__main__":
    main()
