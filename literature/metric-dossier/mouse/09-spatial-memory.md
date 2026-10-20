# Mouse spatial memory (Barnes maze, Morris water maze)

Axis: A3 brain (axes.md @116f1f8). Related: A1 muscle and A5 vision (both can fake a memory deficit).

## What it measures

Over several days of training, the mouse learns where a hidden escape is, using cues around the room. Then a "probe" trial removes the escape and measures how much the mouse searches the right spot.
- **Morris water maze (MWM):** the mouse swims in a pool to find a hidden platform.
- **Barnes maze:** the mouse runs on a dry, brightly lit round table to find one escape hole among many.

Readings: time to find the escape (latency), distance travelled, errors, search strategy (spatial vs checking holes one by one), and time in the target zone during the probe. A "reversal" phase moves the escape to test flexible relearning.

## Training or reference population

No standard reference. Age data are mostly from male C57BL/6 (B6). No UM-HET3 reference curve found `[to verify]`.

## Noise

- Scores were reliable across repeated test sessions (de Fiebre 2006); no reliability coefficient was in the parts read `[to verify]`.
- Spread between mice grows with age: modest but significant age-related increases in the variance of several MWM learning measures (de Fiebre 2006).
- Smallest real change for one mouse: not published.

## Known failure modes

- **Several skills, not one score.** In the MWM, initial learning, reversal learning and the probe trial were largely independent of each other. Learning deficits appeared earlier in life than reversal deficits (de Fiebre 2006). The metric must say which reading it uses.
- **Swimming speed and vision.** Differences in swim speed and eyesight can bias latency; a visible-platform trial controls for vision (Kennard 2011). Old mice swam more slowly (de Fiebre 2006; Yanai 2021), so **distance is safer than latency**. In de Fiebre 2006, the motor deficits of the oldest mice did not correlate with path-length or probe measures.
- **Strain.** At 25 months, B6 mice were impaired in MWM probe trials, while an F1 hybrid strain showed no age effect (Fordyce 1993). Do not assume B6 results hold for UM-HET3.
- **Repeat testing.** Mice remember the maze, so retesting the same mouse is not a fresh measure. Longitudinal use needs a new escape position or new cues `[to verify]` (no source read today).

## Functional anchor and age trajectory

| Source | Mice | Finding |
| --- | --- | --- |
| Hendrickx 2022 | C57BL/6J, several ages | Signs of age-related memory loss in the MWM and in novel object recognition from 6 months onward |
| Kennard 2011 (review) | C57BL/6 | Barnes maze deficits reported as early as 12 months (citing Bach 1999); 12- and 18-month mice made more errors than 3- and 6-month mice and searched hole by hole more than spatially |
| Yanai 2021 | Male C57BL/6J, five age cohorts | Barnes: 22-month mice took longer and travelled farther than 3-month mice; deficit may start around 12 months. MWM: latency rose with age and swim speed fell, but swim distance did not differ between cohorts. Memory of the precise platform location declined more steeply than less demanding memory. Forgetting over 10–30 days was about equal at all ages |
| Fordyce 1993 | B6 vs F1 hybrid, 3 vs 25 months | B6 at 25 months impaired in the probe trial; F1 hybrid not affected by age |

Take-home: spatial memory declines in B6 from middle age (6–12 months, depending on the test). Of the readings, the precise-location probe is the most age-sensitive. Latency is skewed by slower swimming, while swim distance is not (Yanai 2021). The Barnes maze avoids swimming, but has less mouse ageing data (Kennard 2011).

## Confounders

Swimming or running speed, eyesight, anxiety and stress (cold water, bright light on the Barnes table), strain, sex, room cues, maze size and design, prior test experience.

## Sources

- de Fiebre et al. Spatial learning and psychomotor performance of C57BL/6 mice: age sensitivity and reliability of individual differences. AGE 2006. doi:10.1007/s11357-006-9027-3 (Crossref OK)
- Kennard et al. Age sensitivity of behavioral tests and brain substrates of normal aging in mice. Front Aging Neurosci 2011. doi:10.3389/fnagi.2011.00009 (Crossref OK)
- Yanai et al. Functional aging in male C57BL/6J mice across the life-span: a systematic behavioral analysis of motor, emotional, and memory function to define an aging phenotype. Front Aging Neurosci 2021. doi:10.3389/fnagi.2021.697621 (Crossref OK)
- Hendrickx et al. Age-related cognitive decline in spatial learning and memory of C57BL/6J mice. Behav Brain Res 2022. doi:10.1016/j.bbr.2021.113649 (Crossref OK)
- Fordyce et al. Effects of aging on spatial learning and hippocampal protein kinase C in mice. Neurobiol Aging 1993. doi:10.1016/0197-4580(93)90116-s (Crossref OK)
