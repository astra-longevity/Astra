# Mouse grip strength

Axis: A1 muscle (axes.md @116f1f8). Human counterpart: `human-tests/` grip file.

## What it measures

The mouse grips a bar or grid fixed to a force meter and is pulled gently backwards by the tail; the meter records peak pull force in grams. Variants: forelimbs only or all four limbs, bar or grid. Other grip methods with no meter: weight-lift test (Deacon: the mouse lifts sponge-covered weights of rising mass) and cage-lift test (Bigossi 2025).

## Training or reference population

No single reference population. Published age data are almost all from C57BL/6 (B6) males; one study adds CB6F1 males (Ge 2016). No UM-HET3 reference curve found `[to verify]`.

## Noise

| Study | Setting | Result |
| --- | --- | --- |
| Bigossi 2025 | 160 aged C57BL/6J, 5 methods (Deacon weight lift, modified wire-weight lift, cage lift, meter with bar, meter with grid); 782 longitudinal readings | Mean coefficient of variation 7–17% depending on method ("good repeatability") |
| Takeshita 2017 | Young and 2-year-old C57BL/6, conventional vs modified meter test | Modified test had lower trial-to-trial and day-to-day variability; the conventional test was noisier in young males than in young females and in old males. Numeric values `[to verify]` |

- Smallest real change for one mouse: not published. `[to verify]`
- Practical rule from both studies: average several trials and several days. Bigossi 2025 proposes composite scores of 2 or 3 methods (SS2, SS3) to cut noise.

## Known failure modes

- **Normalising to body weight is biased in old mice.** In geriatric mice, weight changes with tumours, fluid in the abdomen (ascites), enlarged liver and spleen, and muscle wasting, none of which is grip (Bigossi 2025). In one 2-year study grip correlated with body mass in young mice (r = 0.62) but not at 18 months (r = −0.01; the authors' unpublished observation) (Tavoian 2021). Use **absolute force** as the primary reading; report normalised values only beside it.
- Normalisation can also hide or create an age effect: in 10 males followed from 10 to 25 months, absolute forelimb grip fell between 15 and 24 months while weight-normalised and all-limb grip did not change (Sheth 2018).
- Tester handling and pulling technique (Takeshita 2017).
- Motivation and learning: not quantified in these sources `[to verify]`.

## Functional anchor and age trajectory

The test is itself a functional anchor (strength). Published trajectories:

| Source | Mice | Finding |
| --- | --- | --- |
| Tavoian 2021 | Followed over 2 years; strain `[to verify]` | Forelimb grip force (g), males: 192.3 at 4 mo, 172.2 at 7, 174.0 at 12, 159.0 at 15, 163.2 at 18, 150.6 at 21, 148.7 at 24. Females: 177.4, 153.6, 169.0, 150.4, 144.6, 148.7, 153.1 at the same ages. Peak performance age differs by task. |
| Ge 2016 | B6 NIA and CB6F1 NIA males, 4/12/20/28 months (abstract also mentions 8 months `[to verify]`) | Weight-normalised forelimb grip highest at 4 months, then lower in each older group; 4 and 12 months differ from 20 and 28; no strain difference |
| Graber 2013 | C57BL/6, adult / old / elderly groups (ages `[to verify]`) | Grip 39% lower old vs adult, 61% lower elderly vs adult; old vs elderly not different |
| Sheth 2018 | Males; cross-section very old vs young, plus 10 mice followed 10–25 mo | Normalised forelimb grip 3.3 g/g (95% CI 2.9–4.2) very old vs 5.8 (5.3–6.3) young |
| Owendoff 2023 | Young vs aged, both sexes (ages `[to verify]`) | Bilateral forelimb grip: males 38.6 ± 18.8 aged vs 37.5 ± 5.9 g young (no difference); females 13% lower aged. Normalised all-limb grip 18% lower in aged males, 33% lower in aged females |
| Kerr 2024 | Old vs young, both sexes | Normalised grip lower in all old male groups; 36% lower in old females, gone after normalising to muscle mass |

Take-home: grip falls with age in B6 mice, but the size of the fall depends heavily on method, sex and normalisation, and the spread among old mice is large (SD 18.8 g on a mean of 38.6 g in aged males, Owendoff 2023). Grip loss shows earlier and more clearly than loss of lean mass (Owendoff 2023), but in mice aged 23–32 months treadmill running time fell more than grip (cited in Kerr 2024).

## Confounders

Sex, strain, body weight and its non-muscle components (tumours, ascites, organ enlargement), method (bar vs grid, forelimb vs all-limb), tester, number of trials and days, time of day `[to verify]`.

## Sources

- Bigossi G et al. A comparative analysis of grip strength evaluation methods in a large cohort of aged mice. J Cachexia Sarcopenia Muscle 2025. doi:10.1002/jcsm.70050 (Crossref OK)
- Takeshita H et al. Modified forelimb grip strength test detects aging-associated physiological decline in skeletal muscle function in male mice. Sci Rep 2017. doi:10.1038/srep42323 (Crossref OK)
- Owendoff G et al. Optimization and construct validity of approaches to preclinical grip strength testing. J Cachexia Sarcopenia Muscle 2023. doi:10.1002/jcsm.13300 (Crossref OK)
- Ge X et al. Grip strength is potentially an early indicator of age-related decline in mice. Pathobiol Aging Age Relat Dis 2016. doi:10.3402/pba.v6.32981 (Crossref OK)
- Tavoian D et al. Age of peak performance differs by functional task in mice tracked over 2 years. J Gerontol A 2021. doi:10.1093/gerona/glab048 (Crossref OK)
- Graber TG et al. C57BL/6 neuromuscular healthspan scoring system. J Gerontol A 2013. doi:10.1093/gerona/glt032 (Crossref OK)
- Sheth KA et al. Muscle strength and size are associated with motor unit connectivity in aged mice. Neurobiol Aging 2018. doi:10.1016/j.neurobiolaging.2018.03.016 (Crossref OK)
- Kerr HL et al. Mouse sarcopenia model reveals sex- and age-specific differences in phenotypic and molecular characteristics. J Clin Invest 2024. doi:10.1172/jci172890 (Crossref OK)
