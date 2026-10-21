#!/usr/bin/env python3
"""A78 M5 — label-strip driver (rule 7a).

For every candidate chart panel found by my own pixel scan (axis_scan.json), OCR
ONLY the narrow strip immediately LEFT of the y-axis, which holds the printed tick
labels. Prints, per panel, the OCR text boxes so I can pair each label with the
closest tick found by my own pixel scan.

Nothing here looks at bars, caps or the plot area.
"""
import json
import re
import subprocess

SCAN = json.load(open("axis_scan.json"))

# collapse the many near-duplicate axis detections (one per pixel column)
seen, panels = set(), []
for p in SCAN:
    key = (p["axis_x"] // 6, p["y_top"] // 8, p["y_bottom"] // 8)
    if key in seen:
        continue
    seen.add(key)
    interior = [y for y in p["ticks_px"] if p["y_top"] + 5 < y < p["y_bottom"] - 5]
    if len(interior) >= 2:
        panels.append({**p, "ticks": interior})

print(f"{len(panels)} distinct panels with >=2 interior ticks\n")

NUM = re.compile(r"^-?\d+(?:[.,]\d+)?$")

for idx, p in enumerate(panels):
    ax, y0, y1 = p["axis_x"], p["y_top"] - 18, p["y_bottom"] + 10
    x0, x1 = max(0, ax - 105), ax - 3
    out = subprocess.run(["./ocrtext", "fig1.png", str(x0), str(y0), str(x1), str(y1)],
                         capture_output=True, text=True).stdout
    boxes = []
    for line in out.strip().splitlines():
        parts = [s.strip() for s in line.split("|")]
        if len(parts) < 2:
            continue
        text = parts[0]
        try:
            bx0, by0, bx1, by1 = [float(v) for v in parts[1].split(",")]
        except ValueError:
            continue
        boxes.append((text, bx0, by0, bx1, by1))

    print(f"--- panel {idx}: axis_x={ax} y {p['y_top']}..{p['y_bottom']} "
          f"ticks={p['ticks']}")
    if not boxes:
        print("    (no text recognised)")
        continue
    for text, bx0, by0, bx1, by1 in boxes:
        cy = (by0 + by1) / 2
        nearest = min(p["ticks"], key=lambda t: abs(t - cy))
        d = abs(nearest - cy)
        tag = "NAME" if (NUM.match(text) and d <= 14) else "     "
        print(f"    {tag} {text!r:12s} cy={cy:7.1f} -> tick {nearest} (d={d:4.1f})")
