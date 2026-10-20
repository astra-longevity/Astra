# L3 obstacle rows (O3, O4, O6, O10)

Status: v0, 2026-10-20 (A70, for A2). Schema: `README.md` in this folder. All DOIs checked on Crossref on 2026-10-20.

These rows are descriptive. Lifespan results appear only in O4 and O6, which are about lifespan by design, and never feed a mapping row. O4 and O6 counts come from the L1 table (`literature/lifespan/itp/`, 132 rows, ITP cohorts 2004 to 2017). The held-out arms of the metric split (`research/metric/v0/split.csv`, ITP 2021 to 2025) are not in L1 and were not read; no 2021+ ITP paper is cited here.

## O3: causes of death in control mice

**Record.**

| Source | Mice | Finding |
| --- | --- | --- |
| Brayton 2012, doi:10.1177/0300985811430696 | review table, C57BL/6J (UW colony) | neoplasia as cause or contributor of death: 50% of males, 43% of females; lymphoma or other blood cancers 38% / 35% |
| Zhang 2013, doi:10.1093/gerona/glt056 | C57BL/6 male controls | presumed cause of death neoplasia in 28 of 45; lymphoma found in 27 of 45 |
| Miller 2020, doi:10.1172/jci.insight.140019 | UM-HET3 male controls (ITP) | 36 of 44 evaluable controls died of neoplasia |
| Blackwell 1995, doi:10.1177/019262339502300503 | C57BL/6, NIH-31 diet | in females, lymphoma 29% and pituitary tumours 37% at death (ad libitum) |
| Elies 2024, doi:10.1177/01926233241248658 | C57BL/6J, 18-month carcinogenicity controls | tumours caused 40% of deaths in females and 8.5% in males; most male deaths non-tumoral |

How interventions shift it: Miller 2020 reports that in canagliflozin-treated mice (and in earlier ITP necropsies of 17α-estradiol, acarbose and rapamycin) the spectrum of lethal lesions was similar to controls "but delayed in timing". Neff 2013 (doi:10.1172/JCI67674) argues rapamycin extends lifespan in B6 males largely by delaying cancer, with limited effects on other ageing traits.

**What this means for the model.** In both strains, most control deaths are cancer, mainly lymphoma, and the share differs by sex and colony (Elies 2024 vs Brayton 2012). A simulation that predicts lifespan from organ damage alone is in practice predicting time to lymphoma. Necropsy data are end-of-life only: they say what killed the mouse, not when the cancer started.

**Missing experiment.** Cross-sectional necropsy series (serial sacrifice at fixed ages) of treated vs control UM-HET3 mice, both sexes, scoring tumour incidence and size at each age. That separates "cancer delayed" from "cancer prevented" from "cancer tolerated longer", which predict different model structures (shift of onset vs lower rate vs longer survival with tumour).

**Novelty search.** PubMed, 2026-10-20: `rapamycin[tiab] AND mice[tiab] AND (neoplasia[tiab] OR tumor incidence[tiab]) AND (serial sacrifice[tiab] OR cross-sectional[tiab] OR age-specific[tiab])` → 0 hits. Partial precedent for B6 males only: Neff 2013 sampled 3 age groups.

**Evidence gaps.** No UM-HET3 female control cause-of-death counts here (Miller 2020 row is males only). No age at tumour onset for either strain, only cause at death. B6 figures come from different colonies and diets, so they are not pooled.

## O4: combination studies

**Record.** L1 holds two ITP combinations:

| Arm | Cohort | Effect on median lifespan (L1) | Single-drug arms in L1 |
| --- | --- | --- | --- |
| metformin + rapamycin, from 9 mo | 2011 | F +23.3%, M +22.9% | metformin alone, same cohort: F −0.2%, M +7.1% (both null); rapamycin alone: not in the same cohort |
| rapamycin + acarbose, from 9 mo | 2017 | F +27.5%, M +36.6% | neither drug alone in the same cohort |
| rapamycin + acarbose, from 16 mo | 2017 | F +12.4%, M +13.5% | same |

Sources: Strong 2016 doi:10.1111/acel.12496; Strong 2022 doi:10.1111/acel.13724. Earlier single-drug arms at similar doses exist (rapamycin 2009, Miller 2014 doi:10.1111/acel.12194; acarbose 2009, Harrison 2014 doi:10.1111/acel.12170), but in other cohorts, with control medians that differ between cohorts.

**What this means for the model.** Whether two interventions add up can't be read from the ITP: no combination has both single arms run alongside it in the same cohort. Any comparison crosses cohorts, so cohort differences and the effect of combining are mixed together. R-Sim has no published basis for how two mapped interventions combine.

**Missing experiment.** A full 2×2 factorial (control, A, B, A+B) in one cohort, both sexes, with a pre-registered test of the interaction, for rapamycin × acarbose first (largest combined effect). It would also measure the metric's axes at fixed ages, so the interaction can be seen on organ readings, not only on lifespan.

**Novelty search.** PubMed, 2026-10-20: `lifespan[tiab] AND mice[tiab] AND factorial[tiab] AND (drug[tiab] OR compound[tiab])` → 0 hits.

## O6: ITP results by sex and site

**Record** (L1, 132 rows, counted by script on 2026-10-20; role = pooled-site log-rank p < 0.05):

- 64 compound/dose/start groups have both sexes: both extend 15, males only 9, females only 2, neither 38. Sex-specific results are mostly male-only.
- 48 arms carry per-site results. Of the 27 arms that extend lifespan pooled, 18 have per-site median effects; in 6 of them one site shows the opposite sign (1,3-butanediol F; rapamycin + acarbose from 16 mo M; captopril F and M; glycine F and M). Across all 48, 21 arms have sites of opposite sign.
- Gap in L1: per-site medians are not extracted for the 2004-2009 arms (printed only as p90 or not printed); these 9 arms are blank in the count above.

Mechanism of the sex split, partly: Garratt 2017 (doi:10.1111/acel.12656) found that removing the gonads changes the male-only benefit of acarbose and 17α-estradiol, so sex hormones are involved.

**What this means for the model.** An effect found in one sex does not transfer to the other, and a pooled effect can hide sites that disagree. R-Sim mappings need sex as a factor, and calibration should be checked per site, not only pooled.

**Missing experiment.** For a male-only drug (acarbose or 17α-estradiol), a both-sex study measuring target engagement and the metric's axes in the same mice over time, with castrated and ovariectomised arms. It would show whether females fail to respond at the target or downstream of it, which decides where sex enters the model.

**Novelty search.** PubMed, 2026-10-20: `(gonadectomy[tiab] OR castration[tiab] OR ovariectomy[tiab]) AND lifespan[tiab] AND (acarbose[tiab] OR 17alpha-estradiol[tiab] OR 17-alpha estradiol[tiab] OR canagliflozin[tiab] OR rapamycin[tiab])` → 6 hits (Garratt 2017 and follow-ups on 17α-estradiol, Arriola Apelo 2020 doi:10.7554/eLife.56177 on ovariectomy); none measures target engagement over time in both sexes.

## O10: long-lived species

**Record.**

| Species | Mechanism | Source | Tested in mice? |
| --- | --- | --- | --- |
| naked mole-rat | very high-molecular-mass hyaluronan (from its Has2) makes cells resist malignant transformation | Tian 2013, doi:10.1038/nature12234 | yes: mice carrying naked mole-rat Has2 had fewer spontaneous and induced tumours, less inflammation and a longer median lifespan (Zhang 2023, doi:10.1038/s41586-023-06463-0) |
| Myotis bats | telomeres do not shorten with age in the longest-lived genus | Foley 2018, doi:10.1126/sciadv.aao0926 | no |
| bats | bat ASC2 dampens the inflammasome | Ahn 2023, doi:10.1016/j.cell.2023.03.036 | yes, for inflammation: mice carrying bat ASC2 had milder inflammatory disease; lifespan not tested |

Each mechanism gets its own edge row in `edges.md` once A68's edge list lands.

**What this means for the model.** Only one mechanism (naked mole-rat hyaluronan) has been moved into mice and tested on lifespan; it works through cancer, which fits O3. The bat mechanisms are untested on ageing in mice.

**Missing experiment.** Mice carrying bat ASC2 (or Myotis telomere-maintenance genes) followed to death, both sexes, with necropsy cause of death and the metric's axes measured over time. It tests whether lowering inflammation alone slows organ ageing, which R-Sim currently has no causal data for (H11 has no right-way control, see metric dossier).

**Novelty search.** PubMed, 2026-10-20: `(naked mole-rat[tiab] OR bat[tiab]) AND transgenic[tiab] AND mice[tiab] AND (lifespan[tiab] OR longevity[tiab] OR cancer[tiab])` → 9 hits; only Zhang 2023 tests lifespan; Ahn 2023 tests inflammation only.

**Evidence gaps.** One lifespan test only (Zhang 2023, one lab), not replicated. Bat telomere data are from wild bats, no causal test. Other proposed mechanisms (e.g. naked mole-rat protein stability, bat DNA repair) are not covered in this row yet; they get edge rows only if `edges.md` lists them.
