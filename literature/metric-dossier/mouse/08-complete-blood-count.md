# Mouse complete blood count (CBC)

Axis: A12 blood (axes.md @116f1f8). Related: A11 immune (lymphocyte counts).

## What it measures

A small blood sample is run on an automated counter. Main readings:
- **Red cells:** red blood cell count (RBC), haemoglobin (HGB), haematocrit (HCT, % of blood volume that is red cells), red cell size (MCV) and red cell size spread (RDW).
- **White cells:** total white blood cells (WBC) and the split into neutrophils, lymphocytes, monocytes, eosinophils and basophils, as counts or %.
- **Platelets** (PLT) and platelet size (MPV).

Ageing readouts: anaemia (falling RBC and HGB), and a shift from lymphocytes to neutrophils.

## Training or reference population

Vendor reference ranges exist for young adult B6 mice, not for old mice.
- Charles River C57BL/6NCrl, North American colonies 2008–2012: WBC mean 8.90 ×10³/µL in males (95% interval 4.45–13.96, n = 123) and 8.44 in females (3.90–13.94, n = 125); lymphocytes 77.4% (males) and 79.6% (females) of WBC; neutrophils 15.9% and 14.0%.
- Ages 3–24 months, every 3 months, B6J and B6N, both sexes (Almunia 2026).

No UM-HET3 reference curve found `[to verify]`.

## Noise

- No published same-mouse test-retest coefficient (ICC or CV) found. `[to verify]`
- Reference ranges are wide: the 95% interval for WBC spans about threefold in young B6NCrl males (4.45–13.96 ×10³/µL, Charles River). One reading in one mouse says little; trends over repeated samples are more informative.
- Lymphocyte and total WBC counts "were subject to considerable variation" over the lifespan (Leuenberger 1976).
- Smallest real change for one mouse: not published.

## Known failure modes

- **Sampling site.** In anaesthetised BALB/c mice, WBC was highest in tail blood, lower in eye blood and significantly lower in heart blood; RBC, platelets and haematocrit were also higher in tail than heart blood (Nemzek 2001). One site must be used for the whole study.
- **Strain and substrain.** At 24 months, B6J males had higher WBC (especially lymphocytes) than B6N; CD4+ T cells fell between 6 and 24 months in B6N and in females, not in B6J males (Almunia 2026). Do not pool substrains.
- **Sex.** Young female B6 differ from males in red cells and neutrophils (Taconic B6 summary: HCT 48.8% vs 41.8%, neutrophils 0.41 vs 1.28 ×10³/µL) `[to verify]` (vendor web page, ages and method not checked).
- **Late-life disease.** Lymphoma and other blood cancers are common in old B6 mice and can raise WBC; a rising WBC late in life may be disease, not ageing `[to verify]` (not checked in a source today).

## Functional anchor and age trajectory

| Source | Mice | Finding |
| --- | --- | --- |
| Leuenberger 1976 | 1,052 C57BL/6J (296 males, 756 females), 6 weeks to natural death | RBC and haemoglobin fell gradually across life. WBC and lymphocytes fell in the first two-thirds of life and rose in the last third. Neutrophils stable with a late upward trend. Lymphocyte % fell in favour of neutrophils. No important sex differences |
| Almunia 2026 | B6J and B6N, both sexes, 3–24 months, every 3 months | RBC and HGB fell with age in all strains; platelets rose |

Take-home: falling RBC/HGB (anaemia) and a lower lymphocyte-to-neutrophil balance are the most consistent age changes. Total WBC is not monotonic and is easily driven by disease, so it is a poor ageing readout on its own. Blood counts are cheap and repeatable from small volumes, so they suit longitudinal tracking.

## Confounders

Sampling site, anaesthesia, sample volume and repeat-bleed frequency, analyser and its mouse settings, strain and substrain, sex, time of day, stress from handling, infection, late-life blood cancers.

## Sources

- Leuenberger HG, Kunstýr I. Gerontological data of C57BL/6J mice. II. Changes in blood cell counts in the course of natural aging. J Gerontol 1976. doi:10.1093/geronj/31.6.648 (Crossref OK)
- Almunia JA et al. Hematologic and biochemical changes associated with age and strain in aged B6 mice breeding in the National Center of Geriatrics and Gerontology. Exp Anim 2026. doi:10.1538/expanim.25-0071 (Crossref OK)
- Nemzek JA et al. Differences in normal values for murine white blood cell counts and other hematological parameters based on sampling site. Inflamm Res 2001. doi:10.1007/PL00000229 (Crossref OK)
- Charles River. C57BL/6 mouse hematology, North American colonies, January 2008 – December 2012. https://www.criver.com/resources/c57bl6-mouse-clinical-pathology-data (vendor data, no DOI)
- Taconic. B6 physiological data summary. https://www.taconic.com/resources/phenotypic-data/b6-physiological-data-summary (vendor data, no DOI)
