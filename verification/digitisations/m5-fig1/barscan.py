"""Audit scan: for each bar (walls from baseline scan), print per-row span fraction
of bar-class pixels between the walls, and the whisker/cap rows above the bar."""
import json, sys
import numpy as np
from PIL import Image
a = np.asarray(Image.open("fig1.png").convert("RGB")).astype(int)

def klass(p):
    if p.min() >= 200: return "."
    if p.max() - p.min() < 30: return "k" if p.max() < 120 else "g"
    if p[0] > p[1] + 40 and p[0] > p[2] + 40: return "R"
    if p[2] > p[0] + 30: return "B"
    return "?"

def is_c(y, x, c):
    k = klass(a[y, x]); return k == c or (c == "k" and k == "g")

BARS = json.load(open(sys.argv[1]))
for b in BARS:
    L, R, c, base = b["left"], b["right"], b["cls"], b["base"]
    w = R - L + 1
    print(f"== {b['name']} cls={c} walls {L}-{R} base {base}")
    rows = []
    for y in range(base - 6, base - 700, -1):
        f = sum(is_c(y, x, c) for x in range(L, R + 1)) / w
        rows.append((y, f))
    full = [y for y, f in rows if f >= 0.95]
    print("  rows span>=0.95 (top 12):", sorted(full)[:12])
    # whisker: centre columns above the topmost full row
    cx = (L + R) // 2
    top = min(full) if full else None
    if top is None: continue
    seg = []
    for y in range(top - 1, top - 400, -1):
        on = [x for x in range(cx - 3, cx + 4) if is_c(y, x, c)]
        span = [x for x in range(L, R + 1) if is_c(y, x, c)]
        seg.append((y, len(on), (min(span), max(span)) if span else None))
    # print first 60 rows above
    for y, n, sp in seg[:60]:
        if n or sp: print(f"  y{y} centre_on={n} span={sp}")
