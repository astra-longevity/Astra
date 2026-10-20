# Hannum 2013 blood clock

## What it measures

An estimate of age from methylation at 71 sites, in **whole blood only**. Trained to predict calendar age.

## Training population

- Whole blood of 656 people aged 19 to 101, Illumina 450K (Hannum 2013; consistent across reviews).
- Two cohorts: 482 people for training, 174 for validation, processed the same way (Hannum 2013).
- Ancestry: 426 Caucasian and 230 Hispanic people (Hannum 2013). Other ancestries are outside training range.
- Consequence: valid for adult blood only. Any non-blood sample or child is outside training range.

## Noise

- Reported fit: training cohort r = 0.96, error 3.9 years; validation cohort r = 0.91, error 4.9 years (Hannum 2013). The validation figures are the honest ones.
- Replicate spread: the original version disagrees between technical replicates more than its principal-component version (Higgins-Chen 2022). No value or smallest-real-change figure is quoted here.

## Known failure modes

- Blood-only: skewed estimates in other tissues and in children.
- Depends strongly on blood cell-type mix.

## Functional anchor and age trajectory

None directly; tracks calendar age in adults.

## Confounders

- Sex: men's methylome aged about 4% faster than women's, confirmed in the validation cohort; BMI had no significant effect (Hannum 2013).
- Ancestry and diabetes could not be tested because they were tangled with array batch (Hannum 2013): batch is a confounder.
- Blood cell-type mix.

## Sources

- Hannum G et al. Genome-wide methylation profiles reveal quantitative views of human aging rates. Molecular Cell 2013. doi:10.1016/j.molcel.2012.10.016 (Crossref OK)
- Higgins-Chen 2022, doi:10.1038/s43587-022-00248-2 (Crossref OK)
