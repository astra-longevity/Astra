# R-Sim v0 sealed baseline: Gompertz + one hazard ratio per class

**Binding:** D115 (T2-A50-1, T2-A28-6, T2-A50-2, T2-A10-4). Code: `py/baseline.py`.

The baseline (the "null") is a curve fit with no mechanism. The simulation
must beat it to claim anything (T2-A1-2). Its form and its scoring rules
are sealed **before** any test-set result is seen.

## 1. Definition

Per sex `s`:

```
h0_s(t) = a_s · exp(b_s · t)               Gompertz hazard of untreated controls
h(t | class k, sex s) = HR_k · h0_s(t)     proportional hazards, one HR per class
```

- `a_s, b_s`: maximum likelihood on **right-censored control** lifespans only
  (ITP UM-HET3 controls, 3 sites pooled; site as a stratum is issue B2).
  The code refuses any treated arm (`fit_gompertz`).
- `HR_k`: closed-form MLE with `h0` fixed, `HR_k = deaths / Σ H0(t)` over the
  **training** arms of class `k`. The code refuses any arm whose split
  isn't `train` (`fit_class_hr`).
- Class labels come from the sealed class list (T2-A2-4). The baseline never
  draws classes itself.

**Free parameters:** 2 per sex + 1 per training class = `4 + K`. Published
next to the simulation's count with every score.

## 2. Predictions by stratum (T2-A28-6)

| Stratum | Held-out item | Null prediction |
|---|---|---|
| (a) | new intervention, class seen in training | `HR_k` of its class |
| (b) | new class | pooled training HR (all training deaths / all training `H0`) |
| (c) | combination, or effect transferred to the other sex | pooled training HR, applied to `h0` of the target sex |

On (b) and (c) the null has nothing better than the pooled average, by design.

## 3. Score

Each held-out item is scored by the **log score**: the sum over its mice of
`log f(t)` for deaths and `log S(t)` for censored mice (`log_score`). The
simulation is scored the same way, from its simulated survival curve
(smoothed; the smoothing rule is issue B3). Item difference
`Δ = score_model − score_null`.

- **Stratum (a):** the model must be **no worse** than the null: the lower bound
  of the interval of mean `Δ` must stay above `−m`. The margin `m` and its
  interval method are fixed in the seal record before scoring (A50 amendment).
- **Strata (b), (c):** the model must **win** to claim it captures mechanism.

## 4. Sequential test per stratum (T2-A10-4, T2-A50-2)

Each stratum has its own Wald SPRT on per-item wins (`Δ > 0`):

- H0: model wins with probability `p0`; H1: with probability `p1`.
- Stop at `log LR ≥ log((1−β)/α)` → "model better"; `≤ log(β/(1−α))` → "not better".
- Otherwise **"undecided"**, published as such, never as "passed".

`p0 = 0.5, p1 = 0.75, α = 0.05, β = 0.2` in the code are **placeholders**.
A50 fixes the bounds in `verification/procedures/` before the first item
resolves. With these placeholders, 6 straight wins are needed to cross the
upper bound, so in year 1 most strata will read "undecided".

"Claims mechanism" requires (b) or (c) to cross its upper bound; a point
estimate is never enough.

## 5. What this code never does

- Never fits on treated arms for `h0`, nor on any arm in the test split.
- Never reads test-set lifespans: the split is sealed by Verification and
  held-out items arrive only after predictions are sealed.

## 6. Known issues

| # | Issue |
|---|---|
| B1 | Win/loss SPRT drops the size of `Δ`; a log-score-based test may have more power (for A50) |
| B2 | Site effects: pooled sites vs per-site `h0` (ITP site differences) |
| B3 | Smoothing of the simulated survival curve for the log score |
| B4 | No ITP data loaded yet: waits for L1 lifespan table (A71) and P1 manifests |
| B5 | Combination items in (c): null uses pooled HR, not a product of class HRs; record which in the seal |
