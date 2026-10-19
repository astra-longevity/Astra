# 00 — Overview: the map of aging in plain language

This file summarises the 13 other files in the folder for a reader who is not a biologist. Each technical term is explained the first time it appears. For details and all sources, follow the links to the specialised files.

## 1. What is aging?

Aging is the **progressive loss of the body's capacities over time**. Tissues repair less well, organs work less well, and the risk of falling ill or dying increases. In adult humans, this risk roughly doubles every 8 years.

Aging is often distinguished from diseases: aging affects everyone, whereas a disease affects only part of the population. But this boundary is blurry. Most major diseases (cancer, heart disease, Alzheimer's, type 2 diabetes) become much more frequent with age, because they rest on the same underlying mechanisms. Hence the central idea of **geroscience** (the study of the links between aging and diseases): acting on aging itself could delay several diseases at once ([Kennedy 2014](https://doi.org/10.1016/j.cell.2014.10.039)).

Why do we age? Evolutionary theories give a general answer: natural selection poorly protects what happens after reproductive age. Defects that only show up late in life are therefore not eliminated. Details in [framework.md](framework.md).

### How is it measured?

Calendar age says little about how much a body has aged. Researchers therefore use **biomarkers** (biological measurements that reflect a state):

- **Epigenetic clocks**: they read chemical marks placed on DNA (*methylation*, small tags that switch genes on or off). These marks change in a regular way with age. From a blood sample, a "biological age" is calculated. Examples: the Horvath clock ([2013](https://doi.org/10.1186/gb-2013-14-10-r115)), GrimAge, which predicts mortality ([Lu 2019](https://doi.org/10.18632/aging.101684)), and DunedinPACE, which measures a *pace* of aging ([Belsky 2022](https://doi.org/10.7554/eLife.73420)).
- **Frailty index**: a person's health problems are counted (diseases, difficulties in daily life). The higher the total, the more the person has "aged" ([Mitnitski 2001](https://doi.org/10.1100/tsw.2001.58)).
- **Functional measures**: walking speed, grip strength, breathing capacity.

Important limit: none of these measures is yet recognised as a sufficient criterion to prove that a treatment slows aging. A drug that lowers a clock does not necessarily lengthen life.

## 2. The 12 "hallmarks" of aging

In 2013, then in 2023, a team led by Carlos López-Otín listed the **12 biological characteristics** (*hallmarks*) found in every organism that ages ([López-Otín 2023](https://doi.org/10.1016/j.cell.2022.11.001)). To be on the list, a mechanism must appear with age, and worsening it must accelerate aging while correcting it must slow it down (at least in animals).

These 12 characteristics fall into three groups.

### Group 1 — The initial damage (primary causes)

This is damage that builds up in cells over time.

- **[01 — Genomic instability](01-genomic-instability.md)** : DNA, the "instruction manual" of every cell, suffers thousands of lesions every day (UV rays, by-products of respiration, copying errors). Repair systems let a few slip through, and they accumulate. Result: cells that work poorly or become cancerous.
- **[02 — Telomere attrition](02-telomere-attrition.md)** : telomeres are the protective caps at the ends of chromosomes, a bit like the plastic tip of a shoelace. They shorten each time the cell divides. When too short, they trigger the permanent arrest of the cell.
- **[03 — Epigenetic alterations](03-epigenetic-alterations.md)** : epigenetics is the set of settings that decide which genes are active, without changing the DNA itself. With age, these settings drift: genes switch on or off at the wrong time. This is also what epigenetic clocks read.
- **[04 — Loss of proteostasis](04-loss-of-proteostasis.md)** : proteins are the small machines of the cell. They must be correctly folded to work. With age, quality control weakens and misfolded proteins clump together, as in Alzheimer's or Parkinson's.
- **[05 — Disabled macroautophagy](05-disabled-macroautophagy.md)** : autophagy ("self-eating") is the cell's recycling service: it wraps up waste and worn-out parts to destroy them. This service slows down with age and waste builds up.

### Group 2 — Responses that become harmful (antagonistic)

These mechanisms first protect the cell against damage, then become harmful when they last too long.

- **[06 — Deregulated nutrient sensing](06-deregulated-nutrient-sensing.md)** : the cell has sensors that tell it whether there is food (insulin, mTOR) or a shortage (AMPK, sirtuins). Plenty = growth; shortage = maintenance and repair. With age, these sensors stay set to "plenty" too often. This is the best-studied hallmark: eating less or blocking mTOR lengthens the life of many animals.
- **[07 — Mitochondrial dysfunction](07-mitochondrial-dysfunction.md)** : mitochondria are the power plants of the cell. With age, they produce less energy and more harmful chemical waste (free radicals).
- **[08 — Cellular senescence](08-cellular-senescence.md)** : a senescent cell is a damaged cell that has stopped dividing but refuses to die. Useful in the short term (it prevents a cancer), it becomes a nuisance as it accumulates, because it sends out inflammatory signals that damage its neighbours.

### Group 3 — The consequences at the scale of the body (integrative)

When the damage from the first two groups exceeds what the body can compensate for, tissues and organs suffer.

- **[09 — Stem cell exhaustion](09-stem-cell-exhaustion.md)** : stem cells are the reserves that renew tissues (blood, skin, muscle, gut). They become fewer or less effective, so tissues repair less well.
- **[10 — Altered intercellular communication](10-altered-intercellular-communication.md)** : cells and organs talk to each other through hormones and molecules circulating in the blood. With age, these messages get scrambled. Experiments in which the circulation of a young mouse and an old mouse are joined show that young blood can rejuvenate some tissues.
- **[11 — Chronic inflammation](11-chronic-inflammation.md)** : inflammation is the body's defence reaction (redness, swelling after an injury). With age, a weak but permanent inflammation sets in, called "inflammaging", which quietly wears tissues down.
- **[12 — Dysbiosis](12-dysbiosis.md)** : the gut microbiota is the set of billions of bacteria that live in our gut. Its composition falls out of balance with age (this is dysbiosis), which promotes inflammation.

### Everything is linked

These 12 mechanisms are not independent. Each file contains a table of its links with the 11 others. Examples: worn-out mitochondria (07) must be removed by autophagy (05); senescent cells (08) sustain inflammation (11); the microbiota (12) produces molecules that act on autophagy. This is why a single intervention can act on several hallmarks at once, and why it is hard to know which one is the cause and which one the consequence.

## 3. The most solid interventions

"Level of evidence" indicates how far research has gone: **preclinical** = in animals or in the laboratory; **observational** = people were observed without being given a treatment (risk of confounding); **randomised trial** = who receives the treatment and who receives a placebo was decided by drawing lots, the most reliable method. **Phase 1** trials test safety on a few people, **phase 2** looks for a first effect, **phase 3** confirms on a large number.

To date, **no treatment has been shown in humans to slow aging as a whole.**

| Intervention | What it targets | Level of evidence | What is known |
|---|---|---|---|
| Caloric restriction (eating ~10-25 % less without deficiency) | Nutrient sensors (06) | Solid preclinical (including monkeys); human randomised trial CALERIE (phase 2, 2 years) | Lengthens the life of many animals. In humans: improved cardiac risk factors ([Kraus 2019](https://doi.org/10.1016/S2213-8587(19)30151-2)), DunedinPACE pace of aging slowed by 2-3 % ([Waziry 2023](https://doi.org/10.1038/s43587-022-00357-y)) |
| Rapamycin (drug that blocks mTOR) | Nutrient sensors (06), autophagy (05) | Very solid preclinical; human phase 2 trials | Lengthens the life of mice, even when started late ([Harrison 2009](https://doi.org/10.1038/nature08221)). In humans, well tolerated at low dose, modest effects ([Moel 2025](https://doi.org/10.18632/aging.206235)) |
| Metformin (diabetes drug) | Nutrient sensors (06) | Observational data; large TAME trial designed but not launched | Diabetics on metformin seem to live as long as non-diabetics, but these data may be biased ([Barzilai 2016](https://doi.org/10.1016/j.cmet.2016.05.011)). It reduces the benefits of exercise in older people ([Konopka 2018](https://doi.org/10.1111/acel.12880)) |
| Senolytics (dasatinib + quercetin, fisetin): drugs that kill senescent cells | Senescence (08) | Solid preclinical; human phase 1-2 trials | In old mice, better physical fitness and longer survival ([Zhu 2015](https://doi.org/10.1111/acel.12344)). In humans, they do remove senescent cells ([Hickson 2019](https://doi.org/10.1016/j.ebiom.2019.08.069)) but with no net benefit on bone ([Farr 2024](https://doi.org/10.1038/s41591-024-03096-2)). One candidate (UBX0101) failed in phase 2 |
| NAD+ precursors (NMN, NR): supplements that raise a key molecule of cellular energy | Mitochondria (07), DNA repair (01) | Solid preclinical; human phase 1-2 trials | In mice, they lessen several age-related declines ([Mills 2016](https://doi.org/10.1016/j.cmet.2016.09.013)). In humans, safe and raise NAD+ ([Martens 2018](https://doi.org/10.1038/s41467-018-03421-7)), with no demonstrated clinical benefit |
| Spermidine (molecule found in wheat germ and aged cheese) | Autophagy (05) | Preclinical; human observational; one randomised trial | Lengthens the life of yeast, worms and flies, protects the mouse heart ([Eisenberg 2016](https://doi.org/10.1038/nm.4222)) |
| Urolithin A (produced by gut bacteria from pomegranate and walnuts) | Mitochondria (07), autophagy (05) | Preclinical; human phase 1-2 trials | Safe; improved mitochondrial signature ([Andreux 2019](https://doi.org/10.1038/s42255-019-0073-4)); improved muscle endurance, but primary walking endpoint not met ([Liu 2022](https://doi.org/10.1001/jamanetworkopen.2021.44279)) |
| Physical activity | Almost all hallmarks | Many observational studies and randomised trials | The best-proven intervention for health in old age (files 07, 09, 11) |

## 4. The big open questions

- **What comes first?** It is not known which of the 12 hallmarks triggers the others, nor whether there is a single cause.
- **How can one prove that a treatment slows aging in humans?** Waiting for participants to die would take decades. Reliable biomarkers are needed, and none is yet validated for this use.
- **Does what works in mice work in humans?** Mice live 2-3 years; many results do not carry over.
- **At what age should one act, and for how long?** Some treatments (short courses of rapamycin, intermittent senolytics) seem effective without continuous intake.
- **What side effects?** Blocking growth or inflammation can weaken defences or wound healing. The benefit/risk balance in a healthy person remains unknown.
- **Can we repair rather than slow down?** Leads such as partial reprogramming of cells (bringing them back to a younger state without making them lose their identity) are promising in animals but still far from humans.

Each file ends with its own "Open questions" section.

## 5. Limits of this map

**What it is.** A synthesis written by agents from the published scientific literature, in October 2026. It is not medical advice. None of the interventions cited should be taken without medical advice.

**Quality control carried out (2026-10-19).**
- Consistency: the names of the 12 hallmarks were harmonised across files (variants corrected for 04, 05 and 10). The levels of evidence of the interventions cited in several files (rapamycin, metformin, senolytics, NAD+, urolithin A, caloric restriction) are consistent with each other.
- Sources: 39 DOIs (unique identifiers of scientific articles) drawn at random, 3 per file, were checked against the Crossref database (author, year, title). All 39 match the citation.

**Unverified points, marked [to verify] in the files:**
- [framework.md](framework.md): Strehler 1962 (*Time, Cells and Aging*) and Medawar 1952 (*An Unsolved Problem of Biology*): two classic books without a DOI, not verifiable online.
- [02](02-telomere-attrition.md): change in methylation of regions near telomeres in humans; link between telomeres and dysbiosis.
- [03](03-epigenetic-alterations.md): link between microbiota metabolites and epigenetic aging; absence of effect of NAD+ precursors on clocks in humans.
- [06](06-deregulated-nutrient-sensing.md): exact status of the TAME trial (no recent primary source).

**Fundamental limits.**
- The 12-hallmark framework is an organising tool, not an established truth; other classifications exist (see [framework.md](framework.md)).
- Most results come from laboratory animals.
- The field moves fast: ongoing trials may change the conclusions of the table above.

## Key sources

- López-Otín C, et al. Hallmarks of aging: an expanding universe. *Cell* 2023. [doi:10.1016/j.cell.2022.11.001](https://doi.org/10.1016/j.cell.2022.11.001)
- Kennedy BK, et al. Geroscience: linking aging to chronic disease. *Cell* 2014. [doi:10.1016/j.cell.2014.10.039](https://doi.org/10.1016/j.cell.2014.10.039)
- Harrison DE, et al. Rapamycin fed late in life extends lifespan in genetically heterogeneous mice. *Nature* 2009. [doi:10.1038/nature08221](https://doi.org/10.1038/nature08221)
- Barzilai N, et al. Metformin as a tool to target aging. *Cell Metab* 2016. [doi:10.1016/j.cmet.2016.05.011](https://doi.org/10.1016/j.cmet.2016.05.011)
- Waziry R, et al. Effect of long-term caloric restriction on DNA methylation measures of biological aging. *Nat Aging* 2023. [doi:10.1038/s43587-022-00357-y](https://doi.org/10.1038/s43587-022-00357-y)

The other references are in each file, section 7.
