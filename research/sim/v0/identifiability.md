# R-Sim v0: which parameters can control survival curves recover?

**Owner:** A28 (Research). **Status:** synthetic study, no real data used, trust level L0.
Spec issue S6. Code: `py/studies/identifiability.py`, `py/studies/recovery.py`.

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

Code: `py/studies/readouts.py`, output `results/readouts.json`. Same simulator, same
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
  Repeated measures tested in section 8: they do not do it.
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

## 7. What necropsy cause-of-death data would add for vital `Xc` (no run)

Survival pins one number per sex about the vital thresholds (the margin in
direction 1); readouts pin each organ's damage/repair balance but not its
threshold (section 6.2). Cause-of-death data (O3, T4-A2-1) records, for each
dead mouse, which vital organ failed first. In the model this is exactly the
axis that crossed its `Xc` first, so the data become a multinomial over
(month of death × organ) instead of month only. Given the per-organ
trajectories from readouts, the share of deaths per organ and its age profile
depend mainly on each organ's margin `Xc_i`. Expected gain: up to 8 relative
vital thresholds (9 vital axes, minus the overall level survival already
fixes), and the only route to `A17_neoplasia`, which has no living readout:
tumour deaths are its sole signal. Limits: pathologists often report several
lesions or "undetermined" causes, so the observation is a probability over
organs, not one label [to verify, rate per ITP/JAX necropsy protocol]; the
hard-threshold death rule (I4) is too sharp for multi-organ deaths. To be
quantified with the same cross-seed Fisher once cause-of-death sampling is
added to `model.simulate` (cheap: the crossing axis is already known).

## 8. Repeated measures and realistic assays (synthetic)

Code: `py/studies/longitudinal.py`, output `results/longitudinal.json`. Same
simulator and cross-seed Fisher (seeds 11/12, `n_sim` 2000, 54 parameters),
readouts scored alone, n = 30 mice per age, assay noise 0.5 × biological SD.

**Designs.** *Cross-sectional*: different mice at 12, 18, 24 and 30 months.
*Repeated*: the same mice enrolled at 12 months and measured at every age
they reach; mice are grouped by the last age reached (1 %, 11 %, 39 % and
49 % of enrolled mice), each group is a multivariate Gaussian over its ages,
so the within-mouse correlation between ages is used. The dropout itself is
survival information and is not counted.

**Realistic assays** (A70 dossier, mouse/10–12, as relayed by A28): lung is
terminal (cross-sectional in both designs); kidney gives two values (GFR and
albuminuria) from the same mice; GTT (per lean mass) and fat mass rise then
fall (peak placed at a placeholder damage level); vaccine titre needs a new
antigen at each age, so only its spread across mice counts, not its mean.
`ideal:` = the organ state itself, repeatable, for comparison.

### 8.1 Result

| Design | Directions (n = 10 / 30 / 100) | `eta` share | `beta` share | `eta + beta` | `eta − beta` |
|---|---|---|---|---|---|
| ideal, cross-sectional | 12 / 13 / 13 | 0.52–0.58 | 0.33–0.41 | ≤ 0.04 | 0.83–0.97 |
| ideal, repeated | 13 / 13 / 13 | 0.53–0.56 | 0.35–0.42 | ≤ 0.02 | 0.86–0.96 |
| realistic, cross-sectional | 11 / 11 / 11 | see 8.2 | | ≤ 0.01 | see 8.2 |
| realistic, repeated | 6 / 6 / 6 | | | ≤ 0.01 | |

(Shares per organ at n = 30; 1 = pinned. `eta` and `beta` sit near 0.55 and
0.40 only because each is half inside the pinned balance direction.)

- **Repeated measures do not separate `eta` from `beta`.** In every organ,
  `eta + beta` (overall speed) stays at ≤ 0.02, same as cross-sectional.
  In this model the within-mouse correlation between ages carries the same
  balance information as the cross-sectional means and spreads. Production
  and repair enter the readouts only through their balance at this
  operating point.
- **With realistic assays, repeated is worse than cross-sectional here**
  (6 vs 11 directions). This is a simulation artefact, not a property of
  the design: the 4-age covariance blocks have a higher Monte-Carlo noise
  floor at `n_sim` 2000 (57 vs 32 at n = 30), and directions that fall below
  3 × floor are dropped. It does not mean repeated designs lose information.
- **Mice per age:** no new direction beyond n = 10 in any design (as in 6.3).
- **What can split `eta` from `beta`:** (1) an intervention with a mapped,
  known effect on one term (e.g. a senolytic on clearance, mappings.yaml),
  measured with readouts, so the perturbation itself is the contrast;
  (2) a repair model whose response differs from production, e.g.
  saturating repair or a recovery test after an acute challenge (organ
  measured before and after a standard insult). Both are model changes or
  new data, not more of the same readouts. Proposal: study (1) next, on
  synthetic data, with the 4 draft mappings.

### 8.2 Realistic assays, cross-sectional, n = 30 (`eta − beta` share)

| Axis | Assay | Share | Change vs ideal |
|---|---|---|---|
| A1 muscle | grip | 0.95 | = |
| A3 brain | behaviour | 0.81 | = |
| A6 heart | echo | 0.87 | = |
| A8 lung | forced oscillation (terminal) | 0.89 | = |
| A9 kidney | GFR + albuminuria | 0.81 | −0.08 (second value is redundant: same state) |
| A10 liver/metabolism | GTT per lean mass (rise then fall) | **0.24** | **lost** |
| A11 immune | vaccine titre, new antigen per age | **0.09** | **lost** |
| A12 blood | CBC | 0.86 | = |
| A13–A15 | skin, gut, fat mass (rise then fall) | 0.91–0.97 | = |

- **GTT and the vaccine lose their organ.** A rise-then-fall readout is flat
  near its peak, so a small parameter change barely moves it; a readout
  whose mean is unusable (new antigen) keeps only its spread, which is too
  little. Fat mass keeps its share because most mice stay on the rising
  side at the placeholder peak.
- **Fixes to ask for:** GTT paired with a second, monotonic liver readout
  (ALT or liver fat by imaging) to resolve which side of the peak a mouse
  is on; vaccine titre calibrated against a reference antigen given at a
  fixed age to every cohort, so the mean becomes usable again. Both go to
  the data-need list (6.4) as [to verify] with A70.
- Readout ranking (single readout, directions): CBC 3; grip, behaviour,
  echo, lung, kidney, GTT, vaccine, skin, gut, frailty, clock 2; fat mass 1.
  Direction counts barely move; what changes is which organ a direction
  belongs to (8.2 table).

### 8.3 Limits

| # | Limit |
|---|---|
| I11 | Peak position of the rise-then-fall readouts is a placeholder (`PEAK` = 1.0); the GTT loss depends on where real mice sit relative to the peak |
| I12 | Kidney's two values are both functions of one state; a real second value (albuminuria) may carry independent damage, so the redundancy is a model limit |
| I13 | Repeated-design floor higher at `n_sim` 2000; the 6 vs 11 comparison needs the Actions run (`actions-job.md`) |
| I14 | Dropout information (which mice die between visits) is counted with survival, not here |

## Reproduce

```
cd research/sim/v0/py/studies
python3 identifiability.py 2000 > ident.json   # ~6.5 min, stdlib only
python3 recovery.py eta,xc > rec2.json
python3 recovery.py eta,beta,xc > rec3.json
READOUTS_CACHE=/tmp/readouts.pkl python3 readouts.py 2000 > ../../results/readouts.json   # ~6 min, then instant from cache
LONG_CACHE=/tmp/long.pkl python3 longitudinal.py 2000 > ../../results/longitudinal.json   # ~7 min, then instant from cache
```
