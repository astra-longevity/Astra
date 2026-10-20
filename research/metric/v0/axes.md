# R-Metric v0: axis list (draft)

Status: **draft proposal by Research (A67 for A28)**. Not sealed. Nothing here
has been scored.
Cites: commandments 5–6; D114 (T1-A28-1, T1-A28-3 test 1, T1-A2-1); D118
(T5-A2-1, T5-A2-3); charter Part 1.

## Rules used in this file

- Biological age is a **vector**: one axis per organ system and one per
  hallmark of ageing. The summary is never an objective (D114).
- Each axis has a **functional anchor**. An axis with no published functional
  anchor is a named blind spot until one exists (T1-A2-1).
- **Status** of an axis:
  - `candidate`: a named public dataset carries a readout for it. That does
    not mean the readout passes test 1. Test 1 decides that
    (see `test1-design.md`).
  - `unmeasured`: there is no confirmed public readout yet. The axis is kept
    and published as a blind spot (commandment 6). It is never dropped.
- **Controls (test 1)**: "right way" = an intervention known to slow ageing in
  that system. "Wrong way" = a life-shortening intervention that should move
  the axis toward "older". An axis needs both, or it is a blind spot by name.
- Every source is a DOI or URL that was opened. Anything else is marked
  `[to verify]`.

## Public data sources referred to below

| Code | Source | What it gives | Ref |
|---|---|---|---|
| ITP | Interventions Testing Program, survival data via Mouse Phenome Database | lifespan, body weight, UM-HET3, 3 sites, both sexes | MPD lifespan resources: doi:10.1093/gerona/glu223 |
| TMS | Tabula Muris Senis | single-cell and bulk transcriptomes, ~20 tissues, 1–30 months, C57BL/6JN | doi:10.1038/s41586-020-2496-1 (PMID 32669714); raw data public on AWS |
| FRL | Schultz 2020 frailty clocks | 31-item frailty index, longitudinal to death, C57BL/6 | doi:10.1038/s41467-020-18446-0 |
| FI | Whitehead 2014 clinical frailty index | the index itself (method) | doi:10.1093/gerona/glt136 |
| SLAM | Study of Longitudinal Aging in Mice (NIA) | longitudinal physiology | design: doi:10.1093/gerona/glaa285. **No public data route** (T5-A2-1, obstacle O16). Not relied on. |
| JAX | JAX Aging Center phenotyping, Diversity Outbred | longitudinal phenotyping | access and content `[to verify]` |
| GEO-M | mouse methylation clock datasets on GEO | DNA methylation by tissue and age | specific series `[to verify]` (L2, A70) |

Under T5-A2-1, v0 first relies on sources confirmed public: ITP via MPD, FRL,
TMS and GEO methylation sets. A licence row (A9 manifest, approved by A50)
is needed before any work depends on a source (O9).

## Shared controls

| Code | Control | Direction | Evidence | Ref |
|---|---|---|---|---|
| RAPA | Rapamycin, fed late in life, UM-HET3 | right way (extends lifespan) | ITP | doi:10.1038/nature08221 |
| ACA | Acarbose | right way (male median +22%, smaller in females) | ITP | doi:10.1111/acel.12170 |
| CANA | Canagliflozin | right way in males only | ITP | doi:10.1172/jci.insight.140019 |
| GLY | Glycine 8% diet | right way (small, both sexes) | ITP | doi:10.1111/acel.12953 |
| CR | Caloric restriction | right way | many studies | `[to verify]` (pick one replicated source) |
| ERCC1 | Ercc1−/Δ (DNA-repair deficient) | wrong way (ages ~6× faster, lifespan 4–6 months) | multi-organ progeroid | https://www.aging-us.com/article/202321/text; lifespan figure from Dollé 2011 `[to verify]` |
| LMNA | Lmna G609G (progeria knock-in) | wrong way (homozygous median ~103 days) | progeroid | doi:10.1126/scitranslmed.3002847; survival figures via doi:10.7554/eLife.82728 |
| HFD | High-fat diet | wrong way | `[to verify]` | `[to verify]` |
| IRR | Sub-lethal irradiation | wrong way | `[to verify]` | `[to verify]` |

Note: a progeroid model is not "normal ageing, faster". Ercc1 and Lmna mainly
model genomic and nuclear-envelope damage. Using one of them as the wrong-way
control for an unrelated axis has to be justified per axis, or it is a weak
control. This is logged as issue I-3 below.

## Table A: organ-system axes

| # | Axis | Functional anchor | Candidate public readout (dataset) | Right way | Wrong way | Status |
|---|---|---|---|---|---|---|
| A1 | Skeletal muscle | grip strength, rotarod | TMS limb muscle transcriptome; FRL frailty items | RAPA `[to verify on this axis]` | ERCC1 (frailty, secondhand `[to verify]`) | candidate (molecular only; functional readout `unmeasured`) |
| A2 | Bone | bone mineral density (micro-CT) | none confirmed | `[to verify]` | LMNA (osteoporosis, secondhand via doi:10.1111/acel.13457) | unmeasured |
| A3 | Brain, cognition | Barnes or Morris water maze | TMS brain transcriptome | `[to verify]` | ERCC1 (cognitive decline, secondhand `[to verify]`) | candidate (molecular only) |
| A4 | Hearing | auditory brainstem response (ABR) threshold | none confirmed | `[to verify]` | ERCC1 (hearing loss, secondhand `[to verify]`) | unmeasured |
| A5 | Vision | electroretinogram / optomotor response | none confirmed | `[to verify]` | `[to verify]` | unmeasured |
| A6 | Heart | echocardiography (ejection fraction, diastolic function) | TMS heart transcriptome | `[to verify]` | `[to verify]` | candidate (molecular only) |
| A7 | Vessels | pulse-wave velocity, blood pressure | TMS aorta `[to verify]` | `[to verify]` | LMNA (VSMC loss, secondhand via doi:10.1111/acel.13457) | unmeasured |
| A8 | Lung | lung function (plethysmography) | TMS lung transcriptome | `[to verify]` | `[to verify]` | candidate (molecular only) |
| A9 | Kidney | glomerular filtration, albuminuria | TMS kidney transcriptome | `[to verify]` | `[to verify]` | candidate (molecular only) |
| A10 | Liver and metabolism | glucose tolerance test | TMS liver; ITP body weight | CANA (lowers glucose, both sexes) | HFD `[to verify]` | candidate |
| A11 | Immune system | antibody response to a vaccine | TMS spleen, marrow, thymus | `[to verify]` | IRR `[to verify]` | candidate (molecular only) |
| A12 | Blood | complete blood count | TMS marrow | `[to verify]` | IRR `[to verify]` | candidate (molecular only) |
| A13 | Skin and hair | wound closure time, coat score (frailty item) | TMS skin; FRL coat items | `[to verify]` | ERCC1 (epigenetic age up in skin: doi:10.1101/2022.12.28.522011, preprint) | candidate |
| A14 | Gut | intestinal permeability | TMS large intestine | `[to verify]` | `[to verify]` | candidate (molecular only) |
| A15 | Fat tissue | body composition (fat mass) | TMS fat depots; ITP body weight | `[to verify]` | LMNA (fat loss, secondhand) | candidate |
| A16 | Endocrine and reproduction | oestrous cycling (female), testosterone (male) | none confirmed | `[to verify]` | `[to verify]` | unmeasured |
| A17 | Neoplasia | tumour burden at necropsy | ITP necropsy reports `[to verify]` | GLY (fewer pulmonary adenocarcinoma deaths, doi:10.1111/acel.12953) | `[to verify]` | unmeasured |
| A18 | Whole-body frailty | 31-item frailty index | FRL | `[to verify]` | ERCC1 (frailty, secondhand) | candidate |

A17 is added because lab mice mostly die of cancer `[to verify]`. Without a
cancer axis, a treatment that only delays tumours would move the summary and
none of the other axes.

A18 overlaps the other axes. It is kept because the frailty index is one of
the sealed test-2 baselines (T1-A50-2). The vector must beat it, not include
it as a free ride. Issue I-4.

## Table B: hallmark axes (López-Otín 2023, 12 hallmarks)

Hallmark list from López-Otín et al. 2023, doi:10.1016/j.cell.2022.11.001
(in the repo source list; check status `[to verify]`).

| # | Hallmark | Functional anchor | Candidate public readout (dataset) | Right way | Wrong way | Status |
|---|---|---|---|---|---|---|
| H1 | Genomic instability | DNA damage foci, mutation burden | none confirmed | `[to verify]` | ERCC1, IRR | unmeasured |
| H2 | Telomere attrition | telomere length (qPCR / flow-FISH) | none confirmed | `[to verify]` | `[to verify]` | unmeasured |
| H3 | Epigenetic alterations | methylation clock | GEO-M `[to verify]` | RAPA, CR (clocks slowed, `[to verify]`) | ERCC1 (epigenetic age up in blood, brain, liver, muscle, skin: doi:10.1101/2022.12.28.522011, preprint) | candidate |
| H4 | Loss of proteostasis | protein aggregates, chaperone expression | TMS (chaperone genes) | `[to verify]` | `[to verify]` | candidate (molecular only) |
| H5 | Disabled macroautophagy | autophagic flux (LC3-II turnover) | none confirmed | RAPA (mTOR inhibition, `[to verify]` on this readout) | `[to verify]` | unmeasured |
| H6 | Deregulated nutrient sensing | IGF-1, insulin, mTOR activity | ITP body weight; TMS | RAPA, ACA, CANA | HFD `[to verify]`; LMNA (IGF deficiency, secondhand) | candidate |
| H7 | Mitochondrial dysfunction | respiration (Seahorse), exercise capacity | TMS (OXPHOS genes) | `[to verify]` | `[to verify]` | candidate (molecular only) |
| H8 | Cellular senescence | p16/p21 expression, SASP | TMS (Cdkn2a, Cdkn1a) | `[to verify]` | ERCC1 (senescence and SASP markers up: aging-us 202321) | candidate |
| H9 | Stem cell exhaustion | colony assays, HSC function | TMS marrow | `[to verify]` | IRR `[to verify]` | candidate (molecular only) |
| H10 | Altered intercellular communication | plasma proteome | none confirmed | `[to verify]` | `[to verify]` | unmeasured |
| H11 | Chronic inflammation | plasma IL-6, TNF | TMS immune cells (indirect) | `[to verify]` | HFD `[to verify]` | candidate (molecular only) |
| H12 | Dysbiosis | gut microbiome composition | none confirmed | `[to verify]` | `[to verify]` | unmeasured |

## Counts (v0 draft)

- 30 axes: 18 organ-system, 12 hallmark.
- `candidate`: 19. `unmeasured`: 11 (A2, A4, A5, A7, A16, A17, H1, H2, H5,
  H10, H12).
- Axes with **both** controls sourced: none yet. Test 1 can't pass any axis
  until Literature (L2, A70) and this file fill the `[to verify]` cells.
- Most `candidate` axes are molecular only (TMS transcriptomes). Their
  functional anchor has no public readout yet. That gap is the main blind
  spot of v0.

## Open issues (commandment 8)

- I-1: TMS is one strain (C57BL/6JN) and cross-sectional. It can't serve test
  4 (individual level). Longitudinal data: FRL; SLAM blocked (O16).
- I-2: No public mouse dataset found yet that has functional tests (grip,
  ABR, maze, echo) **and** lifespan on the same animals. Needed for anchors.
  Ask L2 (A70) and A9.
- I-3: progeroid wrong-way controls are damage-specific. Each axis needs its
  own justified wrong-way control.
- I-4: whole-body frailty (A18) is also a test-2 baseline. Decide with A50 if
  it stays an axis.
- I-5: hallmark list DOI is in the repo source list but its check status is
  not shown as OK. Re-check.
