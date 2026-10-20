# Mouse kidney function (GFR, albuminuria)

Axis: A9 kidney (axes.md @116f1f8). Related: A15 body size, A7 vessels.

## What it measures

- **GFR (glomerular filtration rate):** how fast the kidneys clean the blood. The transcutaneous method injects a fluorescent marker (FITC-sinistrin) that only the kidneys remove. A small sensor glued to the shaved skin of an awake mouse records how fast the glow fades. Reading: elimination half-life (t1/2), converted to GFR (Schreiber 2012).
- **Albuminuria:** protein (albumin) leaking into the urine, a sign of damaged kidney filters. Usually given as an albumin/creatinine ratio or as excretion over 24 h in a metabolic cage.
- **Blood markers:** cystatin C and creatinine, indirect signs of GFR.

## Training or reference population

Reference for young adults: male C57BL/6, n = 200, GFR 1.215 ± 0.201 mL/min/100 g body weight by the transcutaneous method (Wang 2026). No age-specific reference curve and no UM-HET3 data found.

## Noise

- Two transcutaneous measurements within three days in four strains of young mice and in 24-month C57BL/6 mice: highly reproducible, with tight variability within groups (Schock-Kusch 2013). No coefficient in the parts read, so none quoted.
- The method agreed with plasma clearance in the same lean mice; before this it had been validated only in disease models (Mullins 2020).
- Smallest real change for one mouse: not published.

## Known failure modes

- **Strain.** GFR fell with age in C57BL/6 and CBA inbred mice but stayed constant in the F1 hybrid (Hackbarth 1982). Do not assume B6 results hold for UM-HET3.
- **Body and kidney size.** The strain difference in GFR disappeared once GFR was expressed per kidney weight (Hackbarth 1982), and old kidneys grow larger (Sataranatarajan 2012). How GFR is normalised changes the answer.
- **Time of day and cold.** GFR varies with time of day and rises after cold stress (Wang 2026). Measure at a fixed time and temperature.
- **Old mice and metabolic cages.** Collecting urine over 24 h in a metabolic cage stresses old, frail mice; the transcutaneous method is gentler (Schock-Kusch 2013).
- **Function vs damage.** GFR and albuminuria are separate readings: albumin leakage and kidney scarring can appear with only modest changes in GFR. The metric should keep both.

## Functional anchor and age trajectory

| Source | Mice | Finding |
| --- | --- | --- |
| Hackbarth 1982 | C57BL/6, CBA/HT6, B6CBAF1 | GFR fell with age in both inbred strains, constant in the F1 hybrid; kidneys grew larger, which the authors read as compensation |
| Sataranatarajan 2012 | C57BL/6, young, middle-aged, old | Albuminuria and serum cystatin C rose with age; kidney and glomeruli enlarged with more matrix from middle age |
| Schock-Kusch 2013 | C57BL/6, 3–4 vs 24 months | Slower marker elimination (lower GFR) at 24 months; method usable repeatedly in old mice |
| Geraci 2017 | C57BL/6, adults up to 49 weeks | t1/2 trended upward between 20 and 49 weeks; fewer, larger glomeruli with age |
| Liao 2024 | C57BL/6J vs CB6F1J males | Kidney lesion severity rose with age in both strains |

Take-home: GFR falls and albuminuria rises with age in B6, with structural damage from middle age. The transcutaneous GFR test is repeatable and gentle, so it suits a longitudinal metric. But the age decline is strain-dependent and sensitive to how GFR is normalised for size.

## Confounders

Strain, sex, body and kidney weight (normalisation), time of day, temperature, hydration, anaesthesia (some protocols anaesthetise for injection), diet, blood pressure, sampling method for urine.

## Sources

- Schreiber et al. Transcutaneous measurement of renal function in conscious mice. Am J Physiol Renal Physiol 2012. doi:10.1152/ajprenal.00279.2012 (Crossref OK)
- Schock-Kusch et al. Reliability of transcutaneous measurement of renal function in various strains of conscious mice. PLoS One 2013. doi:10.1371/journal.pone.0071519 (Crossref OK)
- Mullins et al. Validation of non-invasive transcutaneous measurement for glomerular filtration rate in lean and obese C57BL/6J mice. Nephrology 2020. doi:10.1111/nep.13713 (Crossref OK)
- Wang et al. Physiological glomerular filtration rate and responses to Zeitgeber time and cold stress in C57BL/6 mice. Renal Failure 2026. doi:10.1080/0886022X.2026.2648347 (Crossref OK)
- Hackbarth et al. Changes with age in renal function and morphology in C57BL/6, CBA/HT6, and B6CBAF1 mice. J Gerontol 1982. doi:10.1093/geronj/37.5.540 (Crossref OK)
- Sataranatarajan et al. Molecular events in matrix protein metabolism in the aging kidney. Aging Cell 2012. doi:10.1111/acel.12008 (Crossref OK)
- Geraci et al. Combining new tools to assess renal function and morphology: a holistic approach to study the effects of aging and a congenital nephron deficit. Am J Physiol Renal Physiol 2017. doi:10.1152/ajprenal.00329.2015 (Crossref OK)
- Liao et al. Comparison of age-related decline in C57BL/6J and CB6F1J male mice. PLoS One 2024. doi:10.1371/journal.pone.0306201 (Crossref OK)
