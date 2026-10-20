"""Shard / gather mode for the joint job on GitHub Actions (actions-job.md).

One shard = one `model.simulate` call for (seed, parameter, sign). The same
simulation feeds both studies: survival bins and readout moments (readouts.py)
and per-mouse tracks (longitudinal.py). Gather rebuilds each study's
derivatives from the shards and runs its analysis unchanged.

    python3 shard.py --list                                  # one spec per line, for the job matrix
    python3 shard.py --shard 11,eta.A1_muscle,+ --n-sim 20000 --out shards/
    python3 shard.py --gather shards/ --n-sim 20000 > result.json

Spec `seed,param,sign`: sign is + or - (log step ±H) or 0 with param `base`.
Shards run to T_MAX for all studies, so longitudinal numbers differ from a
direct `longitudinal.py` run (which stops at 30 months: other random draws
after that age) by Monte-Carlo noise only. Readout numbers are identical.
Standard library only, no network, no data/.
"""

from __future__ import annotations

import argparse
import json
import math
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))  # py/: model

import model  # noqa: E402
import longitudinal  # noqa: E402
import readouts  # noqa: E402
from identifiability import H, T_MAX, full_names  # noqa: E402

SEEDS = (11, 12)


def specs() -> list[str]:
    out = []
    for s in SEEDS:
        out.append(f"{s},base,0")
        out += [f"{s},{nm},{sg}" for nm in full_names() for sg in "+-"]
    return out


def _fname(spec: str) -> str:
    seed, nm, sg = spec.split(",")
    return f"{seed}_{nm}_{ {'+': 'up', '-': 'dn', '0': 'base'}[sg] }.json".replace(" ", "")


def simulate(spec: str, n_sim: int) -> dict:
    seed, nm, sg = spec.split(",")
    if (nm == "base") != (sg == "0") or sg not in "+-0" or (nm != "base" and nm not in full_names()):
        raise ValueError(f"bad spec {spec!r}")
    mult = {} if sg == "0" else {nm: math.exp(H if sg == "+" else -H)}
    ages = sorted(set(readouts.AGES) | set(longitudinal.AGES))
    rec = {a: [] for a in readouts.AGES}
    track = {a: {} for a in ages}
    pert = model.Perturbation(mult) if mult else None
    ls = model.simulate(n_sim, int(seed), perturbation=pert, t_max=T_MAX, record=rec, track=track)
    return {"spec": spec, "n_sim": n_sim, "readouts": readouts.summarise(ls, rec),
            "longitudinal": longitudinal.summarise({a: track[a] for a in longitudinal.AGES})}


def _keys_to_num(x):
    """JSON turns float/int dict keys into strings; restore them."""
    if isinstance(x, dict):
        out = {}
        for k, v in x.items():
            try:
                k = int(k) if k.isdigit() else float(k)
            except (AttributeError, ValueError):
                pass
            out[k] = _keys_to_num(v)
        return out
    if isinstance(x, list):
        return [_keys_to_num(v) for v in x]
    return x


def _ro(s: dict) -> tuple:
    p, m = s["readouts"]
    return p, {a: {r: tuple(v) for r, v in ms.items()} for a, ms in m.items()}


def gather(load, n_sim: int) -> dict:
    """`load(spec)` returns a shard dict (already key-restored)."""
    names = full_names()
    ro, lo = [], []
    for s in SEEDS:
        base = load(f"{s},base,0")
        ups = [load(f"{s},{nm},+") for nm in names]
        dns = [load(f"{s},{nm},-") for nm in names]
        for x in [base] + ups + dns:
            assert x["n_sim"] == n_sim, f"shard {x['spec']} has n_sim {x['n_sim']}"
        ro.append(readouts.derive(_ro(base), [_ro(u) for u in ups], [_ro(d) for d in dns]))
        lo.append(longitudinal.derive(base["longitudinal"], [u["longitudinal"] for u in ups],
                                      [d["longitudinal"] for d in dns]))
    return {"n_sim": n_sim, "seeds": SEEDS,
            "readouts": readouts.analyse(ro[0], ro[1], n_sim, names),
            "longitudinal": longitudinal.analyse(lo[0], lo[1], n_sim, names)}


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    g = ap.add_mutually_exclusive_group(required=True)
    g.add_argument("--list", action="store_true")
    g.add_argument("--shard", metavar="SEED,PARAM,SIGN")
    g.add_argument("--gather", metavar="DIR")
    ap.add_argument("--n-sim", type=int, default=20000)
    ap.add_argument("--out", default=".")
    a = ap.parse_args()
    if a.list:
        print("\n".join(specs()))
    elif a.shard:
        os.makedirs(a.out, exist_ok=True)
        with open(os.path.join(a.out, _fname(a.shard)), "w") as fh:
            json.dump(simulate(a.shard, a.n_sim), fh, sort_keys=True)
    else:
        def load(spec: str) -> dict:
            with open(os.path.join(a.gather, _fname(spec))) as fh:
                return _keys_to_num(json.load(fh))
        json.dump(gather(load, a.n_sim), sys.stdout, indent=1)


if __name__ == "__main__":
    main()
