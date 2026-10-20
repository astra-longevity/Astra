# Horvath 2013 multi-tissue clock

## What it measures

An estimate of age from chemical marks (DNA methylation) at 353 sites of the genome. It was trained to predict **calendar age**, not health. The gap between the estimate and calendar age ("age acceleration") is what ageing studies use.

## Training population

- About 8,000 samples from 82 public methylation datasets, 51 healthy tissues and cell types, ages 0 to 100, Illumina 27K and 450K arrays (Horvath 2013; consistent across secondary reviews).
- Number actually used to fit the model: secondary sources give 3,931 (training set) or 7,844 (all non-cancer samples). `[to verify]` in the primary paper.
- Sex and ancestry mix of the training set: not yet extracted. `[to verify]`
- Consequence for the president: an adult blood sample is inside the training range. The 450K/27K CpG set must be present on the array used (EPIC drops some sites). `[to verify]`

## Noise

- Reported fit: correlation about 0.96 with age, median error about 3.6 years in test data (Horvath 2013, as quoted by several reviews). `[to verify]` exact figure in primary.
- Technical replicates of the same sample can differ by several years; principal-component versions reduce this (Higgins-Chen 2022). Exact replicate spread per clock: `[to verify]`.
- Smallest real change for one person: not yet established; must come from replicate data, not from the fit error.

## Known failure modes

- Trained on calendar age: the better a clock fits calendar age, the less room it leaves for biological differences between people of the same age.
- Sensitive to array type, batch and preprocessing; missing CpGs on newer arrays.
- Cancer tissue reads very differently (Horvath 2013 reported on ~6,000 cancer samples separately). `[to verify]`

## Functional anchor and age trajectory

- No direct functional anchor: it tracks calendar age. Fast tick in childhood, then roughly linear in adults (log-linear transform below age 20). `[to verify]` transform details.

## Confounders

Blood cell-type mix, array batch, sample handling. Others `[to verify]`.

## Sources

- Horvath S. DNA methylation age of human tissues and cell types. Genome Biology 2013. doi:10.1186/gb-2013-14-10-r115 (Crossref OK)
- Higgins-Chen AT et al. A computational solution for bolstering reliability of epigenetic clocks. Nature Aging 2022. doi:10.1038/s43587-022-00248-2 (Crossref OK)
