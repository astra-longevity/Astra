# R-Sim v0 calibration: pre-registration

Status: **pre-registered** (A76, task from A28, G173 2026-10-22). Written and
committed **before** any real survival or readout file was opened by the writer,
and before any fit code exists. Tag: `prereg/sim-v0-calibration`.
**Frozen at merge; any change is a new version with a diff, never after a fit on
real data.**

## 1. What is fitted, on what

- **Model:** reduced v0 (`spec.md` §5, `identifiability.md` §4). Per sex `s`, two
  free parameters on the log scale:
  - `u_eta_s`: shared log-multiplier on `eta_i` of the **vital** axes (`model.VITAL`);
  - `u_xc_s`: shared log-multiplier on `Xc_i` of the vital axes.
  All other 50 parameters stay at the `spec.md` §5 defaults and are labelled
  **"prior, not identified"**. Published free-parameter count: **2 per sex, 4 total**,
  the size of the sealed Gompertz null.
- **Data:** right-censored lifespans of **untreated control** mice, **training
  split only**, the same arms the sealed null (`baseline.md`) is fitted on: ITP
  UM-HET3 controls, 3 sites pooled (site = issue B2), per sex.
- **Never in a fit:** treated arms, test-split arms, calorie restriction,
  fasting, or any diet arm other than the standard control diet. A committed
  filter script selects arms (`role == "control"`, `split == "train"`, diet =
  control); the fit function raises on any other arm, as `baseline.fit_gompertz`
  does. A test proves the guard.
- **Data guard (A28):** no real file is opened until A28 confirms in G173 that
  A65 has cleared the source against the sealed split.

## 2. Likelihood

Simulated likelihood on monthly bins (as `studies/recovery.py`):

- Simulate `N_SIM = 4000` mice per sex at `(u_eta, u_xc)`, common random numbers
  (fixed `SIM_SEED`), `t_max = 6` years.
- Death in month `k`: contributes `log p_k`. Censored at age `c`: contributes
  `log S(c)`, the simulated share alive at the end of month `floor(12c)`.
- `p_k = (n_k + 0.5) / (N_SIM + 0.5 · B)`, `B` = number of bins (Laplace
  smoothing, so an empty bin is never `log 0`).

## 3. Priors and fit

- **Prior:** flat on the box `|u| ≤ 1.5` (multipliers 0.22–4.5); the estimate is
  the maximum of the likelihood (no informative prior).
- **Optimiser:** Nelder-Mead (`baseline._nelder_mead`), 3 starts: `(0, 0)`,
  `(+0.3, −0.3)`, `(−0.3, +0.3)`; best `logL` kept.
- **Convergence (all required):**
  1. the 3 starts agree within 0.05 on each `u`;
  2. Monte-Carlo stability: refit with 3 other `SIM_SEED`s; spread of each `u`
     below 0.05;
  3. estimate not within 0.05 of the box edge.
- **Uncertainty:** nonparametric bootstrap over mice, 200 replicates (GitHub
  Actions), 95 % percentile interval per parameter; also the correlation of
  `u_eta` and `u_xc` (expected high: ridge of direction 1, `identifiability.md` §3).

## 4. Comparison with the Gompertz null

- The null `(a_s, b_s)` is fitted with `baseline.fit_gompertz` on the same arms.
- Both models are scored with the **same binned likelihood** (Gompertz bin
  probabilities computed exactly from `H0`), so `logL` values are comparable.
  Same parameter count (2 per sex), so no penalty term.
- Reported per sex: `ΔlogL = logL_sim − logL_null`.

## 5. Pass / fail (per sex, decided before any fit)

**Pass** if all hold:

1. converged (§3, criteria 1–3);
2. goodness of fit: binned χ² of observed vs simulated deaths (adjacent months
   merged until expected ≥ 5), `p ≥ 0.01`;
3. not clearly worse than the null: `ΔlogL ≥ −2`.

**Fail** otherwise. A fail is reported as is. No fixed parameter (beta, kappa,
eps, c, per-axis ratios) is retuned to rescue it: the next step is a new,
pre-registered version (e.g. the soft hazard of issue S2). Beating the null on
stratum (a) is **not** a pass criterion: with equal size, v0 has no edge there
(`spec.md` §5).

## 6. Synthetic recovery (before any real data)

The pipeline must first recover known parameters from simulated controls.

- Truth per sex: `u_eta = log 1.2`, `u_xc = log 0.9`, and a second truth at
  `(0, 0)`; data seeds 101, 102, 103.
- Sizes: `n = 1000` per sex (about an ITP control sex across 3 sites) and
  `n = 300`; censoring: none, and administrative censoring at 3.5 years.
- **Recovered** if `|error| < 0.10` (n 1000) and `< 0.20` (n 300) on each `u` in
  every seed, and the 95 % bootstrap interval covers the truth in at least 2 of
  3 seeds (bootstrap on Actions).
- If recovery fails, the real fit does not run.

## 7. Readouts (not in this fit)

Organ readouts are not used to fit v0 here. Their role is scored separately as
identifiability on the coverage that actually exists (A9,
`data/datasets/coverage/rsim-v0-readouts.md`; `identifiability.md` §10 and its
calibration follow-up), from the coverage table only, no readout values.

## 8. Reported with every fit

`u_eta_s`, `u_xc_s` with intervals and correlation; the 50 "prior, not
identified" parameters and their values; `ΔlogL` against the null; χ² and p;
convergence diagnostics; run manifest (code hash, seeds, data file hashes, arm
filter output).
