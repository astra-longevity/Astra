"""Cost curve of the acute-challenge design (identifiability.md §11), from shards only.

Reads the 462 shards of the challenge Actions run (studies/shard_challenge.py)
and, per kappa and organ, adds the baseline Fisher to the challenge Fisher of
every (start age, dose) cohort in a design. Reports, at n mice per time point,
which organs split eta/beta (share on eta + beta >= 0.8, §9.1) and which pin Xc
(share on xc >= 0.8). No new simulation: the derivatives are the shards'.

Design = set of start ages x set of doses. Mice per sex = n x (1 baseline
cohort + one cohort per (age, dose)): a repeatable readout re-reads the same n
mice at all post times. The Fisher treats those times as independent draws, so
within-mouse correlation is ignored (optimistic; limit I19).

    python3 cost_curve.py <shards_dir> > ../../results/cost-curve.json

Standard library only, no network, no data/.
"""

from __future__ import annotations

import itertools
import json
import os
import sys

_PY = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path[:0] = [_PY, os.path.join(_PY, "studies")]  # py/: model; py/studies/: study modules

import challenge  # noqa: E402
import shard_challenge as sc  # noqa: E402

SPLIT = 0.8
# Readout repeatable in the same mouse (identifiability.md §9.5): heart echo,
# kidney tGFR, blood CBC, gut weight/score. Others terminal or [to verify].
REPEATABLE = ("A6_heart", "A9_kidney", "A12_blood", "A14_gut")


def designs() -> list[tuple[tuple[int, ...], tuple[int, ...]]]:
    ages = [(0,), (1,), (0, 1)]
    doses = [c for k in (1, 2, 3) for c in itertools.combinations(range(len(challenge.DOSES)), k)]
    return [(a, d) for a in ages for d in doses]


def _fishers(load, ki: int) -> dict:
    out = {}
    for organ in challenge.organs():
        names = challenge.names_for(organ, False)
        b = [sc._restore(load(f"b,{s},{ki},{organ}")) for s in sc.SEEDS]
        fc = {}
        for ti in range(len(challenge.T0S)):
            for di in range(len(challenge.DOSES)):
                c = [sc._restore(load(f"c,{s},{ki},{ti},{di},{organ}")) for s in sc.SEEDS]
                fc[ti, di] = challenge.fisher(c[0], c[1], 1.0)
        out[organ] = (names, challenge.fisher(b[0], b[1], 1.0), fc)
    return out


def run(load) -> dict:
    out = []
    for ki, kappa in enumerate(challenge.KAPPAS):
        challenge.set_grid(kappa, challenge.T0S[0])
        fis = _fishers(load, ki)
        rows = []
        for ages, doses in designs():
            for n in challenge.N_SWEEP:
                split, xc, sums = [], [], {}
                for organ, (names, fb, fc) in fis.items():
                    f = challenge._add(fb, *[fc[t, d] for t in ages for d in doses])
                    s = challenge.shares([[n * x for x in r] for r in f], names)
                    sums[organ] = s["sum"]
                    if s["sum"] >= SPLIT:
                        split.append(organ)
                    if s["xc"] >= SPLIT:
                        xc.append(organ)
                rows.append({"ages_mo": [round(12 * challenge.T0S[t]) for t in ages],
                             "doses_xc": [challenge.DOSES[d] for d in doses], "n": n,
                             "mice_per_sex": n * (1 + len(ages) * len(doses)),
                             "split": split, "split_repeatable": [o for o in split if o in REPEATABLE],
                             "xc_pinned": xc, "sum_share": sums})
        out.append({"kappa": "prior" if kappa is None else kappa, "rows": rows})
    return {"split_threshold": SPLIT, "repeatable": REPEATABLE, "n_sweep": challenge.N_SWEEP,
            "mice_per_sex": "n x (1 baseline + one cohort per (age, dose))", "kappas": out}


def main() -> None:
    d = sys.argv[1]

    def load(spec: str) -> dict:
        with open(os.path.join(d, sc._cfname(spec))) as fh:
            return json.load(fh)
    json.dump(run(load), sys.stdout, indent=1)


if __name__ == "__main__":
    main()
