"""Synthetic recovery grid, design/calibration.md §6 (pre-registered af2413e).

Truths (u_eta, u_xc): (log 1.2, log 0.9) and (0, 0); data seeds 101-103;
n per sex 1000 and 300; censoring none or administrative at 3.5 years.
Pass: |error| < 0.10 (n 1000) / < 0.20 (n 300) on each u in every seed.
(The bootstrap-coverage part of §6 runs separately on Actions.)

Heavy at the pre-registered n_sim 4000: run on GitHub Actions, one cell per job.

    python3 recovery.py --list                      # cell specs
    python3 recovery.py --cell truth,n,cens,seed    # one cell -> JSON on stdout
    python3 recovery.py --bootstrap-list            # 20 shard specs (v2): <cell>|<first rep>
    python3 recovery.py --bootstrap-shard SPEC      # 10 bootstrap reps of one cell -> JSON
    python3 recovery.py --gather DIR [BOOT_DIR]     # cell (+ shard) JSONs -> verdict JSON
    python3 recovery.py --pin                       # pinned small cell for byte-exact re-run
    python3 recovery.py --quick                     # tiny local smoke (n_sim 400)

Bootstrap shards and --pin are execution splits; the pre-registered grid is unchanged.
"""

from __future__ import annotations

import glob
import json
import math
import os
import random
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import fit as F  # noqa: E402
from fit import baseline, model  # noqa: E402

TRUTHS = {"shifted": (math.log(1.2), math.log(0.9)), "null": (0.0, 0.0)}
SIZES = (1000, 300)
CENSOR = {"none": None, "admin3.5": 3.5}
SEEDS = (101, 102, 103)
TOL = {1000: 0.10, 300: 0.20}
# v2 (calibration.md §3, A28 G173): bootstrap only the two quoted design-point
# cells, 150 reps each (~150 runner-hours), for the reported SE/interval. The
# 24-cell point-recovery grid (cells()) is unchanged.
BOOT_CELLS = ("shifted,1000,none,101", "shifted,300,none,101")
BOOT_REPS = 150
BOOT_SHARD = 15
BOOT_SEED_BASE = 10000
PIN_CELL = "shifted,300,none,101"
PIN_N_SIM = 200


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


def bootstrap_shards() -> list[str]:
    """Execution split only (v2): BOOT_REPS per BOOT_CELL in shards of BOOT_SHARD -> '<cell>|<first rep>'."""
    return [f"{c}|{r}" for c in BOOT_CELLS for r in range(0, BOOT_REPS, BOOT_SHARD)]


def resample(a: baseline.Arm, rep: int, cell_seed: int) -> baseline.Arm:
    """Nonparametric bootstrap over mice; seed fixed by (cell seed, rep) so any shard re-runs exactly."""
    rng = random.Random(BOOT_SEED_BASE * cell_seed + rep)
    idx = [rng.randrange(len(a.times)) for _ in a.times]
    return baseline.Arm(tuple(a.times[i] for i in idx), tuple(a.events[i] for i in idx), a.role, a.split)


def run_bootstrap_shard(spec: str, n_sim: int = F.N_SIM) -> dict:
    """One shard: BOOT_SHARD reps, each a single-start refit from (0, 0), no MC-stability refits.

    The 3 starts and the MC stability are checked once per cell (--cell); a
    bootstrap replicate only needs the estimate.
    """
    cell, first = spec.split("|")
    t, n, c, s = cell.split(",")
    a = synthetic_arm(TRUTHS[t], int(n), CENSOR[c], int(s))
    reps = []
    for rep in range(int(first), min(int(first) + BOOT_SHARD, BOOT_REPS)):
        times, events = F._pool([resample(a, rep, int(s))])
        r = F._fit_once(times, events, F.STARTS[0], n_sim, F.SIM_SEED, 80)
        reps.append({"rep": rep, "u_eta": r["u_eta"], "u_xc": r["u_xc"]})
    return {"cell": cell, "first": int(first), "reps": reps, "n_sim": n_sim}


def _percentile(v: list[float], q: float) -> float:
    v = sorted(v)
    k = q * (len(v) - 1)
    lo = int(k)
    return v[lo] + (v[min(lo + 1, len(v) - 1)] - v[lo]) * (k - lo)


def _mean_sd(v: list[float]) -> tuple[float, float]:
    m = sum(v) / len(v)
    return m, math.sqrt(sum((x - m) ** 2 for x in v) / (len(v) - 1)) if len(v) > 1 else 0.0


def bootstrap_summary(shards: list[dict]) -> dict:
    """v2: per BOOT_CELL, the 95 % percentile interval, SE and point of each u.

    SE = bootstrap standard deviation of the replicate estimates (calibration.md
    §3). No cross-seed grouping: the bootstrap is reported only for the quoted
    design points; recovery across seeds/censoring is the grid's job (§6).
    """
    by_cell: dict = {}
    for sh in shards:
        by_cell.setdefault(sh["cell"], []).extend(sh["reps"])
    out = {}
    for cell, reps in sorted(by_cell.items()):
        truth = TRUTHS[cell.split(",")[0]]
        per = {}
        for k, tv in zip(("u_eta", "u_xc"), truth):
            vals = [r[k] for r in reps]
            mean, sd = _mean_sd(vals)
            lo, hi = _percentile(vals, 0.025), _percentile(vals, 0.975)
            per[k] = {"mean": mean, "se": sd, "ci95": [lo, hi], "covers_truth": lo <= tv <= hi}
        out[cell] = {"reps": len(reps), "complete": len(reps) == BOOT_REPS, "truth": truth, "params": per}
    return {"cells": out,
            "all_complete": set(out) == set(BOOT_CELLS) and all(c["complete"] for c in out.values())}


def gather(d: str, boot_dir: str | None = None) -> dict:
    res = [json.load(open(p)) for p in sorted(glob.glob(os.path.join(d, "*.json")))]
    missing = sorted(set(cells()) - {r["cell"] for r in res})
    out = {"cells": len(res), "missing": missing,
           "all_recovered": not missing and all(r["recovered"] for r in res),
           "failed": [r["cell"] for r in res if not r["recovered"]]}
    if boot_dir:
        shards = [json.load(open(p)) for p in sorted(glob.glob(os.path.join(boot_dir, "*.json")))]
        bs = bootstrap_summary(shards)
        out["bootstrap"] = bs
        out["recovered_v2"] = out["all_recovered"] and bs["all_complete"]
    return out


def main(argv: list[str]) -> None:
    if argv[:1] == ["--list"]:
        print("\n".join(cells()))
    elif argv[:1] == ["--cell"]:
        print(json.dumps(run_cell(argv[1]), indent=1))
    elif argv[:1] == ["--bootstrap-list"]:
        print("\n".join(bootstrap_shards()))
    elif argv[:1] == ["--bootstrap-shard"]:
        print(json.dumps(run_bootstrap_shard(argv[1]), indent=1))
    elif argv[:1] == ["--gather"]:
        print(json.dumps(gather(argv[1], argv[2] if len(argv) > 2 else None), indent=1))
    elif argv[:1] == ["--pin"]:  # small byte-exact re-run for the Runs check (not a §6 cell)
        print(json.dumps(run_cell(PIN_CELL, n_sim=PIN_N_SIM, stability_seeds=()), indent=1))
    elif argv[:1] == ["--quick"]:
        print(json.dumps(run_cell("shifted,1000,none,101", n_sim=400, stability_seeds=()), indent=1))
    else:
        sys.exit(__doc__)


if __name__ == "__main__":
    main(sys.argv[1:])
