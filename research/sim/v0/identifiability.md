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

## 6. Which axis readouts unlock the rest (synthetic)

Code: `py/readouts.py`, output `py/readouts.json`. Same simulator, same
cross-seed Fisher (seeds 11/12, `n_sim` 2000, all 54 parameters).

**Readout model.** One quantity per measured organ axis (11 axes; the 6
"no readout" axes are skipped), plus the frailty index (A18) and a crude
"clock" proxy (mean damage over all axes; H3 is not a state, so this is a
stand-in). Measured cross-sectionally in `n` live mice at 12, 18, 24 and 30
months. Assay noise SD = 0.5 × the between-mouse SD (placeholder).
Information comes from the mean and the variance among survivors (Gaussian).

**Readouts are scored on their own.** The survival estimate's Monte-Carlo
noise floor (~1 200) is far above what 30 mice per age contribute (~300 per
direction), so adding readouts to the survival matrix shows nothing at
`n_sim` 2000 (`all_readouts_plus_survival_by_n`: 1–2 directions). Readouts
are independent data, so their information adds to survival's; the readout
matrix alone has its own, much lower, floor (5–18).

### 6.1 Ranked by directions unlocked (n = 30 per age)

| Readout | Reads | Example assay | Directions | Params identified alone |
|---|---|---|---|---|
| echo | A6 heart | echocardiography | 3 | 0 |
| immune | A11 | blood flow cytometry | 3 | 0 |
| behaviour | A3 brain | open field, Y-maze | 2 | 0 |
| lung | A8 | plethysmography | 2 | 0 |
| kidney | A9 | BUN, urine albumin, GFR | 2 | 0 |
| gtt | A10 | glucose tolerance, ALT | 2 | 0 |
| cbc | A12 | complete blood count | 2 | 0 |
| skin | A13 | coat/skin score | 2 | 0 |
| gut | A14 | gut permeability | 2 | 0 |
| frailty | A18, all axes | clinical frailty index | 2 | 0 |
| clock (proxy) | all axes | epigenetic clock | 2 | 0 |
| grip | A1 muscle | grip strength, rotarod | 1 | 0 |
| body_comp | A15 fat | NMR / DEXA | 1 | 0 |
| **all 13** | | | **12** | **1 (`eps`)** |

Greedy order (cumulative directions): echo 3, immune 4, frailty 4,
behaviour 5, lung 6, kidney 7, gtt 8, cbc 9, skin 10, gut 11, clock 11,
grip 11, body_comp 12. Frailty and the clock proxy add no new direction once
the organ readouts are in: they average the same organs.

### 6.2 What each organ readout pins

All 13 readouts, n = 30 per age: share of each per-organ combination inside
the recoverable subspace (1 = pinned, 0 = invisible).

| Organ axis | `eta − beta` (damage/repair balance) | `eta + beta` (speed) | `Xc` (threshold) |
|---|---|---|---|
| A1 muscle | 0.94 | 0.01 | 0.00 |
| A3 brain | 0.82 | 0.02 | 0.11 |
| A6 heart | 0.86 | 0.02 | 0.09 |
| A8 lung | 0.85 | 0.01 | 0.10 |
| A9 kidney | 0.87 | 0.02 | 0.07 |
| A10 liver/metabolism | 0.84 | 0.01 | 0.10 |
| A11 immune | 0.85 | 0.01 | 0.07 |
| A12 blood | 0.86 | 0.01 | 0.09 |
| A13 skin/hair | 0.94 | 0.01 | 0.00 |
| A14 gut | 0.93 | 0.01 | 0.00 |
| A15 fat | 0.95 | 0.01 | 0.00 |

Plus `eps` (shared noise, share 0.92). So:

- **Each organ readout pins one number: that organ's damage/repair balance**
  (SE ≈ 0.05–0.06 log units at n = 30). It does not separate production
  (`eta`) from repair (`beta`), and it says nothing about the failure
  threshold `Xc`.
- **`Xc` of vital organs** can only come from survival, and per organ only
  from cause of death (necropsy, I5).
- **Separating `eta` from `beta`** needs a different design: the same mouse
  measured repeatedly (true longitudinal slopes) or a perturbation with a
  known effect on one term (an intervention with a mechanistic mapping).
  Not modelled here; next study.
- Expected v0 count after readouts: 4 (survival, per sex) + 11 organ
  balances + `eps` = **16 identified per sex**, 38 still "prior, not
  identified" (non-vital `Xc`, all `eta/beta` splits, `kappa`, `c`, 6
  unmeasured axes).

### 6.3 Mice per age

Per-mouse information per organ direction is ~9–13. SE below 0.2 needs
n ≥ 3 per age; SE ≈ 0.1 needs n ≈ 10 per age. More mice do not add
directions at `n_sim` 2000 (12 at n = 10, 30, 100): the limit is the
simulation's own noise. **Ask: n ≥ 10 per age per sex, 4 ages.**

### 6.4 Data-need list (for A9)

Sources not opened in this study: dataset contents are **[to verify]** by A9.

| Readout | Axis | Ages (months) | n per age per sex | Candidate public source |
|---|---|---|---|---|
| echocardiography | A6 | 12, 18, 24, 30 | ≥ 10 | JAX Nathan Shock Center aging studies via MPD [to verify] |
| blood flow cytometry | A11 | same | ≥ 10 | JAX aging studies / MPD [to verify] |
| complete blood count | A12 | same | ≥ 10 | SLAM (NIA) [to verify]; JAX / MPD [to verify] |
| glucose tolerance / ALT | A10 | same | ≥ 10 | SLAM [to verify] |
| kidney (GFR, albumin, BUN) | A9 | same | ≥ 10 | JAX / MPD [to verify]; GFR route per mouse/11 (Schock-Kusch 2013) |
| lung function | A8 | same | ≥ 10 | none known; terminal assay (mouse/10) [to verify] |
| behaviour (open field, Y-maze) | A3 | same | ≥ 10 | SLAM [to verify]; MPD [to verify] |
| grip / rotarod | A1 | same | ≥ 10 | SLAM [to verify] |
| body composition (NMR) | A15 | same | ≥ 10 | SLAM [to verify] |
| skin/coat score, gut permeability | A13, A14 | same | ≥ 10 | frailty index items [to verify] |
| frailty index | A18 | same | ≥ 10 | SLAM [to verify] |
| cause of death (necropsy) | vital `Xc` | at death | all deaths | ITP / JAX necropsy records [to verify] |

Priority (greedy order above): echo, immune, CBC, kidney, GTT first; they
cover vital organs.

### 6.5 Limits

| # | Limit |
|---|---|
| I6 | Readout = the organ state plus Gaussian noise. A real assay has an unknown scale and offset; calibrating them per assay costs part of the level information. Not modelled |
| I7 | Assay noise 0.5 × biological SD is a placeholder; per-assay values needed |
| I8 | Cross-sectional design; repeated measures on the same mouse would add slope information (likely what separates `eta` from `beta`) |
| I9 | Survival + readouts jointly needs `n_sim` ≫ 2000 to lower the survival noise floor (GitHub Actions) |
| I10 | Clock proxy is a mean of organ states, not a model of H3 |

## Reproduce

```
cd research/sim/v0/py
python3 identifiability.py 2000 > ident.json   # ~6.5 min, stdlib only
python3 recovery.py eta,xc > rec2.json
python3 recovery.py eta,beta,xc > rec3.json
READOUTS_CACHE=/tmp/readouts.pkl python3 readouts.py 2000 > readouts.json   # ~6 min, then instant from cache
```
