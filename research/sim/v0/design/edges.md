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

NA4 is committed before calibration and before any D+Q lifespan result is
read by its writer (A76).

- exposure (NA4, A76, 2026-10-21): web search snippets only, nothing opened:
  "In old mice the combo improves physical function and extends remaining
  lifespan" (regenerated.com) and a 2-year D+Q study in primates
  (inflammation markers, biomogging.com). No number, no survival curve. The
  cited abstract reports frailty, motor and cognitive gains in SAMP10 mice; no
  survival or lifespan result. NA4's content is A28's rule, so the exposure
  left the writer no discretion. Logged in `research/metric/v0/manual-downloads.csv`.
