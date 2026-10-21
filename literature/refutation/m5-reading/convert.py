#!/usr/bin/env python3
"""A78 M5 reading: convert committed pixel coordinates (coords.json) to values.

Deterministic. Linear axis from the two outermost label-matched ticks; a middle
tick is reported as a linearity check. SEM = |cap - bar| in data units.
Treated/control ratio D+Q / Old; 95 % interval by the delta method on the log
ratio with t(0.975, n-1) (README rules 7a and 8).

Optional: set M5_IMAGE=/path/fig1.png to check the image sha256.
"""
import hashlib
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
T975 = {4: 2.776, 5: 2.571}


def axis(ticks):
    lo, hi = min(ticks, key=lambda t: t["y"]), max(ticks, key=lambda t: t["y"])
    slope = (hi["label"] - lo["label"]) / (hi["y"] - lo["y"])

    def f(y):
        return lo["label"] + (y - lo["y"]) * slope

    checks = [{"label": t["label"], "y": t["y"], "predicted": round(f(t["y"]), 3)}
              for t in ticks if t not in (lo, hi)]
    return f, checks


def panel(p, treated, control):
    if not p.get("ticks"):
        return {"size": "no_mapping", "reason": p["reason"]}
    f, checks = axis(p["ticks"])
    vals = {}
    for s, b in p["series"].items():
        v, c = f(b["bar_top"]), f(b["cap"])
        vals[s] = {"mean": round(v, 4), "sem": round(abs(c - v), 4)}
    t, c = vals[treated], vals[control]
    r = t["mean"] / c["mean"]
    se = math.sqrt((t["sem"] / t["mean"]) ** 2 + (c["sem"] / c["mean"]) ** 2)
    tc = T975[p["n"] - 1]
    return {"values": vals, "linearity_checks": checks, "n": p["n"],
            "ratio": round(r, 4), "se_log": round(se, 4), "t": tc,
            "ci95": [round(r * math.exp(-tc * se), 4), round(r * math.exp(tc * se), 4)],
            "comparable_with_A66": p.get("comparable_with_A66", True)}


def main():
    d = json.load(open(os.path.join(HERE, "coords.json")))
    img = os.environ.get("M5_IMAGE")
    out = {"image_sha256": d["image_sha256"]}
    if img:
        out["image_matches"] = hashlib.sha256(open(img, "rb").read()).hexdigest() == d["image_sha256"]
    out["panels"] = {k: panel(p, d["treated"], d["control"]) for k, p in d["panels"].items()}
    print(json.dumps(out, indent=1))


if __name__ == "__main__":
    main()
