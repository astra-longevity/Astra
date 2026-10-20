# Hannum 2013 blood clock

## What it measures

An estimate of age from methylation at 71 sites, in **whole blood only**. Trained to predict calendar age.

## Training population

- Whole blood of 656 people aged 19 to 101, Illumina 450K (Hannum 2013; consistent across reviews).
- Training/validation split: secondary sources give 482 training + 174 validation. `[to verify]`
- Ancestry: secondary reviews describe the cohort as US-based; exact ancestry mix `[to verify]`.
- Consequence: valid for adult blood only. Any non-blood sample or child is outside training range.

## Noise

- Reported fit: correlation 0.91–0.96, error about 3.9–4.9 years depending on the review quoted. `[to verify]` exact figures in primary.
- Replicate spread: several years between technical replicates for the original version; reduced by principal-component version (Higgins-Chen 2022). `[to verify]` value.

## Known failure modes

- Blood-only: skewed estimates in other tissues and in children.
- Depends strongly on blood cell-type mix.

## Functional anchor and age trajectory

None directly; tracks calendar age in adults.

## Confounders

Blood cell-type mix, sex, array batch. `[to verify]` list from primary.

## Sources

- Hannum G et al. Genome-wide methylation profiles reveal quantitative views of human aging rates. Molecular Cell 2013. doi:10.1016/j.molcel.2012.10.016 (Crossref OK)
- Higgins-Chen 2022, doi:10.1038/s43587-022-00248-2 (Crossref OK)
