"""R-Sim v0: organ-level stochastic damage/repair model (spec: ../spec.md).

Pure standard library so any machine (and CI) can re-run it bit for bit
with the same Python version and seed.

State = the organ-system axes of R-Metric v0 (research/metric/v0/axes.md,
A1-A17). A18 (whole-body frailty) is a derived readout, not a state.
Hallmarks H1-H12 are not states: each is a multiplier `m_h` (1 in untreated
mice) on damage production, repair or coupling of the organ axes it drives.
Interventions act on hallmark multipliers or on named organ parameters.

Per organ axis i (time t in years):

    dX_i = [eta_i * P_i * t * (1 + c * C * I) - beta_i * R_i * X_i / (kappa + X_i)] dt
           + sqrt(2 * eps) dW_i,      X_i >= 0
    I    = mean_j (X_j / Xc_j)        (systemic coupling: inflammation, hormones)
    P_i  = mean of m_h over production hallmarks driving axis i
    R_i  = mean of m_h over repair hallmarks driving axis i
    C    = mean of m_h over coupling hallmarks (H10, H11)

A mouse dies at the first time a VITAL axis crosses its threshold Xc_i.
All default values are UNCALIBRATED placeholders (trust level L0).
"""

from __future__ import annotations

import math
import random
from dataclasses import dataclass, field, replace

# Organ-system axes, codes and names from axes.md Table A (A18 is derived).
AXES = (
    "A1_muscle",
    "A2_bone",
    "A3_brain",
    "A4_hearing",
    "A5_vision",
    "A6_heart",
    "A7_vessels",
    "A8_lung",
    "A9_kidney",
    "A10_liver_metabolism",
    "A11_immune",
    "A12_blood",
    "A13_skin_hair",
    "A14_gut",
    "A15_fat",
    "A16_endocrine_reproduction",
    "A17_neoplasia",
)
VITAL = frozenset(
    {
        "A3_brain",
        "A6_heart",
        "A7_vessels",
        "A8_lung",
        "A9_kidney",
        "A10_liver_metabolism",
        "A11_immune",
        "A12_blood",
        "A17_neoplasia",
    }
)
# axes.md status "unmeasured": the state exists, no public readout yet.
NO_READOUT = frozenset(
    {"A2_bone", "A4_hearing", "A5_vision", "A7_vessels", "A16_endocrine_reproduction", "A17_neoplasia"}
)

# Hallmarks (axes.md Table B, Lopez-Otin 2023). Role: what the multiplier scales.
# Targets: organ axes driven; None = all organ axes. Every link is a causal
# edge and needs an L3 refutation row before calibration (spec issue S10).
HALLMARKS = {
    "H1_genomic_instability": ("production", None),
    "H2_telomere_attrition": ("production", ("A11_immune", "A12_blood", "A13_skin_hair", "A14_gut")),
    "H3_epigenetic_alterations": ("production", None),
    "H4_proteostasis": ("production", None),
    "H5_autophagy": ("repair", None),
    "H6_nutrient_sensing": ("production", None),
    "H7_mitochondria": ("production", None),
    "H8_senescence": ("repair", None),
    "H9_stem_cells": (
        "repair",
        ("A1_muscle", "A2_bone", "A11_immune", "A12_blood", "A13_skin_hair", "A14_gut"),
    ),
    "H10_intercellular_communication": ("coupling", None),
    "H11_inflammation": ("coupling", None),
    "H12_dysbiosis": ("production", ("A10_liver_metabolism", "A11_immune", "A14_gut")),
}
HALLMARKS_NO_READOUT = frozenset(
    {"H1_genomic_instability", "H2_telomere_attrition", "H5_autophagy", "H10_intercellular_communication", "H12_dysbiosis"}
)


def _drivers(role: str, axis: str) -> tuple:
    return tuple(
        h for h, (r, targets) in HALLMARKS.items() if r == role and (targets is None or axis in targets)
    )


@dataclass(frozen=True)
class Params:
    """Free parameters of v0. `free_parameter_count` is published with every score.

    Hallmark multipliers are fixed at 1 in untreated mice and only move through
    sealed intervention mappings, so they are not free parameters.
    """

    eta: dict = field(default_factory=lambda: {a: 1.0 for a in AXES})  # production slope, 1/yr^2
    beta: dict = field(default_factory=lambda: {a: 2.0 for a in AXES})  # max repair rate, 1/yr
    xc: dict = field(default_factory=lambda: {a: 3.6 for a in AXES})  # failure threshold
    kappa: float = 0.5  # half-saturation of repair (shared)
    eps: float = 0.6  # noise intensity (shared)
    c: float = 0.3  # systemic coupling strength (shared)
    hallmark: dict = field(default_factory=lambda: {h: 1.0 for h in HALLMARKS})

    def free_parameter_count(self) -> int:
        return len(self.eta) + len(self.beta) + len(self.xc) + 3


@dataclass(frozen=True)
class Perturbation:
    """A sealed intervention mapping applied as multipliers from `start_age` (years).

    `multipliers` keys look like "eta.A9_kidney", "beta.*", "c", "hallmark.H6_nutrient_sensing".
    Only rows from a sealed research/sim/mappings.yaml may build one.
    """

    multipliers: dict
    start_age: float = 0.0


def _apply(p: Params, pert: Perturbation) -> Params:
    eta, beta, xc, hm = dict(p.eta), dict(p.beta), dict(p.xc), dict(p.hallmark)
    scalars = {"kappa": p.kappa, "eps": p.eps, "c": p.c}
    tables = {"eta": (eta, AXES), "beta": (beta, AXES), "xc": (xc, AXES), "hallmark": (hm, tuple(HALLMARKS))}
    for key, m in pert.multipliers.items():
        name, _, target = key.partition(".")
        if name in scalars and not target:
            scalars[name] *= m
            continue
        if name not in tables:
            raise KeyError(f"unknown parameter {key!r}")
        table, keys = tables[name]
        for k in keys if target == "*" else (target,):
            if k not in table:
                raise KeyError(f"unknown target in {key!r}")
            table[k] *= m
    return replace(p, eta=eta, beta=beta, xc=xc, hallmark=hm, **scalars)


def _effective(p: Params) -> tuple:
    """Per-axis (production, repair, threshold) and coupling after hallmark factors."""
    rows = []
    for a in AXES:
        prod = _drivers("production", a)
        rep = _drivers("repair", a)
        pf = sum(p.hallmark[h] for h in prod) / len(prod) if prod else 1.0
        rf = sum(p.hallmark[h] for h in rep) / len(rep) if rep else 1.0
        rows.append((a, p.eta[a] * pf, p.beta[a] * rf, p.xc[a]))
    coup = _drivers("coupling", AXES[0])
    cf = sum(p.hallmark[h] for h in coup) / len(coup) if coup else 1.0
    return rows, p.c * cf


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
    treated = _apply(base, perturbation) if perturbation else base
    phases = ((base, _effective(base)), (treated, _effective(treated)))
    start = perturbation.start_age if perturbation else math.inf
    rng = random.Random(seed)
    sq = math.sqrt(dt)
    lifespans = []
    for _ in range(n):
        x = dict.fromkeys(AXES, 0.0)
        t, death = 0.0, t_max
        while t < t_max:
            p, (rows, c) = phases[t >= start]
            noise = math.sqrt(2 * p.eps) * sq
            coupling = 1 + c * sum(x[a] / xc for a, _, _, xc in rows) / len(rows)
            for a, eta, beta, _ in rows:
                drift = eta * t * coupling - beta * x[a] / (p.kappa + x[a])
                x[a] = max(0.0, x[a] + drift * dt + noise * rng.gauss(0.0, 1.0))
            t += dt
            if any(x[a] >= xc for a, _, _, xc in rows if a in VITAL):
                death = t
                break
        lifespans.append(death)
    return lifespans


def frailty(x: dict, params: Params | None = None) -> float:
    """A18 readout: mean scaled damage over organ axes (derived, not a state)."""
    p = params or Params()
    return sum(min(1.0, x[a] / p.xc[a]) for a in AXES) / len(AXES)


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
