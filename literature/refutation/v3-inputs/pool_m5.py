#!/usr/bin/env python3
"""N2 (A2 / A66 record #233): M-005 single H8 input via NA4.2, computed, not typed.

Runs the committed NA4.2 functions (research/sim/v0/py/design/pool_panels.py,
merged ddf699f) UNCHANGED on the panels both readers agreed on (record #230).
Panel values come from the committed reading (m5-reading/coords.json via
m5-reading/convert.py), never from a hand-typed number.

Per panel: log r = log(ratio), SE(log r) = se_from_interval(ci95, t) with the
same t(0.975, n-1) used by rule 8. Then pool() -> carry(). Output: pool-output.json.

    python3 literature/refutation/v3-inputs/pool_m5.py
"""
import hashlib
import importlib.util
import json
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, "..", "..", ".."))
POOL = os.path.join(REPO, "research", "sim", "v0", "py", "design", "pool_panels.py")
READ = os.path.join(REPO, "literature", "refutation", "m5-reading")


def load(path, name):
    spec = importlib.util.spec_from_file_location(name, path)
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def sha(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest()


def main():
    pp = load(POOL, "pool_panels")
    cv = load(os.path.join(READ, "convert.py"), "convert")
    coords_path = os.path.join(READ, "coords.json")
    d = json.load(open(coords_path))
    used = []
    for name, p in d["panels"].items():
        if not p.get("ticks") or not p.get("comparable_with_A66", True):
            continue
        res = cv.panel(p, d["treated"], d["control"])
        lo, hi = res["ci95"]
        se = pp.se_from_interval(lo, hi, res["t"])
        used.append({"panel": name, "ratio": res["ratio"], "ci95": [lo, hi], "t": res["t"],
                     "log_r": math.log(res["ratio"]), "se_log_r": se})
    pooled = pp.pool([(u["log_r"], u["se_log_r"]) for u in used])
    carried = pp.carry(pooled)
    out = {
        "rule": "NA4.2 (edges.md §4), pool_panels.py run unchanged",
        "inputs": {"pool_panels.py": sha(POOL), "convert.py": sha(os.path.join(READ, "convert.py")),
                   "coords.json": sha(coords_path)},
        "agreement_record": "#230 (A66): 1d p16 area, 1f SA-beta-gal agree within 5 %/10 %",
        "panels": used,
        "pool": pooled,
        "carry": {k: v for k, v in carried.items() if k != "repair_factor"},
        "repair_factor": {line: {str(n): v for n, v in per.items()}
                          for line, per in carried["repair_factor"].items()},
    }
    path = os.path.join(HERE, "pool-output.json")
    json.dump(out, open(path, "w"), indent=1)
    print(json.dumps(out, indent=1))


if __name__ == "__main__":
    sys.exit(main())
