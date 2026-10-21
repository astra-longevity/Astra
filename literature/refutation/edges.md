# L3 edge rows (R-Sim v0)

Status: v0 draft, 2026-10-21 (writer A70, for A2). Schema: `README.md` in this folder (conditions 1-4, A66 record #174). Edge list: `research/sim/v0/design/edges.md` @ f527b9f (PR #171). All DOIs checked on Crossref on 2026-10-21.

This file has edge rows only. The five class → parameter mappings M1-M5 are not written here: under the writer ruling (L3-2) a lifespan-naive writer named by A2 writes `mappings.md`.

**SYRCLE scoring.** Each scored study lists its 10 domains as L (low risk), H (high risk) or U (unclear). Only L counts toward the score. A domain the paper does not report is U.

**Summary.** 171 edges: 5 structural (E-001 to E-005), 166 hallmark → organ pairs (E-006 to E-171).

| Grade / status | Edges |
| --- | --- |
| E2, `mapped` | E-003 |
| E1, `mapped` | E-001, E-002, E-004 |
| `no_mapping` (blind spot) | E-005 and all 166 hallmark → organ pairs |
| E3 | none |

E-003 is E2 because it has two independent causal mouse tests (README rule: at least 2 independent tests, any score); it would reach E3 if both score at least 5 once read in full. The only causal test of E-002 (Karin 2019 bleomycin challenge) scores 1 of 10.

## 1. Structural edges

### E-001: damage production rises with age

| Field | Content |
| --- | --- |
| `id` | E-001 |
| `sim_ref` | E1 in `research/sim/v0/design/edges.md` @ f527b9f |
| `claim` | In every organ, the production rate of damage rises linearly with age (`eta_i · t`), raising damage. |
| `supporting` | Karin 2019, doi:10.1038/s41467-019-13192-4. Design: model comparison (16 models) fitted to longitudinal p16-luciferase whole-body signal, 33 mice imaged every 8 weeks from 8 to 80 weeks (data of Burd 2013, doi:10.1016/j.cell.2012.12.010); the best model has production rising linearly with age. Nothing manipulated: association. Karin 2021, doi:10.1007/s11357-020-00286-x: re-analysis of published heterochronic parabiosis data; rejects models where only removal changes with age. Re-analysis, primary parabiosis studies not opened. |
| `contradicting` | None found. Search below. |
| `refuting_experiment` | Pulse-label new senescent cells (inducible lineage reporter of p16 activation) at fixed ages in the same mice, measuring the production rate directly. Refutation: production flat with age, or rising non-linearly (e.g. step at mid-life) with the rise explained by removal alone. |
| `already_run` | No. PubMed 2026-10-21: `senescent[tiab] AND mice[tiab] AND (production rate[tiab] OR induction rate[tiab]) AND age[tiab] AND (longitudinal[tiab] OR half-life[tiab])` → 0 hits. |
| `grade` | E1 (association only; no causal test of the production term) |
| `status` | `mapped` |
| `excluded` | none |
| `flags` | One marker (p16) stands in for all organ damage. Burd 2013 mice are p16 heterozygous (luciferase knocked into one p16 allele). Strain C57BL/6-background, not UM-HET3. Linearity is a model choice, tested against 15 alternatives only. |

### E-002: repair saturates as damage grows

| Field | Content |
| --- | --- |
| `id` | E-002 |
| `sim_ref` | E2 @ f527b9f |
| `claim` | Damage removal slows as damage accumulates (`beta_i · X_i / (kappa + X_i)`), so damage half-life lengthens with age. |
| `supporting` | Karin 2019 (above). Causal test: intratracheal bleomycin 1.5 U/kg (vs PBS) in C57BL/6 mice aged 3 mo (n = 17 treated, 13 PBS) and 22 mo (n = 13 treated, 6 PBS); senescent lung epithelial cells (SA-β-gal+, pan-cytokeratin+, HMGB1−) counted by imaging flow cytometry at 14 to 38 days. Half-life 4.7 d young vs 18 d old (p = 0.038, bootstrap). SYRCLE 1/10: sequence generation U, baseline U, allocation concealment U, random housing U, caregiver blinding U, random outcome assessment U, assessor blinding U, incomplete outcome data U, selective reporting L (source data file), other U. Karin 2021 parabiosis re-analysis (above) also supports removal falling with age. |
| `contradicting` | Tokarsky-Amiel 2013, doi:10.1158/0008-5472.CAN-12-3730: senescent cells induced in mouse epidermis by p14ARF (no inflammation) persisted for weeks even in young mice, so fast removal is not universal; it may depend on tissue, inducer and SASP. Not SYRCLE-scored. |
| `refuting_experiment` | Same bleomycin (or a second inducer) challenge in young and old mice of both sexes, in two organs, randomised and blinded, with a dose series in young mice. Refutation: half-life does not lengthen with the induced load in young mice (saturation is a load effect in the model, not an age effect). |
| `already_run` | Partly. PubMed 2026-10-21: `mice[tiab] AND (senescent cell[tiab] OR senescent cells[tiab]) AND (clearance[tiab] OR removal[tiab]) AND (half-life[tiab] OR turnover[tiab]) AND young[tiab]` → 2 hits (Karin 2019, Karin 2021). No dose series, no second lab, males/females not reported. |
| `grade` | E1 (one causal test, score 1 < 5) |
| `status` | `mapped` |
| `excluded` | none |
| `flags` | Lung only, one inducer. Sex not reported. C57BL/6, not UM-HET3. The test shows removal slower in old mice; it does not separate "slowed by load" (the model) from "slowed by age" (A68 #121: η/β trade-off). |

### E-003: damage spreads between organs

| Field | Content |
| --- | --- |
| `id` | E-003 |
| `sim_ref` | E3 @ f527b9f |
| `claim` | Systemic damage (mean normalised damage over organs) raises damage production in every organ (factor `1 + c · C · I`). |
| `supporting` | Farrell 2016, doi:10.1103/PhysRevE.94.052409: network model of interacting deficits fitted to human frailty-index data. Model fit, humans, no manipulation: association. Franco 2024, doi:10.1111/acel.14340: senescent fibroblasts transplanted into the dermis of young mice raised senescence markers in proximal and distant host tissues, with peripheral and cognitive decline. Causal mouse test, not yet SYRCLE-scored (abstract only). Yousefzadeh 2021, doi:10.1038/s41586-021-03547-7: mice with senescent immune cells (Ercc1 deletion in blood cells) showed senescence and damage in non-blood organs. Causal mouse test, not yet SYRCLE-scored. |
| `contradicting` | None found. Search below. |
| `refuting_experiment` | Damage induced in one organ only (local challenge, e.g. lung bleomycin) in young mice, with damage markers measured in non-challenged organs over time vs sham. Refutation: no rise in distant organs beyond sham. |
| `already_run` | Partly. PubMed 2026-10-21: `mice[tiab] AND (senescence[tiab] OR damage[tiab]) AND (distant organs[tiab] OR non-lymphoid organs[tiab] OR remote organs[tiab]) AND (transplant*[tiab] OR tissue-specific[tiab])` → 7 hits; relevant: Franco 2024, Yousefzadeh 2021. Both transplant or genetic, not a local challenge. |
| `grade` | E2: two independent causal mouse tests (Franco 2024, Passos lab; Yousefzadeh 2021, Niedernhofer lab), not yet SYRCLE-scored. E3 only if both score at least 5. |
| `status` | `mapped` |
| `excluded` | Xu 2018, doi:10.1038/s41591-018-0092-9 (senescent cell transplant spreads dysfunction), code `X-LS`: the same paper reports lifespan of senolytic-treated mice. |
| `flags` | Model mechanism (one coupling constant for all organs) is not what the mouse tests measure (one source organ). Human fit for the functional form. |

### E-004: death when a vital organ crosses its threshold

| Field | Content |
| --- | --- |
| `id` | E-004 |
| `sim_ref` | E4 @ f527b9f (vital set A3, A6, A7, A8, A9, A10, A11, A12, A17 = NA3) |
| `claim` | A mouse dies when damage in any vital organ reaches that organ's threshold (`X_i ≥ Xc_i`). |
| `supporting` | Farrell 2016 (above): death modelled as failure of the two most connected nodes; fitted to human mortality. Model fit, humans: association. Karin 2019 (above): a threshold on senescent-cell level reproduces C57BL/6J mortality curves (Mouse Phenome Database). Model fit only. |
| `contradicting` | O3 (`obstacles.md`): most control mice die of cancer, mainly lymphoma (Brayton 2012, doi:10.1177/0300985811430696; Miller 2020, doi:10.1172/jci.insight.140019), not of a failing heart, kidney or lung. In the model, cancer is one axis (A17) among nine. |
| `refuting_experiment` | Serial measurement of the vital-organ readouts in the same mice up to death, with necropsy cause of death. Refutation: deaths not preceded by any vital readout reaching its estimated threshold, or the readout at death no different from age-matched survivors. |
| `already_run` | No. PubMed 2026-10-21: `mice[tiab] AND aging[tiab] AND (cause of death[tiab]) AND (organ failure[tiab] OR threshold[tiab]) AND (heart[tiab] OR kidney[tiab])` → 0 hits. |
| `grade` | E1 (model fits only) |
| `status` | `mapped` |
| `excluded` | none |
| `flags` | The vital set is a modelling choice (NA3, S11). Threshold mortality is supported for one marker, not for each organ. |

### E-005: individual noise in damage

| Field | Content |
| --- | --- |
| `id` | E-005 |
| `sim_ref` | E5 @ f527b9f |
| `claim` | Damage in each organ has random fluctuation (`sqrt(2·eps) dW_i`), giving variation between mice. |
| `supporting` | None admissible. edges.md lists it as a modelling choice. Karin 2019 fits a noise term to p16 data, but that is a fit of the same form, not evidence for it in each organ. |
| `contradicting` | Not searched (no claim to refute beyond the form). |
| `refuting_experiment` | Repeated readouts per organ in genetically identical co-housed mice. Refutation: between-mouse spread fully explained by measurement noise and fixed differences (no random-walk growth of spread with age). |
| `already_run` | Not searched. |
| `grade` | not graded (no dated search) |
| `status` | `no_mapping`: listed blind spot |
| `excluded` | none |
| `flags` | — |

## 2. Hallmark → organ pairs (166)

edges.md @ f527b9f opens no source for any hallmark → organ link. Each pair is therefore `no_mapping`, a listed blind spot, and is not graded (a grade needs a dated search per pair). A28 has asked A68 to attach a source per pair where one exists (G153, 2026-10-21); a pair gets a full row when a source is given. The table is generated by script from the edges.md table (counts checked: 166).

Columns: id, pair id, hallmark, organ axis, term, sign on damage when the hallmark multiplier > 1, status, grade.

| `id` | Pair | Hallmark | Axis | Term | Sign | `status` | `grade` |
| --- | --- | --- | --- | --- | --- | --- | --- |
| E-006 | E-H1-A1 | H1 genomic instability | A1 | production | + | `no_mapping` | not graded |
| E-007 | E-H1-A2 | H1 genomic instability | A2 | production | + | `no_mapping` | not graded |
| E-008 | E-H1-A3 | H1 genomic instability | A3 | production | + | `no_mapping` | not graded |
| E-009 | E-H1-A4 | H1 genomic instability | A4 | production | + | `no_mapping` | not graded |
| E-010 | E-H1-A5 | H1 genomic instability | A5 | production | + | `no_mapping` | not graded |
| E-011 | E-H1-A6 | H1 genomic instability | A6 | production | + | `no_mapping` | not graded |
| E-012 | E-H1-A7 | H1 genomic instability | A7 | production | + | `no_mapping` | not graded |
| E-013 | E-H1-A8 | H1 genomic instability | A8 | production | + | `no_mapping` | not graded |
| E-014 | E-H1-A9 | H1 genomic instability | A9 | production | + | `no_mapping` | not graded |
| E-015 | E-H1-A10 | H1 genomic instability | A10 | production | + | `no_mapping` | not graded |
| E-016 | E-H1-A11 | H1 genomic instability | A11 | production | + | `no_mapping` | not graded |
| E-017 | E-H1-A12 | H1 genomic instability | A12 | production | + | `no_mapping` | not graded |
| E-018 | E-H1-A13 | H1 genomic instability | A13 | production | + | `no_mapping` | not graded |
| E-019 | E-H1-A14 | H1 genomic instability | A14 | production | + | `no_mapping` | not graded |
| E-020 | E-H1-A15 | H1 genomic instability | A15 | production | + | `no_mapping` | not graded |
| E-021 | E-H1-A16 | H1 genomic instability | A16 | production | + | `no_mapping` | not graded |
| E-022 | E-H1-A17 | H1 genomic instability | A17 | production | + | `no_mapping` | not graded |
| E-023 | E-H2-A11 | H2 telomere attrition | A11 | production | + | `no_mapping` | not graded |
| E-024 | E-H2-A12 | H2 telomere attrition | A12 | production | + | `no_mapping` | not graded |
| E-025 | E-H2-A13 | H2 telomere attrition | A13 | production | + | `no_mapping` | not graded |
| E-026 | E-H2-A14 | H2 telomere attrition | A14 | production | + | `no_mapping` | not graded |
| E-027 | E-H3-A1 | H3 epigenetic alterations | A1 | production | + | `no_mapping` | not graded |
| E-028 | E-H3-A2 | H3 epigenetic alterations | A2 | production | + | `no_mapping` | not graded |
| E-029 | E-H3-A3 | H3 epigenetic alterations | A3 | production | + | `no_mapping` | not graded |
| E-030 | E-H3-A4 | H3 epigenetic alterations | A4 | production | + | `no_mapping` | not graded |
| E-031 | E-H3-A5 | H3 epigenetic alterations | A5 | production | + | `no_mapping` | not graded |
| E-032 | E-H3-A6 | H3 epigenetic alterations | A6 | production | + | `no_mapping` | not graded |
| E-033 | E-H3-A7 | H3 epigenetic alterations | A7 | production | + | `no_mapping` | not graded |
| E-034 | E-H3-A8 | H3 epigenetic alterations | A8 | production | + | `no_mapping` | not graded |
| E-035 | E-H3-A9 | H3 epigenetic alterations | A9 | production | + | `no_mapping` | not graded |
| E-036 | E-H3-A10 | H3 epigenetic alterations | A10 | production | + | `no_mapping` | not graded |
| E-037 | E-H3-A11 | H3 epigenetic alterations | A11 | production | + | `no_mapping` | not graded |
| E-038 | E-H3-A12 | H3 epigenetic alterations | A12 | production | + | `no_mapping` | not graded |
| E-039 | E-H3-A13 | H3 epigenetic alterations | A13 | production | + | `no_mapping` | not graded |
| E-040 | E-H3-A14 | H3 epigenetic alterations | A14 | production | + | `no_mapping` | not graded |
| E-041 | E-H3-A15 | H3 epigenetic alterations | A15 | production | + | `no_mapping` | not graded |
| E-042 | E-H3-A16 | H3 epigenetic alterations | A16 | production | + | `no_mapping` | not graded |
| E-043 | E-H3-A17 | H3 epigenetic alterations | A17 | production | + | `no_mapping` | not graded |
| E-044 | E-H4-A1 | H4 proteostasis | A1 | production | + | `no_mapping` | not graded |
| E-045 | E-H4-A2 | H4 proteostasis | A2 | production | + | `no_mapping` | not graded |
| E-046 | E-H4-A3 | H4 proteostasis | A3 | production | + | `no_mapping` | not graded |
| E-047 | E-H4-A4 | H4 proteostasis | A4 | production | + | `no_mapping` | not graded |
| E-048 | E-H4-A5 | H4 proteostasis | A5 | production | + | `no_mapping` | not graded |
| E-049 | E-H4-A6 | H4 proteostasis | A6 | production | + | `no_mapping` | not graded |
| E-050 | E-H4-A7 | H4 proteostasis | A7 | production | + | `no_mapping` | not graded |
| E-051 | E-H4-A8 | H4 proteostasis | A8 | production | + | `no_mapping` | not graded |
| E-052 | E-H4-A9 | H4 proteostasis | A9 | production | + | `no_mapping` | not graded |
| E-053 | E-H4-A10 | H4 proteostasis | A10 | production | + | `no_mapping` | not graded |
| E-054 | E-H4-A11 | H4 proteostasis | A11 | production | + | `no_mapping` | not graded |
| E-055 | E-H4-A12 | H4 proteostasis | A12 | production | + | `no_mapping` | not graded |
| E-056 | E-H4-A13 | H4 proteostasis | A13 | production | + | `no_mapping` | not graded |
| E-057 | E-H4-A14 | H4 proteostasis | A14 | production | + | `no_mapping` | not graded |
| E-058 | E-H4-A15 | H4 proteostasis | A15 | production | + | `no_mapping` | not graded |
| E-059 | E-H4-A16 | H4 proteostasis | A16 | production | + | `no_mapping` | not graded |
| E-060 | E-H4-A17 | H4 proteostasis | A17 | production | + | `no_mapping` | not graded |
| E-061 | E-H5-A1 | H5 macroautophagy | A1 | clearance | − | `no_mapping` | not graded |
| E-062 | E-H5-A2 | H5 macroautophagy | A2 | clearance | − | `no_mapping` | not graded |
| E-063 | E-H5-A3 | H5 macroautophagy | A3 | clearance | − | `no_mapping` | not graded |
| E-064 | E-H5-A4 | H5 macroautophagy | A4 | clearance | − | `no_mapping` | not graded |
| E-065 | E-H5-A5 | H5 macroautophagy | A5 | clearance | − | `no_mapping` | not graded |
| E-066 | E-H5-A6 | H5 macroautophagy | A6 | clearance | − | `no_mapping` | not graded |
| E-067 | E-H5-A7 | H5 macroautophagy | A7 | clearance | − | `no_mapping` | not graded |
| E-068 | E-H5-A8 | H5 macroautophagy | A8 | clearance | − | `no_mapping` | not graded |
| E-069 | E-H5-A9 | H5 macroautophagy | A9 | clearance | − | `no_mapping` | not graded |
| E-070 | E-H5-A10 | H5 macroautophagy | A10 | clearance | − | `no_mapping` | not graded |
| E-071 | E-H5-A11 | H5 macroautophagy | A11 | clearance | − | `no_mapping` | not graded |
| E-072 | E-H5-A12 | H5 macroautophagy | A12 | clearance | − | `no_mapping` | not graded |
| E-073 | E-H5-A13 | H5 macroautophagy | A13 | clearance | − | `no_mapping` | not graded |
| E-074 | E-H5-A14 | H5 macroautophagy | A14 | clearance | − | `no_mapping` | not graded |
| E-075 | E-H5-A15 | H5 macroautophagy | A15 | clearance | − | `no_mapping` | not graded |
| E-076 | E-H5-A16 | H5 macroautophagy | A16 | clearance | − | `no_mapping` | not graded |
| E-077 | E-H5-A17 | H5 macroautophagy | A17 | clearance | − | `no_mapping` | not graded |
| E-078 | E-H6-A1 | H6 nutrient sensing | A1 | production | + | `no_mapping` | not graded |
| E-079 | E-H6-A2 | H6 nutrient sensing | A2 | production | + | `no_mapping` | not graded |
| E-080 | E-H6-A3 | H6 nutrient sensing | A3 | production | + | `no_mapping` | not graded |
| E-081 | E-H6-A4 | H6 nutrient sensing | A4 | production | + | `no_mapping` | not graded |
| E-082 | E-H6-A5 | H6 nutrient sensing | A5 | production | + | `no_mapping` | not graded |
| E-083 | E-H6-A6 | H6 nutrient sensing | A6 | production | + | `no_mapping` | not graded |
| E-084 | E-H6-A7 | H6 nutrient sensing | A7 | production | + | `no_mapping` | not graded |
| E-085 | E-H6-A8 | H6 nutrient sensing | A8 | production | + | `no_mapping` | not graded |
| E-086 | E-H6-A9 | H6 nutrient sensing | A9 | production | + | `no_mapping` | not graded |
| E-087 | E-H6-A10 | H6 nutrient sensing | A10 | production | + | `no_mapping` | not graded |
| E-088 | E-H6-A11 | H6 nutrient sensing | A11 | production | + | `no_mapping` | not graded |
| E-089 | E-H6-A12 | H6 nutrient sensing | A12 | production | + | `no_mapping` | not graded |
| E-090 | E-H6-A13 | H6 nutrient sensing | A13 | production | + | `no_mapping` | not graded |
| E-091 | E-H6-A14 | H6 nutrient sensing | A14 | production | + | `no_mapping` | not graded |
| E-092 | E-H6-A15 | H6 nutrient sensing | A15 | production | + | `no_mapping` | not graded |
| E-093 | E-H6-A16 | H6 nutrient sensing | A16 | production | + | `no_mapping` | not graded |
| E-094 | E-H6-A17 | H6 nutrient sensing | A17 | production | + | `no_mapping` | not graded |
| E-095 | E-H7-A1 | H7 mitochondria | A1 | production | + | `no_mapping` | not graded |
| E-096 | E-H7-A2 | H7 mitochondria | A2 | production | + | `no_mapping` | not graded |
| E-097 | E-H7-A3 | H7 mitochondria | A3 | production | + | `no_mapping` | not graded |
| E-098 | E-H7-A4 | H7 mitochondria | A4 | production | + | `no_mapping` | not graded |
| E-099 | E-H7-A5 | H7 mitochondria | A5 | production | + | `no_mapping` | not graded |
| E-100 | E-H7-A6 | H7 mitochondria | A6 | production | + | `no_mapping` | not graded |
| E-101 | E-H7-A7 | H7 mitochondria | A7 | production | + | `no_mapping` | not graded |
| E-102 | E-H7-A8 | H7 mitochondria | A8 | production | + | `no_mapping` | not graded |
| E-103 | E-H7-A9 | H7 mitochondria | A9 | production | + | `no_mapping` | not graded |
| E-104 | E-H7-A10 | H7 mitochondria | A10 | production | + | `no_mapping` | not graded |
| E-105 | E-H7-A11 | H7 mitochondria | A11 | production | + | `no_mapping` | not graded |
| E-106 | E-H7-A12 | H7 mitochondria | A12 | production | + | `no_mapping` | not graded |
| E-107 | E-H7-A13 | H7 mitochondria | A13 | production | + | `no_mapping` | not graded |
| E-108 | E-H7-A14 | H7 mitochondria | A14 | production | + | `no_mapping` | not graded |
| E-109 | E-H7-A15 | H7 mitochondria | A15 | production | + | `no_mapping` | not graded |
| E-110 | E-H7-A16 | H7 mitochondria | A16 | production | + | `no_mapping` | not graded |
| E-111 | E-H7-A17 | H7 mitochondria | A17 | production | + | `no_mapping` | not graded |
| E-112 | E-H8-A1 | H8 senescence (clearance of senescent damage) | A1 | clearance | − | `no_mapping` | not graded |
| E-113 | E-H8-A2 | H8 senescence (clearance of senescent damage) | A2 | clearance | − | `no_mapping` | not graded |
| E-114 | E-H8-A3 | H8 senescence (clearance of senescent damage) | A3 | clearance | − | `no_mapping` | not graded |
| E-115 | E-H8-A4 | H8 senescence (clearance of senescent damage) | A4 | clearance | − | `no_mapping` | not graded |
| E-116 | E-H8-A5 | H8 senescence (clearance of senescent damage) | A5 | clearance | − | `no_mapping` | not graded |
| E-117 | E-H8-A6 | H8 senescence (clearance of senescent damage) | A6 | clearance | − | `no_mapping` | not graded |
| E-118 | E-H8-A7 | H8 senescence (clearance of senescent damage) | A7 | clearance | − | `no_mapping` | not graded |
| E-119 | E-H8-A8 | H8 senescence (clearance of senescent damage) | A8 | clearance | − | `no_mapping` | not graded |
| E-120 | E-H8-A9 | H8 senescence (clearance of senescent damage) | A9 | clearance | − | `no_mapping` | not graded |
| E-121 | E-H8-A10 | H8 senescence (clearance of senescent damage) | A10 | clearance | − | `no_mapping` | not graded |
| E-122 | E-H8-A11 | H8 senescence (clearance of senescent damage) | A11 | clearance | − | `no_mapping` | not graded |
| E-123 | E-H8-A12 | H8 senescence (clearance of senescent damage) | A12 | clearance | − | `no_mapping` | not graded |
| E-124 | E-H8-A13 | H8 senescence (clearance of senescent damage) | A13 | clearance | − | `no_mapping` | not graded |
| E-125 | E-H8-A14 | H8 senescence (clearance of senescent damage) | A14 | clearance | − | `no_mapping` | not graded |
| E-126 | E-H8-A15 | H8 senescence (clearance of senescent damage) | A15 | clearance | − | `no_mapping` | not graded |
| E-127 | E-H8-A16 | H8 senescence (clearance of senescent damage) | A16 | clearance | − | `no_mapping` | not graded |
| E-128 | E-H8-A17 | H8 senescence (clearance of senescent damage) | A17 | clearance | − | `no_mapping` | not graded |
| E-129 | E-H9-A1 | H9 stem cell exhaustion (regenerative repair) | A1 | clearance | − | `no_mapping` | not graded |
| E-130 | E-H9-A2 | H9 stem cell exhaustion (regenerative repair) | A2 | clearance | − | `no_mapping` | not graded |
| E-131 | E-H9-A11 | H9 stem cell exhaustion (regenerative repair) | A11 | clearance | − | `no_mapping` | not graded |
| E-132 | E-H9-A12 | H9 stem cell exhaustion (regenerative repair) | A12 | clearance | − | `no_mapping` | not graded |
| E-133 | E-H9-A13 | H9 stem cell exhaustion (regenerative repair) | A13 | clearance | − | `no_mapping` | not graded |
| E-134 | E-H9-A14 | H9 stem cell exhaustion (regenerative repair) | A14 | clearance | − | `no_mapping` | not graded |
| E-135 | E-H10-A1 | H10 intercellular communication | A1 | coupling (`C` in E3) | + | `no_mapping` | not graded |
| E-136 | E-H10-A2 | H10 intercellular communication | A2 | coupling (`C` in E3) | + | `no_mapping` | not graded |
| E-137 | E-H10-A3 | H10 intercellular communication | A3 | coupling (`C` in E3) | + | `no_mapping` | not graded |
| E-138 | E-H10-A4 | H10 intercellular communication | A4 | coupling (`C` in E3) | + | `no_mapping` | not graded |
| E-139 | E-H10-A5 | H10 intercellular communication | A5 | coupling (`C` in E3) | + | `no_mapping` | not graded |
| E-140 | E-H10-A6 | H10 intercellular communication | A6 | coupling (`C` in E3) | + | `no_mapping` | not graded |
| E-141 | E-H10-A7 | H10 intercellular communication | A7 | coupling (`C` in E3) | + | `no_mapping` | not graded |
| E-142 | E-H10-A8 | H10 intercellular communication | A8 | coupling (`C` in E3) | + | `no_mapping` | not graded |
| E-143 | E-H10-A9 | H10 intercellular communication | A9 | coupling (`C` in E3) | + | `no_mapping` | not graded |
| E-144 | E-H10-A10 | H10 intercellular communication | A10 | coupling (`C` in E3) | + | `no_mapping` | not graded |
| E-145 | E-H10-A11 | H10 intercellular communication | A11 | coupling (`C` in E3) | + | `no_mapping` | not graded |
| E-146 | E-H10-A12 | H10 intercellular communication | A12 | coupling (`C` in E3) | + | `no_mapping` | not graded |
| E-147 | E-H10-A13 | H10 intercellular communication | A13 | coupling (`C` in E3) | + | `no_mapping` | not graded |
| E-148 | E-H10-A14 | H10 intercellular communication | A14 | coupling (`C` in E3) | + | `no_mapping` | not graded |
| E-149 | E-H10-A15 | H10 intercellular communication | A15 | coupling (`C` in E3) | + | `no_mapping` | not graded |
| E-150 | E-H10-A16 | H10 intercellular communication | A16 | coupling (`C` in E3) | + | `no_mapping` | not graded |
| E-151 | E-H10-A17 | H10 intercellular communication | A17 | coupling (`C` in E3) | + | `no_mapping` | not graded |
| E-152 | E-H11-A1 | H11 chronic inflammation | A1 | coupling (`C` in E3) | + | `no_mapping` | not graded |
| E-153 | E-H11-A2 | H11 chronic inflammation | A2 | coupling (`C` in E3) | + | `no_mapping` | not graded |
| E-154 | E-H11-A3 | H11 chronic inflammation | A3 | coupling (`C` in E3) | + | `no_mapping` | not graded |
| E-155 | E-H11-A4 | H11 chronic inflammation | A4 | coupling (`C` in E3) | + | `no_mapping` | not graded |
| E-156 | E-H11-A5 | H11 chronic inflammation | A5 | coupling (`C` in E3) | + | `no_mapping` | not graded |
| E-157 | E-H11-A6 | H11 chronic inflammation | A6 | coupling (`C` in E3) | + | `no_mapping` | not graded |
| E-158 | E-H11-A7 | H11 chronic inflammation | A7 | coupling (`C` in E3) | + | `no_mapping` | not graded |
| E-159 | E-H11-A8 | H11 chronic inflammation | A8 | coupling (`C` in E3) | + | `no_mapping` | not graded |
| E-160 | E-H11-A9 | H11 chronic inflammation | A9 | coupling (`C` in E3) | + | `no_mapping` | not graded |
| E-161 | E-H11-A10 | H11 chronic inflammation | A10 | coupling (`C` in E3) | + | `no_mapping` | not graded |
| E-162 | E-H11-A11 | H11 chronic inflammation | A11 | coupling (`C` in E3) | + | `no_mapping` | not graded |
| E-163 | E-H11-A12 | H11 chronic inflammation | A12 | coupling (`C` in E3) | + | `no_mapping` | not graded |
| E-164 | E-H11-A13 | H11 chronic inflammation | A13 | coupling (`C` in E3) | + | `no_mapping` | not graded |
| E-165 | E-H11-A14 | H11 chronic inflammation | A14 | coupling (`C` in E3) | + | `no_mapping` | not graded |
| E-166 | E-H11-A15 | H11 chronic inflammation | A15 | coupling (`C` in E3) | + | `no_mapping` | not graded |
| E-167 | E-H11-A16 | H11 chronic inflammation | A16 | coupling (`C` in E3) | + | `no_mapping` | not graded |
| E-168 | E-H11-A17 | H11 chronic inflammation | A17 | coupling (`C` in E3) | + | `no_mapping` | not graded |
| E-169 | E-H12-A10 | H12 dysbiosis | A10 | production | + | `no_mapping` | not graded |
| E-170 | E-H12-A11 | H12 dysbiosis | A11 | production | + | `no_mapping` | not graded |
| E-171 | E-H12-A14 | H12 dysbiosis | A14 | production | + | `no_mapping` | not graded |
