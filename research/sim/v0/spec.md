# R-Sim v0 specification: organ-level damage and repair

**Owner:** A28 (Research). **Status:** scaffold, uncalibrated, trust level L0.
**Binding:** commandments 1, 8, 9; charter part 2; D115 (`arcanum/session-1/topics/T2.md`).

## 1. What the model is

A stochastic model of one mouse. Each **axis** (organ system) carries a
damage level `X_i ≥ 0` that is produced, repaired, and coupled to the other
axes through the blood (inflammation, hormones). The mouse dies when a vital
axis fails. Deeper levels (cell populations, molecular pathways) are added
**only** after a recorded failure of this level (T2-A28-2), each tied to an
issue in the register.

Parent models, which v0 extends:

- Karin, Agrawal, Porat, Krizhanovsky, Alon 2019, *Senescent cell turnover
  slows with age providing an explanation for the Gompertz law*, Nat Commun
  10:5495, doi:10.1038/s41467-019-13192-4. Damage production that rises with
  age, saturating removal, and noise give Gompertz mortality. **Source of the
  per-axis equation below.**
- Farrell, Mitnitski, Rockwood, Rutenberg 2016, *Network model of human aging:
  frailty limits and information measures*, Phys Rev E 94:052409,
  doi:10.1103/PhysRevE.94.052409. Damage spreads between connected nodes and
  death follows from failed hub nodes. **Source of the coupling and of the
  vital-axis death rule.**

Both DOIs were checked on the publisher pages. Their published fits and known
failure modes still need checking by Literature (T2-A2-3); until then v0 takes
no claim from them.

## 2. State: the axes

The state variables **are the metric's axes** (T2-A28-1), taken from
`research/metric/v0/axes.md` (A67, PR #45). Its 30 axes map as follows:

- **Organ-system axes A1–A17** are the state `X_i` (17 states).
- **A18 whole-body frailty** is a derived readout, `frailty(x) = mean_i min(1, X_i/Xc_i)`,
  not a state. Making it a state would double-count the other axes
  (axes.md issue I-4).
- **Hallmarks H1–H12** are processes, not organs. Each is a multiplier `m_h`
  (1 in untreated mice) on damage production, repair or coupling of the organ
  axes it drives (section 3). Interventions act mainly here (section 6).

"No readout" = axes.md status `unmeasured`: the state is simulated and
predicted, but no public data can check it yet (commandment 6: kept and
published as a blind spot, never dropped).

| Axis | Vital | Readout |
|---|---|---|
| A1 muscle | no | molecular only |
| A2 bone | no | **no readout** |
| A3 brain | yes | molecular only |
| A4 hearing | no | **no readout** |
| A5 vision | no | **no readout** |
| A6 heart | yes | molecular only |
| A7 vessels | yes | **no readout** |
| A8 lung | yes | molecular only |
| A9 kidney | yes | molecular only |
| A10 liver and metabolism | yes | candidate |
| A11 immune | yes | molecular only |
| A12 blood | yes | molecular only |
| A13 skin and hair | no | candidate |
| A14 gut | no | molecular only |
| A15 fat | no | candidate |
| A16 endocrine and reproduction | no | **no readout** |
| A17 neoplasia | yes | **no readout** (O3, issue S3) |
| A18 frailty | derived | candidate |

| Hallmark | Scales | Organ axes driven | Readout |
|---|---|---|---|
| H1 genomic instability | production | all | **no readout** |
| H2 telomere attrition | production | A11, A12, A13, A14 (high-turnover tissues) | **no readout** |
| H3 epigenetic alterations | production | all | candidate |
| H4 proteostasis | production | all | molecular only |
| H5 macroautophagy | repair | all | **no readout** |
| H6 nutrient sensing | production | all | candidate |
| H7 mitochondria | production | all | molecular only |
| H8 senescence | repair | all | candidate |
| H9 stem cell exhaustion | repair | A1, A2, A11, A12, A13, A14 | molecular only |
| H10 intercellular communication | coupling | all | **no readout** |
| H11 chronic inflammation | coupling | all | molecular only |
| H12 dysbiosis | production | A10, A11, A14 | **no readout** |

Count check: 6 organ + 5 hallmark = 11 unmeasured, as in axes.md.

Vital set: an axis is vital when its failure kills a mouse quickly. Muscle,
bone, senses, skin, gut, fat and endocrine failure are disabling, not
directly lethal in a lab cage. This is a modelling choice (issue S11).

Each hallmark-to-organ link is a causal edge. Under T2-A28-2 each needs an L3
refutation row before calibration (issue S10).

## 3. Dynamics

For each axis `i`, with `t` the age in years:

```
dX_i = [ eta_i · P_i · t · (1 + c · C · I)  −  beta_i · R_i · X_i / (kappa + X_i) ] dt  +  sqrt(2·eps) dW_i
I    = mean_j ( X_j / Xc_j )
P_i  = mean of m_h over production hallmarks driving axis i
R_i  = mean of m_h over repair hallmarks driving axis i
C    = mean of m_h over coupling hallmarks (H10, H11)
```

In untreated mice every `m_h = 1`, so `P_i = R_i = C = 1`.

| Term | Meaning |
|---|---|
| `eta_i · t` | **damage production**, rising linearly with age (Karin 2019) |
| `beta_i · X_i / (kappa + X_i)` | **repair / removal**, saturating: repair capacity is overwhelmed as damage grows |
| `1 + c · I` | **coupling**: systemic damage (inflammation, hormones) speeds production on every axis (Farrell 2016) |
| `sqrt(2·eps) dW_i` | **noise**: independent per axis, gives individual variation in lifespan |

`X_i` is reflected at 0. Integrated by Euler–Maruyama with a weekly step
(`dt = 7/365.25`), seeded generator (`random.Random(seed)`, T2-A10-1).

## 4. Mapping to mortality

Death occurs at the first time a **vital** axis crosses its threshold `Xc_i`.
Hazard isn't a separate input: it comes out of first-passage times, which is
what makes the Gompertz shape a result rather than an assumption. Simulated
mice still alive at `t_max` (6 years) are right-censored.

Open choice (issue S2): a soft hazard `h = h0 · exp(Σ w_i X_i/Xc_i)` would
allow deaths before a threshold is reached. It adds `w_i` parameters, so it only
enters after the hard threshold fails a test.

## 5. Free parameters (published with every score)

| Parameter | Per | Count |
|---|---|---|
| `eta_i` damage production slope | organ axis | 17 |
| `beta_i` maximum repair rate | organ axis | 17 |
| `Xc_i` failure threshold | organ axis | 17 |
| `kappa` half-saturation of repair | shared | 1 |
| `eps` noise intensity | shared | 1 |
| `c` coupling strength | shared | 1 |
| **Total** | | **54** |

Hallmark multipliers `m_h` are fixed at 1 in untreated mice and move only
through sealed mappings (section 6). They are never fitted, so they add 0 free
parameters. Each mapping's prior is published with the score.

Identifiability (`identifiability.md`): control survival alone recovers only
2 directions of the 54 parameters. **Reduced v0 (accepted by A28):** per sex,
fit a shared multiplier on `eta` and one on `Xc` over the vital axes
(4 fitted parameters for both sexes, the size of the Gompertz null). The other
52 stay at the stated defaults and are labelled **"prior, not identified"**
in this spec and in every score's parameter count, until axis readouts pin
them (`identifiability.md`, readouts section). `Params.free_parameter_count()`
returns 54; a fit that ties parameters reports the reduced count in its
manifest.

Consequence for scoring: with 4 fitted parameters on controls, the simulation
has no edge over the null on stratum (a). It can only beat the null through
the mechanistic mappings (section 6), i.e. on strata (b) and (c).

Defaults are **uncalibrated placeholders** (`eta = 1`, `beta = 2`, `Xc = 3.6`,
`kappa = 0.5`, `eps = 0.6`, `c = 0.3`) chosen only so that the untreated median
lifespan lands near 2.5 years. Calibration on ITP **control** survival (UM-HET3,
both sexes, 3 sites, per sex) comes later, under a run manifest.

## 6. Interventions: mapping from mechanism only

An intervention is a **perturbation**: multipliers on named parameters from a
start age, e.g. `{"eta.*": 0.8}` from 20 months. Rules (T2-A2-2, T2-A10-2,
T2-A50-3):

1. Each mapping is a row in `research/sim/mappings.yaml` (format in that
   file): target parameter, direction, prior size with interval, the
   mechanistic evidence (target engagement, biomarker change; **never
   lifespan**), and the refutation-file row (L3) it rests on. An edge without
   an L3 row can't enter the model (T2-A2-1).
2. The mapping is sealed by Verification (A50) **before** the modeller sees
   that intervention's lifespan result. Class labels, split and strata are
   sealed together in one record (T2-A10-5).
3. A mapping changed after a lifespan result is seen makes every later
   prediction using it exploratory until a new held-out item resolves.
4. The code accepts only known parameter names (`KeyError` otherwise); it
   doesn't read `mappings.yaml` yet (issue S4).

Combinations multiply their mappings' multipliers. **This is a named
assumption (NA1 in `design/edges.md`), not calibrated:** no ITP combination has
both singles in the same cohort, so additivity can't be fitted from ITP (A70,
obstacle O4). Every combination prediction is labelled "NA1, uncalibrated";
stratum (c) tests it, and a factorial arm (both singles + combo, same cohort)
is a first-mouse-request candidate.

A senolytic acts on **H8 clearance only**: its multiplier raises H8
clearance, production is unchanged, so a senescence-marker fall is read as
clearance. **This is named assumption NA4** (`design/edges.md`, mapping
M-005), set before calibration. Pre-registered sensitivity: the same marker
fall read as production down instead; calibration reports both.

Size (**NA4.1**, `design/edges.md`): a marker ratio r maps to the H8
multiplier **m = r^−0.5**, the log-midpoint of the model-consistent range
[1, 1/r]. That range comes from the parent law (Karin 2019) applied to an
auxiliary marker pool: m = (1/r)(K + rS)/(K + S), 1/r if clearance is
first-order, 1 if saturated; S/K is unknown in v0. m reaches an organ only
through the hallmark mean (NA2): repair factor (m + n_i − 1)/n_i.
Sensitivity, all reported: m = 1/r; m = 1; production × r; stress line
m = r^−2 (outside model range, never a bound). Marker not at steady state is a
named limit (m then understated). **Frozen at merge; any change is a new version
with a diff, never after an M-005 size or a calibration run.**

Several panels (**NA4.2**, `design/edges.md`, A28 rule, text verbatim; generic for
every L3 mapping with ≥ 2 agreed panels):

1. **Panels:** every panel on which both readers agree within the 5%/10% rule;
   no choice among them, no preference by marker (p16 and SA-β-gal equal standing).
2. **Combine:** random-effects meta-analysis on log r (DerSimonian–Laird), weights
   from each panel's own interval; with k < 3, use the wider of the fixed- and
   random-effects intervals.
3. **Carry through:** the pooled log r and its interval go through NA4.1 (r^-0.5
   primary; 1/r, 1, production × r always reported) and then (m+n−1)/n. If the
   pooled interval includes r = 1, m's interval includes 1 and calibration carries
   it; no "effect assumed".
4. **Always reported:** each panel alone as a sensitivity line, plus the
   between-panel heterogeneity (I², τ²), named as a limit when panels disagree.

Done by `py/design/pool_panels.py`; fixed-effect result reported next to the pooled
line; at k = 2, τ² reported as "weakly estimated". **Frozen at merge; any change is a
new version with a diff, never after an M-005 size or a calibration run.**

## 7. Scoring

The model is scored against the sealed Gompertz + hazard-ratio null on
three strata. The rules are in `baseline.md`. The simulation ranks no
treatment before the metric passes its six tests (commandment 9, D115).

## 8. Known issues (commandment 8)

| # | Issue |
|---|---|
| S1 | Done on this branch: state = axes.md A1–A17, A18 derived, H1–H12 multipliers. Re-sync when axes.md changes |
| S2 | Hard threshold vs soft hazard (section 4) |
| S3 | Cancer, the main cause of death in mice, is axis A17 with the same damage/repair form as the others; real dynamics (clonal growth, immune surveillance from A11) are needed (O3) |
| S4 | `mappings.yaml` isn't parsed; CI check of the mapping seal (T2-A10-2) not built |
| S5 | Sex: one parameter set per sex for now; sex transfer (stratum c) needs a shared structure |
| S6 | Identifiability (section 5): survival alone recovers 2 directions of 54 (`identifiability.md`); survival-only calibration fits 2 params per sex until longitudinal axis data exist |
| S7 | Manifest is a minimal local format; align with A10's P2 run manifests |
| S8 | Parent models' fits and failure modes not yet checked by Literature (T2-A2-3) |
| S9 | Pure-Python speed: ~0.3 s per 300 mice; calibration will need vectorising or the Rust port (T2-A10-3) |
| S10 | Hallmark → organ links (section 2) are asserted, not sourced; each needs an L3 refutation row and a mechanistic source before calibration |
| S11 | Vital set is a modelling choice; check against ITP necropsy causes of death when available |
| S12 | Hallmark factors are a plain mean; a hallmark driving all axes weighs the same as one driving three. Weights would add parameters, so only after a test fails |

These move to the repo issue register when it exists.
