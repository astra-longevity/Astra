"""R-Sim v0: organ-level stochastic damage/repair model (spec: ../spec.md).

Pure standard library so any machine (and CI) can re-run it bit for bit
with the same Python version and seed.

Per axis i (time t in years):

    dX_i = [eta_i * t * (1 + c * I) - beta_i * X_i / (kappa + X_i)] dt
           + sqrt(2 * eps) dW_i,      X_i >= 0
    I    = mean_j (X_j / Xc_j)        (systemic coupling: inflammation, hormones)

A mouse dies at the first time a VITAL axis crosses its threshold Xc_i.
Non-vital axes still feed the coupling I.

All default values are UNCALIBRATED placeholders chosen so that the
untreated median lifespan is in the mouse range. Calibration on ITP control
survival happens later, under a run manifest.
"""

from __future__ import annotations

import math
import random
from dataclasses import dataclass, field, replace

# Placeholder axes until research/metric/v0/axes.md lands (spec.md, section 2).
AXES = (
    "cardiovascular",
    "renal",
    "metabolic_hepatic",
    "immune_haematopoietic",
    "neural",
    "musculoskeletal",
    "neoplastic",  # placeholder cancer axis (O3, spec issue S3)
)
VITAL = frozenset(AXES) - {"musculoskeletal"}


@dataclass(frozen=True)
class Params:
    """Free parameters of v0. `free_parameter_count` is published with every score."""

    eta: dict = field(default_factory=lambda: {a: 1.0 for a in AXES})  # production slope, 1/yr^2
    beta: dict = field(default_factory=lambda: {a: 2.0 for a in AXES})  # max removal rate, 1/yr
    xc: dict = field(default_factory=lambda: {a: 3.0 for a in AXES})  # failure threshold
    kappa: float = 0.5  # half-saturation of removal (shared)
    eps: float = 0.6  # noise intensity (shared)
    c: float = 0.3  # systemic coupling strength (shared)

    def free_parameter_count(self) -> int:
        return len(self.eta) + len(self.beta) + len(self.xc) + 3


@dataclass(frozen=True)
class Perturbation:
    """A sealed intervention mapping applied as multipliers from `start_age` (years).

    `multipliers` keys look like "eta.renal", "beta.*", "c".
    Only rows from a sealed research/sim/v0/mappings.yaml may build one.
    """

    multipliers: dict
    start_age: float = 0.0


def _apply(p: Params, pert: Perturbation) -> Params:
    eta, beta, xc = dict(p.eta), dict(p.beta), dict(p.xc)
    scalars = {"kappa": p.kappa, "eps": p.eps, "c": p.c}
    for key, m in pert.multipliers.items():
        name, _, axis = key.partition(".")
        if name in scalars and not axis:
            scalars[name] *= m
            continue
        table = {"eta": eta, "beta": beta, "xc": xc}.get(name)
        if table is None:
            raise KeyError(f"unknown parameter {key!r}")
        targets = AXES if axis == "*" else (axis,)
        for a in targets:
            if a not in table:
                raise KeyError(f"unknown axis in {key!r}")
            table[a] *= m
    return replace(p, eta=eta, beta=beta, xc=xc, **scalars)


def simulate(
    n: int,
    seed: int,
    params: Params | None = None,
    perturbation: Perturbation | None = None,
    dt: float = 7 / 365.25,
    t_max: float = 6.0,
) -> list[float]:
    """Return n simulated lifespans in years (t_max if still alive: right-censored)."""
    base = params or Params()
    treated = _apply(base, perturbation) if perturbation else None
    rng = random.Random(seed)
    sq = math.sqrt(dt)
    lifespans = []
    for _ in range(n):
        x = {a: 0.0 for a in AXES}
        t, death = 0.0, t_max
        while t < t_max:
            p = treated if treated and t >= perturbation.start_age else base
            noise = math.sqrt(2 * p.eps) * sq
            coupling = 1 + p.c * sum(x[a] / p.xc[a] for a in AXES) / len(AXES)
            for a in AXES:
                drift = p.eta[a] * t * coupling - p.beta[a] * x[a] / (p.kappa + x[a])
                x[a] = max(0.0, x[a] + drift * dt + noise * rng.gauss(0.0, 1.0))
            t += dt
            if any(x[a] >= p.xc[a] for a in VITAL):
                death = t
                break
        lifespans.append(death)
    return lifespans


def survival_curve(lifespans: list[float], step: float = 1 / 12) -> list[tuple[float, float]]:
    """Empirical survival S(t) on a monthly grid."""
    n = len(lifespans)
    s = sorted(lifespans)
    out, i, t = [], 0, 0.0
    t_end = s[-1] if s else 0.0
    while t <= t_end + 1e-12:
        while i < n and s[i] <= t:
            i += 1
        out.append((round(t, 6), (n - i) / n))
        t += step
    return out
