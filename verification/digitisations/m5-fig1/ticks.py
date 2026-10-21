"""Scan for y-axis lines, baselines and tick marks in a panel box. Prints everything."""
import sys
from PIL import Image
import numpy as np
a = np.asarray(Image.open('fig1.png').convert('RGB')).astype(int)
x0, y0, x1, y1 = map(int, sys.argv[1:5])
sub = a[y0:y1, x0:x1]
H, W, _ = sub.shape
dark = sub.sum(2) < 230
col = dark.sum(0); row = dark.sum(1)
vax = [i for i, v in enumerate(col) if v > 0.4 * H]
hax = [i for i, v in enumerate(row) if v > 0.25 * W]
print('box', x0, y0, x1, y1, 'vertical axis cols (abs x):', [x0 + i for i in vax])
print('baseline rows (abs y):', [y0 + i for i in hax])
# group axis columns into runs
runs = []
for c in vax:
    if runs and c - runs[-1][-1] <= 1: runs[-1].append(c)
    else: runs.append([c])
for r in runs:
    left = r[0]
    # tick marks: short dark runs immediately left of axis line (3..14 px)
    strip = dark[:, max(0, left - 14):left]
    cnt = strip.sum(1)
    ticks = [i for i, v in enumerate(cnt) if v >= 6]
    # group consecutive rows
    g = []
    for t in ticks:
        if g and t - g[-1][-1] <= 1: g[-1].append(t)
        else: g.append([t])
    centres = [y0 + (t[0] + t[-1]) / 2 for t in g]
    print(f'axis at abs x {x0+r[0]}-{x0+r[-1]}: tick centres abs y = {centres}')
