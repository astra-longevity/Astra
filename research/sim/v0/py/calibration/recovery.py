"""Synthetic recovery grid, design/calibration.md §6 (pre-registered af2413e).

Truths (u_eta, u_xc): (log 1.2, log 0.9) and (0, 0); data seeds 101-103;
n per sex 1000 and 300; censoring none or administrative at 3.5 years.
Pass: |error| < 0.10 (n 1000) / < 0.20 (n 300) on each u in every seed.
(The bootstrap-coverage part of §6 runs separately on Actions.)

Heavy at the pre-registered n_sim 4000: run on GitHub Actions, one cell per job.

    python3 recovery.py --list                      # cell specs
    python3 recovery.py --cell truth,n,cens,seed    # one cell -> JSON on stdout
    python3 recovery.py --gather DIR                # cell JSONs -> verdict JSON
    python3 recovery.py --quick                     # tiny local smoke (n_sim 400)
"""

from __future__ import annotations

import glob
import json
import math
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import fit as F  # noqa: E402
from fit import baseline, model  # noqa: E402

TRUTHS = {"shifted": (math.log(1.2), math.log(0.9)), "null": (0.0, 0.0)}
SIZES = (1000, 300)
CENSOR = {"none": None, "admin3.5": 3.5}
SEEDS = (101, 102, 103)
TOL = {1000: 0.10, 300: 0.20}


def cells() -> list[str]:
    return [f"{t},{n},{c},{s}" for t in TRUTHS for n in SIZES for c in CENSOR for s in SEEDS]


def synthetic_arm(truth: tuple, n: int, censor, seed: int) -> baseline.Arm:
    mult = {**{f"eta.{a}": math.exp(truth[0]) for a in model.VITAL},
            **{f"xc.{a}": math.exp(truth[1]) for a in model.VITAL}}
    life = model.simulate(n, seed, perturbation=model.Perturbation(mult), t_max=F.T_MAX)
    if censor is None:
        return baseline.Arm(tuple(life), tuple([1] * n), "control", "train")
    return baseline.Arm(tuple(min(x, censor) for x in life),
                        tuple(int(x < censor) for x in life), "control", "train")


def run_cell(spec: str, n_sim: int = F.N_SIM, stability_seeds=F.STABILITY_SEEDS) -> dict:
    t, n, c, s = spec.split(",")
    truth, n, seed = TRUTHS[t], int(n), int(s)
    a = synthetic_arm(truth, n, CENSOR[c], seed)
    r = F.fit([a], n_sim=n_sim, stability_seeds=stability_seeds)
    est = r["estimate"]
    err = {"u_eta": est["u_eta"] - truth[0], "u_xc": est["u_xc"] - truth[1]}
    return {"cell": spec, "truth": truth, "estimate": est, "error": err,
            "recovered": all(abs(v) < TOL[n] for v in err.values()),
            "convergence": r["convergence"], "n_sim": n_sim}


def gather(d: str) -> dict:
    res = [json.load(open(p)) for p in sorted(glob.glob(os.path.join(d, "*.json")))]
    missing = sorted(set(cells()) - {r["cell"] for r in res})
    return {"cells": len(res), "missing": missing,
            "all_recovered": not missing and all(r["recovered"] for r in res),
            "failed": [r["cell"] for r in res if not r["recovered"]]}


def main(argv: list[str]) -> None:
    if argv[:1] == ["--list"]:
        print("\n".join(cells()))
    elif argv[:1] == ["--cell"]:
        print(json.dumps(run_cell(argv[1]), indent=1))
    elif argv[:1] == ["--gather"]:
        print(json.dumps(gather(argv[1]), indent=1))
    elif argv[:1] == ["--quick"]:
        print(json.dumps(run_cell("shifted,1000,none,101", n_sim=400, stability_seeds=()), indent=1))
    else:
        sys.exit(__doc__)


if __name__ == "__main__":
    main(sys.argv[1:])
