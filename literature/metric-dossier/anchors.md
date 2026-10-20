# Anchors: one row per axis of R-Metric v0

Axis list: `research/metric/v0/axes.md` @116f1f8 (draft by A67; re-check at its PR). 30 axes: 18 organ systems (A1–A18), 12 hallmarks (H1–H12).

Rules for this file:
- Each row points to a dossier file. Every claim here comes from that file, whose sources are Crossref-checked (`dossier.check.tsv`). No number appears here that is not in the file.
- **anchored** = the dossier has a published functional test with an age trajectory in mice.
- **blind spot** = no functional anchor in the dossier yet. A blind spot is named, never dropped (commandment 6).
- No row has data in UM-HET3 (the ITP strain). Every age trend below is from other strains, mostly C57BL/6, and is not assumed to hold in UM-HET3.

## Table A: organ systems

| # | Axis | Anchor test | Dossier file | Age trajectory (mice) | Main confounders | Status |
|---|---|---|---|---|---|---|
| A1 | Skeletal muscle | grip, rotarod, treadmill endurance | mouse/02, 03, 04 | All fall with age in B6; treadmill falls earliest and furthest, grip earlier than lean mass, rotarod weakest | method and normalisation (grip), body weight and learning (rotarod), motivation (treadmill), sex | anchored |
| A2 | Bone | micro-CT bone density | none | — | — | blind spot |
| A3 | Brain, cognition | Morris water maze, Barnes maze | mouse/09 | Spatial memory declines from middle age (6–12 months, test-dependent) | swim speed (use distance, not latency), strain | anchored |
| A4 | Hearing | auditory brainstem response | none (human audiogram: tests/06) | — | — | blind spot |
| A5 | Vision | electroretinogram, optomotor | none | — | — | blind spot |
| A6 | Heart | echocardiography | mouse/05 | Thicker, heavier ventricles and slower relaxation; ejection fraction falls late | heart rate under anaesthesia, substrain; studies disagree | anchored |
| A7 | Vessels | pulse-wave velocity, blood pressure | none | — | — | blind spot |
| A8 | Lung | lung mechanics (forced oscillation) | mouse/10 | Compliance **rises** (floppier lung), mostly in young adulthood; reverses in the oldest survivors | body weight; test is terminal | anchored (non-monotonic, endpoint only) |
| A9 | Kidney | transcutaneous GFR, albuminuria | mouse/11 | GFR falls, albuminuria rises in B6; not seen in an F1 hybrid | strain, size normalisation, time of day, temperature | anchored (repeatable) |
| A10 | Liver, metabolism | glucose tolerance test | mouse/06 | AUC rises then falls with age, tracking fat mass; clamp studies still show lower insulin sensitivity | fat mass, dose by weight vs lean mass, fasting | anchored (non-monotonic; weak alone) |
| A11 | Immune | antibody response to a new vaccine | mouse/12 | Falls with age in B6 (T helper / germinal centre failure); higher in aged BALB/c | strain, sex, adjuvant, booster; antigen not reusable in one mouse | anchored |
| A12 | Blood | complete blood count | mouse/08 | Anaemia and a shift from lymphocytes to neutrophils; total white count not monotonic | disease, sampling site, substrain | anchored (repeatable) |
| A13 | Skin, hair | wound closure, coat score | mouse/13 | Healing slows, often already by middle age; coat loss in B6 driven more by sex, substrain and barbering than age | sex, strain, splinting, wound area vs re-epithelialisation, housing | anchored (wound); coat score weak |
| A14 | Gut | FITC-dextran permeability | mouse/14 | Rises with age in most studies; depends on microbiota and TNF; absent in one accelerated-ageing model | microbiota, cage, diet, protocol (dose, timing); no test-retest data | anchored (unstandardised) |
| A15 | Fat tissue | body composition | mouse/07 | Fat mass rises then falls; late fat and weight loss means decline | measure on the same day as A1/A10 tests; lean mass is not muscle | anchored (non-monotonic) |
| A16 | Endocrine, reproduction | oestrous cycling, testosterone | none | — | — | blind spot |
| A17 | Neoplasia | tumour burden at necropsy | none | — | — | blind spot |
| A18 | Whole-body frailty | 31-item frailty index | mouse/01 | Rises with age in C57BL/6J (Whitehead 2014); lowered by calorie restriction and resveratrol (Kane 2015) | sex, substrain, scorer | anchored; also a test-2 baseline (issue I-4) |

## Table B: hallmarks

The hallmark anchors in axes.md are molecular (foci, telomeres, LC3 flux, p16, cytokines…), not function tests. The dossier covers them only where a file exists.

| # | Hallmark | Dossier file | Note | Status |
|---|---|---|---|---|
| H1 | Genomic instability | none | — | blind spot |
| H2 | Telomere attrition | none | — | blind spot |
| H3 | Epigenetic alterations | clocks/06 (mouse clocks) | Clocks track calendar age, not function; claimed signal is slower ticking under calorie restriction and in dwarf mice | readout exists, **no functional anchor** |
| H4 | Loss of proteostasis | none | — | no dossier file |
| H5 | Disabled autophagy | none | — | blind spot |
| H6 | Nutrient sensing | mouse/06, 07 (indirect) | GTT and body composition touch it, but are non-monotonic | partial |
| H7 | Mitochondrial dysfunction | mouse/04 (indirect) | Treadmill endurance is the exercise-capacity part of the anchor | partial |
| H8 | Cellular senescence | none | — | no dossier file |
| H9 | Stem cell exhaustion | none | — | no dossier file |
| H10 | Intercellular communication | none | — | blind spot |
| H11 | Chronic inflammation | none (TNF role noted in mouse/14) | — | no dossier file |
| H12 | Dysbiosis | none (microbiota drives A14, mouse/14) | — | blind spot |

## Named blind spots

- **No functional anchor in the dossier (11, as in axes.md):** A2 bone, A4 hearing, A5 vision, A7 vessels, A16 endocrine, A17 neoplasia, H1 genomic instability, H2 telomeres, H5 autophagy, H10 intercellular communication, H12 dysbiosis.
- **Candidate in axes.md but no dossier file yet:** H4, H8, H9, H11.
- **Readout without functional anchor:** H3 (clocks).

## Cross-cutting flags for the metric

- **Non-monotonic axes:** A8 lung, A10 glucose tolerance, A15 fat. A "better" value in an old mouse can mean decline; score on trajectory, not a single value.
- **Terminal or single-use tests:** A8 forced oscillation (terminal); A11 (new antigen needed at each time point); A13 wound (each test is a new injury); germinal-centre cell counts (terminal).
- **Repeatable on the same mouse:** A1, A9 (transcutaneous GFR), A12, A15, A18.
- **Mid-life rather than late-life signal:** A3, A8, A13. These may separate young from middle-aged mice better than old from very old.
- **Strain:** the direction of the age effect changes with strain for A9 and A11. No axis has UM-HET3 data.
