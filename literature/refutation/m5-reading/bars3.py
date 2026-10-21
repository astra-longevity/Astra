#!/usr/bin/env python3
"""A78 M5 — bar/cap scan v3 (rule 7a, own pixel scan, own cap rule).

Structure seen in the raw pixels (ASCII dumps, bars2 diagnostics):
  - bar = two vertical side lines + a horizontal top edge (hollow outline);
  - error bar = a thin vertical STEM (2-4 px) rising from the top edge, ending
    in a horizontal CAP (~3 rows thick, much wider than the stem);
  - data dots = hollow rings floating above/below, NOT connected to the stem.
bars2 took the stem's flare into the top edge as a 'cap'. v3 fixes that from
structure alone:

  CAP RULE (A78, written blind to any other reader's rule):
    start 3 px above bar_top at the bar centre; walk up while a 2-5 px wide
    ink run centred within 3 px of the centre continues (the stem); the stem
    end is the last such row. Among the rows stem_end-4 .. stem_end, take the
    widest ink run that contains the stem column; if it is >= 12 px wide that
    row's centre-of-thickness (mean of its contiguous wide rows) is the cap.
    If the stem is < 4 px long or no wide run exists -> cap NONE (panel bar
    unlocatable -> no_mapping for that panel).
  Side lines: vertical runs >= 8 px (short bars allowed); a bar needs a top
  edge covering >= 80 % of the gap. bar_top = centre row of the top edge
  (mean of its contiguous rows).

Prints raw pixel coordinates only. No values, no calibration.
"""
import json

import numpy as np
from PIL import Image

A = np.asarray(Image.open("fig1.png").convert("RGB")).astype(int)
H, W, _ = A.shape
ink = (A[:, :, 0] < 200) | (A[:, :, 1] < 200) | (A[:, :, 2] < 200)

PANELS = {
    "A_y1250_x175": {"y_top": 1250, "baseline_y": 1614, "x0": 180, "x1": 441},
    "B_y1248_x644": {"y_top": 1248, "baseline_y": 1614, "x0": 650, "x1": 916},
    "C_y1242_x1773": {"y_top": 1242, "baseline_y": 1605, "x0": 1781, "x1": 2043},
    "D_y1794_x1776": {"y_top": 1794, "baseline_y": 2161, "x0": 1781, "x1": 2043},
}


def run_through(y, x, lo, hi):
    """ink run in row y containing column x, clipped to [lo, hi]; None if x blank."""
    if not ink[y, x]:
        return None
    a = x
    while a - 1 >= lo and ink[y, a - 1]:
        a -= 1
    b = x
    while b + 1 <= hi and ink[y, b + 1]:
        b += 1
    return a, b


def side_lines(p):
    yt, by = p["y_top"] + 4, p["baseline_y"] - 3
    found = []
    for x in range(p["x0"], p["x1"]):
        col = ink[:, x]
        y = yt
        while y < by:
            if col[y]:
                n = 0
                while y + n < by and col[y + n]:
                    n += 1
                if n >= 8 and y + n >= by - 2:   # side lines reach the baseline
                    found.append((x, y, y + n - 1))
                y += max(n, 1)
            else:
                y += 1
    out = []
    for x, y0, y1 in found:
        if out and x - out[-1]["x1"] <= 2 and abs(out[-1]["y0"] - y0) < 8:
            out[-1]["x1"] = x
            out[-1]["y0"] = min(out[-1]["y0"], y0)
        else:
            out.append({"x0": x, "x1": x, "y0": y0, "y1": y1})
    for s in out:
        s["xc"] = (s["x0"] + s["x1"]) / 2
    return out


def top_edge(xl, xr, y_guess):
    rows = []
    for y in range(y_guess - 4, y_guess + 6):
        r = run_through(y, (xl + xr) // 2, xl - 3, xr + 3)
        if r and r[1] - r[0] >= 0.8 * (xr - xl):
            rows.append(y)
    if not rows:
        return None
    # first contiguous block of rows
    blk = [rows[0]]
    for y in rows[1:]:
        if y == blk[-1] + 1:
            blk.append(y)
        else:
            break
    return blk


def cap_for(c, top_rows, y_limit):
    """v4 cap rule (A78): walk up the centre band (c±3) from the top edge while
    ink continues (gaps <= 1 row; stem and stem-overlapping dots both count).
    Flag rows whose run through the centre is 18..40 px wide (cap-like; dots
    are <= 13 px, top edges ~50 px). Cap = topmost contiguous flagged block."""
    c = int(round(c))
    y, gap, flagged, last_ink = top_rows[0] - 1, 0, [], None
    while y > y_limit:
        band = ink[y, c - 3:c + 4]
        if band.any():
            gap, last_ink = 0, y
            best = None
            for x in range(c - 3, c + 4):
                r = run_through(y, x, c - 45, c + 45)
                if r and (best is None or r[1] - r[0] > best[1] - best[0]):
                    best = r
            if best and 18 <= best[1] - best[0] + 1 <= 40:
                flagged.append((y, best))
        else:
            gap += 1
            if gap >= 2:
                break
        y -= 1
    if not flagged:
        return None
    # topmost contiguous block (flagged is in descending y order)
    blk = [flagged[-1]]
    for row in reversed(flagged[:-1]):
        if row[0] == blk[-1][0] + 1:
            blk.append(row)
        else:
            break
    ys = [b[0] for b in blk]
    return {"y": sum(ys) / len(ys), "rows": sorted(ys), "x0": blk[0][1][0],
            "x1": blk[0][1][1], "walk_top": last_ink}


def scan(p):
    sides = side_lines(p)
    bars, used = [], set()
    for i, s in enumerate(sides):
        if i in used:
            continue
        for j in range(i + 1, len(sides)):
            t = sides[j]
            gap = t["xc"] - s["xc"]
            if gap < 30 or gap > 70 or abs(s["y0"] - t["y0"]) > 5:
                continue
            xl, xr = int(round(s["xc"])), int(round(t["xc"]))
            blk = top_edge(xl, xr, min(s["y0"], t["y0"]))
            if not blk:
                continue
            used.update((i, j))
            bar = {"x_left": xl, "x_right": xr, "centre": (xl + xr) / 2,
                   "top_rows": blk, "bar_top_px": sum(blk) / len(blk)}
            bar["cap"] = cap_for(bar["centre"], blk, p["y_top"])
            bars.append(bar)
            break
    return bars


out = {}
for name, p in PANELS.items():
    bars = scan(p)
    out[name] = bars
    print(f"\n=== {name}")
    for b in bars:
        cap = b["cap"]
        cs = (f"cap y={cap['y']:.1f} rows={cap['rows']} x {cap['x0']}-{cap['x1']} "
              f"walk_top={cap['walk_top']}") if cap else "cap NONE"
        print(f"  bar x {b['x_left']}-{b['x_right']} top={b['bar_top_px']:.1f} "
              f"rows={b['top_rows']}  {cs}")
json.dump(out, open("bars3_scan.json", "w"), indent=1)
print("\nwrote bars3_scan.json")
