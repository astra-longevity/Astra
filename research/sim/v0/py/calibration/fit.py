"""R-Sim v0 calibration fit, as pre-registered in design/calibration.md (af2413e).

Reduced v0, per sex: u_eta, u_xc = shared log-multipliers on eta and Xc of the
VITAL axes. Everything else stays at the spec.md §5 defaults ("prior, not
identified"). Fitted on train-split untreated control arms only.

    from fit import select_controls, fit, gompertz_loglik, chi2_gof, verdict

Standard library only. Tests: test_fit.py. Full synthetic recovery: recovery.py.
"""

from __future__ import annotations

import math
import os
import sys

_PY = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, _PY)  # py/: model, baseline

import baseline  # noqa: E402
import model  # noqa: E402

N_SIM = 4000
SIM_SEED = 7
STABILITY_SEEDS = (8, 9, 10)
T_MAX = 6.0
BOX = 1.5
STARTS = ((0.0, 0.0), (0.3, -0.3), (-0.3, 0.3))
AGREE_TOL = 0.05
EDGE_TOL = 0.05
N_BINS = int(round(T_MAX * 12)) + 1  # last bin = alive at T_MAX
MIN_EXPECTED = 5.0


# ---------- arm guard (§1) ----------

def select_controls(arms: list, diets: dict | None = None) -> tuple[list, list]:
    """Keep role == control, split == train, diet == control.

    `diets` maps arm index -> diet label (missing = "control"). Returns
    (kept arms, [(index, reasons)] rejected) so the filter output can go in the
    run manifest.
    """
    kept, rejected = [], []
    for i, arm in enumerate(arms):
        diet = (diets or {}).get(i, "control")
        why = [w for w, bad in (("not control", arm.role != "control"),
                                 ("not train", arm.split != "train"),
                                 (f"diet {diet}", diet != "control")) if bad]
        if why:
            rejected.append((i, why))
        else:
            kept.append(arm)
    return kept, rejected


def _guard(arms: list) -> None:
    if not arms:
        raise ValueError("no arm to fit")
    for arm in arms:
        if arm.role != "control" or arm.split != "train":
            raise ValueError("calibration fits train-split untreated control arms only")


def _pool(arms):
    return [t for a in arms for t in a.times], [e for a in arms for e in a.events]


# ---------- likelihood (§2) ----------

def _month(t: float) -> int:
    return min(N_BINS - 1, int(t * 12))


def sim_bins(u_eta: float, u_xc: float, n_sim: int = N_SIM, seed: int = SIM_SEED) -> list[float]:
    """Smoothed monthly death probabilities p_k = (n_k + 0.5) / (n_sim + 0.5 B); last bin = alive at T_MAX."""
    mult = {}
    for a in model.VITAL:
        mult[f"eta.{a}"] = math.exp(u_eta)
        mult[f"xc.{a}"] = math.exp(u_xc)
    life = model.simulate(n_sim, seed, perturbation=model.Perturbation(mult), t_max=T_MAX)
    counts = [0] * N_BINS
    for x in life:
        counts[_month(x)] += 1
    tot = n_sim + 0.5 * N_BINS
    return [(c + 0.5) / tot for c in counts]


def _surv_after(p: list[float]) -> list[float]:
    """surv[k] = P(alive at the end of month k) = sum of p_j for j > k."""
    out, acc = [0.0] * N_BINS, 0.0
    for k in range(N_BINS - 1, -1, -1):
        out[k] = acc
        acc += p[k]
    return out


def binned_loglik(times, events, p: list[float]) -> float:
    """Death in month k -> log p_k; censored at c -> log S(end of month floor(12c))."""
    surv = _surv_after(p)
    ll = 0.0
    for t, e in zip(times, events):
        k = _month(t)
        ll += math.log(p[k]) if e else math.log(max(surv[k], 1e-300))
    return ll


def gompertz_bins(g) -> list[float]:
    """Exact monthly bin probabilities of a Gompertz null, same bins as the simulation."""
    s = [math.exp(-g.cum_hazard(k / 12)) for k in range(N_BINS)]
    p = [s[k] - s[k + 1] for k in range(N_BINS - 1)] + [s[N_BINS - 1]]
    return [max(x, 1e-300) for x in p]


def gompertz_loglik(arms: list) -> tuple:
    """Null fitted on the same arms (baseline.fit_gompertz), scored with the binned likelihood (§4)."""
    _guard(arms)
    g = baseline.fit_gompertz(arms)
    times, events = _pool(arms)
    return g, binned_loglik(times, events, gompertz_bins(g))


# ---------- fit (§3) ----------

def _fit_once(times, events, start, n_sim, seed, max_iter):
    def nll(x):
        if any(abs(v) > BOX for v in x):
            return 1e12
        return -binned_loglik(times, events, sim_bins(x[0], x[1], n_sim, seed))

    x = baseline._nelder_mead(nll, list(start), step=0.3, tol=1e-4, max_iter=max_iter)
    return {"u_eta": x[0], "u_xc": x[1], "loglik": -nll(x)}


def _spread(runs, key):
    v = [r[key] for r in runs]
    return max(v) - min(v)


def fit(arms: list, n_sim: int = N_SIM, seed: int = SIM_SEED,
        stability_seeds=STABILITY_SEEDS, max_iter: int = 80) -> dict:
    """Pre-registered fit: 3 starts, best logL kept; MC-stability refits; convergence criteria 1-3."""
    _guard(arms)
    times, events = _pool(arms)
    runs = [_fit_once(times, events, s, n_sim, seed, max_iter) for s in STARTS]
    best = max(runs, key=lambda r: r["loglik"])
    stab = [_fit_once(times, events, (best["u_eta"], best["u_xc"]), n_sim, s, max_iter)
            for s in stability_seeds]
    keys = ("u_eta", "u_xc")
    conv = {
        "starts_agree": all(_spread(runs, k) < AGREE_TOL for k in keys),
        "mc_stable": all(_spread(stab + [best], k) < AGREE_TOL for k in keys),
        "off_edge": all(abs(best[k]) < BOX - EDGE_TOL for k in keys),
    }
    conv["converged"] = all(conv.values())
    return {"estimate": best, "starts": runs, "stability": stab, "convergence": conv,
            "n_sim": n_sim, "sim_seed": seed, "free_parameters": 2, "n_mice": len(times)}


# ---------- goodness of fit and verdict (§5) ----------

def chi2_sf(x: float, df: int) -> float:
    """Upper tail P(X >= x) of a chi-square with df degrees of freedom (regularised gamma)."""
    if df <= 0:
        return float("nan")
    a, z = df / 2, x / 2
    if z <= 0:
        return 1.0
    log_pre = -z + a * math.log(z) - math.lgamma(a)
    if z < a + 1:  # series for the lower tail
        term = total = 1 / a
        n = a
        for _ in range(1000):
            n += 1
            term *= z / n
            total += term
            if term < total * 1e-15:
                break
        return max(0.0, 1 - total * math.exp(log_pre))
    b = z + 1 - a  # continued fraction for the upper tail (Lentz)
    c, d = 1e300, 1 / b
    h = d
    for i in range(1, 1000):
        an = -i * (i - a)
        b += 2
        d = an * d + b
        d = 1 / (d if abs(d) > 1e-300 else 1e-300)
        c = b + an / c
        c = c if abs(c) > 1e-300 else 1e-300
        h *= d * c
        if abs(d * c - 1) < 1e-15:
            break
    return h * math.exp(log_pre)


def chi2_gof(times, events, p: list[float], n_params: int = 2) -> dict:
    """Observed vs expected deaths per month, adjacent months merged until expected >= 5.

    Every mouse (dead or censored) is at risk from month 0 to its own month k;
    expected deaths in month j add the model hazard q_j = p_j / P(alive at start of j)
    for each mouse at risk. The last bin (alive at T_MAX) is not a death cell.
    """
    start = [0.0] * N_BINS  # P(alive at start of month j)
    acc = 0.0
    for j in range(N_BINS - 1, -1, -1):
        acc += p[j]
        start[j] = acc
    q = [p[j] / start[j] for j in range(N_BINS)]
    at_risk_until = [0] * N_BINS
    obs = [0.0] * N_BINS
    for t, e in zip(times, events):
        k = _month(t)
        at_risk_until[k] += 1
        if e:
            obs[k] += 1
    n_risk = [0] * N_BINS  # mice at risk in month j = mice whose own month >= j
    run = 0
    for j in range(N_BINS - 1, -1, -1):
        run += at_risk_until[j]
        n_risk[j] = run
    exp_ = [n_risk[j] * q[j] for j in range(N_BINS - 1)]
    cells, eo, ee = [], 0.0, 0.0
    for o, x in zip(obs[:-1], exp_):
        eo, ee = eo + o, ee + x
        if ee >= MIN_EXPECTED:
            cells.append([eo, ee])
            eo, ee = 0.0, 0.0
    if ee > 0 or eo > 0:
        if cells:
            cells[-1][0] += eo
            cells[-1][1] += ee
        else:
            cells.append([eo, ee])
    stat = sum((o - x) ** 2 / x for o, x in cells if x > 0)
    df = len(cells) - 1 - n_params
    return {"chi2": stat, "df": df, "p": chi2_sf(stat, df), "cells": len(cells)}


def verdict(fit_result: dict, gof: dict, delta_loglik: float) -> dict:
    """§5 pass/fail, per sex. Beating the null is not a criterion."""
    checks = {"converged": bool(fit_result["convergence"]["converged"]),
              "gof_p_ge_0.01": bool(gof["p"] >= 0.01),
              "delta_loglik_ge_-2": bool(delta_loglik >= -2)}
    return {"pass": all(checks.values()), "checks": checks, "delta_loglik": delta_loglik}
