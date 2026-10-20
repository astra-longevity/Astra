"""Local identifiability of R-Sim v0 parameters from control survival alone.

Question (A28, G146): which of the 54 free parameters can be recovered from
untreated survival curves, and which need axis readouts?

Method (stdlib only, synthetic data, no real data touched):
- Data model: N_DATA control lifespans, grouped in monthly death bins
  (multinomial). Fisher information for log-parameters theta:
      F = N_DATA * sum_k (dp_k/dtheta)(dp_k/dtheta)^T / p_k
  with p_k = P(death in month k), right-censoring at T_MAX in the last bin.
- dp_k/dtheta by central differences (+/- H in log units) with common random
  numbers (same seed for both sides), N_SIM simulated mice per evaluation.
- J is estimated twice with independent seeds; F uses the cross product
  J1 J2^T (cross_fisher), which removes the upward Monte-Carlo bias of J J^T.
- Eigen-decomposition of F (Jacobi). An eigen-direction is "recoverable" when
  its standard error 1/sqrt(lambda) is below SE_OK (log units, 0.2 = ~20%)
  AND its eigenvalue is 3x above the noise floor (|most negative eigenvalue|).

Run: python3 identifiability.py [n_sim] > out.json   (about 3 min per seed at 2000)
"""

from __future__ import annotations

import json
import math
import sys

import model

N_DATA = 2000  # control mice per sex, order of a pooled ITP control set (placeholder)
T_MAX = 6.0
H = 0.1
SE_OK = 0.2


def full_names() -> list[str]:
    names = [f"{k}.{a}" for k in ("eta", "beta", "xc") for a in model.AXES]
    return names + ["kappa", "eps", "c"]


def grouped() -> dict:
    """9 groups: eta/beta/xc x vital/non-vital, plus 3 shared."""
    g = {}
    for k in ("eta", "beta", "xc"):
        g[f"{k}.vital"] = [f"{k}.{a}" for a in model.AXES if a in model.VITAL]
        g[f"{k}.nonvital"] = [f"{k}.{a}" for a in model.AXES if a not in model.VITAL]
    for k in ("kappa", "eps", "c"):
        g[k] = [k]
    return g


def bin_probs(lifespans: list[float]) -> list[float]:
    nb = int(round(T_MAX * 12)) + 1  # last bin = alive at T_MAX
    counts = [0] * nb
    for x in lifespans:
        counts[min(nb - 1, int(x * 12))] += 1
    n = len(lifespans)
    return [c / n for c in counts]


def probs(mult: dict, n_sim: int, seed: int) -> list[float]:
    pert = model.Perturbation(mult, start_age=0.0) if mult else None
    return bin_probs(model.simulate(n_sim, seed, perturbation=pert, t_max=T_MAX))


def jacobian(groups: dict, n_sim: int, seed: int) -> tuple[list[float], list[list[float]]]:
    p0 = probs({}, n_sim, seed)
    cols = []
    for members in groups.values():
        up = probs({m: math.exp(H) for m in members}, n_sim, seed)
        dn = probs({m: math.exp(-H) for m in members}, n_sim, seed)
        cols.append([(u - d) / (2 * H) for u, d in zip(up, dn)])
    return p0, cols


def fisher(p0: list[float], cols: list[list[float]]) -> list[list[float]]:
    m = len(cols)
    f = [[0.0] * m for _ in range(m)]
    for k, p in enumerate(p0):
        if p <= 0:
            continue
        for i in range(m):
            for j in range(i, m):
                v = N_DATA * cols[i][k] * cols[j][k] / p
                f[i][j] += v
                if i != j:
                    f[j][i] += v
    return f


def jacobi_eigen(a: list[list[float]], sweeps: int = 100) -> tuple[list[float], list[list[float]]]:
    """Eigenvalues (descending) and eigenvectors (columns) of a symmetric matrix."""
    n = len(a)
    a = [row[:] for row in a]
    v = [[float(i == j) for j in range(n)] for i in range(n)]
    for _ in range(sweeps):
        off = sum(a[i][j] ** 2 for i in range(n) for j in range(n) if i != j)
        if off < 1e-22 * max(1.0, sum(a[i][i] ** 2 for i in range(n))):
            break
        for p in range(n - 1):
            for q in range(p + 1, n):
                if abs(a[p][q]) < 1e-300:
                    continue
                theta = (a[q][q] - a[p][p]) / (2 * a[p][q])
                t = math.copysign(1.0, theta) / (abs(theta) + math.sqrt(theta * theta + 1))
                c = 1 / math.sqrt(t * t + 1)
                s = t * c
                for k in range(n):
                    akp, akq = a[k][p], a[k][q]
                    a[k][p], a[k][q] = c * akp - s * akq, s * akp + c * akq
                for k in range(n):
                    apk, aqk = a[p][k], a[q][k]
                    a[p][k], a[q][k] = c * apk - s * aqk, s * apk + c * aqk
                for k in range(n):
                    vkp, vkq = v[k][p], v[k][q]
                    v[k][p], v[k][q] = c * vkp - s * vkq, s * vkp + c * vkq
    order = sorted(range(n), key=lambda i: -a[i][i])
    return [a[i][i] for i in order], [[v[k][i] for k in range(n)] for i in order]


def cross_fisher(p1: list[float], c1: list[list[float]], p2: list[float], c2: list[list[float]]) -> list[list[float]]:
    """Unbiased Fisher estimate from two independent Jacobian estimates.

    The naive N * sum J J^T / p is biased upward by the Monte-Carlo noise in J
    (a parameter with no effect still gets SE ~ 0.07 at n_sim = 2000). Noise
    is independent between seeds, so the symmetrised cross product J1 J2^T has
    the true F as expectation. Pure-noise directions then scatter around 0
    and the most negative eigenvalue measures the noise floor.
    """
    m = len(c1)
    f = [[0.0] * m for _ in range(m)]
    for k in range(len(p1)):
        p = 0.5 * (p1[k] + p2[k])
        if p <= 0:
            continue
        for i in range(m):
            for j in range(m):
                f[i][j] += N_DATA * 0.5 * (c1[i][k] * c2[j][k] + c2[i][k] * c1[j][k]) / p
    return f


def _corr(a: list[float], b: list[float]) -> float:
    na, nb = math.sqrt(sum(x * x for x in a)), math.sqrt(sum(x * x for x in b))
    return sum(x * y for x, y in zip(a, b)) / (na * nb) if na and nb else 0.0


def analyse(groups: dict, n_sim: int, seeds: tuple = (11, 12)) -> dict:
    names = list(groups)
    (p1, c1), (p2, c2) = (jacobian(groups, n_sim, s) for s in seeds)
    naive = [sum(1 for l in jacobi_eigen(fisher(p, c))[0] if l > 1 / SE_OK**2) for p, c in ((p1, c1), (p2, c2))]
    f = cross_fisher(p1, c1, p2, c2)
    lam, vecs = jacobi_eigen(f)
    floor = max(0.0, -min(lam))  # noise floor
    threshold = max(1 / SE_OK**2, 3 * floor)
    directions = []
    for l, vec in zip(lam, vecs):
        top = sorted(zip(names, vec), key=lambda kv: -abs(kv[1]))[:4]
        directions.append(
            {"eigenvalue": l, "se_log": 1 / math.sqrt(l) if l > 0 else None, "recoverable": l > threshold,
             "loadings": [[n, round(w, 3)] for n, w in top if abs(w) >= 0.1]}
        )
    per_param = {
        n: {"conditional_se_log": 1 / math.sqrt(f[i][i]) if f[i][i] > 0 else None,
            "seed_corr": round(_corr(c1[i], c2[i]), 3)}
        for i, n in enumerate(names)
    }
    return {
        "seeds": list(seeds),
        "n_params": len(names),
        "naive_n_recoverable_per_seed": naive,  # biased, for comparison only
        "noise_floor": floor,
        "threshold": threshold,
        "n_recoverable": sum(d["recoverable"] for d in directions),
        "directions": directions,
        "per_param": per_param,  # conditional SE = if every other parameter were known
    }


def main() -> None:
    n_sim = int(sys.argv[1]) if len(sys.argv) > 1 else 2000
    full = {n: [n] for n in full_names()}
    out = {
        "n_data": N_DATA, "n_sim": n_sim, "h": H, "se_ok": SE_OK,
        "grouped": analyse(grouped(), n_sim),
        "full": analyse(full, n_sim),
    }
    json.dump(out, sys.stdout, indent=1)


if __name__ == "__main__":
    main()
