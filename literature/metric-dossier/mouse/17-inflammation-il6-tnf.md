# Mouse inflammation (plasma IL-6 and TNF)

Axis: H11 inflammation (axes.md @116f1f8). Readout as used by A67: plasma IL-6 and TNF. Controls named by A28: no right-way control found yet (named obstacle); wrong-way = high-fat diet (via fat-tissue cytokines; source in A67's controls). Related: A14 gut permeability, H12 microbiome, A11 immune, H8 senescence.

## What it measures

IL-6 and TNF are signalling proteins (cytokines) that drive inflammation. A small blood sample is taken and their level in plasma is read, usually by ELISA or a bead-based multiplex assay. "Inflammaging" is the name for the slow, low-grade rise of such signals with age (Franceschi 2000). A blood sample can be repeated in the same mouse.

## Training or reference population

No standard reference curve. Mouse age data in the sources here are C57BL/6 (B6) (Dugan 2009) and conventional vs germ-free mice (Thevaranjan 2017). No UM-HET3 curve in the sources here (named gap).

## Noise

- No same-mouse test-retest coefficient (ICC or CV) for plasma IL-6 or TNF in the sources here (named gap).
- Smallest real change for one mouse: not in the sources here (named gap).
- Lower detection limit in young-mouse plasma: not in the sources here (named gap). The assay and its detection limit must be stated, and values below it handled explicitly.

## Known failure modes

- **Microbiota-dependent.** Germ-free mice show no age-related rise in circulating pro-inflammatory cytokines; co-housing germ-free mice with old (not young) conventional mice raises them; TNF-deficient mice are protected (Thevaranjan 2017). The reading depends on the gut bacteria and on housing, not only on age.
- **Acute illness and challenge.** Infection or injury raises these cytokines far above the resting level, and old mice respond more strongly (Puchta 2016: TNF drives monocyte dysfunction with age). Sick mice must be excluded or flagged.
- **Human data conflict.** One human study found no significant difference in circulating IL-6 between young and elderly volunteers, with a subgroup in both ages having very high levels (Beharka 2001). The rise is not universal.
- **No clean right-way control.** Anti-TNF treatment reversed the age-linked microbiota changes (Thevaranjan 2017), but it blocks the readout itself, so it cannot validate the reading (circular). No intervention that lowers plasma IL-6/TNF by slowing ageing was found in the sources here (named obstacle).

## Functional anchor and age trajectory

| Source | Animals | Finding |
| --- | --- | --- |
| Dugan 2009 | Healthy B6, old (24 mo) vs young (4 mo), ELISA | Plasma IL-6 nearly 15-fold higher in old mice; linked to forebrain interneuron loss and cognitive impairment |
| Thevaranjan 2017 | Young and old, conventional vs germ-free mice | Plasma TNF and IL-6 higher in old conventional mice; no age rise in germ-free mice; a higher share of germ-free mice lived to 600 days |
| Puchta 2016 | Young vs old mice | Age-related TNF rise drives monocyte dysfunction and impaired defence against pneumococcus (title finding) |
| Ferrucci 2005 | Humans | Age-related pro-inflammatory state (title finding; human context) |
| Beharka 2001 | Humans, young vs elderly | Circulating IL-6 not significantly different |

Take-home: plasma IL-6/TNF rise in old conventional B6 mice, but the rise depends on the microbiota and is absent in germ-free mice. A high value may reflect gut bacteria, housing or illness rather than ageing. Germ-free vs conventional housing is a candidate contrast for testing the reading, but it is not a treatment.

## Confounders

Microbiota and cage (co-housing), acute infection or injury, diet (high-fat diet raises it), strain, sex (not reported in the sources here), assay type and detection limit, time of sampling.

## Sources

- Franceschi C et al. Inflamm-aging: an evolutionary perspective on immunosenescence. Ann N Y Acad Sci 2000. doi:10.1111/j.1749-6632.2000.tb06651.x (Crossref OK)
- Dugan LL et al. IL-6 mediated degeneration of forebrain GABAergic interneurons and cognitive impairment in aged mice through activation of neuronal NADPH oxidase. PLoS One 2009. doi:10.1371/journal.pone.0005518 (Crossref OK)
- Thevaranjan N et al. Age-associated microbial dysbiosis promotes intestinal permeability, systemic inflammation, and macrophage dysfunction. Cell Host Microbe 2017. doi:10.1016/j.chom.2017.03.002 (Crossref OK)
- Puchta A et al. TNF drives monocyte dysfunction with age and results in impaired anti-pneumococcal immunity. PLoS Pathog 2016. doi:10.1371/journal.ppat.1005368 (Crossref OK)
- Ferrucci L et al. The origins of age-related proinflammatory state. Blood 2005. doi:10.1182/blood-2004-07-2599 (Crossref OK)
- Beharka AA et al. Interleukin-6 production does not increase with age. J Gerontol A 2001. doi:10.1093/gerona/56.2.b81 (Crossref OK)
