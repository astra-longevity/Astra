"""Run the v0 skeleton: simulate N untreated mice with a seed, write survival + manifest.

    python3 run.py --n 500 --seed 1 --out /tmp/sim-v0

Writes survival.csv (t_years, S) and manifest.json (commit, seed, parameters,
Python version, output hash). The manifest format will follow A10's P2 run
manifests (research/runs/) once they land; this is the minimum T2-A10-1 asks.
"""

from __future__ import annotations

import argparse
import dataclasses
import hashlib
import json
import pathlib
import platform
import subprocess

import model


def _commit() -> str:
    try:
        out = subprocess.run(["git", "rev-parse", "HEAD"], capture_output=True, text=True, check=True)
        dirty = subprocess.run(["git", "status", "--porcelain"], capture_output=True, text=True, check=True)
        return out.stdout.strip() + ("-dirty" if dirty.stdout.strip() else "")
    except (OSError, subprocess.CalledProcessError):
        return "unknown"


def main(argv: list[str] | None = None) -> pathlib.Path:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--n", type=int, default=500)
    ap.add_argument("--seed", type=int, required=True)
    ap.add_argument("--out", type=pathlib.Path, required=True)
    args = ap.parse_args(argv)

    params = model.Params()
    lifespans = model.simulate(args.n, args.seed, params)
    curve = model.survival_curve(lifespans)

    args.out.mkdir(parents=True, exist_ok=True)
    csv = "t_years,survival\n" + "".join(f"{t:.6f},{s:.6f}\n" for t, s in curve)
    (args.out / "survival.csv").write_text(csv)
    manifest = {
        "model": "research/sim/v0",
        "commit": _commit(),
        "seed": args.seed,
        "n": args.n,
        "params": dataclasses.asdict(params),
        "free_parameters": params.free_parameter_count(),
        "python": platform.python_version(),
        "outputs": {"survival.csv": hashlib.sha256(csv.encode()).hexdigest()},
        "calibrated": False,
    }
    (args.out / "manifest.json").write_text(json.dumps(manifest, indent=2, sort_keys=True) + "\n")
    return args.out


if __name__ == "__main__":
    print(main())
