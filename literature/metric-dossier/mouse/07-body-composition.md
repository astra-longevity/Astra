# Mouse body composition (fat and lean mass)

Axis: A15 fat (axes.md @116f1f8). Related: A1 muscle (lean mass), A10 metabolism (fat drives the glucose test, see `06-glucose-tolerance.md`).

## What it measures

Grams of fat and lean tissue in a live mouse. Two methods:
- **EchoMRI** (quantitative magnetic resonance, QMR): measures fat, lean mass and water. The mouse can be awake; no image and no bone.
- **DXA** (dual-energy X-ray absorptiometry, also written DEXA): X-ray scan of an anaesthetised mouse. Measures fat, lean mass and bone mineral.

"Lean mass" is everything that is not fat or bone: muscle, organs and water. It is not muscle mass.

## Training or reference population

No standard reference. Age data are mostly from male C57BL/6 (B6). No UM-HET3 reference curve found `[to verify]`.

## Noise

| Source | Method | Repeat-scan CV |
| --- | --- | --- |
| Tinsley 2004 | EchoMRI, live mice | Fat 0.8–1.6%, lean 2.0–3.6% |
| Tinsley 2004 | EchoMRI vs DXA, dead mice | Fat 0.34–0.71% (EchoMRI) vs 3.06–12.60% (DXA) |
| Nagy 2000 | DXA, live mice | Fat 2.20%, lean 0.86% |
| Brommage 2003 | DXA, live mice | Fat content 1.4%, lean 0.4% |
| Halldorsdottir 2009 | DXA vs NMR, mice | Fat 2.3% (DXA) and 2.8% (NMR); lean 1.0% (DXA) and 2.2% (NMR) |

Both methods are precise next to the age change, where fat changes several-fold (below). Smallest real change for one mouse: not published.

## Known failure modes

- **Bias versus chemical analysis.** DXA overestimated lean mass by 1.7 ± 1.3 g (~8%) against carcass chemistry (Halldorsdottir 2009). Do not mix machines or methods in one dataset without cross-calibration.
- **Anaesthesia for DXA.** DXA needs the mouse anaesthetised; EchoMRI works on free-moving mice (Tinsley 2004), so EchoMRI is better for frequent repeat measures.
- **Lean mass is not muscle.** Lean mass includes organs and water, so it can stay flat or rise while muscle shrinks. Read it with grip strength (`02-grip-strength.md`).
- **Fat does not change in one direction with age.** It rises to middle age, then falls in late life (below). One fat value cannot tell a young lean mouse from an old wasting one.

## Functional anchor and age trajectory

| Source | Mice | Finding |
| --- | --- | --- |
| Reynolds 2019 | B6J, young vs aged | Males: fat 3.27 → 18.67 g, lean 22.64 → 18.84 g (lean loss). Females: no significant change in % body fat, lean rose (14.67 → 16.86 g). From 20 to 28 months (n = 7): fat 12.5 → 4.8 g, lean 20.1 → 23.4 g |
| Reynolds 2024 | Male B6J, young / old / very old | Fat 6.03 / 23.8 / 11.0 g; body fat 19.1 / 49.2 / 26.3% |
| Börsch 2021 | Aged mice, 8–28 months | Body mass stable 8–18 months, then fell 15.2 ± 3.1% by 28 months; EchoMRI showed progressive fat loss; epididymal fat (fat around the testes) 81.6% lower at 28 months (normalised to body mass) |
| Petr 2021 | Male mice, cross-section over the lifespan | Whole-body lean percentage largely preserved with age |

Take-home: fat mass rises, then falls. In late life, losing fat and weight is a sign of decline, not of health. For the metric, read the **trajectory** (peak, then loss) and the lean/fat pattern, not a single value. Body composition also affects the glucose test (06), grip (02), rotarod (03) and treadmill (04), so measure it on the same day as those tests.

## Confounders

Sex (in Reynolds 2019, females kept lean mass and did not gain % fat), strain and substrain, diet, tumours and fluid build-up (counted as lean), machine and calibration, anaesthesia (DXA).

## Sources

- Tinsley et al. Evaluation of a quantitative magnetic resonance method for mouse whole body composition analysis. Obes Res 2004. doi:10.1038/oby.2004.20 (Crossref OK)
- Nagy et al. Precision and accuracy of dual-energy X-ray absorptiometry for determining in vivo body composition of mice. Obes Res 2000. doi:10.1038/oby.2000.47 (Crossref OK)
- Brommage. Validation and calibration of DEXA body composition in mice. Am J Physiol Endocrinol Metab 2003. doi:10.1152/ajpendo.00470.2002 (Crossref OK)
- Halldorsdottir et al. Reproducibility and accuracy of body composition assessments in mice by dual energy x-ray absorptiometry and time domain nuclear magnetic resonance. Int J Body Compos Res 2009. PMID 21909234 (PubMed OK; no DOI listed)
- Reynolds et al. The impact of age and sex on body composition and glucose sensitivity in C57BL/6J mice. Physiol Rep 2019. doi:10.14814/phy2.13995 (Crossref OK)
- Reynolds et al. The oldest of old male C57B/6J mice are protected from sarcopenic obesity. Int J Mol Sci 2024. doi:10.3390/ijms251910278 (Crossref OK)
- Börsch et al. Molecular and phenotypic analysis of rodent models reveals conserved and species-specific modulators of human sarcopenia. Commun Biol 2021. doi:10.1038/s42003-021-01723-z (Crossref OK)
- Petr et al. A cross-sectional study of functional and metabolic changes during aging through the lifespan in male mice. eLife 2021. doi:10.7554/eLife.62952 (Crossref OK)
