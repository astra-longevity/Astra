#!/usr/bin/env python3
"""A78 — M5 (senolytic -> H8) figure digitisation, rule 7 of the L3 README.

Independent, blind to A66's values. Reads only the raw figure image
(NPJ Regen Med 2026, doi:10.1038/s41536-026-00490-x, Fig 1), never a page
or panel that shows survival.

Rule 7 (binding, A50 2026-10-21):
  - each axis calibrated on 2 tick marks matched to their PRINTED labels
    (never the frame edge);
  - raw pixel coordinates of ticks, bar tops and error-bar caps committed,
    plus the image sha256; this script converts them;
  - a panel without 2 locatable label-matched ticks, or a bar/cap that cannot
    be located, has size `no_mapping` for that panel (direction kept);
  - no OCR-estimated values.

This script is deterministic: it reads coords.json, checks the image hash,
and converts each panel's pixel coordinates to values + a delta-method ratio
interval, OR emits `no_mapping` for any panel whose coords.json entry is
marked `locatable: false`.

WHY THIS RUN EMITS no_mapping FOR EVERY PANEL
  The writer has no image viewer in this environment and OCR-estimated values
  are forbidden by rule 7. Candidate axis lines can be found by pixel scan
  (see explore.py / panels.py), but matching a tick to its PRINTED label
  cannot be done blind without OCR. Per rule 7, a panel whose ticks cannot be
  matched to printed labels has size `no_mapping`, direction kept. coords.json
  records this honestly (locatable: false) rather than fabricating values.
  The direction (senescence markers fall under D+Q -> H8 clearance up) is
  taken from the figure legend / text, not from pixel values.
"""
import hashlib
import json
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
IMG = os.path.join(HERE, "fig1.png")
COORDS = os.path.join(HERE, "coords.json")

T975 = {4: 2.776, 5: 2.571, 6: 2.447, 7: 2.365, 8: 2.306,
        9: 2.262, 10: 2.228, 14: 2.145}


def sha256(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 16), b""):
            h.update(chunk)
    return h.hexdigest()


def px_to_value(px, axis):
    """Linear map from two label-matched ticks: (px0,val0),(px1,val1)."""
    (p0, v0), (p1, v1) = axis["tick0"], axis["tick1"]
    return v0 + (px - p0) * (v1 - v0) / (p1 - p0)


def panel_result(name, panel):
    if not panel.get("locatable", False):
        return {"panel": name, "size": "no_mapping",
                "reason": panel.get("reason", "ticks not matchable to labels"),
                "direction": panel["direction"]}
    out = {"panel": name, "direction": panel["direction"], "series": {}}
    axis = panel["y_axis"]
    for s, spec in panel["series"].items():
        val = px_to_value(spec["bar_top_px"], axis)
        cap = px_to_value(spec["cap_px"], axis)
        sem = abs(cap - val)
        out["series"][s] = {"value": val, "sem": sem, "n": spec["n"]}
    c = out["series"][panel["control"]]
    t = out["series"][panel["treated"]]
    r = t["value"] / c["value"]
    se_log = math.sqrt((t["sem"] / t["value"]) ** 2 + (c["sem"] / c["value"]) ** 2)
    df = min(t["n"], c["n"]) - 1
    tc = T975[df]
    out["ratio"] = r
    out["ratio_ci"] = [r * math.exp(-tc * se_log), r * math.exp(tc * se_log)]
    return out


def main():
    coords = json.load(open(COORDS))
    img_ok = os.path.exists(IMG) and sha256(IMG) == coords["image_sha256"]
    report = {"image_sha256_expected": coords["image_sha256"],
              "image_present_and_matches": img_ok,
              "panels": [panel_result(n, p) for n, p in coords["panels"].items()]}
    print(json.dumps(report, indent=2))


if __name__ == "__main__":
    main()
