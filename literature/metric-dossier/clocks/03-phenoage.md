# DNAm PhenoAge (Levine 2018)

## What it measures

Built in two steps. First, a "phenotypic age" was made from calendar age plus nine routine blood markers, weighted by how well they predict death. Second, 513 methylation sites were fitted to predict that phenotypic age. So it aims at **mortality-linked ageing**, not calendar age.

## Training population

- Step 1 (phenotypic age): US national health survey data (NHANES). `[to verify]` which survey waves and n.
- Step 2 (methylation): secondary sources give 456 blood samples (InCHIANTI, Italy). `[to verify]`
- Age range quoted 21–100. `[to verify]`
- Consequence: an adult European-ancestry blood sample is closest to training; other ancestries `[to verify]`.

## Noise

- Original version: technical replicates can differ by several years; one of the noisiest common clocks before the principal-component fix (Higgins-Chen 2022). `[to verify]` value.

## Known failure modes

- Inherits the nine blood markers' biases (infection, recent illness move them).
- Replicate noise large enough to swamp small intervention effects unless the PC version is used.

## Functional anchor and age trajectory

Anchored to mortality and disease risk, not to a function test. Reported associations: all-cause mortality, cancer, heart disease, Alzheimer's (secondary review). `[to verify]` effect sizes.

## Confounders

Blood cell-type mix, acute inflammation, smoking. `[to verify]`

## Sources

- Levine ME et al. An epigenetic biomarker of aging for lifespan and healthspan. Aging 2018. doi:10.18632/aging.101414 (Crossref OK)
- Higgins-Chen 2022, doi:10.1038/s43587-022-00248-2 (Crossref OK)
