"""NA4.2: pool agreed figure panels into one H8 input, then carry it through NA4.1.

Rule (design/edges.md §4 NA4.2, spec.md §6). Generic for every L3 mapping with
>= 2 agreed panels; written without any panel value.

Step 2, pool: random effects on log r (DerSimonian-Laird). Panel i gives
log r_i and SE_i = SE(log r_i) from rule 8 (delta method); weights 1/SE_i^2.
k < 3: the pooled interval is the wider of the fixed- and random-effects
intervals. k = 1: that panel alone. k = 0: no_mapping. The fixed-effect
result is returned next to the pooled one; at k = 2 tau2 is flagged
"weakly estimated".

Step 3, carry: pooled log r and its interval -> NA4.1 lines (m = r^-0.5
primary; m = 1/r; m = 1; production x r; stress m = r^-2) -> repair factor
(m + n - 1)/n, n = 2 or 3. If the pooled interval of r includes 1, every
m interval includes 1: no effect is assumed.

    from pool_panels import pool, carry, se_from_interval

Standard library only. Tests: test_pool_panels.py (made-up inputs only).
"""

from __future__ import annotations

import math

Z95 = 1.959963984540054  # normal 0.975 quantile, pooled intervals


def se_from_interval(lo: float, hi: float, t_q: float) -> float:
    """SE(log r) back from a rule-8 interval exp(log r +- t_q * SE)."""
    if not 0 < lo < hi or t_q <= 0:
        raise ValueError("need 0 < lo < hi and t_q > 0")
    return (math.log(hi) - math.log(lo)) / (2 * t_q)


def pool(panels: list[tuple[float, float]]) -> dict:
    """panels: (log_r, se) per agreed panel. Returns pooled log r, its 95% interval, tau2, I2."""
    for y, s in panels:
        if not (math.isfinite(y) and s > 0):
            raise ValueError("each panel needs a finite log r and se > 0")
    k = len(panels)
    if k == 0:
        return {"status": "no_mapping", "k": 0}
    if k == 1:
        y, s = panels[0]
        return {"status": "single_panel", "k": 1, "log_r": y, "se": s,
                "lo": y - Z95 * s, "hi": y + Z95 * s, "tau2": None, "i2": None, "q": None}
    w = [1 / s**2 for _, s in panels]
    sw = sum(w)
    mu_fe = sum(wi * y for wi, (y, _) in zip(w, panels)) / sw
    se_fe = math.sqrt(1 / sw)
    q = sum(wi * (y - mu_fe) ** 2 for wi, (y, _) in zip(w, panels))
    c = sw - sum(wi**2 for wi in w) / sw
    tau2 = max(0.0, (q - (k - 1)) / c)
    i2 = max(0.0, (q - (k - 1)) / q) if q > 0 else 0.0
    wr = [1 / (s**2 + tau2) for _, s in panels]
    mu_re = sum(wi * y for wi, (y, _) in zip(wr, panels)) / sum(wr)
    se_re = math.sqrt(1 / sum(wr))
    fe = (mu_fe - Z95 * se_fe, mu_fe + Z95 * se_fe)
    re = (mu_re - Z95 * se_re, mu_re + Z95 * se_re)
    lo, hi = re
    if k < 3 and fe[1] - fe[0] > re[1] - re[0]:
        lo, hi = fe
    return {"status": "pooled", "k": k, "log_r": mu_re, "se": se_re, "lo": lo, "hi": hi,
            "fe_log_r": mu_fe, "fe": fe, "re": re, "tau2": tau2, "i2": i2, "q": q,
            "tau2_note": "weakly estimated" if k == 2 else None,
            "interval_used": "fixed" if (lo, hi) == fe and fe != re else "random"}


def _m_lines(log_r: float, lo: float, hi: float) -> dict:
    """NA4.1 multiplier lines as (point, low, high); log r low <-> m high."""
    def line(a: float) -> tuple[float, float, float]:  # m = r^(-a)
        return (math.exp(-a * log_r), math.exp(-a * hi), math.exp(-a * lo))
    return {"primary_r^-0.5": line(0.5), "first_order_1/r": line(1.0), "saturated_1": (1.0, 1.0, 1.0),
            "stress_r^-2_outside_range": line(2.0)}


def carry(pooled: dict, n_values: tuple[int, ...] = (2, 3)) -> dict:
    """Pooled log r -> NA4.1 lines -> repair factor (m + n - 1)/n per n."""
    if pooled["status"] == "no_mapping":
        return {"status": "no_mapping"}
    y, lo, hi = pooled["log_r"], pooled["lo"], pooled["hi"]
    m = _m_lines(y, lo, hi)
    repair = {name: {n: tuple((v + n - 1) / n for v in vals) for n in n_values} for name, vals in m.items()}
    return {"status": pooled["status"], "r": (math.exp(y), math.exp(lo), math.exp(hi)),
            "r_interval_includes_1": lo <= 0 <= hi, "m": m,
            "production_x_r": (math.exp(y), math.exp(lo), math.exp(hi)), "repair_factor": repair}
