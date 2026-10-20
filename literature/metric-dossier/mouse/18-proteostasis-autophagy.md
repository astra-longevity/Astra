# Mouse protein maintenance (autophagy flux and protein aggregates)

Axis: H4 proteostasis (axes.md @116f1f8). Readout: not yet named by A67; to be aligned at A67's 10-23 PR. Controls named by A28: calorie restriction and rapamycin (right-way). This file covers the two candidate readouts A28 asked for: autophagy flux and protein aggregates. Related: H8 senescence, A1 muscle, A6 heart.

## What it measures

Cells clear damaged proteins mainly by autophagy: waste is wrapped in a membrane bubble (autophagosome) and digested in the lysosome.
- **Autophagy flux:** the *rate* of this clearance, not the amount of bubbles at one moment. The usual marker is the protein LC3-II, which sits on the bubbles. Flux is read by comparing LC3-II with and without a drug that blocks the final digestion step, for example leupeptin (Haspel 2011) or colchicine (Ju 2010). Reporter mice carrying a fluorescent LC3 (GFP-LC3, Mizushima 2004; tandem RFP-GFP-LC3, Carosi 2025) show the bubbles in tissue.
- **Protein aggregates / build-up:** clumped or tagged-for-disposal proteins, read as ubiquitinated proteins or the autophagy cargo p62 on a western blot (Zhou 2017).

Both readings need tissue, so the mouse is killed (terminal). The blocker must be given in a living mouse hours to days before sampling.

## Training or reference population

No standard reference curve. Age data in the sources here come from laboratory mice (Zhou 2017; Sebti 2022; Carosi 2025); the Carosi 2025 age findings are in males, and strain and sex for the others are not checked here. No UM-HET3 curve in the sources here (named gap).

## Noise

- No test-retest coefficient for flux or aggregate readings in the sources here (named gap; a same-mouse retest is not possible because sampling is terminal).
- Smallest real change: not in the sources here (named gap).

## Known failure modes

- **A single LC3-II level misleads.** More bubbles can mean more clearance or blocked clearance. In mouse muscle, rapamycin or fasting alone did not raise LC3-II, but adding colchicine revealed higher flux (Ju 2010). Only flux (with a blocker) is a valid reading.
- **Organ-dependent, even opposite.** In tandem-LC3 reporter mice, ageing lowered flux in the heart, raised it in blood cells and did not change it in motor-cortex neurons (males; Carosi 2025). Its review of prior work also reports a decline in liver, heart, kidney and muscle but a rise in white fat (Carosi 2025). The organ must be fixed in advance.
- **Size of the decline differs by organ.** Flux fell with age in all organs studied, but less in skeletal muscle than in heart, kidney and liver; the liver lost it most (Sebti 2022).
- **Diet.** Carosi 2025 tested regular chow vs high-fat diet; diet must be fixed and recorded.
- **Reporter mice are modified lines**, so the LC3 reporter cannot be read on UM-HET3 mice as they are; the blocker-based western blot can.
- **No mouse aggregate (insoluble proteome) age curve** in the sources here (named gap). Ubiquitinated protein and p62 are indirect markers.

## Functional anchor and age trajectory

| Source | Animals | Finding |
| --- | --- | --- |
| Zhou 2017 | Mice, young 5 mo vs aged 27 mo; skeletal and cardiac muscle | Less autophagic breakdown of p62 and more ubiquitinated proteins in old mice: macroautophagy declines |
| Sebti 2022 | Wild-type vs Becn1 F121A knock-in (higher autophagy), young vs old | Flux fell with age in every organ studied; the mutation kept flux higher in old heart, kidney and muscle and reduced age-related muscle fibre shrinkage (benefit organ-dependent) |
| Carosi 2025 | Tandem RFP-GFP-LC3 mice, chow or high-fat diet | Males: flux down in heart, up in blood, unchanged in motor-cortex neurons |
| Cho 2021 | Old mice, treadmill training started late in life | Training restored heart autophagy, aggregate clearance and function (title finding) |
| Harrison 2009 | UM-HET3 mice, rapamycin from late life | Rapamycin, an mTOR inhibitor that induces autophagy (Rubinsztein 2011), extended lifespan (title finding); A28's right-way control |

Take-home: protein clearance falls with age in heart, liver, kidney and muscle, and raising it (gene change, late exercise) improves function, so the axis has a functional anchor. But the direction differs by organ (blood up, brain flat), and a single LC3-II level can't tell more clearance from blocked clearance. Use flux with a blocker, in a fixed organ (heart or liver are the clearest).

## Confounders

Organ and cell type, fasting state and time of day (autophagy responds to feeding), diet, blocker dose and timing, sex (Carosi 2025 age data male only), strain and reporter line, terminal sampling.

## Sources

- Mizushima N et al. In vivo analysis of autophagy in response to nutrient starvation using transgenic mice expressing a fluorescent autophagosome marker. Mol Biol Cell 2004. doi:10.1091/mbc.e03-09-0704 (Crossref OK)
- Haspel J et al. Characterization of macroautophagic flux in vivo using a leupeptin-based assay. Autophagy 2011. doi:10.4161/auto.7.6.15100 (Crossref OK)
- Ju JS et al. Quantitation of "autophagic flux" in mature skeletal muscle. Autophagy 2010. doi:10.4161/auto.6.7.12785 (Crossref OK)
- Zhou J et al. Changes in macroautophagy, chaperone-mediated autophagy, and mitochondrial metabolism in murine skeletal and cardiac muscle during aging. Aging 2017. doi:10.18632/aging.101181 (Crossref OK)
- Sebti S et al. BECN1 F121A mutation increases autophagic flux in aged mice and improves aging phenotypes in an organ-dependent manner. Autophagy 2022. doi:10.1080/15548627.2022.2111852 (Crossref OK)
- Carosi JM et al. Autophagy across tissues of aging mice. PLoS One 2025. doi:10.1371/journal.pone.0325505 (Crossref OK)
- Cho JM et al. Late-in-life treadmill training rejuvenates autophagy, protein aggregate clearance, and function in mouse hearts. Aging Cell 2021. doi:10.1111/acel.13467 (Crossref OK)
- Harrison DE et al. Rapamycin fed late in life extends lifespan in genetically heterogeneous mice. Nature 2009. doi:10.1038/nature08221 (Crossref OK)
- Rubinsztein DC et al. Autophagy and aging. Cell 2011. doi:10.1016/j.cell.2011.07.030 (Crossref OK; review, concept only)
