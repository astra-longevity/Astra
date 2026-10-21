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
| JAX | JAX Aging Center phenotyping, Diversity Outbred | longitudinal phenotyping | public via Mouse Phenome Database (phenome.jax.org); offerings incl. frailty index, grip, rotarod, acoustic startle, CBC, GTT, urine, echo, body composition (Korstanje 2021, doi:10.1007/s11357-021-00421-2, PMC8599562, opened 2026-10-21); which DO aging sets are in MPD: to pick |
| GEO-M | mouse methylation clock datasets on GEO | DNA methylation by tissue and age | GSE80672 (whole-blood RRBS, 255 samples, ages, strains and diets incl. CR; GEO record opened 2026-10-21); other tissues: further series still to pick (L2, A70) |

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
| CR | Caloric restriction | right way | many studies | Weindruch 1986 (PMID 3958810, abstract opened 2026-10-21): female mice of a long-lived strain, restricted after weaning, mean and maximum lifespan 35–65% above ad libitum |
| ERCC1 | Ercc1−/Δ (DNA-repair deficient) | wrong way (ages ~6× faster, lifespan 4–6 months) | multi-organ progeroid | https://www.aging-us.com/article/202321/text; median lifespan 20 weeks vs 118 weeks in wild-type siblings (Ercc1−/Δ7, Dollé 2011, doi:10.3402/pba.v1i0.7219, PMC3417667, opened 2026-10-21) |
| LMNA | Lmna G609G (progeria knock-in) | wrong way (homozygous median ~103 days) | progeroid | doi:10.1126/scitranslmed.3002847; survival figures via doi:10.7554/eLife.82728 |
| HFD | High-calorie (high-fat) diet | wrong way (survival lower than standard diet; magnitude `[to verify]`) | C57BL/6, from 1 year | doi:10.1038/nature05354; context: https://www.sciencedirect.com/science/article/pii/S1550413108001824 |
| IRR | Sub-lethal total-body irradiation (6.5 Gy) | wrong way (sustained HSC loss, HSC senescence; lifespan effect `[to verify]`) | C57BL/6 | doi:10.1182/blood-2005-04-1418 |

Caveat on RAPA (Neff 2013, doi:10.1172/JCI67674): rapamycin extended
lifespan, but many of its effects on traits also appeared in **young** mice,
so they were drug effects, not slowed ageing. A right-way control must show
an age × treatment interaction, not just a treatment effect (see
`test1-design.md`). Issue I-6.

Note: a progeroid model is not "normal ageing, faster". Ercc1 and Lmna mainly
model genomic and nuclear-envelope damage. Using one of them as the wrong-way
control for an unrelated axis has to be justified per axis, or it is a weak
control. This is logged as issue I-3 below.

## Table A: organ-system axes

| # | Axis | Functional anchor | Candidate public readout (dataset) | Right way | Wrong way | Status |
|---|---|---|---|---|---|---|
| A1 | Skeletal muscle | grip strength, rotarod | TMS limb muscle transcriptome; FRL frailty items | RAPA fed from 19 months, C57BL/6: rotarod and stride length better than controls at 31 months but not at 25 (age-dependent), grip strength unchanged (Zhang 2014, https://academic.oup.com/biomedgerontology/article/69A/2/119/514438); UM-HET3 rapamycin from 4 or 16 months improves rotarod at 22 months (PMC9977253), own readout (rotarod; grip does not move) | ERCC1: grip strength and rotarod decline from 8 to 16 weeks in Ercc1Δ/− (doi:10.1007/s00401-010-0715-9), own readout | candidate (molecular only; functional readout `unmeasured`) |
| A2 | Bone | bone mineral density (micro-CT) | none confirmed | CR in aged (17–20 month) male mice attenuated trabecular bone loss on tibia micro-CT, no effect in young (doi:10.3389/fendo.2024.1394263), own readout, age × diet. RAPA does **not** qualify: 2 mg/kg i.p. every 5 days for 8 weeks in 20-month female C57BL/6 left bone microarchitecture and strength unchanged (JBMR Plus ziae001, https://academic.oup.com/jbmrplus/article-pdf/8/2/ziae001/57406113/ziae001.pdf) | LMNA (osteoporosis, secondhand via doi:10.1111/acel.13457) | unmeasured |
| A3 | Brain, cognition | Barnes or Morris water maze | TMS brain transcriptome | RAPA chow, C57BL/6 both sexes: blocks age-associated cognitive decline in old mice (doi:10.1016/j.neuroscience.2012.06.054); also improves young mice, so age × treatment needed (I-6) | ERCC1: accelerated age-related cognitive decline in Ercc1 mutants, F1 C57BL/6J-FVB/N (Borgesius 2011, doi:10.1523/JNEUROSCI.1589-11.2011), own readout (task `[to verify]`) | candidate (molecular only) |
| A4 | Hearing | auditory brainstem response (ABR) threshold | none confirmed | CR, 10 months, C57BL/6J: prevents the age rise in ABR thresholds at 12 months in wild-type, not in Sirt3−/− (Someya 2010, doi:10.1016/j.cell.2010.10.002, PMC3018849, opened 2026-10-21); flag: C57BL/6J carries early-onset hearing loss, so this is one strain | ERCC1 (hearing loss, secondhand `[to verify]`) | unmeasured |
| A5 | Vision | electroretinogram / optomotor response | none confirmed | `[to verify]` | `[to verify]` | unmeasured |
| A6 | Heart | echocardiography (ejection fraction, diastolic function) | TMS heart transcriptome | RAPA, 3 months from 24 months, female C57BL/6J: age-related heart changes reversed or attenuated (doi:10.1111/acel.12109); CR or RAPA for 10 weeks in 27-month mice reversed diastolic dysfunction and hypertrophy (PMC4040127), own readout | HFD for 8 weeks in 76-week male C57BL/6J: heart failure, lower ejection fraction; young mice unaffected (PMID 40637820), own readout, age × diet | candidate (molecular only) |
| A7 | Vessels | pulse-wave velocity, blood pressure | TMS aorta `[to verify]` | `[to verify]` | LMNA (VSMC loss, secondhand via doi:10.1111/acel.13457) | unmeasured |
| A8 | Lung | lung function (plethysmography) | TMS lung transcriptome | **No known right-way intervention** on a valid readout. Only hit: clearing senescent cells "restores lung compliance" in aged mice (cited in doi:10.1038/ncomms14532, an IPF model paper), but compliance *rises* with age (note below), so this is not a right-way pass. Search: "intervention attenuates age-related decline lung function aged mice flexiVent compliance treatment old" (2026-10-20) | `[to verify]` | candidate (molecular only) |
| A9 | Kidney | glomerular filtration, albuminuria | TMS kidney transcriptome | CR, **rat only** (prevents age rise in albuminuria, F344: PMID 1415585; less glomerulosclerosis, doi:10.1152/ajprenal.00307.2006); mouse source `[to verify]` | HFD, C57BL/6 16 weeks: albuminuria and raised BUN/creatinine with glomerular filtration-barrier defects (doi:10.1038/s41419-020-03122-4); 45% fat 14 weeks: albuminuria and glomerular hypertrophy (PMC8295594), own readout (albuminuria) | candidate (molecular only) |
| A10 | Liver and metabolism | glucose tolerance test | TMS liver; ITP body weight | CANA 180 ppm from 7 months, UM-HET3: lower fasting glucose and better glucose tolerance in both sexes at 22 months (Miller 2020, as reported in doi:10.1111/acel.13653), own readout; glucose lowering is also a drug effect in young mice, so age × treatment needed (I-6), and A10 is non-monotonic (below) | HFD (doi:10.1038/nature05354; glucose intolerance own readout: HFD-fed C57BL/6J raise IPGTT AUC within 3 weeks and keep it elevated, doi:10.1016/j.aohep.2020.100621 / S2095882X20300621; two-phase IPGTT-AUC rise PMID 25170916), own readout. HFD impairs the GTT at all ages, so this is a level move consistent with the non-monotonic rule | candidate |
| A11 | Immune system | antibody response to a vaccine | TMS spleen, marrow, thymus | RAPA in old mice: vaccinated rapamycin-treated mice all survived a lethal influenza challenge vs 9/12 vehicle deaths (doi:10.1126/scisignal.2000559), own readout | IRR `[to verify]` | candidate (molecular only) |
| A12 | Blood | complete blood count | TMS marrow | RAPA in old mice restored hematopoiesis (doi:10.1126/scisignal.2000559; blood count `[to verify]`). **No right-way control on the blood count found.** DR from 12 months in male C57BL/6 cut lymphoma but itself *induced* lymphopenia (PMID 8187095), so DR moves the count the "older" way; DR effects on HSC ageing are strain-dependent and old B6 marrow repopulates *better* than young (Blood 111(3):1709, https://ashpublications.org/blood/article/111/3/1709/25596/). Search: "aged mice complete blood count anemia lymphopenia reversed rapamycin OR dietary restriction old C57BL/6 hematology" (2026-10-20) | IRR (HSC loss, doi:10.1182/blood-2005-04-1418; blood count `[to verify]`) | candidate (molecular only) |
| A13 | Skin and hair | wound closure time, coat score (frailty item) | TMS skin; FRL coat items | SkQ1 (mitochondria-targeted antioxidant) speeds the impaired wound healing of old mice: faster re-epithelialisation and granulation (doi:10.18632/aging.100772), own readout; D+Q improves burn-wound healing in aged mice (PMID 42320016), own readout. Negative: long-term CR does **not** improve aged wound healing (https://www.sciencedirect.com/science/article/abs/pii/004763749601737X). Age × treatment not shown in either; limit: wound closure may track mid-life (note below) | ERCC1 (epigenetic age up in skin: doi:10.1101/2022.12.28.522011, preprint) | candidate |
| A14 | Gut | intestinal permeability | TMS large intestine | Young-donor microbiota transplant into old mice lowers FITC-dextran permeability and gut inflammation (doi:10.1038/s41522-025-00677-y), own readout; also a CR-enriched strain lowered permeability in germ-free mice given old microbiota (doi:10.1186/s40168-018-0440-5). Microbiota transfer is a procedure, not a drug; no drug right-way found. Search: "aged mice intestinal permeability FITC-dextran reduced by treatment old mice restores barrier" (2026-10-20) | HFD raises 4 kDa FITC-dextran permeability in C57BL/6 (doi:10.1038/s41598-017-04989-8; doi:10.3390/cells10113168), own readout | candidate (molecular only) |
| A15 | Fat tissue | body composition (fat mass) | TMS fat depots; ITP body weight | **No known right-way intervention** on the trajectory readout (A15 is non-monotonic, so lower fat in an old mouse is not a pass). Only hit: orexin injections reduced adiposity in old mice (secondhand, researchgate 317189676). Search: "aged mice adipose tissue dysfunction fat mass redistribution prevented late-life intervention old C57BL/6 body composition" (2026-10-20) | LMNA (fat loss, secondhand) | candidate |
| A16 | Endocrine and reproduction | oestrous cycling (female), testosterone (male) | none confirmed | Alternate-day feeding 3.5–10.5 months, C57BL/6J females: after return to ad libitum all still cycled vs 80% of controls acyclic; follicle reserve doubled at 12.5 months (Nelson 1985, doi:10.1095/biolreprod32.3.515, PMID 4039610, abstract opened 2026-10-21); female only | `[to verify]` | unmeasured |
| A17 | Neoplasia | tumour burden at necropsy | ITP necropsy reports `[to verify]` | GLY (fewer pulmonary adenocarcinoma deaths, doi:10.1111/acel.12953) | `[to verify]` | unmeasured |
| A18 | Whole-body frailty | 31-item frailty index | FRL | Enalapril 30 mg/kg/day in feed, C57BL/6: frailty index lower in middle-aged and old females and old males (doi:10.1093/gerona/gly219), own readout | ERCC1: social frailty index (mSFI) higher in Ercc1−/Δ and Xpg−/− than age-matched wild type, both sexes (doi:10.1007/s11357-024-01263-4); 31-item clinical frailty index in Ercc1−/Δ still `[to verify]` (frailty signs reduced by senolytics, Fuhrmann-Stroissnigg 2017, secondhand via S1568163723002775) | candidate |

**Non-monotonic axes: A10 and A15.** In C57BL/6, glucose tolerance worsens
up to ~18 months and then improves at 26–28 months as fat mass falls
(Reynolds 2019, doi:10.14814/phy2.13995; Reynolds 2024, Kondo 2023 via A70,
L2 PR #70, mouse/06–07). Clamp studies still show lower insulin sensitivity
in old mice (Ehrhardt 2019, Marmentini 2021, via A70). So for A10 and A15 the
score is the **trajectory** (peak, then loss), not the level. GTT is dosed per
lean mass with same-day body composition. A "better" value in an old mouse
does **not** count as a right-way pass in test 1.

**Lung (A8): non-monotonic, weight-adjusted, endpoint-only.** Lung
compliance *rises* with age (elastance and resistance fall), most change at
3–6 months, and reverses at 30 months in survivors (Huang 2007
doi:10.1152/japplphysiol.00400.2006; Elliott 2016
doi:10.1152/ajplung.00232.2016; Schulte 2019 doi:10.3389/fphys.2019.01466,
via A70 mouse/10). Body weight predicts all readings (Isbatan 2025, via
A70). flexiVent is terminal, so there is no within-mouse trajectory.
Higher compliance does **not** count as a right-way pass. Limit: **no
repeatable mouse lung test found** (blind-spot risk, commandment 6).

**Kidney (A9): longitudinal, mouse-only, two readings.**
- GFR by transcutaneous FITC-sinistrin, awake mice, repeatable over 3 days
  incl. 24-month B6 (Schock-Kusch 2013 doi:10.1371/journal.pone.0071519;
  matches plasma clearance, Mullins 2020 doi:10.1111/nep.13713, via A70
  mouse/11). This replaces the rat-only source as the readout. **No mouse
  right-way control on GFR or albuminuria found.** Closest: 50% CR from 8
  weeks in male C57BL/6 attenuated age-related kidney changes on histology
  (H&E, PAS, trichrome) at 12–16 months (PMID 41873790), not a GFR or
  albuminuria readout; the rat CR source (PMID 1415585) also found no GFR
  change with age in SPF F344. Search: "aged mice caloric restriction OR
  rapamycin albuminuria glomerular filtration rate kidney aging improved
  C57BL/6" (2026-10-20). Right-way stays `[to verify]`.
- Albuminuria / cystatin C, rising from middle age (Sataranatarajan 2012
  doi:10.1111/acel.12008).
- Protocol: fixed clock time and temperature (Wang 2026, via A70); the
  normalisation (body vs kidney weight) is stated.
- Limit: GFR falls with age in B6 and CBA but not in an F1 hybrid, and the
  gap vanishes per kidney weight (Hackbarth 1982 doi:10.1093/geronj/37.5.540).
  The B6 trend is not assumed for UM-HET3.

**Immune (A11): vaccine response.**
- Decline with age in B6 is mainly T follicular helper / germinal-centre
  failure, not B cells (Nicoletti 1994; Silva-Cayetano 2023
  doi:10.1038/s41590-023-01519-9; Fisher 2024 doi:10.1111/acel.14140, via
  A70 mouse/12).
- Protocol: fixed antigen, adjuvant and dose, no booster (adjuvant or booster
  masks the decline: Baldwin 2018 doi:10.3389/fimmu.2018.00295; Maxwell
  2026). If longitudinal, a new antigen at each time point.
- Titre and avidity reported separately (titre falls while avidity rises in
  females, Bufan 2020 doi:10.1016/j.exger.2020.110857); sex is a factor
  (Dhakal 2024, mbio.00326-24).
- Limit: strain can reverse the trend (aged BALB/c higher than young,
  Nicoletti 1991). The B6 trend is not assumed for UM-HET3.

**Skin (A13): wound closure and coat.**
- Wound closure: B6J males at 15 and 26–27 months did not differ (both
  slower than 6 months), so it may track mid-life more than late ageing
  (Cohen 1987 doi:10.1093/geronj/42.3.295, via A70 mouse/13). Named limit.
- Protocol: fixed splinted model; wound area and re-epithelialisation
  (sections) reported separately, they disagree (Bagood 2021
  doi:10.1016/j.jid.2020.10.022); sex and oestrous stage as factors (age
  effect in females only in BALB/cByJ, Mellers 2018; oestrogen rescue, Mukai
  2019 doi:10.1371/journal.pone.0225880). Each test is a new injury.
- Coat / alopecia: driven more by sex, substrain, barbering and fighting
  than by age (Ogiso 2025; Crum 2025, via A70). Low weight in the axis;
  housing notes required.
- The B6 trend is not assumed for UM-HET3.

**Gut (A14): FITC-dextran permeability.**
- 4 kDa FITC-dextran permeability rises with age in most studies, but
  depends on microbiota and TNF: absent in old germ-free and TNF-KO mice,
  transferable with old microbiota (Thevaranjan 2017
  doi:10.1016/j.chom.2017.03.002; Parker 2022 doi:10.1186/s40168-022-01243-w).
  No age change in SAMR1/SAMP1 (Kurihara 2025 doi:10.1016/j.jphyss.2025.100030).
- Confounds: high-fat diet and injury raise it (Ahmadi 2020
  doi:10.1093/gerona/glaa056; Najarro 2022 doi:10.1016/j.exger.2021.111654),
  so ill or diet-changed mice are excluded.
- Protocol: fixed probe, dose (fixed or per weight, stated), fasting and
  sampling time (1 h = small intestine, Woting 2018 doi:10.3390/nu10060685);
  microbiota and cage recorded as covariates.
- Limits: no test-retest or same-mouse longitudinal data found; the trend
  is not assumed for UM-HET3.

Sources in the five notes above were opened by A70 (L2), not by A67.

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
| H3 | Epigenetic alterations | methylation clock | GEO-M: GSE80672 (blood RRBS, 141 C57BL/6 males, Petkovich 2017) | CR slows blood methylome ageing (PMC5946071, data GSE80672), own readout; RAPA and CR in liver (doi:10.1186/s13059-017-1186-2, title only `[to verify]`) | ERCC1 (epigenetic age up in blood, brain, liver, muscle, skin: doi:10.1101/2022.12.28.522011, preprint) | candidate |
| H4 | Loss of proteostasis | autophagy **flux** (LC3-II ± lysosomal blocker), heart fixed in advance | TMS (chaperone genes) | CR or RAPA for 10 weeks in 27-month mouse hearts: longer protein half-lives, less oxidative damage and ubiquitination (PMC4040127), own readout (heart) | `[to verify]` | candidate (molecular only). **Flux, not level** (A70 mouse/18): direction is organ-dependent (Carosi 2025 pone.0325505); pick heart in advance; terminal. Raising flux improves function (Sebti 2022 15548627.2022.2111852) → candidate right-way |
| H5 | Disabled macroautophagy | autophagic flux (LC3-II turnover) | none confirmed | RAPA (mTOR inhibition, `[to verify]` on this readout) | `[to verify]` | unmeasured |
| H6 | Deregulated nutrient sensing | IGF-1, insulin, mTOR activity | ITP body weight; TMS | RAPA, ACA, CANA | HFD `[to verify]`; LMNA (IGF deficiency, secondhand) | candidate |
| H7 | Mitochondrial dysfunction | respiration (Seahorse), exercise capacity | TMS (OXPHOS genes) | SS-31 (elamipretide) in aged mice: better mitochondrial quality and exercise tolerance (PMID 30597195), own readout | HFD 8 weeks, C57BL/6J: work to exhaustion −36%, peak VO2 −16%, lower ADP-stimulated respiration and complex I/III activity (doi:10.1152/ajpheart.00267.2009), own readout (exercise capacity, respiration) | candidate (molecular only) |
| H8 | Cellular senescence | p16/p21 expression, SASP | TMS (Cdkn2a, Cdkn1a) | Dasatinib + quercetin (senolytic): fewer p16 and SA-β-gal positive cells in fat of old mice (PMC9924942); lower p16, p19, IL-6 in discs when started at 6, 14 or 18 months (doi:10.1038/s41467-021-25453-2), own readout | ERCC1 (senescence and SASP markers up: aging-us 202321); IRR (p16 and SA-β-gal up in HSCs: doi:10.1182/blood-2005-04-1418) | candidate |
| H9 | Stem cell exhaustion | competitive repopulation (HSC **function**, not count) | TMS marrow | RAPA in old mice restored HSC self-renewal (doi:10.1126/scisignal.2000559), own readout | IRR (sustained loss of HSC number and function: doi:10.1182/blood-2005-04-1418) | candidate (molecular only). **Score function, not count** (A70 mouse/16): B6 HSC count *rises* ~5× with age while function falls (Morrison 1996 nm0996-1011; Dykstra 2011 jem.20111490); DBA/2 count falls (Geiger 2005 pnas.0408654102). A right-way pass on count does not count; terminal readout |
| H10 | Altered intercellular communication | plasma proteome | none confirmed | `[to verify]` | `[to verify]` | unmeasured |
| H11 | Chronic inflammation | plasma IL-6, TNF | TMS immune cells (indirect) | **No known right-way intervention on plasma IL-6/TNF.** Tissue only: liver NF-κB p65 lower with rapamycin, acarbose, canagliflozin, 17α-estradiol, CR (PMC12181462); CR lowers macrophage IL-6/TNF ex vivo (PMID 1768737). Search: "aged mice plasma IL-6 TNF reduced rapamycin OR caloric restriction OR senolytic inflammaging old C57BL/6 serum" (2026-10-20) | HFD in male C57BL/6J: Tnf, Il1b, Mcp1 up in fat by 24 weeks (PMC4429090); tissue, not plasma (plasma `[to verify]`) | candidate (molecular only) |
| H12 | Dysbiosis | gut microbiome composition | none confirmed | Short-term dietary restriction in old C57BL/6J reverts the age-related microbiota imbalance (doi:10.1007/s10522-019-09830-5), own readout | HFD in C57BL/6: Firmicutes up, Bacteroidetes down, lower diversity (dysbiosis) (PubMed 40136712; doi:10.1016/j.cca.2023.117606 / S0167488923002161), own readout (composition) | candidate (molecular only) |

## DRiDO trait → axis map (test 4 / O5 dataset)

DRiDO (Di Francesco 2024, doi:10.1038/s41586-024-08026-3; data figshare
10.6084/m9.figshare.24600255, CC BY 4.0): ~960 Diversity Outbred **female**
mice, 194 traits, measured longitudinally from 3 months to death, with
per-mouse lifespan. This is the candidate dataset for test 4 (individual
level) and O5. The map below is built from the paper's published trait list
only; no per-mouse value and no lifespan result was opened.

| DRiDO trait group | Axis |
|---|---|
| grip strength | A1 skeletal muscle |
| rotarod | A1 skeletal muscle |
| bone (DXA BMD) | A2 bone |
| acoustic startle / PPI (hearing proxy) | A4 hearing `[to verify]` |
| echocardiography | A6 heart |
| glucose tolerance (GTT) | A10 glucose control (non-monotonic) |
| body composition (fat / lean mass) | A15 fat mass (non-monotonic) |
| complete blood count | A12 blood counts |
| frailty index (FRIGHT / Schultz items) | A18 whole-body frailty |
| flow-cytometry immune subsets | H9 stem cell / immune `[to verify]` |

Limits (stated for the seal and for test 4): females only, so no sex factor
within DRiDO; Diversity Outbred, not UM-HET3, so the strain-specific trends
in the axis rows are not assumed to transfer; the ad-libitum arm is the
untreated reference, while the CR 20/40% and intermittent-fasting arms are
interventions (CR is a known class) and go to **train** only. Data enters
through A9's manifest, not a hand download.



- 30 axes: 18 organ-system, 12 hallmark.
- `candidate`: 19. `unmeasured`: 11 (A2, A4, A5, A7, A16, A17, H1, H2, H5,
  H10, H12).
- Axes with **both** controls named from an opened source: 15. A1, A2, A3,
  A6, A9, A10, A12 (blood count `[to verify]`), A13 (wrong way preprint),
  A14, A18 (wrong way partly secondhand), H3, H7, H8, H9, H12.
- Both controls on the axis's **own readout**: 9 — the axes test 1 runs on in
  v0 (see `test1-design.md`): A1 (RAPA rotarod / Ercc1 grip and rotarod),
  A3 (RAPA blocks cognitive decline / Ercc1 accelerates it), A6 (CR or RAPA
  reverse diastolic dysfunction / HFD heart failure, both echo), A9 (right
  way still rat-only, so A9 is own-readout on the **wrong** side only — see
  obstacle table; counted here for the HFD albuminuria readout but test 1
  cannot pass it), A14 (young FMT lowers / HFD raises FITC-dextran),
  H3 (CR slows blood methylome / ERCC1 raises epigenetic age, preprint),
  H7 (SS-31 improves / HFD reduces exercise capacity and respiration),
  H8 (D+Q lowers p16 / IRR and ERCC1 raise it), H9 (RAPA restores HSC
  self-renewal / IRR depletes HSCs). A9 has no mouse right-way, so test 1
  runs on **8** (A1, A3, A6, A14, H3, H7, H8, H9).
- **Obstacle table — no known right-way intervention** after a recorded
  search (finding under commandment 6, not a gap to hide; for A28's obstacle
  register under O1):

  | Axis | Readout | What a right-way control would need | Search run (2026-10-20) |
  |---|---|---|---|
  | A8 | lung function | a drug/diet that improves a valid aged-mouse lung readout; compliance *rises* with age, so "restored compliance" is not a pass | "intervention attenuates age-related decline lung function aged mice flexiVent compliance treatment old" |
  | A9 | GFR / albuminuria | a **mouse** intervention that lowers aged albuminuria or raises GFR; only rat CR found, and 50% CR in B6 moved histology, not GFR/albuminuria | "aged mice caloric restriction OR rapamycin albuminuria glomerular filtration rate kidney aging improved C57BL/6" |
  | A12 | complete blood count | an intervention that corrects an aged CBC; dietary restriction itself induces lymphopenia (moves it "older") and old B6 marrow repopulates better than young | "aged mice complete blood count anemia lymphopenia reversed rapamycin OR dietary restriction old C57BL/6 hematology" |
  | A15 | fat mass (trajectory) | a late-life intervention acting on the non-monotonic fat trajectory; lower fat in an old mouse is not a pass | "aged mice adipose tissue dysfunction fat mass redistribution prevented late-life intervention old C57BL/6 body composition" |
  | H11 | plasma IL-6 / TNF | an intervention that lowers aged **plasma** IL-6/TNF; only tissue (liver p65, ex-vivo macrophage) effects found | "aged mice plasma IL-6 TNF reduced rapamycin OR caloric restriction OR senolytic inflammaging old C57BL/6 serum" |
- Flynn 2013 (doi:10.1111/acel.12109): only the abstract was read; the
  muscle and bone tests are not named there, so A1 and A2 right-way readouts
  stay `[to verify]`.
- Kidney (A9) right-way evidence is rat only. Mouse source needed.
- Most `candidate` axes are molecular only (TMS transcriptomes). Their
  functional anchor has no public readout yet. That gap is the main blind
  spot of v0.

## Open issues (commandment 8)

- I-1: TMS is one strain (C57BL/6JN) and cross-sectional. It can't serve test
  4 (individual level). Longitudinal data: FRL; SLAM blocked (O16).
- I-2: DRiDO (Di Francesco 2024) now gives functional tests (grip, rotarod,
  echo, GTT, frailty, CBC) **and** per-mouse lifespan on the same animals —
  but females only and Diversity Outbred, not UM-HET3 (see the DRiDO map
  above). It is the test-4 / O5 dataset. Still missing: a male and/or UM-HET3
  dataset with the same pairing. Ask L2 (A70) and A9.
- I-3: progeroid wrong-way controls are damage-specific. Each axis needs its
  own justified wrong-way control.
- I-4: whole-body frailty (A18) is also a test-2 baseline. Decide with A50 if
  it stays an axis.
- I-5: hallmark list DOI is in the repo source list but its check status is
  not shown as OK. Re-check.
- I-6: rapamycin changes many traits in young mice too (Neff 2013,
  doi:10.1172/JCI67674). As a right-way control it can read a drug effect,
  not slower ageing. Require a young-treated arm (age × treatment).
