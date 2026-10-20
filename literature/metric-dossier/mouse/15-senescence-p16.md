# Mouse cell senescence (p16INK4a; SA-β-gal secondary)

Axis: H8 senescence (axes.md @116f1f8). Readout as used by A67: p16INK4a expression; SA-β-gal secondary. Related: H11 inflammation, A17 neoplasia.

## What it measures

Senescent cells are cells that have stopped dividing for good and stay in the tissue. Two common markers:
- **p16INK4a** (a cell-cycle brake gene): its RNA level in a tissue, read by qPCR, or its activity seen through a light-emitting reporter in a genetically engineered mouse (p16-luciferase, Burd 2013).
- **SA-β-gal** (senescence-associated β-galactosidase): a stain that turns senescent cells blue in cultured cells and in tissue sections (Dimri 1995). Secondary readout here: it needs tissue, gives a count per section, and is harder to compare across labs.

## Training or reference population

No standard reference curve. Age data come from rodent tissues (Krishnamurthy 2004) and from p16-luciferase reporter mice (Burd 2013). The reporter needs a modified mouse line, so it cannot be used on UM-HET3 mice as they are; qPCR on tissue or blood can. No UM-HET3 reference curve in the sources here (named gap).

## Noise

- No same-mouse test-retest coefficient (ICC or CV) for p16 qPCR or SA-β-gal in the sources here (named gap).
- Between mice the spread is large: in p16-luciferase mice housed together, of the same strain and age, the rise with age was "highly variable" from mouse to mouse (Burd 2013).
- Smallest real change for one mouse: not in the sources here (named gap).

## Known failure modes

- **Tissue choice.** The rise is confined to specific compartments within each organ (Krishnamurthy 2004), so the sampling site must be fixed.
- **Floor in young tissue.** In some young tissues (pancreas, bone marrow) p16 is below detection, so fold changes there are minimum estimates only (Krishnamurthy 2004).
- **Tumours.** p16-luciferase lit up in all 14 tumour models tested (Burd 2013): a tumour can raise the signal without broad ageing. Record necropsy findings.
- **Tissue sampling is terminal** for most organs; only blood or the reporter allows repeat readings on the same mouse.
- **Not a cancer predictor:** p16 level with age did not predict cancer development in Burd 2013.

## Functional anchor and age trajectory

| Source | Animals | Finding |
| --- | --- | --- |
| Krishnamurthy 2004 | Mice (15 tissues) and rats (12 tissues) | p16INK4a rose 3-fold or more in 26 of 27 organs; average old/young ratio across the 15 mouse tissues 9.7 (geometric mean), likely an underestimate |
| Krishnamurthy 2004 | Calorie-restricted vs freely fed | Calorie restriction cut the age rise 2- to 16-fold in adrenal, heart, kidney, ovary and testis, but not in lung, lymph node, spleen or liver |
| Burd 2013 | p16-luciferase mice, followed for life | Signal rose exponentially with age |
| Baker 2016 | Mice with an inducible way to kill p16-positive cells | Removing naturally occurring p16-positive cells lengthened healthy lifespan (title finding) |
| Xu 2018 | Old mice treated with senolytics (dasatinib + quercetin) | Physical function improved and lifespan increased (title finding); this is A67's right-way control |
| Liu 2009 | Human peripheral blood T cells | p16INK4a rose exponentially with age; linked to smoking, physical inactivity and plasma IL-6, not to sex or BMI |

Take-home: p16 is a strong, causal-looking ageing marker (removing p16-positive cells helps), but its effect on calorie restriction depends on the organ, and the spread between mice is large. Which tissue is read must be fixed in advance.

## Confounders

Tissue and compartment sampled, tumours, strain (reporter line vs UM-HET3), diet (calorie restriction acts organ by organ). In humans: smoking, physical inactivity.

## Sources

- Krishnamurthy J et al. Ink4a/Arf expression is a biomarker of aging. J Clin Invest 2004. doi:10.1172/JCI22475 (Crossref OK)
- Burd CE et al. Monitoring tumorigenesis and senescence in vivo with a p16INK4a-luciferase model. Cell 2013. doi:10.1016/j.cell.2012.12.010 (Crossref OK)
- Baker DJ et al. Naturally occurring p16Ink4a-positive cells shorten healthy lifespan. Nature 2016. doi:10.1038/nature16932 (Crossref OK)
- Xu M et al. Senolytics improve physical function and increase lifespan in old age. Nat Med 2018. doi:10.1038/s41591-018-0092-9 (Crossref OK)
- Liu Y et al. Expression of p16INK4a in peripheral blood T-cells is a biomarker of human aging. Aging Cell 2009. doi:10.1111/j.1474-9726.2009.00489.x (Crossref OK)
- Dimri GP et al. A biomarker that identifies senescent human cells in culture and in aging skin in vivo. PNAS 1995. doi:10.1073/pnas.92.20.9363 (Crossref OK)
