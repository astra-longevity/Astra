"""Readout identifiability on the real public design (A9 screen, 26336bb).

identifiability.md §6 assumed an ideal design: every organ readout, n per age
per sex at 12/18/24/30 months. A9's screen
(data/datasets/coverage/rsim-v0-readouts.md) found what public data actually
has: DRiDO (females only, Diversity Outbred, ages off the grid) and three MPD
projects (both sexes, n ~7, to 24 months). This script scores that design.

- Model has no sex-specific parameters: the same simulation is scored against
  each sex's design (parameters are fitted per sex).
- Ideal assay (raw organ state) with assay noise NOISE_REL x biological SD, as
  in readouts.py; longitudinal sources are scored as cross-sectional (§8: the
  repeated design adds no direction).
- Cross-seed Fisher (seeds 11, 12) of readout mean and variance; survival not
  included (its Monte-Carlo floor masks readouts at local n_sim, I9).

    python3 public_design.py [n_sim] > ../../results/public-design.json
"""

from __future__ import annotations

import json
import math
import os
import pickle
import sys

_PY = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path[:0] = [_PY, os.path.join(_PY, "studies")]  # py/: model; py/studies/: study modules

import model  # noqa: E402
from identifiability import H, full_names  # noqa: E402
from readouts import NOISE_REL, assess, axis_shares  # noqa: E402

SEEDS = (11, 12)

# source -> (organ axis, {age in months: n per sex})
_DRIDO_ECHO = {10: 182, 22: 123, 34: 17}
FEMALE = {
    "DRiDO echo": ("A6_heart", _DRIDO_ECHO),
    "DRiDO flow": ("A11_immune", {5: 187, 16: 166, 28: 72}),
    "MPD Petkova1 flow": ("A11_immune", {6: 7, 12: 7, 18: 7, 24: 7}),
    "DRiDO CBC": ("A12_blood", {11: 182, 23: 113, 35: 14}),
    "MPD Peters4 CBC": ("A12_blood", {6: 7, 12: 7, 18: 7, 24: 7}),
    "MPD Korstanje1 albuminuria": ("A9_kidney", {12: 7, 18: 7, 24: 7}),
    "DRiDO fasting glucose": ("A10_liver_metabolism", {5: 185, 16: 164, 28: 68}),
    "DRiDO DEXA fat/lean": ("A15_fat", dict(_DRIDO_ECHO)),  # n assumed = echo [to verify]
}
MALE = {k: v for k, v in FEMALE.items() if k.startswith("MPD")}
IDEAL = {f"ideal {a}": (a, {12: 10, 18: 10, 24: 10, 30: 10})
         for a in model.AXES if a not in model.NO_READOUT}  # §6.4 need, n 10
DESIGNS = {"female": FEMALE, "male": MALE, "ideal_6_4": IDEAL}

AGES_MO = sorted({m for d in DESIGNS.values() for _, ages in d.values() for m in ages})
AXES = sorted({a for d in DESIGNS.values() for a, _ in d.values()})


def _moments(states: list[dict]) -> dict:
    out = {}
    for a in AXES:
        ys = [x[a] for x in states]
        n = len(ys)
        m = sum(ys) / n if n else 0.0
        out[a] = (m, sum((y - m) ** 2 for y in ys) / (n - 1) if n > 1 else 0.0)
    return out


def run(mult: dict, n_sim: int, seed: int) -> dict:
    rec = {m / 12: [] for m in AGES_MO}
    pert = model.Perturbation(mult) if mult else None
    model.simulate(n_sim, seed, perturbation=pert, t_max=max(AGES_MO) / 12 + 0.05, record=rec)
    return {m: _moments(rec[m / 12]) for m in AGES_MO}


def derivatives(names: list[str], n_sim: int, seed: int) -> dict:
    base = run({}, n_sim, seed)
    d = []
    for nm in names:
        up, dn = run({nm: math.exp(H)}, n_sim, seed), run({nm: math.exp(-H)}, n_sim, seed)
        d.append({m: {a: tuple((up[m][a][i] - dn[m][a][i]) / (2 * H) for i in (0, 1)) for a in AXES}
                  for m in AGES_MO})
    return {"m0": base, "d": d}


def fisher(d1: dict, d2: dict, design: dict, n_scale: float = 1.0) -> list[list[float]]:
    k = len(d1["d"])
    f = [[0.0] * k for _ in range(k)]
    for a, ages in design.values():
        for m, n in ages.items():
            v_bio = 0.5 * (d1["m0"][m][a][1] + d2["m0"][m][a][1])
            if v_bio <= 0:
                continue
            v = v_bio * (1 + NOISE_REL**2)
            for i_mom, w in ((0, n * n_scale / v), (1, n * n_scale / (2 * v * v))):
                u1 = [c[m][a][i_mom] for c in d1["d"]]
                u2 = [c[m][a][i_mom] for c in d2["d"]]
                for i in range(k):
                    for j in range(k):
                        f[i][j] += w * 0.5 * (u1[i] * u2[j] + u2[i] * u1[j])
    return f


def analyse(d1: dict, d2: dict, names: list[str]) -> dict:
    out = {}
    for label, design in DESIGNS.items():
        f = fisher(d1, d2, design)
        sh = axis_shares(f, names)
        read = sorted({a for a, _ in design.values()})
        out[label] = {
            "sources": {s: {"axis": a, "ages_n": ages} for s, (a, ages) in design.items()},
            "assess": assess(f, names),
            "balance_pinned": sorted(a for a in read if sh[a]["ratio"] >= 0.8),
            "axis_shares": {a: sh[a] for a in read},
        }
    return out


def main() -> None:
    n_sim = int(sys.argv[1]) if len(sys.argv) > 1 else 1000
    names = full_names()
    cache = os.environ.get("PUBLIC_CACHE")
    if cache and os.path.exists(cache):
        with open(cache, "rb") as fh:
            cached_n, d1, d2 = pickle.load(fh)
        assert cached_n == n_sim, "cache built with another n_sim"
    else:
        d1, d2 = (derivatives(names, n_sim, s) for s in SEEDS)
        if cache:
            with open(cache, "wb") as fh:
                pickle.dump((n_sim, d1, d2), fh)
    json.dump({"n_sim": n_sim, "seeds": SEEDS, "noise_rel": NOISE_REL, "source": "A9 coverage 26336bb",
               "designs": analyse(d1, d2, names)}, sys.stdout, indent=1, default=str)


if __name__ == "__main__":
    main()
