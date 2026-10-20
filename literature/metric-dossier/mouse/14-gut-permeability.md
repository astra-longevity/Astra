# Mouse gut permeability (FITC-dextran)

Axis: A14 gut (axes.md @116f1f8). Related: A11 immune, H11 inflammation (TNF, IL-6), H12 microbiome, A10 metabolism (diet).

## What it measures

The mouse is fasted, then given FITC-dextran (a sugar labelled with a fluorescent dye) by mouth through a feeding tube. A healthy gut barrier lets little of it into the blood. After a set time, a blood sample is taken and its fluorescence is read. More fluorescence = leakier gut ("leaky gut").
- **Molecule size:** 4 kDa FITC-dextran is the usual probe. Measured in plasma 1 hour after dosing it reports small-intestine permeability, since a larger 70 kDa probe showed the dose has passed the whole small intestine within 1 hour (Woting 2018, C3H mice).
- **Protocols differ:** blood taken at 45 min (Kurihara 2025), 1 h (Woting 2018) or 4 h (Eskiocak 2025); dose fixed per mouse (Eskiocak 2025) or per body weight (Ahmadi 2020); fasting 4–6 h. In Eskiocak 2025, a pre-dose and a 4-h blood sample were taken from the same mouse.

## Training or reference population

No standard reference. Data come from small groups of young vs old mice, mostly C57BL/6 or not stated. No UM-HET3 data found.

## Noise

- Test-retest noise and smallest real change for one mouse: none found in the sources read.
- Whether the same mouse can be retested over months: no ageing study found that did so.

## Known failure modes

- **The microbiome drives it.** In Thevaranjan 2017, permeability rose with age, but not in old germ-free mice or old TNF-knockout mice; young germ-free mice given old-mouse microbiota became leakier. In Parker 2022, aged-donor microbiota given to young mice raised gut TNF and blood markers of barrier damage. Cage, vendor, diet and microbiota are therefore confounders, not noise.
- **Not universal.** In senescence-accelerated mice (SAMR1 and SAMP1), serum FITC-dextran 45 min after dosing did not change between young and old (Kurihara 2025). The direction found in one model or protocol should not be assumed elsewhere.
- **Diet.** In about 21-month-old mice, a high-fat diet made the gut leakier than a low-fat diet (Ahmadi 2020). Diet must be fixed.
- **Stress unmasks it.** Old mice with a burn injury had higher FITC-dextran than all other groups (Najarro 2022). Illness or injury at testing time will inflate the reading.
- **It measures one route.** 4 kDa FITC-dextran tests one passage route between gut cells. Tight-junction protein loss (ZO-1, occludin) was seen alongside it in some studies (Brandt 2022), but the dye result alone does not say where or why the barrier fails.

## Functional anchor and age trajectory

| Source | Mice | Finding |
| --- | --- | --- |
| Thevaranjan 2017 | WT, 3, 12, 15, 18 months (n = 4–8); germ-free; TNF knockout | Permeability rose with age; not in old germ-free or TNF-knockout mice; old microbiota made young germ-free mice leakier |
| Brandt 2022 | Old vs young mice (and older men) | Old mice: higher small-intestine permeability, lower tight-junction proteins |
| Parker 2022 | Young and aged, microbiota transfer both ways | Aged donor microbiota raised gut TNF and blood barrier-damage markers in young recipients |
| Kurihara 2025 | SAMR1 and SAMP1, young vs old | No change in serum FITC-dextran with age |
| Eskiocak 2025 | Young vs old | Old mice leakier at 4 h; reduced by removing uPAR+ (senescent-like) cells with CAR T cells |
| Khalyfa 2026 | Young vs old males | Old males had markedly higher circulating FITC-dextran |

Take-home: in most studies of normal ageing, old mice have a leakier gut, but the effect depends on microbiota and TNF, and it was absent in one accelerated-ageing model. The test is cheap but its protocol (dose, timing, fasting, probe size) is not standardised, and no source gives its test-retest noise.

## Confounders

Microbiota (cage, vendor, housing), diet, fasting time, dose (fixed vs per weight), sampling time after dosing, probe size, sample type (plasma vs serum) and haemolysis, illness or injury, sex, strain.

## Sources

- Thevaranjan et al. Age-associated microbial dysbiosis promotes intestinal permeability, systemic inflammation, and macrophage dysfunction. Cell Host Microbe 2017. doi:10.1016/j.chom.2017.03.002 (Crossref OK)
- Woting & Blaut. Small intestinal permeability and gut-transit time determined with low and high molecular weight fluorescein isothiocyanate-dextrans in C3H mice. Nutrients 2018. doi:10.3390/nu10060685 (Crossref OK)
- Ahmadi et al. Metformin reduces aging-related leaky gut and improves cognitive function by beneficially modulating gut microbiome/goblet cell/mucin axis. J Gerontol A 2020. doi:10.1093/gerona/glaa056 (Crossref OK)
- Brandt et al. Impairments of intestinal arginine and NO metabolisms trigger aging-associated intestinal barrier dysfunction and 'inflammaging'. Redox Biol 2022. doi:10.1016/j.redox.2022.102528 (Crossref OK)
- Najarro et al. Advanced age exacerbates intestinal epithelial permeability after burn injury in mice. Exp Gerontol 2022. doi:10.1016/j.exger.2021.111654 (Crossref OK)
- Parker et al. Fecal microbiota transfer between young and aged mice reverses hallmarks of the aging gut, eye, and brain. Microbiome 2022. doi:10.1186/s40168-022-01243-w (Crossref OK)
- Kurihara et al. New indicator of small intestinal ageing in senescence-accelerated mice. J Physiol Sci 2025. doi:10.1016/j.jphyss.2025.100030 (Crossref OK)
- Eskiocak et al. Anti-uPAR CAR T cells reverse and prevent aging-associated defects in intestinal regeneration and fitness. Nat Aging 2025. doi:10.1038/s43587-025-01022-w (Crossref OK)
- Khalyfa et al. Gut luminal exosomes in young and old mice: multi-omic characteristics and regulation of gut permeability. Aging Cell 2026. doi:10.1111/acel.70455 (Crossref OK)
