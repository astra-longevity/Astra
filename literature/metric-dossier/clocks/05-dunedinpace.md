# DunedinPACE (Belsky 2022)

## What it measures

Not an age but a **speed**: how many years of biological ageing happen per calendar year (1.0 = average). Trained on how fast 19 markers of organ health changed in the same people over 20 years, then turned into a single blood methylation score (173 sites).

## Training population

- Dunedin birth cohort, New Zealand: everyone born in one city in 1972–73, measured at ages 26, 32, 38 and 45. Organ markers cover heart, metabolism, kidney, liver, immune, gums and lungs.
- All members are the same age. Analysed sample size and ancestry mix are not quoted here (read from Belsky 2022 Methods before use).
- Consequence: trained on midlife adults (26–45). Older readings are extrapolation; flag "outside training range" above 45 until validation data are cited.

## Noise

- Designed for low replicate noise: only methylation sites with good test-retest reliability were allowed into training.
- Technical replicates: ICC 0.96 (0.93–0.98) in the Lehne replicate dataset; 0.97 (0.94–0.98) on EPIC-vs-EPIC replicates; 0.87 (0.82–0.90) when one replicate is on the older 450k array and one on EPIC (Belsky 2022). Training on all sites instead gave ICC 0.72.
- Rule: **one array type for all of one person's samples.**
- Smallest real change for one person: not published in this source (named gap).

## Known failure modes

- One birth cohort, one country.
- Measures pace at the time of sampling; says nothing direct about accumulated damage.

## Functional anchor and age trajectory

Directly anchored to change in organ-function markers (the closest of the five clocks to a functional anchor). In the Normative Aging Study (older US men), each standard deviation of faster DunedinPACE meant 1.26 times the death risk (95% CI 1.14–1.40) and 1.23 times the risk of a new chronic disease (1.07–1.42), adjusted for age; effect sizes were similar to GrimAge (Belsky 2022).

## Confounders

Candidates not quantified in the source here: blood cell-type mix, smoking, socioeconomic status. Array type (450k vs EPIC) is a measured one (see Noise).

## Sources

- Belsky DW et al. DunedinPACE, a DNA methylation biomarker of the pace of aging. eLife 2022. doi:10.7554/eLife.73420 (Crossref OK)
