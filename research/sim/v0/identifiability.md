# R-Sim v0: which parameters can control survival curves recover?

**Owner:** A28 (Research). **Status:** synthetic study, no real data used, trust level L0.
Spec issue S6. Code: `py/identifiability.py`, `py/recovery.py`.

## Answer

From untreated survival curves alone, **only 2 combinations** of the 54 free
parameters can be recovered. Each per-organ parameter needs axis readouts
(metric v0) before it can be fitted. The proposed reduced v0 for
survival-only calibration has **2 free parameters** (section 4).

## 1. Method

- **Synthetic data.** 2000 control mice (`N_DATA`, about one pooled ITP control
  set per sex), deaths counted in monthly bins, censored at 6 years. This is a
  multinomial model.
- **Fisher information** for log-parameters at the default point:
  `F = N_DATA · Σ_k (∂p_k/∂θ)(∂p_k/∂θ)ᵀ / p_k`. The derivatives come from
  central differences (±10 %) with common random numbers, using 2000 simulated
  mice per evaluation.
- **Monte-Carlo bias fix.** The naive `J Jᵀ` counts simulation noise as
  information: a parameter with no effect still gets SE ≈ 0.07. `J` is
  estimated twice with independent seeds (11, 12), and `F` uses the
  symmetrised cross product `J₁ J₂ᵀ`. Its expectation is the true `F`, so
  pure-noise directions scatter around 0. The most negative eigenvalue gives
  the **noise floor**.
- **Recoverable** = eigen-direction with eigenvalue above both `1/0.2²` (SE
  below 20 % in log units) and 3 × the noise floor.
- **Two parameterisations:**
  - *grouped* (9): `eta`, `beta`, `Xc` each tied across vital and across
    non-vital axes, plus `kappa`, `eps`, `c`;
  - *full* (54).

## 2. Results (Fisher)

| Run | Params | Naive count (per seed) | Noise floor | Recoverable |
|---|---|---|---|---|
| grouped | 9 | 9, 8 | 371 | **2** |
| full | 54 | 27, 26 | 1280 | **1** |

The naive count is wrong by an order of magnitude. That is why the cross-seed
estimator is used.

Grouped run, leading directions (eigenvalue, SE in log units, main loadings):

| # | Eigenvalue | SE | Direction | Recoverable |
|---|---|---|---|---|
| 1 | 1.2e5 | 0.003 | `+Xc.vital −eta.vital +beta.vital −eps`: overall damage-to-threshold margin, which sets **median lifespan** | yes |
| 2 | 4.9e3 | 0.014 | `−eta.vital −Xc.vital +eps`: noise vs. deterministic drift, which sets the **spread / Gompertz slope** | yes |
| 3 | 579 | 0.042 | `−beta.vital +Xc.vital +eps` | no (below 3 × floor) |
| 4–9 | ≤ 293 | — | non-vital axes, `kappa`, `c` | no (inside noise) |

Per parameter (grouped). The seed correlation is the correlation of `∂p/∂θ`
between the two seeds; near 1 means a real signal, near 0 means noise.

| Parameter | Seed correlation | Signal in survival? |
|---|---|---|
| eta.vital, beta.vital, Xc.vital | 0.95 | yes, but only through directions 1–2 |
| eps | 0.85 | yes |
| kappa | 0.59 | weak |
| Xc.nonvital, eta.nonvital | 0.42, 0.28 | ≈ none |
| c, beta.nonvital | 0.20, 0.10 | none |

Full run: one direction (dominated by `eps` and `kappa`) clears the floor.
Only the `eps` column reproduces across seeds (correlation > 0.7). With 17
organs competing to kill first, the effect of any single organ on survival is
at the Monte-Carlo noise level.

## 3. Recovery fits

Truth is shifted from the defaults (`eta ×1.2`, `beta ×0.9`, `eps ×1.1`).
Nelder-Mead fits on the binned likelihood, using 1000 simulated mice per
evaluation (fixed seed) and 3 synthetic data seeds. Recovered = error below
0.1 in log units in every seed. Parameters that aren't fitted are held at the
truth.

| Fit | Param | Error, seed 101 | 102 | 103 | Recovered |
|---|---|---|---|---|---|
| 2 params | eta | −0.052 | +0.053 | −0.070 | **yes** |
| 2 params | Xc | −0.047 | +0.033 | −0.057 | **yes** |
| 3 params | eta | +0.056 | +0.025 | +0.031 | yes |
| 3 params | beta | +0.046 | +0.150 | +0.166 | **no** |
| 3 params | Xc | −0.005 | −0.068 | −0.066 | yes |

Adding `beta` breaks recovery: its error reaches 0.17 (about 18 %) because
`beta` trades off against `Xc` along direction 3, which is below the
noise floor. In the 2-parameter fit, the `eta` and `Xc` errors share their
sign in every seed, so the remaining error lies along the ridge of direction 1.
This matches the Fisher result: survival pins 2 directions and no more.

## 4. Proposed reduced v0 (survival-only calibration)

- **Fit 2 parameters** on control survival, per sex: a shared multiplier on
  `eta` (vital axes) and a shared multiplier on `Xc` (vital axes). These span
  directions 1–2.
- **Fix the rest at the defaults:** `beta`, `kappa`, `eps`, `c`, all non-vital
  axes, and the per-axis ratios. Report them as fixed, not fitted. The
  published free-parameter count is **2 per sex**.
- **Per-axis parameters (51) enter only with axis readouts.** Each organ's
  longitudinal readout (metric v0, T5 data) pins that organ's
  `eta/beta/Xc`. Non-vital axes and the coupling `c` are invisible to
  survival, so their *only* route to calibration is readouts.
  Commandment 6: they stay in the model, flagged.
- This matches the sealed null's size (Gompertz `a, b` per sex). The
  simulation then has no parameter advantage over the null on stratum (a).

## 5. Limits (issues)

| # | Limit |
|---|---|
| I1 | Local analysis at one point (the uncalibrated defaults); repeat at the calibrated point |
| I2 | Monthly-binned multinomial ignores within-month timing (small loss) |
| I3 | Finite differences at ±10 % with 2000 simulated mice; the noise floor limits what can be seen. A larger `n_sim` (Actions) could reveal weak directions, e.g. #3 |
| I4 | Hard-threshold death rule (S2); a soft hazard would change the picture |
| I5 | Cause-of-death data (necropsy) would add information per vital organ; not used here |

## Reproduce

```
cd research/sim/v0/py
python3 identifiability.py 2000 > ident.json   # ~6.5 min, stdlib only
python3 recovery.py eta,xc > rec2.json
python3 recovery.py eta,beta,xc > rec3.json
```
