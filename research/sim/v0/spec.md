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

The state variables **are the metric's axes** (T2-A28-1). The metric's axis
list (`research/metric/v0/axes.md`, sister worker A67) hasn't landed yet, so
v0 uses organ systems as placeholders:

| Axis | Vital (death on failure) |
|---|---|
| cardiovascular | yes |
| renal | yes |
| metabolic_hepatic | yes |
| immune_haematopoietic | yes |
| neural | yes |
| musculoskeletal | no (feeds coupling only) |
| neoplastic | yes (placeholder cancer axis: malignant clone burden; O3, issue S3) |

When `axes.md` lands, `AXES` and `VITAL` in `py/model.py` are replaced by its
list. Any axis the metric reads must be a state here, and vice versa
(commandment 6: no blind spots).

## 3. Dynamics

For each axis `i`, with `t` the age in years:

```
dX_i = [ eta_i · t · (1 + c · I)  −  beta_i · X_i / (kappa + X_i) ] dt  +  sqrt(2·eps) dW_i
I    = mean_j ( X_j / Xc_j )
```

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
| `eta_i` damage production slope | axis | 7 |
| `beta_i` maximum repair rate | axis | 7 |
| `Xc_i` failure threshold | axis | 7 |
| `kappa` half-saturation of repair | shared | 1 |
| `eps` noise intensity | shared | 1 |
| `c` coupling strength | shared | 1 |
| **Total** | | **24** |

Identifiability: from survival curves alone, `eta`, `beta` and `Xc` per axis
are not separately identifiable (only their ratios shape mortality). The
longitudinal axis readings (T5 data, metric v0) are what pin each axis
separately. Until those exist, calibration fits shared values and reports the
effective count. `Params.free_parameter_count()` returns 24; a fit that ties
parameters reports the reduced count in its manifest.

Defaults are **uncalibrated placeholders** (`eta = 1`, `beta = 2`, `Xc = 3`,
`kappa = 0.5`, `eps = 0.6`, `c = 0.3`) chosen only so that the untreated median
lifespan lands near 2.3 years. Calibration on ITP **control** survival (UM-HET3,
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

Combinations multiply their mappings' multipliers. This is the simplest
prior; whether combinations add up is obstacle O4, so stratum (c) tests it.

## 7. Scoring

The model is scored against the sealed Gompertz + hazard-ratio null on
three strata. The rules are in `baseline.md`. The simulation ranks no
treatment before the metric passes its six tests (commandment 9, D115).

## 8. Known issues (commandment 8)

| # | Issue |
|---|---|
| S1 | Placeholder axes; replace with `research/metric/v0/axes.md` |
| S2 | Hard threshold vs soft hazard (section 4) |
| S3 | Cancer, the main cause of death in mice, is only a placeholder `neoplastic` axis with the same damage/repair form as the others; real dynamics (clonal growth, immune surveillance from `immune_haematopoietic`) are needed (O3) |
| S4 | `mappings.yaml` isn't parsed; CI check of the mapping seal (T2-A10-2) not built |
| S5 | Sex: one parameter set per sex for now; sex transfer (stratum c) needs a shared structure |
| S6 | Identifiability (section 5) until longitudinal axis data exist |
| S7 | Manifest is a minimal local format; align with A10's P2 run manifests |
| S8 | Parent models' fits and failure modes not yet checked by Literature (T2-A2-3) |
| S9 | Pure-Python speed: ~0.3 s per 300 mice; calibration will need vectorising or the Rust port (T2-A10-3) |

These move to the repo issue register when it exists.
