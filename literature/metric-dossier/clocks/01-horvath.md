# Horvath 2013 multi-tissue clock

## What it measures

An estimate of age from chemical marks (DNA methylation) at 353 sites of the genome. It was trained to predict **calendar age**, not health. The gap between the estimate and calendar age ("age acceleration") is what ageing studies use.

## Training population

- About 8,000 samples from 82 public methylation datasets, 51 healthy tissues and cell types, ages 0 to 100, Illumina 27K and 450K arrays (Horvath 2013).
- The model itself was fitted on 3,931 training samples; the rest served as test data (Horvath 2013, Figure 1 vs Figure 2).
- Sex and ancestry mix of the training set: not extracted here; treat as unknown.
- Consequence for the president: an adult blood sample is inside the training range. All 353 sites must be present on the array used; newer arrays drop some sites, so check coverage before use.

## Noise

- Reported fit: in test data, correlation 0.96 with age and median error 3.6 years, i.e. half of people read within 3.6 years of their calendar age (Horvath 2013). Training-data fit (0.97, 2.9 years) is over-optimistic by the author's own statement.
- Technical replicates of the same sample can differ by several years; principal-component versions reduce this (Higgins-Chen 2022).
- Smallest real change for one person: not yet established; must come from replicate data, not from the fit error.

## Known failure modes

- Trained on calendar age: the better a clock fits calendar age, the less room it leaves for biological differences between people of the same age.
- Sensitive to array type, batch and preprocessing; missing sites on newer arrays.
- Cancer: the 2013 paper reported strong age acceleration across ~6,000 cancer samples, but the 2015 erratum found a calibration error; after correction the author no longer observes positive age acceleration in cancer. Do not cite the 2013 cancer result.

## Functional anchor and age trajectory

- No direct functional anchor: it tracks calendar age. Ticks fast in childhood, then linearly in adults; the model uses a log transform of age below 20 (Horvath 2013).

## Confounders

Blood cell-type mix, array type and batch, sample handling. List not exhaustive.

## Sources

- Horvath S. DNA methylation age of human tissues and cell types. Genome Biology 2013. doi:10.1186/gb-2013-14-10-r115 (Crossref OK)
- Horvath S. Erratum to: DNA methylation age of human tissues and cell types. Genome Biology 2015. doi:10.1186/s13059-015-0649-6 (Crossref OK)
- Higgins-Chen AT et al. A computational solution for bolstering reliability of epigenetic clocks. Nature Aging 2022. doi:10.1038/s43587-022-00248-2 (Crossref OK)
