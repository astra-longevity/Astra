# Mouse lung function (forced oscillation, plethysmography)

Axis: A8 lung (axes.md @116f1f8). Related: A15 body size (a bigger mouse has bigger lungs).

## What it measures

- **Forced oscillation (flexiVent):** the mouse is anaesthetised, a tube is put into its windpipe, and a ventilator pushes small, known air waves into the lungs. Readings: resistance (how hard air flows through the airways), compliance (how easily the lungs stretch), elastance (stiffness, the opposite of compliance), tissue damping, inspiratory capacity, and pressure-volume curves.
- **Whole-body plethysmography:** the mouse breathes freely in a sealed box and the box measures breathing rate and volume. It needs no anaesthesia, so it can be repeated on the same mouse.

Nearly all the ageing data read today come from forced oscillation.

## Training or reference population

No standard reference. Age data are from C57BL/6J and C57BL/6N mice, groups of a few mice per age (7–8 per age in Schulte 2019). No UM-HET3 data found.

## Noise

- Test-retest noise and smallest real change for one mouse: none found in the sources read.
- For all forced-oscillation readings, body weight and age were both important predictors of lung function (Isbatan 2025). Results must be adjusted for body weight.

## Known failure modes

- **"Higher" means worse.** With age, compliance rises and elastance falls: the lung gets floppier, not stiffer, with enlarged air spaces like mild emphysema (Elliott 2016; Huang 2007; Schulte 2019; Veldhuizen 2019). A score that treats higher compliance as healthier gets the direction wrong.
- **Most change is early adulthood, not old age.** The biggest changes happen between 2–3 and 6 months (Schulte 2019; Elliott 2016, cited by Schulte). In C57BL/6N mice, decline was already visible before 6 months (Kawaguchi 2023). Isbatan 2025 saw no further decline after 1 year. This axis may measure growth and maturation more than ageing in mid-life.
- **Late-life reversal and survivor bias.** At 30 months, resistance rose again and compliance fell compared with 24 months. Only about a quarter of B6 mice survive to 30 months, so the oldest group is a selected group (Elliott 2016). As with GTT and fat (files 06, 07), the curve is not monotonic.
- **Airways and lung tissue age on different clocks.** Most lung-tissue changes happened between 2 and 20 months, while the fall in airway resistance came mostly between 20 and 26 months (Huang 2007).
- **Terminal test.** Forced oscillation needs a tube in the windpipe and is usually done at the end of life, so it cannot follow one mouse over time. Plethysmography can be repeated, but no plethysmography ageing study was read today: a gap for a longitudinal metric.

## Functional anchor and age trajectory

| Source | Mice | Finding |
| --- | --- | --- |
| Huang 2007 | C57BL/6J, 2, 20, 26 months | Lung volume at 30 cmH2O and compliance rose, elastic fibres fell from 2 to 20 months; airway resistance fell mostly from 20 to 26 months, with larger air spaces and more collagen late |
| Elliott 2016 | C57BL/6, 2 to 30 months | Resistance fell and compliance rose from 2 to 24 months (non-linear); reversed at 30 months; air-space size rose with age |
| Schulte 2019 | C57BL/6JRj, 3, 6, 12, 18, 24 months | Compliance and inspiratory capacity up, tissue elastance and resistance down; greatest change from 3 to 6 months; structural changes mostly between middle and old age |
| Veldhuizen 2019 | Young vs aged mice | Higher compliance and lower airway resistance with age; exercise did not change this |
| Kawaguchi 2023 | C57BL/6N (NCGG Aging Farm), both sexes, up to 1 year+ | Decline visible before 6 months, continuing to at least 1 year; compliance higher in older mice of both sexes |
| Isbatan 2025 | C57BL/6, both sexes, 2 to 52 weeks | Resistance and elastance fell, compliance and inspiratory capacity rose with age; no further decline after 1 year |

Take-home: the lung gets floppier with age, mostly in young adulthood. The test is terminal and the curve reverses in the oldest survivors. Useful as an end-of-life reading with body-weight adjustment, weak as a longitudinal one.

## Confounders

Body weight and size, sex, substrain (6J vs 6N), anaesthesia depth, chest wall stiffness, survivor selection in the oldest groups, test protocol (ventilator settings, lung volume history before the reading).

## Sources

- Huang et al. Age-dependent changes of airway and lung parenchyma in C57BL/6J mice. J Appl Physiol 2007. doi:10.1152/japplphysiol.00400.2006 (Crossref OK)
- Elliott et al. Aging-related changes in respiratory system mechanics and morphometry in mice. Am J Physiol Lung Cell Mol Physiol 2016. doi:10.1152/ajplung.00232.2016 (Crossref OK)
- Schulte et al. Age-related structural and functional changes in the mouse lung. Front Physiol 2019. doi:10.3389/fphys.2019.01466 (Crossref OK)
- Veldhuizen et al. The effects of aging and exercise on lung mechanics, surfactant and alveolar macrophages. Exp Lung Res 2019. doi:10.1080/01902148.2019.1605633 (Crossref OK)
- Kawaguchi et al. Age-related changes in lung function in National Center for Geriatrics and Gerontology Aging Farm C57BL/6N mice. Exp Anim 2023. doi:10.1538/expanim.22-0109 (Crossref OK)
- Isbatan et al. Age-dependent correlation of pulmonary function impairment and aortic arch stiffness in mice. Pediatr Discov 2025. doi:10.1002/pdi3.2525 (Crossref OK)
