"""Recovery test: fit a reduced R-Sim v0 to synthetic control survival.

Companion of identifiability.py (spec issue S6). Synthetic only; no real data.

Reduced parameterisation: every organ axis keeps its default, and a fit moves
SHARED log-multipliers on eta.*, beta.*, xc.* plus kappa, eps, c. `free`
picks which of these six are fitted; the rest stay at truth.

For each data seed: simulate N_DATA "control" mice at the truth, then fit by
Nelder-Mead on the binned multinomial log-likelihood (monthly bins, censored
at T_MAX), simulated likelihood with common random numbers (fixed SIM_SEED).
Recovered = |fitted - truth| < 0.1 in log units (about 10%) in every seed.

Run: python3 recovery.py eta,beta,xc,kappa,eps,c > out.json
"""

from __future__ import annotations

import json
import math
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))  # py/: model, baseline

import baseline  # noqa: E402
import model  # noqa: E402
from identifiability import T_MAX, bin_probs  # noqa: E402

N_DATA = 2000
N_SIM = 1000
SIM_SEED = 7
DATA_SEEDS = (101, 102, 103)
MAX_ITER = 60
KEYS = {"eta": "eta.*", "beta": "beta.*", "xc": "xc.*", "kappa": "kappa", "eps": "eps", "c": "c"}
TRUTH = {"eta": math.log(1.2), "beta": math.log(0.9), "xc": 0.0, "kappa": 0.0, "eps": math.log(1.1), "c": 0.0}


def counts_at(logm: dict, n: int, seed: int) -> list[float]:
    pert = model.Perturbation({KEYS[k]: math.exp(v) for k, v in logm.items()})
    p = bin_probs(model.simulate(n, seed, perturbation=pert, t_max=T_MAX))
    return [x * n for x in p]


def fit(free: list[str], data_seed: int) -> dict:
    data = counts_at(TRUTH, N_DATA, data_seed)

    def nll(x: list[float]) -> float:
        logm = dict(TRUTH)
        logm.update(zip(free, x))
        if any(abs(v) > 1.5 for v in x):
            return 1e12
        sim = counts_at(logm, N_SIM, SIM_SEED)
        tot = sum(sim) + 0.5 * len(sim)
        return -sum(d * math.log((s + 0.5) / tot) for d, s in zip(data, sim))

    x0 = [0.0] * len(free)  # start every fitted parameter at the default
    x = baseline._nelder_mead(nll, x0, step=0.3, tol=1e-3, max_iter=MAX_ITER)
    est = dict(zip(free, x))
    err = {k: est[k] - TRUTH[k] for k in free}
    return {"data_seed": data_seed, "estimate_log": est, "error_log": err, "nll": nll(x)}


def main() -> None:
    free = sys.argv[1].split(",") if len(sys.argv) > 1 else list(KEYS)
    runs = [fit(free, s) for s in DATA_SEEDS]
    recovered = {k: all(abs(r["error_log"][k]) < 0.1 for r in runs) for k in free}
    json.dump({"free": free, "truth_log": {k: TRUTH[k] for k in free}, "n_data": N_DATA,
               "n_sim": N_SIM, "runs": runs, "recovered": recovered}, sys.stdout, indent=1)


if __name__ == "__main__":
    main()
