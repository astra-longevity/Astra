"""Shard / gather mode for the joint job on GitHub Actions (../../design/actions-job.md).

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
import challenge  # noqa: E402
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


# ----------------------------------------------------------------------------
# Challenge study (--study challenge): the eta/beta-split cost grid for A28's
# 10-27 PR, sharded so A61 can run it at 20k on Actions. Mapped route dropped.
# Two shard kinds (one cohort each), so no cohort is simulated twice:
#   b,seed,ki,organ            baseline cohort (depends on kappa only)
#   c,seed,ki,ti,di,organ      challenge cohort at (kappa, start age, dose)
# derivs are stored per shard; gather builds the per-(kappa,t0,dose,organ) share
# table (challenge Fisher added to the shared baseline Fisher) at n 10/30/100.
# The design table (which ages/doses to combine, mice/sex) is built from this
# output downstream, costing no extra simulation.
# ----------------------------------------------------------------------------

CKINDS = {"baseline": "baseline", "challenge": "challenge"}


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
    """Point challenge globals at the spec's setting; return the organ and seed's cohort kind."""
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


def _main_challenge(a) -> None:
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


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    g = ap.add_mutually_exclusive_group(required=True)
    g.add_argument("--list", action="store_true")
    g.add_argument("--shard", metavar="SEED,PARAM,SIGN")
    g.add_argument("--gather", metavar="DIR")
    ap.add_argument("--study", choices=("joint", "challenge"), default="joint",
                    help="joint = survival+readouts+longitudinal (default); challenge = eta/beta split grid")
    ap.add_argument("--n-sim", type=int, default=20000)
    ap.add_argument("--out", default=".")
    a = ap.parse_args()
    if a.study == "challenge":
        _main_challenge(a)
        return
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
