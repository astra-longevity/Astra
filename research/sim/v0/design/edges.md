# R-Sim v0: causal edges and class mappings (for the L3 refutation file)

One row per causal edge in the model and one row per class → parameter mapping,
extracted from `../spec.md` (§2–§4, §6), `../py/model.py` and
`../../mappings.yaml`. No new modelling. Each row needs an L3 refutation row
(`literature/refutation/`) before calibration (T2-A28-2, spec issue S10); an
edge or mapping without one can't enter a scored run.

Status: model uncalibrated, L0. Sources below are the ones the spec cites;
"[to verify]" = no source opened for that specific link.

## 1. Structural edges (every axis, every mouse)

| ID | From | To | Term hit | Sign | Parameter(s) | Source |
|---|---|---|---|---|---|---|
| E1 | age `t` | damage `X_i`, every organ axis | production (`eta_i · t`) | + (production rises linearly with age) | `eta.<axis>` | Karin 2019, doi:10.1038/s41467-019-13192-4 |
| E2 | own damage `X_i` | repair of `X_i` | clearance (`beta_i · X_i / (kappa + X_i)`) | − (repair saturates as damage grows) | `beta.<axis>`, `kappa` | Karin 2019 (saturating removal), same DOI |
| E3 | systemic damage `I = mean_j X_j / Xc_j` | damage `X_i`, every axis | production (factor `1 + c · C · I`) | + (damage spreads between organs) | `c` | Farrell 2016, doi:10.1103/PhysRevE.94.052409 |
| E4 | vital axis damage `X_i` (A3, A6, A7, A8, A9, A10, A11, A12, A17) | death | first passage `X_i ≥ Xc_i` | + | `xc.<axis>` | Farrell 2016 (death from failed hub nodes), same DOI; vital set is a modelling choice (S11) [to verify] |
| E5 | noise | damage `X_i` | diffusion `sqrt(2·eps) dW_i` | ± (individual variation) | `eps` | modelling choice [to verify] |

A18 frailty is a derived readout (mean of organ states), not a causal edge.
Non-vital axes (A1, A2, A4, A5, A13, A14, A15, A16) reach death only through E3.

## 2. Hallmark → organ edges

A hallmark multiplier `m_h` (1 in untreated mice) scales one term on the organ
axes it drives (`P_i`, `R_i` = mean of the production / repair multipliers
driving axis `i`; `C` = mean of the coupling ones). Each hallmark × organ pair is
one edge for L3; the table groups pairs by hallmark. Hallmark list: axes.md
Table B (López-Otín 2023 [to verify DOI]). **No organ link below has a source
opened yet** (S10): every link is [to verify].

| ID | Hallmark | Term hit | Sign (on damage) when `m_h` > 1 | Organ axes driven | Pairs | Source of link |
|---|---|---|---|---|---|---|
| E-H1 | H1 genomic instability | production | + | all 17 | 17 | [to verify] |
| E-H2 | H2 telomere attrition | production | + | A11, A12, A13, A14 | 4 | [to verify] (high-turnover tissues) |
| E-H3 | H3 epigenetic alterations | production | + | all 17 | 17 | [to verify] |
| E-H4 | H4 proteostasis | production | + | all 17 | 17 | [to verify] |
| E-H5 | H5 macroautophagy | clearance | − | all 17 | 17 | [to verify] |
| E-H6 | H6 nutrient sensing | production | + | all 17 | 17 | [to verify] |
| E-H7 | H7 mitochondria | production | + | all 17 | 17 | [to verify] |
| E-H8 | H8 senescence (clearance of senescent damage) | clearance | − | all 17 | 17 | [to verify] |
| E-H9 | H9 stem cell exhaustion (regenerative repair) | clearance | − | A1, A2, A11, A12, A13, A14 | 6 | [to verify] |
| E-H10 | H10 intercellular communication | coupling (`C` in E3) | + | all 17 | 17 | [to verify] |
| E-H11 | H11 chronic inflammation | coupling (`C` in E3) | + | all 17 | 17 | [to verify] |
| E-H12 | H12 dysbiosis | production | + | A10, A11, A14 | 3 | [to verify] |

Total hallmark × organ pairs: 166. Pair IDs for L3 rows: `E-H<n>-A<m>`
(e.g. `E-H2-A12`). H8 and H9 multipliers are named after the damage process
but scale its **clearance**: `m_h` > 1 means more removal, less damage
(mappings.yaml direction semantics).

## 3. Class → parameter mappings (DRAFT, superseded)

Drafted by A68 from `../../mappings.yaml` before the L3 rules. A68 was not a
lifespan-naive mapping writer, so these rows carry no prior and no source: each
is superseded by the L3 row `M-00n` in `literature/refutation/mappings.md`
(literature, A78) and **not used in calibration**. Class, parameter, term and
edge are kept only as pointers.

| ID | Class | Parameter | Term | Edge it rides on | Status |
|---|---|---|---|---|---|
| M1 | mTORC1 inhibitor | `hallmark.H6_nutrient_sensing` | production | E-H6 | superseded by L3 `M-001` (literature, A78), not used in calibration |
| M2 | mTORC1 inhibitor | `hallmark.H5_autophagy` | clearance | E-H5 | superseded by L3 `M-002` (literature, A78), not used in calibration |
| M3 | α-glucosidase inhibitor | `hallmark.H6_nutrient_sensing` | production | E-H6 | superseded by L3 `M-003` (literature, A78), not used in calibration |
| M4 | SGLT2 inhibitor | `hallmark.H6_nutrient_sensing` | production | E-H6 | superseded by L3 `M-004` (literature, A78), not used in calibration |
| M5 | senolytic | `hallmark.H8_senescence` | clearance | E-H8 | superseded by L3 `M-005` (literature, A78), not used in calibration |

Caloric-restriction mimetic: no row (class overlaps M3/M4; waits for the
sealed class list).

## 4. Named assumptions (not calibrated)

| ID | Assumption | Why not calibrated | Flag |
|---|---|---|---|
| NA1 | **Combination rule:** a combination multiplies its classes' multipliers (no interaction term) | No ITP combination has both singles in the same cohort (A70, `literature/refutation/obstacles.md` O4; PubMed factorial search 0 hits) | Every combination prediction carries "NA1, uncalibrated"; a factorial arm (both singles + combo, same cohort) is a first-mouse-request candidate |
| NA2 | Hallmark factors are a plain mean over the hallmarks driving an axis | Weights would add parameters (S12) | — |
| NA3 | Vital set (E4) | Modelling choice (S11) | — |
| NA4 | **Senolytic acts on H8 clearance only** (L3 mapping M-005, M5): the H8 clearance multiplier goes up (sign +), damage production is unchanged. A fall in a senescence marker under a senolytic is attributed entirely to clearance. Basis: the class is defined by its mechanism, selective removal of senescent cells (PMID 35165353, doi:10.1038/s41598-022-06448-5, abstract only, class-definition sentence; its in vitro clearance result is not stated in the abstract: [to verify]). Rule set by A28, 2026-10-21 | No source separates clearance from production for D+Q; one marker fall can't split them | **Pre-registered sensitivity:** calibration also runs the same marker fall read as production down (an H8-driven production multiplier < 1, clearance unchanged) and reports both. Size stays `no_mapping` until A78's tick-reading rules commit is on main |
| NA4.1 | **Size transform for NA4:** a senescence-marker ratio r (treated / control, r < 1) maps to the H8 clearance multiplier **m = r^−0.5** (primary, A28 rule v2, 2026-10-21): the log-midpoint of the model's own range [1, 1/r], because S/K is unknown in v0. No data used and no D+Q lifespan result read, so the choice is not outcome-steered. Derivation, no data: v0 has no senescent-cell state (H8 is a multiplier on organ repair only, `model.py`), so the marker is an auxiliary pool S following the parent law of v0 (Karin 2019, doi:10.1038/s41467-019-13192-4): dS/dt = p − k·S/(K + S). At steady state with clearance × m and p unchanged (NA4): **m = (1/r)·(K + r·S)/(K + S)**. First-order clearance (K ≫ S) gives m = 1/r; saturated clearance (S ≫ K) gives m → 1. So 1/r is the upper end of the model-consistent range [1, 1/r] (most favourable case) and 1 the lower end; r^−0.5 sits at its log-midpoint. In v0, m enters organ repair through the plain hallmark mean (NA2): repair factor on axis i = (m + n_i − 1)/n_i, n_i = 2 (H5, H8) or 3 (+ H9: muscle, bone, immune, blood, skin, gut). m is never applied to beta directly | S/K for senescent cells is not known; no source separates it | **Frozen at merge; any change is a new version with a diff, never after an M-005 size or a calibration run.** Pre-registered sensitivity, always reported together: (i) m = 1/r (first-order end, K ≫ S); (ii) m = 1 (saturated end, no effect); (iii) production × r, clearance unchanged (NA4 line); stress line only: m = r^−2, labelled "outside model range", never a bound; **limit, not corrected:** if the source's timing does not let the marker reach steady state, 1/r understates m. Committed before calibration; no D+Q lifespan result read |
| NA4.2 | **Combining agreed panels into one H8 input** (A28 rule, G158 2026-10-21 18:21:52, text verbatim; generic for every L3 mapping with ≥ 2 agreed panels): 1. **Panels:** every panel on which both readers agree within the 5%/10% rule; no choice among them, no preference by marker (p16 and SA-β-gal equal standing). 2. **Combine:** random-effects meta-analysis on log r (DerSimonian–Laird), weights from each panel's own interval; with k < 3, use the wider of the fixed- and random-effects intervals. 3. **Carry through:** the pooled log r and its interval go through NA4.1 (r^-0.5 primary; 1/r, 1, production × r always reported) and then (m+n−1)/n. If the pooled interval includes r = 1, m's interval includes 1 and calibration carries it; no "effect assumed". 4. **Always reported:** each panel alone as a sensitivity line, plus the between-panel heterogeneity (I², τ²), named as a limit when panels disagree. Pooling, the fixed/random comparison and the wider-interval pick are done by `py/design/pool_panels.py` (stdlib; tests on synthetic inputs only); the fixed-effect result is reported next to the pooled line; at k = 2, τ² is reported as "weakly estimated" | Agreement between panels is not known before the readings; weights come from the intervals, not chosen | **Frozen at merge; any change is a new version with a diff, never after an M-005 size or a calibration run.** Heterogeneity between panels = named limit, not corrected |

NA4, NA4.1 and NA4.2 are committed before calibration and before any D+Q lifespan result is
read by its writer (A76).

- exposure (NA4, A76, 2026-10-21): web search snippets only, nothing opened:
  "In old mice the combo improves physical function and extends remaining
  lifespan" (regenerated.com) and a 2-year D+Q study in primates
  (inflammation markers, biomogging.com). No number, no survival curve. The
  cited abstract reports frailty, motor and cognitive gains in SAMP10 mice; no
  survival or lifespan result. NA4's content is A28's rule, so the exposure
  left the writer no discretion. Logged in `research/metric/v0/manual-downloads.csv`.
