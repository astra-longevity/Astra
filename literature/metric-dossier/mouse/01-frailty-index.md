# Mouse clinical frailty index (31 items) and frailty clocks

Axis: A18 whole-body frailty (axes.md @116f1f8). Also feeds A1 and A13 through single items (e.g. coat condition).

## What it measures

A trained person handles and watches the mouse and scores 31 visible health problems (coat, hearing, eyes, gait, tremor, body condition, etc.): 0 = none, 0.5 = mild, 1 = severe. The index is the sum divided by 31, so 0 = no deficit, 1 = every deficit severe (Whitehead 2014). It copies the human deficit-count frailty index.

Two machine-learning "frailty clocks" are built on these items (Schultz 2020):
- **FRIGHT**: predicts the mouse's age from its frailty items;
- **AFRAID**: predicts how long the mouse has left to live.

## Training or reference population

| Tool | Mice | Source |
| --- | --- | --- |
| 31-item index, validation | C57BL/6J, n = 14 followed at 5, 19 and 28 months | Whitehead 2014 |
| FRIGHT and AFRAID clocks | Male C57BL/6Nia, n = 60, scored about every 6 weeks until death; median survival 31 months, 90th percentile 36 months | Schultz 2020 |
| Cross-strain check | 1,564 mice (690 female, 874 male), 17 cohorts, 5 strains: C57BL/6JNIA, C57BL/6N, C57BL/6J, UM-HET3, Diversity Outbred; 3,665 observations, 2,192 linked to age at death | Zhu 2026 |

**Consequence for Astra.** The index and both clocks were built on C57BL/6, and the clocks on **males only**. The ITP uses UM-HET3 of both sexes. Zhu 2026 found frailty rises at a **different rate per strain**, and states that one threshold for all strains may miss frailty in some and overstate it in others. Readings on UM-HET3 or females are "outside training range" for FRIGHT/AFRAID until a validation is cited; the raw index needs strain- and sex-specific reference curves.

## Noise

There is no published same-mouse test-retest study found yet; the published noise is **between raters**.

| Study | Setting | Inter-rater ICC |
| --- | --- | --- |
| Feridooni 2015 | 233 male C57BL/6J, 11.5–14 months, 2 raters, criteria refined during the study | 0.77 (95% CI 0.67–0.83) by the final set |
| Kane 2015 | Longevity-intervention study, mean of 2 raters | 0.88 (0.80–0.92) per PMC full text; a secondary source quotes 0.92 inter-rater and 0.93 intra-rater `[to verify]` which figure is the paper's |
| Kane, rater-factor study (4 raters, 45 mice) | 2 experienced + 2 inexperienced raters | 0.80 overall; 0.76 across experience levels; 0.86 across rater sex `[to verify]` primary (found in a thesis draft and a figure caption) |
| Howlett 2015 (commentary summarising Kane's projects) | Technicians vs researchers | **0.20–0.49** (poor) between raters of different professional background; 0.88 between scientists |

- In plain English: two raters of the same background agree well (ICC ~0.8–0.9); raters from different backgrounds agree poorly. Rule: **one rater (or one trained team of the same background), blinded to treatment, for all readings of a cohort.**
- Smallest real change for one mouse: not published. `[to verify]` Needs a repeat-scoring study (same mouse, same day, same and different raters).
- AFRAID's error on remaining lifespan: median 53 days across ages (Schultz 2020, Nature Communications); the earlier conference abstract gave r² = 0.375 and median error 46.4 days (Schultz 2019 abstract). Use the journal figure.

## Known failure modes

- Rater background and handling technique (above).
- Environment: time and place of scoring, housing. `[to verify]` effect sizes.
- Floor effect in young mice: the index is near 0.02 at 5 months (Whitehead 2014), so it says little before middle age.
- Strain calibration (Zhu 2026, above).

## Functional anchor and age trajectory

- The index is itself a whole-body functional readout.
- C57BL/6J trajectory (31-item index, mean ± SE, n = 14): **0.02 ± 0.005 at 5 months, 0.12 ± 0.008 at 19 months, 0.33 ± 0.02 at 28 months** (Whitehead 2014). Plotted against age scaled to each species' 90% mortality age (925 days for C57BL/6J, 102 years for humans), the mouse and human curves were similar (Whitehead 2014).
- Moves the right way under longevity interventions: calorie restriction and resveratrol lowered the index in C57BL/6J; short-lived DBA/2J scored higher (Kane 2015). This makes it one of the few axes with a published "right-way" control.
- Sex: C57BL/6 females scored higher than males at 21 and 27 months (Collinge 2024); Zhu 2026 found the sex difference only in C57BL/6JNIA.

## Confounders

Strain, sex, rater, handling, housing and time of assessment, acute illness or injury at scoring.

## Sources

- Whitehead JC et al. A clinical frailty index in aging mice: comparisons with frailty index data in humans. J Gerontol A 2014 (issued 2013). doi:10.1093/gerona/glt136 (Crossref OK)
- Schultz MB et al. Age and life expectancy clocks based on machine learning analysis of mouse frailty. Nat Commun 2020. doi:10.1038/s41467-020-18446-0 (Crossref OK)
- Feridooni HA et al. Reliability of a frailty index based on the clinical assessment of health deficits in male C57BL/6J mice. J Gerontol A 2015 (online 2014). doi:10.1093/gerona/glu161 (Crossref OK)
- Kane AE et al. Impact of longevity interventions on a validated mouse clinical frailty index. J Gerontol A 2016 (online 2015). doi:10.1093/gerona/glu315 (Crossref OK)
- Howlett SE et al. Factors that influence reliability of the mouse clinical frailty index. J Gerontol A 2015. doi:10.1093/gerona/glv045 (Crossref OK)
- Zhu et al. The association of frailty with age and lifespan in mice differs by strain and sex. J Gerontol A 2026. doi:10.1093/gerona/glag167 (Crossref OK)
- Collinge et al. The mouse Social Frailty Index (mSFI). GeroScience 2024. doi:10.1007/s11357-024-01263-4 (Crossref OK)
- Schultz MB et al. Machine learning analysis of mouse frailty for prediction of biological age and life expectancy (abstract). Innov Aging 2019. doi:10.1093/geroni/igz038.3299 (Crossref OK)
