"""Shard / gather mode for the challenge cost grid on GitHub Actions (../../design/actions-job.md).

The eta/beta-split cost grid for the 10-27 cost-curve PR, sharded so it runs
at n_sim 20 000 on Actions. Kept apart from shard.py on purpose: shard.py is
pinned by hash in research/runs/sim-v0-joint-2026-10-20/manifest.yaml, so it
must stay byte-identical. Mapped route dropped (identifiability.md §9.4).

Two shard kinds (one cohort each), so no cohort is simulated twice:
    b,seed,ki,organ          baseline cohort (depends on kappa only)
    c,seed,ki,ti,di,organ    challenge cohort at (kappa, start age, dose)
Derivatives are stored per shard; gather builds the per-(kappa, t0, dose, organ)
share table (challenge Fisher added to the shared baseline Fisher) at n 10/30/100.
The design table (which ages/doses to combine, mice/sex) is built from this
output downstream, at no extra simulation.

    python3 shard_challenge.py --list
    python3 shard_challenge.py --shard c,11,0,1,2,A9_kidney --n-sim 20000 --out shards/
    python3 shard_challenge.py --gather shards/ --n-sim 20000 > result.json

Standard library only, no network, no data/.
"""

from __future__ import annotations

import argparse
import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))  # py/: model

import challenge  # noqa: E402
from shard import SEEDS  # noqa: E402


def challenge_specs() -> list[str]:
    """One spec per line. Baseline: b,seed,ki,organ. Challenge: c,seed,ki,ti,di,organ."""
    out = []
    for s in SEEDS:
        for ki in range(len(challenge.KAPPAS)):
            out += [f"b,{s},{ki},{a}" for a in challenge.organs()]
            for ti in range(len(challenge.T0S)):
                for di in range(len(challenge.DOSES)):
                    out += [f"c,{s},{ki},{ti},{di},{a}" for a in challenge.organs()]
    return out


def _cfname(spec: str) -> str:
    p = spec.split(",")
    kappa = challenge.KAPPAS[int(p[2])]
    k = "prior" if kappa is None else kappa
    if p[0] == "b":
        return f"cb_{p[1]}_k{k}_{p[3]}.json"
    t0, dose = challenge.T0S[int(p[3])], challenge.DOSES[int(p[4])]
    return f"cc_{p[1]}_k{k}_t{t0}_d{dose}_{p[5]}.json"


def _set(spec_parts: list[str]) -> str:
    """Point challenge globals at the spec's setting; return the organ."""
    ki = int(spec_parts[2])
    challenge.set_grid(challenge.KAPPAS[ki], challenge.T0S[0])
    if spec_parts[0] == "b":
        return spec_parts[3]
    challenge.set_grid(challenge.KAPPAS[ki], challenge.T0S[int(spec_parts[3])])
    challenge.set_dose(challenge.DOSES[int(spec_parts[4])])
    return spec_parts[5]


def challenge_simulate(spec: str, n_sim: int) -> dict:
    p = spec.split(",")
    if p[0] not in ("b", "c") or p[-1] not in challenge.organs():
        raise ValueError(f"bad challenge spec {spec!r}")
    organ, seed = _set(p), int(p[1])
    kind = "baseline" if p[0] == "b" else "challenge"
    names = challenge.names_for(organ, False)  # mapped route dropped: no free m
    d = challenge._derivs_json(challenge.derivs(kind, organ, names, n_sim, seed))
    return {"spec": spec, "n_sim": n_sim, "kind": kind, "derivs": d}


def challenge_gather(load, n_sim: int) -> dict:
    """Rebuild the per-(kappa, t0, dose, organ) share table from the shards."""
    runs = []
    for ki, kappa in enumerate(challenge.KAPPAS):
        challenge.set_grid(kappa, challenge.T0S[0])
        per_organ = {}
        for organ in challenge.organs():
            names = challenge.names_for(organ, False)
            base = [_restore(load(f"b,{s},{ki},{organ}")) for s in SEEDS]
            fb = challenge.fisher(base[0], base[1], 1.0)
            cells = {}
            for ti, t0 in enumerate(challenge.T0S):
                for di, dose in enumerate(challenge.DOSES):
                    ch = [_restore(load(f"c,{s},{ki},{ti},{di},{organ}")) for s in SEEDS]
                    fc = challenge.fisher(ch[0], ch[1], 1.0)
                    cells[f"t{t0}_d{dose}"] = _share_sweep(challenge._add(fb, fc), names)
            per_organ[organ] = {"baseline": _share_sweep(fb, names), "challenge": cells}
        runs.append({"kappa": kappa if kappa is not None else challenge._p0().kappa,
                     "per_organ": per_organ})
    return {"n_sim": n_sim, "seeds": SEEDS, "weeks": challenge.WEEKS, "doses_xc": challenge.DOSES,
            "t0s": challenge.T0S, "kappas": ["prior" if k is None else k for k in challenge.KAPPAS],
            "noise_rel": challenge.NOISE_REL, "n_sweep": challenge.N_SWEEP, "runs": runs}


def _restore(sh: dict) -> dict:
    assert sh["n_sim"], f"shard {sh.get('spec')} missing n_sim"
    dj = sh["derivs"]
    return {"m0": {float(t): tuple(v) for t, v in dj["m0"].items()},
            "d": [{float(t): tuple(v) for t, v in row.items()} for row in dj["d"]]}


def _share_sweep(f: list, names: list) -> dict:
    """shares() at each n in N_SWEEP for one Fisher matrix (per-mouse-point scaled by n)."""
    return {n: challenge.shares([[n * x for x in row] for row in f], names) for n in challenge.N_SWEEP}


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    g = ap.add_mutually_exclusive_group(required=True)
    g.add_argument("--list", action="store_true")
    g.add_argument("--shard", metavar="SPEC")
    g.add_argument("--gather", metavar="DIR")
    ap.add_argument("--n-sim", type=int, default=20000)
    ap.add_argument("--out", default=".")
    a = ap.parse_args()
    if a.list:
        print("\n".join(challenge_specs()))
    elif a.shard:
        os.makedirs(a.out, exist_ok=True)
        with open(os.path.join(a.out, _cfname(a.shard)), "w") as fh:
            json.dump(challenge_simulate(a.shard, a.n_sim), fh, sort_keys=True)
    else:
        def load(spec: str) -> dict:
            with open(os.path.join(a.gather, _cfname(spec))) as fh:
                return json.load(fh)
        json.dump(challenge_gather(load, a.n_sim), sys.stdout, indent=1)


if __name__ == "__main__":
    main()
