#!/usr/bin/env python3
"""A78 M5 — own deterministic pixel scan (rule 7a).

Locates, for every chart panel in the composite: the y-axis line, the x-axis
baseline, and the y-axis TICK MARKS (short dark horizontal stubs on the left of
the axis). Prints raw pixel coordinates only. No OCR, no model, no fitting.

Tick label TEXT is read separately (allowed by 7a: OCR of label text only) and
pasted by hand into coords.json, paired with these tick coordinates by their
vertical order.
"""
import json

import numpy as np
from PIL import Image

A = np.asarray(Image.open("fig1.png").convert("RGB"))
H, W, _ = A.shape
r, g, b = A[:, :, 0].astype(int), A[:, :, 1].astype(int), A[:, :, 2].astype(int)
ink = (r < 150) & (g < 150) & (b < 150)


def vruns(x, min_len):
    col = ink[:, x]
    out, y = [], 0
    while y < H:
        if col[y]:
            y0 = y
            while y < H and col[y]:
                y += 1
            if y - y0 >= min_len:
                out.append((y0, y - 1))
        else:
            y += 1
    return out


def hruns(y, min_len):
    row = ink[y, :]
    out, x = [], 0
    while x < W:
        if row[x]:
            x0 = x
            while x < W and row[x]:
                x += 1
            if x - x0 >= min_len:
                out.append((x0, x - 1))
        else:
            x += 1
    return out


# candidate y-axes: vertical dark runs of 120..600 px, clustered by x
cands = {}
for x in range(W):
    for y0, y1 in vruns(x, 120):
        if y1 - y0 <= 620:
            cands.setdefault(x, []).append((y0, y1))

axes = []
for x in sorted(cands):
    for y0, y1 in cands[x]:
        if axes and abs(axes[-1]["x"] - x) <= 3 and abs(axes[-1]["y0"] - y0) < 25:
            axes[-1]["xs"].append(x)
            continue
        axes.append({"x": x, "xs": [x], "y0": y0, "y1": y1})

panels = []
for a in axes:
    x, y0, y1 = int(np.median(a["xs"])), a["y0"], a["y1"]
    # baseline: long horizontal run crossing this x, near y1
    base = None
    for y in range(y1 - 4, min(H, y1 + 14)):
        for s, e in hruns(y, 120):
            if s - 6 <= x <= e + 6:
                base = (y, s, e)
                break
        if base:
            break
    if not base:
        continue
    by, bx0, bx1 = base
    # ticks: short dark horizontal stubs immediately left of the axis
    ticks = []
    for y in range(y0, by + 1):
        seg = ink[y, max(0, x - 14):x - 1]
        if seg.sum() >= 4 and ink[y, x - 1]:
            ticks.append(y)
    # collapse consecutive rows into single ticks
    tk = []
    for y in ticks:
        if tk and y - tk[-1] <= 2:
            tk[-1] = (tk[-1] + y) // 2
        else:
            tk.append(y)
    panels.append({"axis_x": x, "y_top": y0, "y_bottom": by,
                   "baseline": [bx0, bx1], "ticks_px": tk})

panels = [p for p in panels if p["baseline"][1] - p["baseline"][0] >= 150]
print(json.dumps(panels, indent=1))
print("n panels:", len(panels))

with open("axis_scan.json", "w") as f:
    json.dump(panels, f, indent=1)
