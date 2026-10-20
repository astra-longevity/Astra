# DNAm GrimAge (Lu 2019)

## What it measures

A score built to predict **time to death**. It combines methylation-based estimates of several blood proteins and of smoking history (pack-years), plus age and sex. Uses 1,030 methylation sites.

## Training population

- Blood from the US Framingham Heart Study (FHS) Offspring cohort, split by family: 1,731 people (622 families) for training, 625 people (266 families) for testing. Mean age 66 (training) and 67 (test) (Lu 2019).
- Validation: 7,375 arrays from 6,935 people in three other cohorts (WHI, JHS, InCHIANTI): 50% European ancestry, 40% African American, 10% Hispanic (Lu 2019).
- Consequence: training was on people in their 60s on average. A reading on a younger adult (such as the president) is further from the training data; the full training age range is not stated here and must be read from Lu 2019 Supplementary Table 1 before use on him.

## Version 2 (GrimAge2)

Same 1,030 sites, plus methylation estimates of two clinical blood markers (CRP and HbA1c). Retrained on FHS (1,833 training, 711 test) and validated in about 13,400 blood samples from nine cohorts; it beat version 1 on time to death and heart disease (Lu 2022).

## Noise

- Among the more stable original clocks between technical replicates (Higgins-Chen 2022). No smallest-real-change figure is quoted here.

## Known failure modes

- Large part of its signal is smoking history: in a non-smoker it carries less information.
- Includes sex and age directly, so "acceleration" must be computed carefully.

## Functional anchor and age trajectory

Anchored to death and age-related disease. In the Lothian Birth Cohort 1936 (an outside cohort), each standard deviation of higher GrimAge meant 1.81 times the death risk (95% CI 1.58–2.07), against 1.17–1.22 for Hannum, Horvath and PhenoAge in the same cohort (Hillary 2019).

## Confounders

Smoking, sex and age (each is a direct input of the score).

## Sources

- Lu AT et al. DNA methylation GrimAge strongly predicts lifespan and healthspan. Aging 2019. doi:10.18632/aging.101684 (Crossref OK)
- Lu AT et al. DNA methylation GrimAge version 2. Aging 2022. doi:10.18632/aging.204434 (Crossref OK)
- Hillary RF et al. An epigenetic predictor of death captures multi-modal measures of brain health. Molecular Psychiatry 2019 (online). doi:10.1038/s41380-019-0616-9 (Crossref OK)
- Higgins-Chen 2022, doi:10.1038/s43587-022-00248-2 (Crossref OK)
