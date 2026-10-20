# Mouse treadmill endurance (run to exhaustion)

Axis: A1 muscle (axes.md @116f1f8); also reflects heart, lungs and mitochondria (hallmark H7). Human counterpart: `human-tests/` VO2max file.

## What it measures

The mouse runs on a motorised treadmill whose speed rises in steps (for example +1 m/min every minute, or +2 m/min every 2 minutes) until it is "exhausted". The reading is time to exhaustion, distance run, or the top speed reached. It mixes aerobic capacity, muscle endurance and willingness to run.

"Exhausted" has no standard definition. Published criteria include:
- 10 seconds in a row on the electric grid at the back of the belt; more than 50% of time on the grid; or no response to gentle prodding (Gibb 2016);
- 5 seconds in a row in a "fatigue zone" at the rear of the belt, so few or no shocks are needed after training (Dougherty 2016).

## Training or reference population

No standard reference. Age data are almost all from male C57BL/6 (B6); one longitudinal study covers both sexes (Pajski 2024). Protocols (speed steps, incline, criterion) differ by lab, so absolute times cannot be compared across labs. No UM-HET3 reference curve found `[to verify]`.

## Noise

- No published same-mouse test-retest coefficient (ICC or CV) found for this test. `[to verify]`
- Smallest real change for one mouse: not published. `[to verify]`
- Practical controls: fix the exhaustion criterion before the study; train all mice the same way on the belt before scoring; one tester; same time of day.

## Known failure modes

- **Motivation, not capacity.** A mouse that stops is not always exhausted. Strains differ: C57BL/6J took more shocks than FVB/NJ, and the authors checked blood lactate before and after the test to confirm a real effort (Gibb 2016). Rule: confirm effort in a subset (lactate) and record the number of shocks.
- **Criterion choice** changes the reading (above); never pool studies with different criteria.
- **Training effect**: running on the belt is itself exercise, so repeated tests can improve the score. Keep the number of tests per mouse fixed and small.
- **Time of day**: running capacity differs with the time of day; a dedicated protocol exists to measure it (Adamovich 2021). Test all mice at the same clock time.
- Old mice may stop because of joint, heart or balance problems; the test does not say which.

## Functional anchor and age trajectory

| Source | Mice | Finding |
| --- | --- | --- |
| Graber 2021 (CFAB battery) | Male C57BL/6, 6 mo (n = 29), 24 mo (n = 24), 28+ mo (n = 28) | 28+-month mice ran 232.3 ± 25.1 s, 27% less than 6-month and 34% less than 24-month mice |
| Pajski 2024 (longitudinal, same mice) | C57BL/6 males and females, tested at 6, 12, 18, 24 and 28 months | Males: treadmill score 41.6% lower at 25 months and 38.5% lower at 29 months than at 6 months; the biggest drop was between 18 and 25 months (−40.0%). Females: changes of −10.8%, +1.9%, −21.9%, −9.9% over the successive intervals |
| Kerr 2024 | Male C57BL/6J, young (4–9 mo) vs old (23–32 mo) | Running time 20% lower at 23–24 months (grip 21% lower, muscle mass 15% lower); 51% lower after 30 months (grip 34%, muscle mass 30%) |
| Baumann 2018 | C57BL/6, followed to 32 months | Time to fatigue fell 13–46% from 23 to 32 months; endurance dropped before grip did (grip fell 9–17% from 29 to 32 months). Frail/pre-frail mice had 17% less endurance than non-frail at 23 months |
| Baumann 2019 | Male and female C57BL/6 | Used as one of the frailty-phenotype criteria: "poor endurance" = lowest 20%, cut-offs 944.2 s and 915.6 s (which sex is which `[to verify]`) |

Take-home: endurance falls by roughly 20% by 2 years and 40–50% by 2.5 years in B6 males, and it falls earlier and further than grip or muscle mass (Kerr 2024; Baumann 2018). That makes it the most age-sensitive of the A1 readouts, at the cost of a motivation confound.

## Confounders

Sex, strain, body weight, training on the belt, exhaustion criterion, shocks and motivation, time of day, incline, joint or heart disease in old mice.

## Sources

- Graber TG et al. Measuring exercise capacity and physical function in adult and older mice. J Gerontol A 2021 (online 2020). doi:10.1093/gerona/glaa205 (Crossref OK)
- Pajski ML et al. Longitudinal decline of exercise capacity in male and female mice. J Gerontol A 2024/2025. doi:10.1093/gerona/glae293 (Crossref OK)
- Kerr HL et al. Mouse sarcopenia model reveals sex- and age-specific differences in phenotypic and molecular characteristics. J Clin Invest 2024. doi:10.1172/jci172890 (Crossref OK)
- Baumann CW et al. Assessing onset, prevalence and survival in mice using a frailty phenotype. Aging 2018. doi:10.18632/aging.101692 (Crossref OK)
- Baumann CW et al. Sex-specific components of frailty in C57BL/6 mice. Aging 2019. doi:10.18632/aging.102114 (Crossref OK)
- Gibb AA et al. FVB/NJ mice are a useful model for examining cardiac adaptations to treadmill exercise. Front Physiol 2016. doi:10.3389/fphys.2016.00636 (Crossref OK)
- Dougherty JP et al. The treadmill fatigue test: a simple, high-throughput assay of fatigue-like behavior for the mouse. J Vis Exp 2016. doi:10.3791/54052 (Crossref OK)
- Adamovich Y et al. Monitoring daytime differences in moderate intensity exercise capacity using treadmill test and muscle dissection. STAR Protoc 2021. doi:10.1016/j.xpro.2021.100331 (Crossref OK)
