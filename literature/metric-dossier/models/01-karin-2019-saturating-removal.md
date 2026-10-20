# Parent model: saturating-removal (SR) model (Karin 2019)

Role in Astra: the parent of R-Sim v0 (organ-level damage/repair model, A68). Related: H8 senescence (mouse/15), H11 inflammation, anchors.md.

## What it models

One damage variable X (in the original paper, senescent cells) in one animal over its life. Three parts (Karin 2019; equation as written in Katzir 2021):
- **Production** rises linearly with age: ηt.
- **Removal** saturates when damage is high, like an enzyme saturated by its substrate: βX/(κ+X). Damage therefore slows its own removal.
- **Noise:** a random term of size ε.
- **Death** occurs when X crosses a threshold Xc.

The model came out of a scan of a wide class of candidate mechanisms. The simplest one that fit the data needed both of the following (Karin 2019): production rising with age, and removal slowed by damage itself rather than directly by age.

## Published fit

| What was fitted | Data | Result (Karin 2019) |
| --- | --- | --- |
| Senescent-cell trajectories | Longitudinal p16-luciferase mice (Burd 2013) | Maximum-likelihood SR parameters |
| Removal speed, predicted then tested | Senescent cells induced in the lung by bleomycin (1.5 U/kg), young vs old mice | Half-life about 5 ± 1 days at 3 months and about 25 ± 6 days at 22 months; old baseline about five-fold higher; measured removal rates agreed with the model |
| Survival | Mouse mortality curve | Good fit with parameters consistent with the half-life and longitudinal data; reproduces the Gompertz law (risk of death rising exponentially with age) and the slowdown of mortality at very old ages, in mice and humans |

Later uses by the same group: disease incidence curves that rise exponentially and then decline at old ages (Katzir 2021); interventions that change the shape (steepness) of mouse survival curves, and parabiosis explained as the young partner sharing removal capacity (Yang 2025); parameter differences between species (Raz 2026).

## Known failure modes

- **One variable.** All ageing is carried by a single damage X. The model says nothing about which organ fails; R-Sim v0 extends it to one X per organ.
- **Calibrated on a modified mouse line.** The trajectories come from p16-luciferase reporter mice, which UM-HET3 mice are not (see mouse/15). The lung test used a different marker (SA-β-gal) in a different organ. That the two agree is evidence for the model, but neither is a UM-HET3 measurement (named gap).
- **X is abstract.** The model works whatever X is, which is also a weakness: a good survival fit alone does not show that X is senescent cells.
- **Parameters can trade off.** The survival curve alone can't tell all parameters apart. Astra's own check (A68, research/sim identifiability, PR #121) found that organ readouts pin down only the balance between production and removal (η vs β), not each one separately, nor the threshold Xc. The half-life test (an induction-and-recovery challenge) is what separates them.
- **Rival models.** A queue-theory damage model also yields the Gompertz law, with constant production and removal that drops with age. Karin et al. found that mechanism at odds with mouse damage trajectories (as reported in Raz 2026).
- **Noise between mice is central, not a nuisance.** Slower removal in old age amplifies the differences between individual mice (Karin 2019), so a single old-age reading is inherently noisy (matches the large spread in mouse/15).

## What it means for the metric

The model predicts that an old animal is marked by *slower recovery after a challenge*, not just a higher level. A recovery-rate readout (induce, then follow the return to baseline) is the test that discriminates. A static level is weaker.

## Sources

- Karin O et al. Senescent cell turnover slows with age providing an explanation for the Gompertz law. Nat Commun 2019. doi:10.1038/s41467-019-13192-4 (Crossref OK)
- Burd CE et al. Monitoring tumorigenesis and senescence in vivo with a p16INK4a-luciferase model. Cell 2013. doi:10.1016/j.cell.2012.12.010 (Crossref OK)
- Katzir I et al. Senescent cells and the incidence of age-related diseases. Aging Cell 2021. doi:10.1111/acel.13314 (Crossref OK)
- Yang Y et al. Compression of morbidity by interventions that steepen the survival curve. Nat Commun 2025. doi:10.1038/s41467-025-57807-5 (Crossref OK)
- Raz et al. A damage accumulation model identifies distinct aging regimes across species. Nat Aging 2026. doi:10.1038/s43587-026-01138-7 (Crossref OK)
