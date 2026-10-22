"""Real-coverage identifiability of the calibration parameters (calibration.md §7).

Question: with the readouts that actually exist (A9 coverage screen 26336bb, as
encoded in design/public_design.py: DRiDO females, MPD n7 to 24 mo), how well are
the two reduced-v0 calibration parameters per sex (u_eta, u_xc: shared
log-multipliers on eta and Xc of the VITAL axes) pinned, alone and on top of
control survival? Coverage metadata only; no readout or survival value is read.

- Survival: multinomial Fisher of the monthly bins of fit.sim_bins, per mouse,
  times n controls per sex (no censoring), cross-seed derivatives (seeds 11, 12).
- Readouts: public_design.fisher on the same two directions (mean + variance,
  assay noise NOISE_REL), designs female / male / ideal_6_4.
- Reported: SE of u_eta, u_xc, their correlation, and the ridge (eigenvalue
  ratio) for survival alone, readouts alone, and both (Fisher information adds:
  independent mice).

    python3 coverage.py [n_sim_surv] [n_sim_readout] > ../../results/calibration-coverage.json
"""

from __future__ import annotations

import json
import math
import os
import sys

_HERE = os.path.dirname(os.path.abspath(__file__))
_PY = os.path.dirname(_HERE)
sys.path[:0] = [_HERE, os.path.join(_PY, "design"), os.path.join(_PY, "studies")]

import fit as F  # noqa: E402
import public_design as PD  # noqa: E402
from fit import model  # noqa: E402

H = 0.1
SEEDS = (11, 12)
N_CONTROLS = (1000, 300)  # per sex: about ITP controls over 3 sites, and a third of it
PARAMS = ("u_eta", "u_xc")


def shared_mult(u_eta: float, u_xc: float) -> dict:
    return {**{f"eta.{a}": math.exp(u_eta) for a in model.VITAL},
            **{f"xc.{a}": math.exp(u_xc) for a in model.VITAL}}


def _dirs():
    return [(H, 0.0), (0.0, H)]


def survival_derivs(n_sim: int, seed: int) -> tuple[list[float], list[list[float]]]:
    p0 = F.sim_bins(0.0, 0.0, n_sim, seed)
    d = []
    for du in _dirs():
        up = F.sim_bins(du[0], du[1], n_sim, seed)
        dn = F.sim_bins(-du[0], -du[1], n_sim, seed)
        d.append([(a - b) / (2 * H) for a, b in zip(up, dn)])
    return p0, d


def survival_fisher(s1, s2, n: int) -> list[list[float]]:
    """Per-mouse multinomial Fisher sum_k dp_i dp_j / p_k, cross-seed, times n."""
    (p1, d1), (p2, d2) = s1, s2
    f = [[0.0, 0.0], [0.0, 0.0]]
    for k in range(len(p1)):
        p = 0.5 * (p1[k] + p2[k])
        for i in range(2):
            for j in range(2):
                f[i][j] += n * 0.5 * (d1[i][k] * d2[j][k] + d2[i][k] * d1[j][k]) / p
    return f


def readout_derivs(n_sim: int, seed: int) -> dict:
    """Same shape as public_design.derivatives, for the two shared directions."""
    base = PD.run({}, n_sim, seed)
    d = []
    for du in _dirs():
        up = PD.run(shared_mult(*du), n_sim, seed)
        dn = PD.run(shared_mult(-du[0], -du[1]), n_sim, seed)
        d.append({m: {a: tuple((up[m][a][i] - dn[m][a][i]) / (2 * H) for i in (0, 1)) for a in PD.AXES}
                  for m in PD.AGES_MO})
    return {"m0": base, "d": d}


def summary(f: list[list[float]]) -> dict:
    """SEs, correlation and ridge of a 2x2 Fisher matrix (None where singular)."""
    a, b, d = f[0][0], 0.5 * (f[0][1] + f[1][0]), f[1][1]
    det = a * d - b * b
    tr = a + d
    disc = math.sqrt(max(tr * tr / 4 - det, 0.0))
    lo, hi = tr / 2 - disc, tr / 2 + disc
    out = {"fisher": [[a, b], [b, d]], "eig": [lo, hi], "ridge": (hi / lo) if lo > 0 else None}
    if det <= 0 or lo <= 0:
        out.update({"se": None, "corr": None, "pinned": []})
        return out
    se = [math.sqrt(d / det), math.sqrt(a / det)]
    out.update({"se": dict(zip(PARAMS, se)), "corr": -b / math.sqrt(a * d),
                "pinned": [p for p, s in zip(PARAMS, se) if s < 0.10]})
    return out


def add(*fs):
    return [[sum(f[i][j] for f in fs) for j in range(2)] for i in range(2)]


def main(argv: list[str]) -> None:
    n_surv = int(argv[0]) if argv else F.N_SIM
    n_read = int(argv[1]) if len(argv) > 1 else 1000
    s1, s2 = (survival_derivs(n_surv, s) for s in SEEDS)
    r1, r2 = (readout_derivs(n_read, s) for s in SEEDS)
    res = {}
    for label, design in PD.DESIGNS.items():
        fr = PD.fisher(r1, r2, design)
        row = {"readouts_only": summary(fr)}
        for n in N_CONTROLS:
            fs = survival_fisher(s1, s2, n)
            row[f"survival_n{n}"] = summary(fs)
            row[f"survival_n{n}+readouts"] = summary(add(fs, fr))
        res[label] = row
    json.dump({"n_sim_survival": n_surv, "n_sim_readout": n_read, "seeds": SEEDS, "h": H,
               "noise_rel": PD.NOISE_REL, "coverage_source": "A9 coverage 26336bb (public_design.py)",
               "pinned_rule": "SE < 0.10 on the log scale", "designs": res},
              sys.stdout, indent=1)


if __name__ == "__main__":
    main(sys.argv[1:])
