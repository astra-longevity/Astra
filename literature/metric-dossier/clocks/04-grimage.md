# DNAm GrimAge (Lu 2019)

## What it measures

A score built to predict **time to death**. It combines methylation-based estimates of several blood proteins and of smoking history (pack-years), plus age and sex. Uses 1,030 methylation sites.

## Training population

- Blood from the US Framingham Heart Study Offspring cohort. Secondary sources give 1,731 training and 6,935 validation samples, ages about 40–92. `[to verify]`
- Consequence: people under ~40 are outside training range. The president's age must be checked against this. `[to verify]`
- Ancestry mainly European (Framingham). `[to verify]`

## Noise

- Among the more stable original clocks between technical replicates (Higgins-Chen 2022). `[to verify]` value.

## Known failure modes

- Large part of its signal is smoking history: in a non-smoker it carries less information.
- Includes sex and age directly, so "acceleration" must be computed carefully.

## Functional anchor and age trajectory

Anchored to death and age-related disease (time to cancer, heart disease reported). `[to verify]` effect sizes.

## Confounders

Smoking, sex, blood cell-type mix. `[to verify]`

## Sources

- Lu AT et al. DNA methylation GrimAge strongly predicts lifespan and healthspan. Aging 2019. doi:10.18632/aging.101684 (Crossref OK)
- Higgins-Chen 2022, doi:10.1038/s43587-022-00248-2 (Crossref OK)
