# Mouse rotarod (accelerating)

Axis: A1 muscle and coordination (axes.md @116f1f8). Also touches A3 brain (motor learning) and balance.

## What it measures

The mouse walks on a rotating rod whose speed rises steadily (e.g. 5 to 40 rpm over 5 minutes, Noda 2020). The reading is the time until it falls ("latency to fall"), sometimes the rod speed at the fall. It mixes balance, coordination, strength, endurance and motor learning; the test cannot separate these (Kennard 2011).

## Training or reference population

No standard reference. Age data are almost all from C57BL/6 (B6). Protocols differ by lab (rod speed, number of trials, training days, average vs best trial), so values are not comparable across labs; accelerating and fixed-speed protocols must never be pooled. No UM-HET3 reference curve in the sources below (named gap).

## Noise

- No same-mouse test-retest coefficient (ICC or CV) for aged mice in the sources below (named gap).
- Mice improve over repeated trials (learning). Both young and old mice got better from trial to trial, with age differences in learning as well as in baseline (Barreto 2010). So a repeat reading is not a pure repeat: the protocol must fix the number of training sessions before the scored trials.
- Example protocol designed to remove learning from the score: 6 trials a day for 5 training days over two weeks, then a 6th test day scored as the best of 6 trials (Fischer 2016).
- Smallest real change for one mouse: not in the sources below (named gap).

## Known failure modes

- **Body weight.** Heavier mice fall sooner, regardless of coordination:
  - C57BL/6Nia males: latency vs body mass R = −0.443 (p = 0.0006); negative in females too (Fischer 2016).
  - Collaborative Cross (genetically diverse) mice: fall speed vs body weight r = −0.56 in females, −0.53 in males (Mao 2015).
  - Shoji 2016 found a significant age × body weight interaction and compared ages only within a 27.5–32.5 g weight band.
  - Not universal: no weight correlation in adolescent/young C57BL/6N mice (Eltokhi 2021); the effect seems to depend on age.
  - Rule: record body weight on the test day; use it as a covariate; never compare groups that differ in weight without it.
- Learning and practice effects (above).
- "Passive rotation": the mouse clings to the rod and rides round, which lengthens latency without real coordination. Define in advance whether this ends the trial. (Protocol rule; no peer-reviewed source cited here.)
- Mixed construct: a drop can come from muscle, balance organ, brain or motivation (Kennard 2011). Vestibular (balance-organ) loss in aged mice shows on the rotarod (Tung 2016).

## Functional anchor and age trajectory

| Source | Mice | Finding |
| --- | --- | --- |
| Graber 2013 | C57BL/6, adult / old / elderly groups | Mean time on rod fell 40% across age groups; with body mass as covariate, adult differed from old and from elderly |
| Noda 2020 | C57BL/6, 10–120 weeks, n = 70 | Latency to fall lower in aged mice, clearly at 120 weeks; body weight did not change between 60, 80 and 120 weeks |
| Shoji 2016 | C57BL/6J, young adult to middle age, large cohort (F test df 3,1205) | Significant age effect on mean latency over 6 trials, after body-weight adjustment |
| Shoji 2019 | Male C57BL/6J from a genetic-stability programme, young to old (to 23 months) | Rotarod performance lower in middle and old age; ANCOVA with body weight |
| Tung 2016 | 1 to 27–28 months | Rotarod performance fell with age (Kruskal-Wallis χ² = 11.4, p = 0.01) |
| Justice 2014 | 20-month mice | Coordination (rotarod + hindlimb grasp) 35% lower; the authors judge this translates less well to human balance decline (6–11% at 50–69 years) |

Take-home: latency falls with age in B6 mice, roughly a 35–40% drop by old age, but it is entangled with body weight and learning. As an A1 readout it is weaker than grip and treadmill; better as a coordination/balance readout beside them.

## Confounders

Body weight, sex, strain, training schedule and number of trials, scoring rule (mean vs best trial), rod diameter and acceleration, passive rotation, tester.

## Sources

- Kennard JA, Woodruff-Pak DS. Age sensitivity of behavioral tests and brain substrates of normal aging in mice. Front Aging Neurosci 2011. doi:10.3389/fnagi.2011.00009 (Crossref OK)
- Noda S et al. Aging-related motor function and dopaminergic neuronal loss in C57BL/6 mice. Mol Brain 2020. doi:10.1186/s13041-020-00585-6 (Crossref OK)
- Barreto G et al. Age-related defects in sensorimotor activity, spatial learning, and memory in C57BL/6 mice. J Neurosurg Anesthesiol 2010. doi:10.1097/ana.0b013e3181d56c98 (Crossref OK)
- Fischer KE et al. A cross-sectional study of male and female C57BL/6Nia mice suggests lifespan and healthspan are not necessarily correlated. Aging 2016. doi:10.18632/aging.101059 (Crossref OK)
- Mao JH et al. Identification of genetic factors that modify motor performance and body weight using Collaborative Cross mice. Sci Rep 2015. doi:10.1038/srep16247 (Crossref OK)
- Shoji H et al. Age-related changes in behavior in C57BL/6J mice from young adulthood to middle age. Mol Brain 2016. doi:10.1186/s13041-016-0191-9 (Crossref OK)
- Shoji H, Miyakawa T. Age-related behavioral changes from young to old age in male mice of a C57BL/6J strain maintained under a genetic stability program. Neuropsychopharmacol Rep 2019. doi:10.1002/npr2.12052 (Crossref OK)
- Tung VWK et al. Motor performance is impaired following vestibular stimulation in ageing mice. Front Aging Neurosci 2016. doi:10.3389/fnagi.2016.00012 (Crossref OK)
- Graber TG et al. C57BL/6 neuromuscular healthspan scoring system. J Gerontol A 2013. doi:10.1093/gerona/glt032 (Crossref OK)
- Justice JN et al. Battery of behavioral tests in mice that models age-associated changes in human motor function. AGE 2014 (online 2013). doi:10.1007/s11357-013-9589-9 (Crossref OK)
- Eltokhi A et al. Comprehensive characterization of motor and coordination functions in three adolescent wild-type mouse strains. Sci Rep 2021. doi:10.1038/s41598-021-85858-3 (Crossref OK)
