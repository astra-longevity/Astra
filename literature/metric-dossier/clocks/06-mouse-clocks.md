# Mouse methylation clocks (Petkovich, Stubbs, Meer, Thompson)

## What they measure

Each estimates a mouse's age from DNA methylation at a fixed set of genome sites, read by sequencing (RRBS: reduced-representation bisulfite sequencing). All four were trained to predict **calendar age**. Lifespan interventions (calorie restriction, dwarf mutants) are then checked for a lower reading than calendar age.

## Training population, per clock

| Clock | Tissue | Sites (CpGs) | Training ages | Strain(s) | Reported fit | Source of figures |
| --- | --- | --- | --- | --- | --- | --- |
| Petkovich 2017 | Blood | 90 | 3–35 months | C57BL/6 (also B6D2F1, GHRKO, Snell dwarf samples in the dataset) | R² > 0.90 | Meer 2018 Table 1; Thompson 2018 Table 2 |
| Stubbs 2017 | Multi-tissue (cortex, heart, liver, lung; also muscle per a review) | 329 | 0.2–9.5 months (oldest 41 weeks) | C57BL/6 | r = 0.839, median error 3.33 weeks (test data) | Stubbs 2017 abstract and results |
| Meer 2018 | Multi-tissue | 435 | 0.2–35 months | C57BL/6 wild type, untreated | R² = 0.89 | Meer 2018 Table 1 |
| Thompson 2018 | Multi-tissue | 582 (elastic net, all sites); 274 (conserved sites) | `[to verify]` | C57BL/6 multi-tissue data, as described by Simpson 2023 | Leave-one-batch-out: r = 0.82, median error 2.5 months; conserved-site clock r = 0.68, 3.8 months | Thompson 2018 Table 1; 1,189 samples in the matrix, 893 used for training |

Dataset composition as reported by Thompson 2018 and Simpson 2023 (secondary counts, not re-read in the Petkovich/Stubbs primaries):

- Petkovich: Thompson 2018 lists 231 blood samples (C57BL/6 161, B6D2F1 22, GHRKO 26, Snell 22); Simpson 2023 gives 129 training + 12 cross-validation C57BL/6 samples for the blood clock itself.
- Stubbs: Thompson 2018 lists 61 samples (cortex 16, heart 15, liver 15, lung 15), ages 0.2–9.4 months.

**Consequence for Astra.** All four clocks were built mainly on C57BL/6 mice. The ITP uses genetically mixed UM-HET3 mice: any reading on UM-HET3 (or another strain) is "outside training range" until a validation on that strain is cited. Stubbs is a **young-mouse** clock: readings past ~10 months are extrapolation.

## Noise

- Within the training data the fit is good (table above).
- Applied to an outside dataset, they degrade sharply: on Thompson 2018's C57BL/6 multi-tissue RRBS data, the Petkovich, Stubbs and Meer clocks had median absolute errors of 2.9–9.6 months and R² from 0.86 down to 0.01 (Simpson 2023). Cause named: uneven RRBS read coverage across labs; averaging methylation over regions instead of single sites improved transfer.
- Test-retest on the same mouse (replicate samples): not reported in any of the sources above. Named gap: no smallest-real-change figure can be given.

## Known failure modes

- Cross-lab transfer (coverage, protocol) as above: a clock must be re-validated on our own lab's data before use.
- Strain and tissue outside training (see above).
- Young-only training (Stubbs).

## Functional anchor and age trajectory

None of the four is anchored to function; they track calendar age. Their claimed biological signal is slower ticking under calorie restriction and in long-lived dwarf mice (Petkovich 2017; Meer 2018). Trade-off: in Thompson 2018 every clock type detected slower ageing under calorie restriction, but only the ridge-regression clocks (all ~194,000 sites) replicated slower ageing in dwarf mice; the most accurate age predictor was not the most sensitive to interventions.

## Confounders

Tissue and cell-type mix, sequencing coverage and lab protocol, strain, sex. Sex effects are not reported in the sources above (named gap).

## Sources

- Petkovich DA et al. Using DNA methylation profiling to evaluate biological age and longevity interventions. Cell Metabolism 2017. doi:10.1016/j.cmet.2017.03.016 (Crossref OK)
- Stubbs TM et al. Multi-tissue DNA methylation age predictor in mouse. Genome Biology 2017. doi:10.1186/s13059-017-1203-5 (Crossref OK)
- Meer MV et al. A whole lifespan mouse multi-tissue DNA methylation clock. eLife 2018. doi:10.7554/eLife.40675 (Crossref OK)
- Thompson MJ et al. A multi-tissue full lifespan epigenetic clock for mice. Aging 2018. doi:10.18632/aging.101590 (Crossref OK)
- Simpson DJ et al. Region-based epigenetic clock design improves RRBS-based age prediction. Aging Cell 2023. doi:10.1111/acel.13866 (Crossref OK)
