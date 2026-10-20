"""Sealed simple baseline (the "null"): Gompertz on controls + one hazard ratio per class.

Spec: ../baseline.md. Pure standard library.

    h0(t) = a * exp(b * t)                     (control hazard, per sex)
    H0(t) = (a / b) * (exp(b * t) - 1)
    h(t | class k) = HR_k * h0(t)              (proportional hazards)

Guards (D115):
- `fit_gompertz` takes CONTROL arms only.
- `fit_class_hr` takes TRAINING arms only: every arm must carry split == "train".
- Strata (b) and (c) are predicted with the pooled training HR.
"""

from __future__ import annotations

import math
from dataclasses import dataclass


@dataclass(frozen=True)
class Arm:
    """One group of mice: lifespans in years, event flags (1 = death, 0 = censored)."""

    times: tuple
    events: tuple
    role: str  # "control" or "treated"
    split: str = "train"  # "train" or "test"; test arms never reach a fit
    cls: str | None = None  # intervention class (sealed list), None for controls


@dataclass(frozen=True)
class Gompertz:
    a: float
    b: float

    def cum_hazard(self, t: float) -> float:
        return self.a / self.b * math.expm1(self.b * t)

    def log_density(self, t: float, event: int, hr: float = 1.0) -> float:
        """Log-likelihood of one mouse: log h(t) if dead, always minus H(t)."""
        ll = -hr * self.cum_hazard(t)
        if event:
            ll += math.log(hr * self.a) + self.b * t
        return ll


def _nelder_mead(f, x0, step=0.5, tol=1e-10, max_iter=5000):
    """Minimal Nelder-Mead; enough for 1-2 parameters, avoids a SciPy dependency."""
    n = len(x0)
    pts = [list(x0)] + [[x0[j] + (step if j == i else 0.0) for j in range(n)] for i in range(n)]
    vals = [f(p) for p in pts]
    for _ in range(max_iter):
        order = sorted(range(n + 1), key=vals.__getitem__)
        pts, vals = [pts[i] for i in order], [vals[i] for i in order]
        if abs(vals[-1] - vals[0]) < tol:
            break
        cen = [sum(p[j] for p in pts[:-1]) / n for j in range(n)]
        refl = [cen[j] + (cen[j] - pts[-1][j]) for j in range(n)]
        fr = f(refl)
        if fr < vals[0]:
            exp_ = [cen[j] + 2 * (cen[j] - pts[-1][j]) for j in range(n)]
            fe = f(exp_)
            pts[-1], vals[-1] = (exp_, fe) if fe < fr else (refl, fr)
        elif fr < vals[-2]:
            pts[-1], vals[-1] = refl, fr
        else:
            con = [cen[j] + 0.5 * (pts[-1][j] - cen[j]) for j in range(n)]
            fc = f(con)
            if fc < vals[-1]:
                pts[-1], vals[-1] = con, fc
            else:
                pts = [pts[0]] + [[pts[0][j] + 0.5 * (p[j] - pts[0][j]) for j in range(n)] for p in pts[1:]]
                vals = [vals[0]] + [f(p) for p in pts[1:]]
    return pts[min(range(n + 1), key=vals.__getitem__)]


def fit_gompertz(arms: list[Arm]) -> Gompertz:
    """MLE of (a, b) on right-censored CONTROL lifespans."""
    for arm in arms:
        if arm.role != "control":
            raise ValueError("baseline Gompertz is fitted on control arms only")
    data = [(t, e) for arm in arms for t, e in zip(arm.times, arm.events)]

    def nll(x):
        g = Gompertz(math.exp(x[0]), math.exp(x[1]))
        return -sum(g.log_density(t, e) for t, e in data)

    la, lb = _nelder_mead(nll, [math.log(0.01), math.log(2.0)])
    return Gompertz(math.exp(la), math.exp(lb))


def fit_class_hr(g: Gompertz, arms: list[Arm]) -> dict:
    """Closed-form MLE of one HR per class with h0 fixed: HR = deaths / sum H0(t)."""
    sums: dict = {}
    for arm in arms:
        if arm.split != "train":
            raise ValueError("held-out arm passed to a fit: the split is sealed")
        if arm.role != "treated" or arm.cls is None:
            raise ValueError("class HR needs treated arms with a sealed class label")
        d, h = sums.get(arm.cls, (0, 0.0))
        sums[arm.cls] = (d + sum(arm.events), h + sum(g.cum_hazard(t) for t in arm.times))
    hrs = {k: d / h for k, (d, h) in sums.items()}
    d_all = sum(d for d, _ in sums.values())
    h_all = sum(h for _, h in sums.values())
    hrs["__pooled__"] = d_all / h_all if h_all else 1.0
    return hrs


def predict_hr(hrs: dict, cls: str, stratum: str) -> float:
    """Null prediction: own class HR on stratum (a); pooled HR on (b) and (c)."""
    if stratum == "a":
        return hrs[cls]
    if stratum in ("b", "c"):
        return hrs["__pooled__"]
    raise ValueError(f"unknown stratum {stratum!r}")


def log_score(g: Gompertz, arm: Arm, hr: float) -> float:
    """Sum of per-mouse log-likelihoods: the score both model and null are judged on."""
    return sum(g.log_density(t, e, hr) for t, e in zip(arm.times, arm.events))


@dataclass(frozen=True)
class SprtBounds:
    """Placeholder values: A50 fixes them in the seal record before any item resolves."""

    p0: float = 0.5  # model wins an item no more often than chance
    p1: float = 0.75  # model wins clearly more often
    alpha: float = 0.05
    beta: float = 0.2


def sprt(wins: list[bool], bounds: SprtBounds = SprtBounds()) -> str:
    """Wald SPRT on per-item wins of the model over the null within one stratum.

    Returns "model better", "not better" or "undecided" (published as such).
    """
    upper = math.log((1 - bounds.beta) / bounds.alpha)
    lower = math.log(bounds.beta / (1 - bounds.alpha))
    llr = 0.0
    for w in wins:
        if w:
            llr += math.log(bounds.p1 / bounds.p0)
        else:
            llr += math.log((1 - bounds.p1) / (1 - bounds.p0))
        if llr >= upper:
            return "model better"
        if llr <= lower:
            return "not better"
    return "undecided"
