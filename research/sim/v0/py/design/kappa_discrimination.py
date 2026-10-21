"""Can a challenge design tell kappa 0.5 / 2 / 8 apart? (design/kappa-discrimination.md)

Pre-registered criterion (e4f530a). Per organ and design, linear (Gauss-Newton)
likelihood-ratio noncentrality per mouse between the true kappa and a wrong one,
the organ's other parameters (eta, beta, Xc, eps, c) refitted: readout-moment
difference between the two kappas, minus its weighted projection on the
nuisance derivatives at the truth, weighted like challenge.fisher. Power >= 0.8
for chi2(1) at alpha 0.05 <=> n x lambda >= 7.85. From the challenge shards only.

    python3 kappa_discrimination.py <shards_dir> > ../../results/kappa-discrimination.json

Standard library only, no network, no data/.
"""

from __future__ import annotations

import json
import math
import os
import sys

_PY = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path[:0] = [_PY, os.path.join(_PY, "studies"), os.path.dirname(os.path.abspath(__file__))]

import challenge  # noqa: E402
import shard_challenge as sc  # noqa: E402
from cost_curve import REPEATABLE, designs  # noqa: E402
from identifiability import jacobi_eigen  # noqa: E402

LAMBDA_80 = 7.85  # chi2(1) noncentrality for power 0.8 at alpha 0.05
CAP = 700  # mice per sex
KAPPA_IX = 3  # names_for: eta, beta, xc, kappa, eps, c


def _avg(pair: list[dict]) -> dict:
    """Seed-averaged moments and derivatives, times in sorted order."""
    ts = sorted(pair[0]["m0"])
    m0 = [tuple(0.5 * (pair[0]["m0"][t][i] + pair[1]["m0"][t][i]) for i in (0, 1)) for t in ts]
    d = [[tuple(0.5 * (pair[0]["d"][k][t][i] + pair[1]["d"][k][t][i]) for i in (0, 1)) for t in ts]
         for k in range(len(pair[0]["d"]))]
    return {"m0": m0, "d": d}


def _cohorts(load, ki: int, organ: str) -> dict:
    out = {"b": _avg([sc._restore(load(f"b,{s},{ki},{organ}")) for s in sc.SEEDS])}
    for ti in range(len(challenge.T0S)):
        for di in range(len(challenge.DOSES)):
            out[ti, di] = _avg([sc._restore(load(f"c,{s},{ki},{ti},{di},{organ}")) for s in sc.SEEDS])
    return out


def _lambda(truth: dict, wrong: dict, keys: list) -> float:
    """Per-mouse noncentrality: weighted residual of (wrong - truth) after refitting nuisances."""
    rows, delta, w = [], [], []
    for key in keys:
        t, f = truth[key], wrong[key]
        for j, (mt, mf) in enumerate(zip(t["m0"], f["m0"])):
            v = mt[1] * (1 + challenge.NOISE_REL**2)
            if v <= 0:
                continue
            for i_mom, wt in ((0, 1 / v), (1, 1 / (2 * v * v))):
                delta.append(mf[i_mom] - mt[i_mom])
                rows.append([t["d"][k][j][i_mom] for k in range(len(t["d"])) if k != KAPPA_IX])
                w.append(wt)
    p = len(rows[0])
    jtj = [[sum(w[r] * rows[r][a] * rows[r][b] for r in range(len(w))) for b in range(p)] for a in range(p)]
    jtd = [sum(w[r] * rows[r][a] * delta[r] for r in range(len(w))) for a in range(p)]
    lam, vecs = jacobi_eigen(jtj)
    tol = max(lam) * 1e-10
    beta = [0.0] * p
    for l, v in zip(lam, vecs):
        if l > tol:
            c = sum(v[a] * jtd[a] for a in range(p)) / l
            beta = [beta[a] + c * v[a] for a in range(p)]
    res = [delta[r] - sum(rows[r][a] * beta[a] for a in range(p)) for r in range(len(w))]
    return sum(w[r] * res[r] ** 2 for r in range(len(w)))


def _min_mice(lam: float, cohorts: int) -> int | None:
    if lam <= 0:
        return None
    m = math.ceil(LAMBDA_80 / lam) * (1 + cohorts)
    return m if m <= CAP else None


PAIRS = ((2, 0), (2, 1), (0, 2), (1, 2))  # (truth ki, wrong ki): criterion of the pre-registration


def run(load) -> dict:
    data = {}
    for ki, kappa in enumerate(challenge.KAPPAS):
        challenge.set_grid(kappa, challenge.T0S[0])
        data[ki] = {o: _cohorts(load, ki, o) for o in REPEATABLE}
    rows = []
    for ages, doses in designs():
        keys = ["b"] + [(t, d) for t in ages for d in doses]
        cohorts = len(keys) - 1
        per = {o: {f"{tr}->{wr}": _lambda(data[tr][o], data[wr][o], keys) for tr, wr in PAIRS} for o in REPEATABLE}
        pooled = {p: sum(per[o][p] for o in REPEATABLE) for p in per[REPEATABLE[0]]}
        need = [_min_mice(pooled[p], cohorts) for p in pooled]
        rows.append({"ages_mo": [round(12 * challenge.T0S[t]) for t in ages],
                     "doses_xc": [challenge.DOSES[d] for d in doses],
                     "lambda_per_mouse": {o: {p: round(x, 4) for p, x in v.items()} for o, v in per.items()},
                     "pooled": {p: round(x, 4) for p, x in pooled.items()},
                     "mice_per_sex_pooled": None if None in need else max(need),
                     "mice_per_sex_best_organ": min(
                         (m for o in REPEATABLE
                          if None not in (ms := [_min_mice(per[o][p], cohorts) for p in pooled])
                          for m in [max(ms)]), default=None)})
    ok = [r for r in rows if r["mice_per_sex_pooled"] is not None]
    best = min(ok, key=lambda r: r["mice_per_sex_pooled"]) if ok else None
    return {"prereg": "design/kappa-discrimination.md (e4f530a)", "lambda_80": LAMBDA_80, "cap": CAP,
            "kappas": ["prior" if k is None else k for k in challenge.KAPPAS],
            "pairs": [f"{t}->{w}" for t, w in PAIRS], "repeatable": REPEATABLE,
            "smallest_pooled": best, "rows": rows}


def main() -> None:
    d = sys.argv[1]

    def load(spec: str) -> dict:
        with open(os.path.join(d, sc._cfname(spec))) as fh:
            return json.load(fh)
    json.dump(run(load), sys.stdout, indent=1)


if __name__ == "__main__":
    main()
