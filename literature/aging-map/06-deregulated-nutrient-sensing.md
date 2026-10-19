# 06 — Deregulated nutrient sensing

> "Antagonistic" hallmark (López-Otín et al., *Cell* 2023, doi:10.1016/j.cell.2022.11.001). In plain terms: the sensors that tell the cell "there is food, grow" or "it is a famine, save and repair" stay stuck too long in "growth" mode.

## 1. Definition

Cells constantly measure the available energy and nutrients through four major networks: **IIS** (insulin / IGF-1), **mTOR** (amino acids, growth), **AMPK** (energy shortage) and **sirtuins** (NAD+ level). With age, the balance tilts towards the anabolic pathways (high IIS, mTOR) and away from the maintenance pathways (AMPK, sirtuins). Slowing growth and activating maintenance extends life in every model species tested.

## 2. Mechanisms and sources

**GH / insulin / IGF-1 axis (IIS).** This is the first longevity pathway discovered: a single mutated gene (*daf-2*, insulin/IGF receptor) doubles the lifespan of the worm *C. elegans* (Kenyon 1993, doi:10.1038/366461a0). The effect works through activation of the transcription factor FOXO (DAF-16 in the worm), which switches on stress resistance genes. In humans:
- IGF-1 receptor variants that reduce its activity are more frequent in Ashkenazi centenarians (Suh 2008, doi:10.1073/pnas.0705467105);
- the **FOXO3A** genotype is one of the most replicated genetic associations with longevity (Willcox 2008, doi:10.1073/pnas.0801030105);
- people with Laron syndrome (deficient growth hormone receptor, very low IGF-1) develop very few cancers and little diabetes (Guevara-Aguirre 2011, doi:10.1126/scitranslmed.3001845).

Paradox: during normal aging, GH and IGF-1 *fall*. This looks like a defensive response of the body, but one that becomes harmful when pushed too far (frailty, sarcopenia).

**mTOR.** mTOR is a kinase that works in two complexes. **mTORC1** senses amino acids (especially leucine), insulin and energy; it stimulates protein and lipid synthesis and slows autophagy (Saxton & Sabatini 2017, doi:10.1016/j.cell.2017.02.004). **mTORC2** mainly serves insulin signalling and survival. If mTORC1 stays permanently active in older people, autophagy is blocked, damaged proteins accumulate and the inflammatory secretion of senescent cells is amplified. The data showing this:
- deleting S6K1, the effector of mTORC1, extends the life of female mice (Selman 2009, doi:10.1126/science.1177221);
- the adverse effects of rapamycin on glucose come from the "collateral" inhibition of mTORC2, which can be separated from the longevity gain (Lamming 2012, doi:10.1126/science.1215135).

**AMPK.** This is the "low fuel" sensor (high AMP/ATP ratio). It inhibits mTORC1, activates autophagy and the formation of new mitochondria (via PGC-1α). Its capacity to be activated decreases with age. AMPK is the indirect target of metformin and exercise.

**Sirtuins.** These are NAD+-dependent enzymes that remove acetyl groups from proteins. They link metabolic state to DNA repair, the epigenome and mitochondrial function. In male mice, overexpressing **SIRT6** extends life (Kanfi 2012, doi:10.1038/nature10815). As NAD+ falls with age, sirtuins run slowly (see hallmark 07). The promises of resveratrol as a "sirtuin activator" were not confirmed in humans.

**Environmental causes.** Overeating, excess protein and sugar, a sedentary lifestyle and obesity keep IIS and mTOR in "growth" mode. In the NHANES cohort, a high protein intake between 50 and 65 is associated with higher IGF-1 and more cancers and deaths, with a reversal after 65 (Levine 2014, doi:10.1016/j.cmet.2014.02.006; observational study, so to be interpreted with caution). Diet-longevity synthesis: Fontana & Partridge 2015 (doi:10.1016/j.cell.2015.02.020).

## 3. Symptoms and clinical manifestations

- **Insulin resistance, type 2 diabetes, metabolic syndrome**: this is the most direct clinical expression. Type 2 diabetes accelerates almost every aspect of aging (cardiovascular, kidney and cognitive diseases).
- **Cancer**: excess IIS/mTOR promotes proliferation. The low IGF-1 of Laron patients protects them (Guevara-Aguirre 2011).
- **Sarcopenia and frailty**: conversely, a blunted anabolic response ("anabolic resistance" of old muscle) leads to loss of muscle mass. The problem is not "too much" or "too little" signal, it is the loss of flexibility.
- **Immunosenescence**: excessive mTOR activity in old immune cells weakens the response to vaccines. Low-dose mTOR inhibitors improve this response (Mannick 2014, doi:10.1126/scitranslmed.3009892).
- **Human genetic evidence**: IGF1R variants in centenarians, FOXO3A, Laron syndrome (see §2).

## 4. Interactions with the other hallmarks


- **Disabled macroautophagy (05)**: active mTORC1 blocks autophagy, AMPK stimulates it. This is the most direct link, and rapamycin acts largely through this route.
- **Loss of proteostasis (04)**: mTORC1 pushes the cell to make proteins faster than it can control their quality.
- **Mitochondrial dysfunction (07)**: AMPK/PGC-1α and sirtuins (SIRT1, SIRT3) control the formation and quality of mitochondria. The fall in NAD+ links the two hallmarks.
- **Cellular senescence (08)**: mTOR regulates the translation of IL-1α and therefore the SASP (inflammatory secretion of senescent cells); rapamycin reduces it (Laberge 2015, doi:10.1038/ncb3195).
- **Epigenetic alterations (03)**: sirtuins (SIRT1, SIRT6) are histone deacetylases, and metabolism supplies the substrates of epigenetic marks (acetyl-CoA, NAD+, SAM).
- **Genomic instability (01)**: SIRT6 takes part in the repair of DNA breaks.
- **Stem cell exhaustion (09)**: hyperactive mTOR exhausts haematopoietic stem cells.
- **Chronic inflammation (11)** and **altered intercellular communication (10)**: hyperinsulinaemia, adipokines, systemic IGF-1 signalling.
- **Dysbiosis (12)**: the microbiota modulates insulin sensitivity and the production of short-chain fatty acids, which signal through AMPK.

## 5. Existing or investigational remedies

| Intervention | Target | Level of evidence | Key result | Source |
|---|---|---|---|---|
| Rapamycin (sirolimus), from 20 months of age in mice | mTORC1 | Robust preclinical (ITP, 3 sites) | Lifespan +9 % (males) and +14 % (females), even when started late | Harrison 2009, doi:10.1038/nature08221 |
| Transient rapamycin (3 months, middle age) | mTORC1 | Preclinical | Up to +60 % remaining life expectancy, lasting effect | Bitto 2016, doi:10.7554/eLife.16351 |
| Low-dose everolimus (RAD001), 6 weeks | mTORC1 | Human phase 2 trial (n≈218, ≥65 years) | Flu vaccine response increased by about 20 % | Mannick 2014, doi:10.1126/scitranslmed.3009892 |
| Low-dose RAD001 + BEZ235 | mTORC1 (+ PI3K) | Human phase 2a trial | Fewer respiratory infections over one year | Mannick 2018, doi:10.1126/scitranslmed.aaq1564 |
| RTB101 (dactolisib) | TORC1 | Human phase 2b then phase 3 trial | Positive phase 2b; phase 3 misses its primary endpoint ("clinically symptomatic" respiratory infections) | Mannick 2021, doi:10.1016/S2666-7568(21)00062-3 |
| Rapamycin 5-10 mg/week, 48 weeks (PEARL) | mTORC1 | Human phase 2 trial (n=114) | Well tolerated; modest signal in women at 10 mg (lean mass, pain); compounded formulation poorly absorbed | Moel 2025, doi:10.18632/aging.206235 |
| Metformin | AMPK / mitochondrial complex I | Approved (diabetes); TAME trial designed but not launched: no registration on ClinicalTrials.gov (search of 19/10/2026), AFAR page offline; several 2026 secondary sources say it is still unfunded [to verify]: no primary source | Diabetics on metformin: observed mortality similar to or lower than non-diabetics (observational data, possible biases) | Bannister 2014, doi:10.1111/dom.12354; Campbell 2017, doi:10.1016/j.arr.2017.08.003; rationale for TAME: Barzilai 2016, doi:10.1016/j.cmet.2016.05.011 |
| Metformin + exercise | — | Human trial (RCT) | Metformin **blunts** the mitochondrial gains of training in older people | Konopka 2018, doi:10.1111/acel.12880 |
| Caloric restriction (≈12 % achieved, 2 years, CALERIE) | IIS, mTOR, AMPK, sirtuins | Human phase 2 trial (non-obese) | Improved cardiometabolic risk factors | Kraus 2019, doi:10.1016/S2213-8587(19)30151-2 |
| CALERIE, biological clocks | — | Secondary analysis of an RCT | Slower pace of aging (DunedinPACE); little effect on GrimAge-type clocks | Belsky 2017, doi:10.1093/gerona/glx096; Waziry 2023, doi:10.1038/s43587-022-00357-y |
| Caloric restriction in primates | — | Preclinical (rhesus monkeys) | Fewer age-related diseases, improved survival; NIA/Wisconsin results reconciled | Colman 2009, doi:10.1126/science.1173635; Mattison 2017, doi:10.1038/ncomms14063 |
| Acarbose | Post-meal glucose spike | Preclinical (ITP); approved for diabetes | Increased lifespan, especially in males | Harrison 2014, doi:10.1111/acel.12170 |
| Protein / methionine restriction | mTOR, IGF-1 | Human observational; preclinical | Lower IGF-1, lower mortality between 50 and 65 | Levine 2014, doi:10.1016/j.cmet.2014.02.006 |
| Genetic inhibition of S6K1 | mTORC1 → S6K1 | Preclinical | Extends life (females) | Selman 2009, doi:10.1126/science.1177221 |
| SIRT6 overexpression | Sirtuin | Preclinical | Extends life (males) | Kanfi 2012, doi:10.1038/nature10815 |
| Physical exercise | AMPK, insulin sensitivity | Recommended (epidemiological evidence and RCTs) | Activates AMPK and PGC-1α, restores insulin sensitivity | Review: Fontana 2015, doi:10.1016/j.cell.2015.02.020 |

For the general pharmacological context (rapalogs, selective mTORC1 inhibitors): Bonkowski & Sinclair 2016, doi:10.1038/nrm.2016.93.

## 6. Open questions

1. **Dose and schedule for rapamycin in humans**: is intermittent dosing needed to spare mTORC2? There is still no reliable biomarker of mTORC1 inhibition in tissues. Selective mTORC1 inhibitors remain to be validated clinically.
2. **Metformin**: a real anti-aging effect, or an artefact of observational studies ("survival" and comparator biases)? TAME has still not been launched, and metformin hinders adaptation to exercise.
3. **The IGF-1 paradox**: does low IGF-1 protect or weaken older people? The effect seems to depend on age, sex and tissue.
4. **Sex differences**: many interventions (acarbose, 17α-estradiol, S6K1, SIRT6) act very differently in males and females.
5. **Caloric restriction vs intermittent fasting vs protein composition**: what really matters? Long-term adherence remains a major obstacle.
6. **Sirtuins and NAD+**: activators (resveratrol, STACs) have not lived up to their promise; NAD+ precursors are covered in hallmark 07.

## 7. References

- López-Otín C et al. Hallmarks of aging: An expanding universe. *Cell* 2023. doi:10.1016/j.cell.2022.11.001
- Kenyon C et al. A C. elegans mutant that lives twice as long as wild type. *Nature* 1993. doi:10.1038/366461a0
- Suh Y et al. Functionally significant IGF-I receptor mutations in centenarians. *PNAS* 2008. doi:10.1073/pnas.0705467105
- Willcox BJ et al. FOXO3A genotype is strongly associated with human longevity. *PNAS* 2008. doi:10.1073/pnas.0801030105
- Guevara-Aguirre J et al. Growth hormone receptor deficiency… *Sci Transl Med* 2011. doi:10.1126/scitranslmed.3001845
- Saxton RA, Sabatini DM. mTOR signaling in growth, metabolism, and disease. *Cell* 2017. doi:10.1016/j.cell.2017.02.004
- Selman C et al. Ribosomal protein S6 kinase 1 signaling regulates mammalian life span. *Science* 2009. doi:10.1126/science.1177221
- Lamming DW et al. Rapamycin-induced insulin resistance is mediated by mTORC2 loss… *Science* 2012. doi:10.1126/science.1215135
- Kanfi Y et al. The sirtuin SIRT6 regulates lifespan in male mice. *Nature* 2012. doi:10.1038/nature10815
- Levine ME et al. Low protein intake is associated with a major reduction in IGF-1, cancer, and overall mortality… *Cell Metab* 2014. doi:10.1016/j.cmet.2014.02.006
- Fontana L, Partridge L. Promoting health and longevity through diet. *Cell* 2015. doi:10.1016/j.cell.2015.02.020
- Laberge RM et al. MTOR regulates the pro-tumorigenic SASP… *Nat Cell Biol* 2015. doi:10.1038/ncb3195
- Harrison DE et al. Rapamycin fed late in life extends lifespan… *Nature* 2009. doi:10.1038/nature08221
- Bitto A et al. Transient rapamycin treatment can increase lifespan and healthspan in middle-aged mice. *eLife* 2016. doi:10.7554/eLife.16351
- Mannick JB et al. mTOR inhibition improves immune function in the elderly. *Sci Transl Med* 2014. doi:10.1126/scitranslmed.3009892
- Mannick JB et al. TORC1 inhibition enhances immune function and reduces infections in the elderly. *Sci Transl Med* 2018. doi:10.1126/scitranslmed.aaq1564
- Mannick JB et al. Targeting the biology of ageing with mTOR inhibitors… phase 2b and phase 3 trials. *Lancet Healthy Longev* 2021. doi:10.1016/S2666-7568(21)00062-3
- Moel M et al. Influence of rapamycin on safety and healthspan metrics after one year: PEARL trial results. *Aging* 2025. doi:10.18632/aging.206235
- Barzilai N et al. Metformin as a tool to target aging. *Cell Metab* 2016. doi:10.1016/j.cmet.2016.05.011
- Bannister CA et al. Can people with type 2 diabetes live longer than those without? *Diabetes Obes Metab* 2014. doi:10.1111/dom.12354
- Campbell JM et al. Metformin reduces all-cause mortality and diseases of ageing… *Ageing Res Rev* 2017. doi:10.1016/j.arr.2017.08.003
- Konopka AR et al. Metformin inhibits mitochondrial adaptations to aerobic exercise training in older adults. *Aging Cell* 2019. doi:10.1111/acel.12880
- Kraus WE et al. 2 years of calorie restriction and cardiometabolic risk (CALERIE). *Lancet Diabetes Endocrinol* 2019. doi:10.1016/S2213-8587(19)30151-2
- Belsky DW et al. Change in the rate of biological aging in response to caloric restriction. *J Gerontol A* 2017. doi:10.1093/gerona/glx096
- Waziry R et al. Effect of long-term caloric restriction on DNA methylation measures of biological aging. *Nat Aging* 2023. doi:10.1038/s43587-022-00357-y
- Colman RJ et al. Caloric restriction delays disease onset and mortality in rhesus monkeys. *Science* 2009. doi:10.1126/science.1173635
- Mattison JA et al. Caloric restriction improves health and survival of rhesus monkeys. *Nat Commun* 2017. doi:10.1038/ncomms14063
- Harrison DE et al. Acarbose, 17-α-estradiol, and NDGA extend mouse lifespan preferentially in males. *Aging Cell* 2014. doi:10.1111/acel.12170
- Bonkowski MS, Sinclair DA. Slowing ageing by design: the rise of NAD+ and sirtuin-activating compounds. *Nat Rev Mol Cell Biol* 2016. doi:10.1038/nrm.2016.93
