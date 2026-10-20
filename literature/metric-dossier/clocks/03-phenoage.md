# DNAm PhenoAge (Levine 2018)

## What it measures

Built in two steps. First, a "phenotypic age" was made from calendar age plus nine routine blood markers, weighted by how well they predict death. Second, 513 methylation sites were fitted to predict that phenotypic age. So it aims at **mortality-linked ageing**, not calendar age.

## Training population

- Step 1 (phenotypic age): US national health survey NHANES III (1988–94), 9,926 adults aged 20+ with complete blood markers and up to 23 years of mortality follow-up; checked in NHANES IV (1999–2014), 6,209 adults (Levine 2018).
- Step 2 (methylation): 456 people from the Italian InCHIANTI study, blood sampled at two time points, ages 21–100; 513 sites chosen from 20,169 (Levine 2018).
- Consequence: an adult blood sample from a European-ancestry population is closest to the methylation training set; fit in other ancestries is not stated here.

## Noise

- Original version: technical replicates can differ by several years; one of the noisiest common clocks before the principal-component fix (Higgins-Chen 2022).

## Known failure modes

- Inherits the nine blood markers' biases (infection, recent illness move them).
- Replicate noise large enough to swamp small intervention effects unless the principal-component version is used.

## Functional anchor and age trajectory

Anchored to mortality and disease risk, not to a function test. Across the validation cohorts, each extra year of DNAm PhenoAge went with about 4.5% higher risk of death (Levine 2018 abstract). Also linked to more coexisting diseases and physical-function limits (Levine 2018).

## Confounders

Blood cell-type mix, acute inflammation, smoking. List not exhaustive.

## Sources

- Levine ME et al. An epigenetic biomarker of aging for lifespan and healthspan. Aging 2018. doi:10.18632/aging.101414 (Crossref OK)
- Higgins-Chen 2022, doi:10.1038/s43587-022-00248-2 (Crossref OK)
