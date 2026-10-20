# Mouse blood stem cells (HSC number and function)

Axis: H9 stem cells (axes.md @116f1f8). Readout as used by A67: haematopoietic stem cell (HSC) number and function. Controls named by A28: rapamycin right-way (Chen 2009), irradiation wrong-way. Related: A12 blood (CBC), A11 immune.

## What it measures

HSCs are the rare bone-marrow cells that make all blood cells for life. Two different readings:
- **Number:** bone marrow is taken and HSCs are counted by flow cytometry using surface markers (for example the LSK fraction: Lin-low, Sca1+, cKit+, and narrower marker sets inside it).
- **Function:** HSCs are transplanted into irradiated recipient mice, and their blood output, homing to the marrow and self-renewal (output in a second round of transplants) are measured over months.

Both need bone marrow, so the donor mouse is killed: no repeat reading on the same mouse. Transplant assays also need recipient mice and months of follow-up.

## Training or reference population

No standard reference curve. Age data are mostly from C57BL/6 (B6) mice, compared with DBA/2 (Geiger 2005). No UM-HET3 curve in the sources here (named gap).

## Noise

- No test-retest coefficient for HSC counts or transplant readouts in the sources here (named gap; a same-mouse retest is not possible).
- Between mice: HSC-marker cell frequency is "highly variable between individual old mice" (10 young 4–5 mo vs 22 old 24–28 mo B6; Dykstra 2011), while it is similar among young mice.
- Smallest real change: not in the sources here (named gap).

## Known failure modes

- **Number goes the "wrong" way.** In old B6 mice HSCs are more numerous, not fewer: five times as frequent in old marrow (Morrison 1996); median LSK proportion up 1.9-fold (Dykstra 2011). A higher count is therefore not a younger reading.
- **Number and function diverge.** Old HSCs home to the marrow worse: one-quarter as efficient at homing and engrafting (Morrison 1996), a twofold drop in homing in Dykstra 2011. Old HSCs are mostly myeloid-biased, make fewer mature blood cells and self-renew less in secondary transplants (Dykstra 2011). Number alone misses this; function needs a transplant.
- **Strain reverses the number trend.** In B6 HSC numbers rise linearly with age; in DBA/2 they follow a bell shape, peaking near 1 year and falling to about 80% of the young pool in old age (Geiger 2005). The B6 trend must not be assumed for UM-HET3.
- **Cell-intrinsic.** Ageing defects persist after transplant into a young host (Rossi 2005, title finding), so a young recipient does not reset the reading.

## Functional anchor and age trajectory

| Source | Animals | Finding |
| --- | --- | --- |
| Morrison 1996 | Old vs young and middle-aged mice | HSCs 5x as frequent in old marrow; single old HSCs had progenitor activity like young ones; old HSCs one-quarter as efficient at homing/engrafting |
| Dykstra 2011 | B6, 10 young (4–5 mo) vs 22 old (24–28 mo) | LSK proportion up 1.9-fold (median), highly variable in old; myeloid-biased clones with less output and self-renewal; homing down twofold |
| Geiger 2005 | B6 vs DBA/2 | B6: linear rise with age; DBA/2: bell-shaped, ~80% of young size in old age |
| Chen 2009 | Old mice given rapamycin | Rapamycin (mTOR inhibitor) restored HSC self-renewal and blood production (title: "therapeutic rejuvenation"); this is A67's right-way control |

Take-home: HSC function declines with age, but the HSC count rises in B6 and falls in other strains. The function reading (transplant) carries the ageing signal; the count needs direction set per strain.

## Confounders

Strain (direction of the count), marker set used for counting, transplant protocol (recipient irradiation, cell dose, follow-up length), terminal sampling, sex (not reported in the sources here).

## Sources

- Morrison SJ et al. The aging of hematopoietic stem cells. Nat Med 1996. doi:10.1038/nm0996-1011 (Crossref OK)
- Dykstra B et al. Clonal analysis reveals multiple functional defects of aged murine hematopoietic stem cells. J Exp Med 2011. doi:10.1084/jem.20111490 (Crossref OK)
- Geiger H et al. Regulation of hematopoietic stem cell aging in vivo by a distinct genetic element. PNAS 2005. doi:10.1073/pnas.0408654102 (Crossref OK)
- Rossi DJ et al. Cell intrinsic alterations underlie hematopoietic stem cell aging. PNAS 2005. doi:10.1073/pnas.0503280102 (Crossref OK)
- Chen C et al. mTOR regulation and therapeutic rejuvenation of aging hematopoietic stem cells. Sci Signal 2009. doi:10.1126/scisignal.2000559 (Crossref OK)
