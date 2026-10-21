"""M5 blind second digitisation (A66), rule 7 + A50 G142 option-1 ruling.

Input: fig1.png (sha256 checked). Per chart (charts.json): y-axis tick centre rows
from ticks.py scan with their printed labels, baseline row, and the bar walls of
each group from the baseline scan. Bars are hollow outlines and ticks are measured
at line centres, so bar tops and caps are measured at line centres too.
Only old (vehicle, control) and old+D+Q (treated) are digitised: rule 8 ratio is
treated/control. Missing tick/bar/cap -> that chart's size = no_mapping.
Every raw pixel coordinate is printed.
"""
import hashlib, json, math, sys
import numpy as np
from PIL import Image

SHA = "696a7e0a2b7b6187f9f2de10789cb599f1f89a2281538283722f5df6e37413f6"
assert hashlib.sha256(open("fig1.png", "rb").read()).hexdigest() == SHA, "image hash mismatch"
a = np.asarray(Image.open("fig1.png").convert("RGB")).astype(int)

def klass(p):
    if p.min() >= 200: return "."
    if p.max() - p.min() < 30: return "k" if p.max() < 120 else "g"
    if p[0] > p[1] + 40 and p[0] > p[2] + 40: return "R"
    if p[2] > p[0] + 30: return "B"
    return "?"

def is_c(y, x, c):
    k = klass(a[y, x]); return k == c or (c == "k" and k == "g")

def span(y, L, R, c):
    xs = [x for x in range(L, R + 1) if is_c(y, x, c)]
    return (min(xs), max(xs), len(xs)) if xs else None

def calib(ticks):
    """ticks: [[row, label], ...] from the scan, labels as printed. Needs >= 2.
    Least-squares line value = alpha + beta * row; residuals printed."""
    if len(ticks) < 2: return None
    ys = np.array([t[0] for t in ticks], float); vs = np.array([t[1] for t in ticks], float)
    beta, alpha = np.polyfit(ys, vs, 1)
    res = (vs - (alpha + beta * ys)).tolist()
    return {"alpha": alpha, "beta": beta, "n_ticks": len(ticks),
            "max_abs_residual": max(abs(r) for r in res)}

def val(y, cal): return cal["alpha"] + cal["beta"] * y

def find_top(L, R, c, base, ymin):
    """Top outline of the bar: topmost contiguous run of rows (above base) where
    class pixels cover >= 95 % of L..R. Returns (first_row, last_row, centre)."""
    w = R - L + 1
    full = [y for y in range(base - 6, ymin, -1)
            if (s := span(y, L, R, c)) and s[2] >= 0.95 * w]
    if not full: return None
    top = min(full); last = top
    while last + 1 in full: last += 1
    return (top, last, (top + last) / 2)

def find_cap(L, R, c, top_row):
    """Walk up the whisker (centre +-3 px) from the bar top; the cap is the
    contiguous run of rows at the whisker's upper end whose class span is 15-70 %
    of the bar width and centred on the bar. Returns (first, last, centre, x0, x1)."""
    cx = (L + R) // 2; w = R - L + 1
    y = top_row - 1; ytop = None
    while y > 0 and any(is_c(y, x, c) for x in range(cx - 3, cx + 4)):
        ytop = y; y -= 1
    if ytop is None or top_row - ytop < 4: return None
    # A cap row is SOLID (no gap: pixel count == extent), 15-70 % of bar width,
    # centred on the bar. The cap = the uppermost run of 2-5 contiguous cap rows
    # whose extents differ by <= 3 px. A data dot overlapping the cap breaks
    # solidity or width constancy -> cap not located -> no_mapping.
    # Cap rule v2 (v1 "solid + isolated" rejected every cap touched by a data dot).
    # cap row = contiguous class run through a whisker column (cx-3..cx+3), extent
    # 15-70 % of bar width, centred +-15 %. cap = uppermost run of 2-5 consecutive
    # cap rows, extents within 3 px of the first; the run through cx on the rows
    # just above and below must be < 50 % of the cap extent. Else not located.
    def run_at(y):
        for x in range(cx - 3, cx + 4):
            if is_c(y, x, c):
                l = x
                while l - 1 >= L and is_c(y, l - 1, c): l -= 1
                r = x
                while r + 1 <= R and is_c(y, r + 1, c): r += 1
                return (l, r)
        return None
    def ok(y):
        s = run_at(y)
        if not s: return None
        ext = s[1] - s[0] + 1
        if not (0.15 * w <= ext <= 0.7 * w): return None
        if abs((s[0] + s[1]) / 2 - cx) > 0.15 * w: return None
        return s
    def ext_at(y):
        s = run_at(y); return 0 if s is None else s[1] - s[0] + 1
    # Data dots on the whisker also give short runs: keep every bounded 2-5 row run
    # and take the widest (ties: uppermost).
    cands = []
    y = ytop
    while y < top_row:
        s = ok(y)
        if s is None: y += 1; continue
        e0 = s[1] - s[0]
        run = [(y, s)]
        while y + 1 < top_row and (s2 := ok(y + 1)) and abs((s2[1] - s2[0]) - e0) <= 3:
            y += 1; run.append((y, s2))
        ext = e0 + 1
        if 2 <= len(run) <= 5 and ext_at(run[0][0] - 1) < 0.5 * ext and ext_at(run[-1][0] + 1) < 0.5 * ext:
            cands.append((ext, -run[0][0], run))
        y += 1
    if not cands: return None
    run = max(cands)[2]
    return (run[0][0], run[-1][0], (run[0][0] + run[-1][0]) / 2, run[0][1][0], run[0][1][1])

T975 = {5: 2.776445, 6: 2.570582}  # t(0.975, n-1)

def chart(c):
    out = {"chart": c["name"], "ticks_px_label": c["ticks"], "baseline_row": c["base"]}
    cal = calib(c["ticks"]); out["calibration"] = cal
    if cal is None: out["result"] = "no_mapping (fewer than 2 ticks)"; return out
    ymin = int(min(t[0] for t in c["ticks"])) - 20
    g = {}
    for name in ("old", "dq"):
        L, R, cls = c["bars"][name]
        top = find_top(L, R, cls, c["base"], ymin)
        if top is None: out["result"] = f"no_mapping (bar {name} not located)"; out["groups"] = g; return out
        cap = find_cap(L, R, cls, top[0])
        if cap is None: out["result"] = f"no_mapping (cap {name} not located)"; out["groups"] = g; return out
        mean, upper = val(top[2], cal), val(cap[2], cal)
        g[name] = {"walls_px": [L, R], "top_rows_px": top[:2], "top_centre_px": top[2],
                   "cap_rows_px": cap[:2], "cap_centre_px": cap[2], "cap_x_px": cap[3:],
                   "mean": round(mean, 4), "sem": round(upper - mean, 4)}
    out["groups"] = g
    n = c["n"]; o, d = g["old"], g["dq"]
    r = d["mean"] / o["mean"]
    se = math.sqrt((d["sem"] / d["mean"]) ** 2 + (o["sem"] / o["mean"]) ** 2)
    z = T975[n]
    out["dq_over_old"] = {"ratio": round(r, 4), "se_log": round(se, 4),
                          "lo": round(r * math.exp(-z * se), 4), "hi": round(r * math.exp(z * se), 4),
                          "n_per_group": n, "t": z}
    out["result"] = "ok"
    return out

print(json.dumps({"image_sha256": SHA, "charts": [chart(c) for c in json.load(open(sys.argv[1]))]}, indent=1))
