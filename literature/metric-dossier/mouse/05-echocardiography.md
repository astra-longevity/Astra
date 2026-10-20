# Mouse echocardiography (heart ultrasound)

Axis: A6 heart (axes.md @116f1f8). Human counterpart: none; echo is not on the human test list (T3-A2-1).

## What it measures

Ultrasound images of the beating heart, usually under light isoflurane anaesthesia. Two groups of readings:
- **Pumping (systolic) function:** ejection fraction (EF, % of blood pushed out per beat) and fractional shortening (FS).
- **Filling (diastolic) function:** Doppler flow across the mitral valve. E = early filling wave, A = filling from atrial contraction, E/A = their ratio; e′ = tissue speed of the valve ring; E/e′; IVRT = time the heart muscle takes to relax before filling.
- **Structure:** left-ventricle (LV) mass, wall thickness, left-atrium size.

In human ageing, EF is mostly preserved and filling worsens (Walter 2025; Dai 2009), so diastolic readings and LV mass are the main ageing readouts, not EF.

## Training or reference population

No standard reference. Age data are mostly from male C57BL/6 (B6), and the B6 substrain matters: in 24-month B6J mice from Janvier, EF fell (to 48.6%) with heart enlargement, while B6J from Charles River kept their EF (Walter 2025). No UM-HET3 reference curve in the sources here (named gap).

Example adult B6 values (review table, Salerno 2025): E/A 1.52 ± 0.40, e′ 43.2 ± 10.9 mm/s, E/e′ 15.2 ± 6.7.

## Noise

| Study | What | Result |
| --- | --- | --- |
| Yang 1999 | Conscious and anaesthetised mice; same mice on different days and different observers | Same observer, same mouse on different days: ICC 0.85–0.93 for wall thickness and LV size. Between observers: ICC 0.91 for LV size but 0.33 (septum) and 0.58 (rear wall) for wall thickness |
| Schnelle 2018 | Diastolic readings | Within-observer CV 1.5–4.8%, between-observer CV 3.2–7.9% (IVRT 5.1%, E/e′ 7.9%, left-atrium area 3.2%) |
| Marterstock 2026 | Diastolic readings, gold-standard Doppler | Between-observer CV 15.1% for IVRT and 2.6% for E/A |
| Reed 2011 | Diastolic study in senescence-accelerated mice | Between-observer 9.7%, within-observer 7.4% |
| Salerno 2025 (review) | EF and FS | Vary by ±8–12% between observers and up to 15% between machines (different probe frequencies or frame rates) |

- In plain English: size measurements repeat well; wall thickness and diastolic readings depend on who measures. Rule: **one trained sonographer, one machine, blinded, for all readings of a cohort.**
- Smallest real change for one mouse: not in the sources here (named gap).

## Known failure modes

- **Anaesthesia and heart rate.** Above ~450–550 beats/min the E and A waves merge and diastolic readings cannot be taken (Wingert 2026; Singuru 2026; Harbo 2026), so diastolic assessment needs anaesthesia to slow the heart (Wingert 2026). Recommended window: about 400–500 beats/min (Singuru 2026); in anaesthetised mice, rates should stay above 400 beats/min, and over 650 suggests stress (Lindsey 2018). Too deep anaesthesia changes filling itself. Rule: record heart rate with every reading; reject or flag readings outside a set window.
- **Weight loss in old age:** LV mass keeps rising while body weight falls after 18–21 months, so mass indexed to body weight inflates late (Ahmet 2026).
- **Substrain** (above) and **testosterone**: in 18-month males, castrated mice had lower E/A (1.3 ± 0.1 vs 1.6 ± 0.2); EF unchanged (Banga 2021).
- **Restrictive filling hides decline:** E/A falls with age but can bounce back above normal (E/A > 3) in severe disease; 3 of 16 old mice showed this (Fanjul 2020). E/A alone is therefore not monotonic; pair it with e′ or IVRT.

## Functional anchor and age trajectory

| Source | Mice | Finding |
| --- | --- | --- |
| Ahmet 2026 | 58 male C57BL/6, echo every 3 months from 6 months to death; median lifespan 24 months | EF fell from 51 ± 2% to 27 ± 11%; LV mass rose continuously; spread between mice in rates of change grew 2–3-fold after the frailty transition (weight loss after 18–21 months) |
| Dai 2009 (longevity cohort) | C57BL/6 cross-section, young adult to oldest | LV mass index about +75% in oldest vs young adults; FS about −10%; clear decline in diastolic function (tissue Ea/Aa); left atrium larger |
| Kang 2023 | Young, middle-aged, old | E/e′ 21.7 ± 1.5 young vs 31.7 ± 2.6 middle and 31.8 ± 4.7 old; E/A 1.49 → 1.42 → 1.39 (not significant) |
| Liao 2024 | C57BL/6J vs CB6F1J males, 4–28 months | No age change in B6J; in CB6F1J only 4 vs 28 months differed |
| Fanjul 2020 | Old wild-type mice | Modest IVRT rise; 3/16 had restrictive filling |
| Walter 2025 | B6J substrains, 5 vs 24 months | Fibrosis up in both; EF down only in Janvier mice |

Take-home: heart ageing in mice shows mainly as thicker, heavier ventricles and slower relaxation (E/e′, IVRT, e′), with EF falling late and mostly near the end of life. Results disagree between studies and substrains, so this axis needs its own reference cohort.

## Confounders

Anaesthesia depth and heart rate, body temperature, sonographer, machine and probe, substrain, sex and sex hormones, body-weight loss in late life.

## Sources

- Ahmet I et al. Emergence of heartbeat frailty in advanced age II: individual cardiovascular aging trajectories revealed by lifelong echocardiography in male mice. GeroScience 2026. doi:10.1007/s11357-026-02392-8 (Crossref OK)
- Lindsey ML et al. Guidelines for measuring cardiac physiology in mice. Am J Physiol Heart Circ Physiol 2018. doi:10.1152/ajpheart.00339.2017 (Crossref OK)
- Wingert et al. Tailored anesthesia protocols for echo-based HFpEF phenotyping in C57BL6/N mice. Am J Physiol Heart Circ Physiol 2026. doi:10.1152/ajpheart.00319.2026 (Crossref OK)
- Singuru et al. Doppler echocardiography for assessment of diastolic function in mice. J Vis Exp 2026. doi:10.3791/71212 (Crossref OK)
- Marterstock et al. Addressing key limitations of diastolic function assessment in mouse echocardiography by enabling robust retrospective analysis from a standard imaging view. Acta Physiol 2026. doi:10.1111/apha.70250 (Crossref OK)
- Harbo et al. Of mice and men: toward mouse-specific diastolic echocardiography (commentary; notes E/A cannot be read when E and A waves fuse at high heart rate). Acta Physiol 2026. doi:10.1111/apha.70269 (Crossref OK)
- Walter et al. Comparative phenotyping of C57BL/6J substrains reveals distinctive patterns of cardiac aging. GeroScience 2025. doi:10.1007/s11357-025-01543-7 (Crossref OK)
- Salerno et al. Echocardiographic assessment of cardiac function in mouse models of heart disease. Int J Mol Sci 2025. doi:10.3390/ijms26135995 (Crossref OK)
- Kang et al. Age-related shift in cardiac and metabolic phenotyping linked to inflammatory cytokines and antioxidant status in mice. Int J Mol Sci 2023. doi:10.3390/ijms242115841 (Crossref OK)
- Yang XP et al. Echocardiographic assessment of cardiac function in conscious and anesthetized mice. Am J Physiol 1999. doi:10.1152/ajpheart.1999.277.5.H1967 (Crossref OK)
- Schnelle M et al. Echocardiographic evaluation of diastolic function in mouse models of heart disease. J Mol Cell Cardiol 2018. doi:10.1016/j.yjmcc.2017.10.006 (Crossref OK)
- Reed AL et al. Diastolic dysfunction is associated with cardiac fibrosis in the senescence-accelerated mouse. Am J Physiol Heart Circ Physiol 2011. doi:10.1152/ajpheart.00407.2010 (Crossref OK)
- Liao et al. Comparison of age-related decline in C57BL/6J and CB6F1J male mice. PLoS One 2024. doi:10.1371/journal.pone.0306201 (Crossref OK)
- Fanjul V et al. Identification of common cardiometabolic alterations and deregulated pathways in mouse and pig models of aging. Aging Cell 2020. doi:10.1111/acel.13203 (Crossref OK)
- Dai DF, Rabinovitch PS. Cardiac aging in mice and humans: the role of mitochondrial oxidative stress. Trends Cardiovasc Med 2009. doi:10.1016/j.tcm.2009.12.004 (Crossref OK)
- Banga et al. Signs of diastolic dysfunction are graded by serum testosterone levels in aging C57BL/6 male mice. Mech Ageing Dev 2021. doi:10.1016/j.mad.2021.111523 (Crossref OK)
