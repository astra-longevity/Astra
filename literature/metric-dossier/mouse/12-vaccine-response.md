# Mouse vaccine antibody response

Axis: A11 immune (axes.md @116f1f8). Related: A12 blood (lymphocyte counts, file 08), A16 endocrine (sex hormones).

## What it measures

The mouse is vaccinated with an antigen it has never met (for example an inactivated influenza vaccine, or a model antigen such as NP or a peptide). Weeks later the antibodies in its blood are measured. Readings:
- **Amount:** antigen-specific antibody titre (ELISA), or functional titres such as haemagglutination inhibition (HAI) for influenza.
- **Quality:** antibody affinity or avidity (how tightly the antibodies bind) and antibody class (IgG1 vs IgG2a/IgG2c).
- **Duration:** how long titres stay up after vaccination.
- **Protection:** survival or weight loss after a live virus challenge.
- **Cells:** germinal centre B cells and T follicular helper (Tfh) cells in the lymph nodes or spleen. This reading is terminal.

## Training or reference population

No standard reference. Data come from small groups of young (2–3 months) vs aged (about 17–24 months) C57BL/6 and BALB/c mice. No UM-HET3 data found.

## Noise

- Test-retest noise and smallest real change for one mouse: none found in the sources read.
- By design the test cannot simply be repeated: once vaccinated, the mouse carries memory of that antigen, and a booster gives a different, stronger response (Baldwin 2018). A longitudinal design needs a new antigen at each time point.

## Known failure modes

- **Strain flips the direction.** Aged C57BL/6 mice (20–24 months) had markedly fewer IgM antibody-forming cells to a bacterial antigen than young mice, while aged BALB/c mice responded more strongly than young ones (Nicoletti 1991). The effect of age on influenza vaccine IgG titres also depended on strain (Bufan 2020). Do not assume B6 results hold for UM-HET3.
- **Sex.** Females mounted higher IgG titres regardless of strain and age (Bufan 2020; Arsenović-Ranin 2019). In C57BL/6, the female advantage seen in adults was lost in aged mice, which correlated with lower estradiol (Dhakal 2024). Sex must be a fixed factor.
- **Amount and quality move in different directions.** Ageing reduced titres, but increased antibody avidity in females of both strains (Bufan 2020). Aged mice also showed reduced affinity maturation (Silva-Cayetano 2023). The metric must say which reading it uses.
- **Adjuvant and booster mask the decline.** An immunostimulatory adjuvant reversed the low antibody response of aged mice and monkeys (Maxwell 2026). A booster dose let aged C57BL/6 mice reach protective titres (Baldwin 2018). Protocol must be fixed: antigen, adjuvant, dose number.
- **The defect is in the environment, not only the B cell.** B cells from aged C57BL/6 donors made normal amounts of antibody in vitro; T helper cells explained the decline (Nicoletti 1994). The aged microenvironment impaired Tfh differentiation (Fisher 2024), and the germinal centre defect was reversible (Silva-Cayetano 2023). A good axis reading, but it measures a system, not one cell type.

## Functional anchor and age trajectory

| Source | Mice | Finding |
| --- | --- | --- |
| Nicoletti 1991 | C57BL/6 and BALB/c, 2–3 vs 20–24 months | B6: markedly reduced IgM response to a bacterial antigen with age; BALB/c: higher in aged mice |
| Baldwin 2018 | C57BL/6 and CB6F1, young vs aged | After two adjuvanted doses, all young B6 had HAI titres above 1:40, but only one of three aged mice in the MF59 and GLA-SE groups; aged mice made no Th1-type IgG2 response; a boost restored protection |
| Bufan 2020 | BALB/c and C57BL/6, both sexes | Age effect on influenza IgG titre strain-specific; females higher; avidity up with age in females |
| Silva-Cayetano 2023 | Aged BALB/c and C57BL/6 | Smaller germinal centres, lower NP-specific titres, less affinity maturation; reversed by Tfh cells placed correctly |
| Dhakal 2024 | C57BL/6CR, 2–3 vs 17 months, both sexes | Aged mice: fewer B cells, less durable antibodies, worse protection against H1N1 challenge |
| Fisher 2024 | Young vs aged mice, peptide immunisation | Aged: less expansion of antigen-specific germinal centre B cells, lower titres |
| Maxwell 2026 | Aged mice and monkeys | Reduced antibody production after immunisation was the main age-associated marker; reversed by adjuvant |

Take-home: the antibody response to a new vaccine falls with age in C57BL/6, mostly through T helper and germinal centre failure. It is a strong functional anchor for the immune axis, but strain, sex, adjuvant and booster can change or even reverse the result, and the same antigen cannot be reused in the same mouse.

## Confounders

Strain, sex and sex hormones, antigen, adjuvant, dose number and interval, route, time from vaccination to sampling, prior infections and housing (pathogen status), assay type (ELISA vs HAI vs avidity).

## Sources

- Nicoletti et al. The repertoire diversity and magnitude of antibody responses to bacterial antigens in aged mice: I. Age-associated changes in antibody responses differ according to the mouse strain. Cell Immunol 1991. doi:10.1016/0008-8749(91)90180-J (Crossref OK)
- Nicoletti et al. Antibody response in aged C57BL/6 mice: T helper cells are responsible for the decline of the primary antibody response to a bacterial antigen in aging. Immunobiology 1994. doi:10.1016/S0171-2985(11)80288-3 (Crossref OK)
- Baldwin et al. Improved immune responses in young and aged mice with adjuvanted vaccines against H1N1 influenza infection. Front Immunol 2018. doi:10.3389/fimmu.2018.00295 (Crossref OK)
- Arsenović-Ranin et al. Influence of aging on germinal centre reaction and antibody response to inactivated influenza virus antigens in mice: sex-based differences. Biogerontology 2019. doi:10.1007/s10522-019-09811-8 (Crossref OK)
- Bufan et al. Strain specificities in influence of ageing on germinal centre reaction to inactivated influenza virus antigens in mice: sex-based differences. Exp Gerontol 2020. doi:10.1016/j.exger.2020.110857 (Crossref OK)
- Silva-Cayetano et al. Spatial dysregulation of T follicular helper cells impairs vaccine responses in aging. Nat Immunol 2023. doi:10.1038/s41590-023-01519-9 (Crossref OK)
- Dhakal et al. Estradiol mediates greater germinal center responses to influenza vaccination in female than male mice. mBio 2024. doi:10.1128/mbio.00326-24 (Crossref OK)
- Fisher et al. The aged microenvironment impairs BCL6 and CD40L induction in CD4+ T follicular helper cell differentiation. Aging Cell 2024. doi:10.1111/acel.14140 (Crossref OK)
- Maxwell et al. Rescuing age-associated vaccine hyporesponsiveness in murine and nonhuman primate models with an immunostimulatory vaccine adjuvant. Hum Vaccin Immunother 2026. doi:10.1080/21645515.2026.2689208 (Crossref OK)
