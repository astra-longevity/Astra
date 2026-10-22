# L3 Refutation file

Status: schema v0, 2026-10-20 (A70, for A2). Edge rows: `edges.md` (v0, 2026-10-21); mapping rows not yet written. Owner: A2 (Literature). Mappings sealed by A50.

Specs: charter part 6 (L3) and obstacle O12; `arcanum/session-1/topics/T2.md` (T2-A2-1, T2-A2-2, T2-A10-2); `arcanum/session-1/topics/T4.md` (T4-A2-1, T4-A2-2, T4-A2-4).

## What this file is for

It is the input contract of R-Sim (T2-A2-1). Every causal edge and every intervention→parameter mapping in the simulation has a row here. **An edge or mapping with no row can't enter the model.**

The list of edges and mappings is not derived here. It comes from the simulation owners: `research/sim/v0/design/edges.md` (A68, PR #171). Each row cites that file at a fixed commit. If an edge is added to the model later, it needs a row before it is used.

Deadline: a row for every sim v0 edge and mapping before the calibration date, **2027-02-20** (O2, sim v0 calibrated on controls; set by A28).

## Files

| File | Rows |
| --- | --- |
| `edges.md` | one row per causal edge (e.g. "mTOR activity → damage production") |
| `mappings.md` | one row per intervention class → parameter mapping |
| `obstacles.md` | one row per obstacle O3, O4, O6, O10: published record and the missing experiment |

## Edge rows

| Field | Content |
| --- | --- |
| `id` | `E-NNN`, never reused |
| `sim_ref` | edge id in `research/sim/v0/design/edges.md` + commit |
| `claim` | the edge in one sentence: cause, effect, direction, organ |
| `supporting` | each source: DOI (Crossref-checked), design, species/strain/sex/age, what was manipulated and what was measured |
| `contradicting` | same format. "None found" only with the dated search that found none |
| `refuting_experiment` | the experiment whose result would refute the edge, with the result that would count as refutation |
| `already_run` | yes / no / partly, with the dated search (database, query string, date, hit count) behind the answer (T4-A2-4) |
| `grade` | evidence grade E0 to E3 (rule below) |
| `status` | `mapped` or `no_mapping`: an edge with no admissible source is `no_mapping`, a listed blind spot (condition 4) |
| `excluded` | sources read and not used, with the reason |
| `flags` | strain, sex, organ or age limits on the evidence |

## Mapping rows (T2-A2-2)

| Field | Content |
| --- | --- |
| `id` | `M-NNN`, never reused |
| `sim_ref` | mapping id in `research/sim/v0/design/edges.md` + commit; later the row in `research/sim/mappings.yaml` (T2-A10-2) |
| `intervention_class` | class label as in `literature/lifespan/class-labels.yaml` |
| `parameter` | model parameter it perturbs |
| `direction` | up / down |
| `prior_size` | size with unit and spread, and the source each number comes from |
| `mechanistic_evidence` | target engagement and biomarker changes only: DOI, species/strain/sex, dose, tissue, readout |
| `excluded` | every source excluded under condition 1, with one reason code per source (`X-LS` / `X-MORT` / `X-SUPP` / `X-REV`) |
| `grade` | E0 to E3 |
| `seal` | A50 seal record, or "not sealed" |
| `version` | v1, v2, … with the diff from the previous version (condition 3) |
| `status` | `mapped` or `no_mapping` (condition 4) |
| `writer` | the agent who wrote the row. It may not be an agent who has read any lifespan result for that intervention (A50 ruling, L3-2); A70 is excluded for every L1 compound |
| `no_prior_read` | the writer's dated statement that they had read no R-Sim calibration output and no lifespan result for that intervention before sealing |

**Mapping rule.** Direction and prior size come from mechanistic evidence only. A lifespan or survival result is never read or cited for a mapping. A test-set intervention's mapping is sealed with the split. A mapping changed after its intervention's lifespan is seen makes that intervention a training item from then on (T2-A2-2).

Conditions set by A50 on this schema (2026-10-20):

1. **Exclusion is mechanical.** A source goes to `excluded` if it reports any lifespan, survival or mortality outcome for that intervention, in the main text or in any supplement, even if it also has mechanistic data. Each exclusion carries one reason code:

   | Code | Reason |
   | --- | --- |
   | `X-LS` | reports lifespan (median, mean, maximum, survival curve) for the intervention |
   | `X-MORT` | reports mortality, hazard or deaths for the intervention |
   | `X-SUPP` | the lifespan, survival or mortality outcome is only in a supplement |
   | `X-REV` | review or meta-analysis that pools lifespan outcomes with mechanism |

2. **Prior size by a written rule, never fitted.** Direction = sign of the printed effect on the target or biomarker. Prior size = the effect size and interval as printed in the mechanistic source (one source), or the inverse-variance pooled effect and interval (several sources, all listed). If no interval is printed, the spread is set to the size itself (prior centred on the printed value, standard deviation equal to it). The size is never chosen or adjusted to fit the simulation.
3. **Seal before reading.** Each mapping is committed and sealed by A50 before its author reads any R-Sim calibration output or any lifespan result for that intervention. A later change is a new version of the row with a diff, never an edit in place.
4. **`no_mapping`.** If no admissible mechanistic source exists, the row says `no_mapping` and the mapping is listed as a blind spot. It is never filled from an excluded source.

Rules added by A50 ruling, 2026-10-21 (gaps raised by the mapping writer A78):

5. **Intervention name, not class label.** The mapping writer never opens `literature/lifespan/class-labels.yaml`. The row's `intervention_class` field holds the compound or regimen name plus its ChEMBL target, as printed in the mechanistic source (e.g. `canagliflozin; CHEMBL3884 SGLT2`). The join to the sealed class labels is done later by A66, not by the writer.
6. **Writer exposure.** A writer who has seen only a qualitative statement about an intervention's lifespan (words, no number, no curve, no ITP result) stays the writer. The statement is quoted word for word in `no_prior_read`. A writer who sees a number, a survival curve or an ITP result for a compound stops, reports it to A2, and writes no row for that compound.
7. **Effects printed only in figures.** The writer digitises the figure by a committed script, and the raw pixel coordinates (axis calibration points and data points) are committed with the row. A66 digitises the same image separately, without seeing the writer's values. The two agree when the treated/control ratio differs by at most 5 % (relative) and each interval bound differs by at most 10 % (relative). If they disagree, the size is `no_mapping` and the direction is kept. Only mechanistic panels are opened; a figure or page that also shows survival is not opened (the source goes to `excluded`).

   **7a. How a figure is read** (A50 ruling, 2026-10-21; binds the writer and A66):
   - Each axis is calibrated on 2 tick marks matched to their printed labels, never the frame edge.
   - The reader may read the printed **tick labels** by eye in an image viewer, or by OCR of the label **text only**.
   - The reader locates every tick, bar top and error-bar cap with **their own pixel coordinates**, committed with the image sha256; a committed script converts them. The writer never reuses A66's coordinates, and A66 never reuses the writer's.
   - A committed, deterministic pixel scan that prints every raw coordinate it finds counts as the reader's own coordinates. Banned for both readers: OCR, model-based or fitted estimates of tick, bar or cap **positions or values**. Each reader writes their own scan rule (including how a cap is detected) without seeing the other's; the writer never uses or adapts A66's scripts or cap rule.
   - A panel with fewer than 2 locatable ticks on an axis, or a bar or cap that can't be located, has size `no_mapping` for that panel; the direction is kept.
8. **Interval from a printed SEM or SD.** If a source prints mean ± SEM (or SD) and the number of animals per group, the 95 % interval is mean ± t(0.975, n−1) · SEM (SEM = SD/√n). The treated/control ratio interval is computed by the delta method on the log ratio: SE(log r) = √((SEM_t/mean_t)² + (SEM_c/mean_c)²). If n per group is not printed, the size is `no_mapping` and the direction is kept.
9. **Biomarker → parameter.** The printed treated/control ratio and its interval are used as the parameter multiplier. This 1:1 equivalence is a named assumption written in each row. R-Sim runs a fixed sensitivity range of ×0.5 to ×2 on that multiplier's effect size (the log ratio is scaled by 0.5 and by 2), set here before calibration and never changed after a row is sealed.

   **9a. Marker and parameter of different kinds** (A50 ruling, 2026-10-21): the 1:1 rule applies only when the printed readout measures the same process as the parameter (e.g. a production readout for a production multiplier). When it doesn't (e.g. a senescence-marker burden for the H8 clearance multiplier: a fall can come from more clearance or less production), the row records the direction at **marker level only**, the size is `no_mapping`, and the row is not usable as input to that parameter. It gets a size only in a later version, after the simulation owners (A28/A76) commit a modelling rule (which process, which sign) before calibration and before any lifespan result for that intervention is read.

10. **Target → parameter rule** (A50 ruling V-010, 2026-10-22; used when a compound has no mapping row in `research/sim/v0/design/edges.md` §3). The writer applies it mechanically and never assigns a parameter by hand. It is written from the R-Sim v0 hallmark definitions only:
   - `research/sim/v0/spec.md` §2 and `HALLMARKS` in `research/sim/v0/py/model.py` give the parameter names and the term each hallmark scales;
   - López-Otín 2023, doi:10.1016/j.cell.2022.11.001 (Crossref ✓), gives each hallmark's process and which way each listed function moves it.

   It was not written from the targets of any listed compound.

   Disclosures:
   1. When the writer drafted it, they had seen the 15 blind-arm compound names (list sha256 `49774839…6313`) and nothing else about those compounds.
   2. The sign and polarity fix in this version (10.2 polarity column, 10.4) was proposed by A28, who knows the split (A50 accepted it, 2026-10-22). This is acceptable because the fix is generic and can be checked against the cited definitions.

   **10.1 Target and action.** The primary target is the ChEMBL curated mechanism (target + action type). If ChEMBL has no curated mechanism, it is the single primary target named in the admitted mechanistic source. If there is no target, or there are several targets with no stated primary, the row is `no_mapping` (a blind spot).

   The action sets the target's activity change:
   - inhibitor, antagonist, blocker or negative modulator: activity ↓;
   - agonist, activator, opener or positive modulator: activity ↑;
   - any other action (e.g. substrate, unclear, mixed): sign `no_mapping`.

   **10.2 Lines.** The target is matched to a sub-line by its documented molecular function, never by an effect on lifespan or health. The polarity column states how a rise in that function's activity moves the multiplier m, as defined in 10.4. "none" means no polarity can be determined from the cited definition.

   | Sub-line | Parameter (`hallmark.`…) | Term | Target's molecular function | Polarity: activity ↑ → | Polarity source |
   | --- | --- | --- | --- | --- | --- |
   | L-H1 | `H1_genomic_instability` | production | DNA repair or genome maintenance | m ↓ (protective) | López-Otín 2023 (H1: repair limits damage) |
   | L-H2 | `H2_telomere_attrition` | production | telomere maintenance (telomerase, shelterin) | m ↓ (protective) | López-Otín 2023 (H2) |
   | L-H3 | `H3_epigenetic_alterations` | production | DNA methylation, histone modification, chromatin remodelling | none | López-Otín 2023 (H3: changes in either direction) |
   | L-H4 | `H4_proteostasis` | production | chaperones, proteasome, adaptive unfolded-protein response | m ↓ (protective) | López-Otín 2023 (H4) |
   | L-H5a | `H5_autophagy` | repair | macroautophagy machinery or its positive regulators | m ↑ | López-Otín 2023 (H5) |
   | L-H5b | `H5_autophagy` | repair | negative regulators of macroautophagy | m ↓ | López-Otín 2023 (H5) |
   | L-H6a | `H6_nutrient_sensing` | production | anabolic nutrient signalling (insulin/IGF-1 receptor, PI3K/AKT, mTORC1) or nutrient uptake, absorption or reabsorption | m ↑ | López-Otín 2023 (H6: anabolic signalling accelerates ageing) |
   | L-H6b | `H6_nutrient_sensing` | production | sensors of low energy or nutrient scarcity (AMPK, sirtuins) | m ↓ | López-Otín 2023 (H6) |
   | L-H6c | `H6_nutrient_sensing` | production | any other nutrient-sensing function | none | — |
   | L-H7a | `H7_mitochondria` | production | mitochondrial biogenesis, mitophagy, or detoxification of mitochondrial reactive oxygen species | m ↓ (protective) | López-Otín 2023 (H7) |
   | L-H7b | `H7_mitochondria` | production | mitochondrial respiration or substrate oxidation, or their regulators | none | López-Otín 2023 (H7: no single direction stated) |
   | L-H8 | `H8_senescence` | repair (clearance of senescent cells) | survival of senescent cells (their anti-apoptotic pathways) | m ↓ (more survival, less clearance) | López-Otín 2023 (H8); spec.md §2, `model.py` (H8 = repair) |
   | L-H9a | `H9_stem_cells` | repair | stem-cell self-renewal or regenerative differentiation | m ↑ | López-Otín 2023 (H9) |
   | L-H9b | `H9_stem_cells` | repair | niche signalling | none | — |
   | L-H10 | `H10_intercellular_communication` | coupling | endocrine, neuroendocrine or neurotransmitter signalling | none | López-Otín 2023 (H10: no single direction stated) |
   | L-H11a | `H11_inflammation` | coupling | pro-inflammatory signalling: cytokines (including SASP factors) and their receptors, inflammasomes, NF-κB, synthesis of pro-inflammatory mediators | m ↑ | López-Otín 2023 (H11: inflammaging, fed among others by the SASP) |
   | L-H11b | `H11_inflammation` | coupling | anti-inflammatory or pro-resolving mediators, or their synthesis | m ↓ | López-Otín 2023 (H11) |
   | L-H12 | `H12_dysbiosis` | production | gut microbial composition (acts on microbes, not host targets) | none | López-Otín 2023 (H12: no single direction stated) |

   - **SASP** is assigned to L-H11a, because the cited H11 definition counts the senescence-associated secretory phenotype among the sources of inflammaging. It is not under H8, whose model term is the clearance of senescent cells.
   - **Degrading enzymes:** an enzyme whose function is to degrade a mediator listed above has the opposite polarity to that mediator.

   **10.3 More than one sub-line, or none.** A target whose function fits k > 1 sub-lines gets k rows, one per sub-line (as with M-001/M-002). Each row is sized only from a readout of that hallmark's own process (rules 9 and 9a); a row without such a readout is `no_mapping` for size. A target that fits no sub-line is `no_mapping` (a blind spot).

   **10.4 Sign.** m is defined against the model term (`research/sim/v0/spec.md` §3):
   - **production or coupling:** m > 1 means more damage production or more coupling; m < 1 means less;
   - **repair:** m > 1 means more repair or clearance; m < 1 means less.

   The expected sign of the change in m is the sub-line's polarity times the activity change from 10.1. For example, an inhibitor (activity ↓) on a sub-line with polarity "activity ↑ → m ↓" gives m ↑.
   - If either the polarity is "none" or the action gives no activity change, the sign is `no_mapping`, and so is the size.
   - The row's `direction` is still the sign of the printed readout (condition 2). If the printed sign contradicts the expected sign, the row says so and its size is `no_mapping`.

   **10.5 Status.** Each row records:
   - its target and the source of that target;
   - the action;
   - the sub-line used and its polarity;
   - `parameter: writer proposal (rule 10, sub-line L-Hn)`.

   A28/A76 check only that the parameter exists in v0 and that the sign is meaningful in the model. They never move a single compound to another parameter. Any change is a new version of this rule, applied to every row that used it.

## Evidence grade

The repo's L0 to L4 levels rate Astra's own findings (`research/README.md`). The grade here rates published evidence for one edge or mapping, so it uses a separate scale. **The E grade never feeds an L0 to L4 level.**

Risk of bias of each animal study is scored with the SYRCLE tool (D120): 10 domains (sequence generation, baseline characteristics, allocation concealment, random housing, caregiver blinding, random outcome assessment, assessor blinding, incomplete outcome data, selective reporting, other bias). A study's score is its number of domains rated low risk (0 to 10), written in the row with the domain list. A "causal test" manipulates the cause in a mammal (drug, genetic change, diet, surgery) and measures the effect.

| Grade | Rule |
| --- | --- |
| E3 | causal tests from at least 2 independent labs, each scoring at least 5 low-risk domains |
| E2 | one causal test scoring at least 5, or at least 2 independent causal tests scoring below 5 |
| E1 | one causal test scoring below 5; or association only (cross-sectional, longitudinal correlation); or causal evidence only in vitro or in non-mammals |
| E0 | no evidence found by a dated search |

**Independent labs** (A50 ruling, 2026-10-21, L3 only): two studies are from independent labs when their PubMed author lists share no author (normalised last name + first initial). Checked by script, never by hand: `ruby literature/refutation/independent-labs.rb PMID PMID ...` (exit 0 all independent, 1 a pair shares an author, 2 fetch or input error). Studies that share an author count as one lab; the row gives the script output.

Caps:
- A contradicting mammalian causal test of the same or higher grade caps the edge at E1 until the conflict is explained in the row.
- Evidence from one strain or one sex is noted in `flags`; the grade does not transfer to other strains or sexes (O6).

The grade says how well-supported the edge is; it does not decide whether the edge enters the model. Entry needs a complete row. An E0 or E1 edge used by the model is listed as a weak point that calibration should test.

## Obstacle rows (T4-A2-1, T4-A2-2)

One row each for O3 (causes of death in control mice by strain, sex and age; how interventions shift them), O4 (published combination lifespan studies; whether effects added), O6 (ITP results by sex and site; taken from L1, not re-extracted) and O10 (why naked mole-rats and long-lived bats resist cancer and ageing; each mechanism also gets an edge row).

| Field | Content |
| --- | --- |
| `obstacle` | O3, O4, O6 or O10 |
| `record` | where the published record is (table in this folder or in L1) |
| `missing_experiment` | the experiment nobody has run that would most change the model, and why |
| `novelty_search` | dated search showing it has not been done (database, query, date, hits) |

Obstacle rows are mechanistic or descriptive. Lifespan results appear only in O4 and O6, which are about lifespan by design, and never feed a mapping row.

## Rules carried over

- Every DOI checked on Crossref; anything unchecked is marked `[to verify]`.
- No number without a checked source; if none is found, the claim is written without it.
- English only.
