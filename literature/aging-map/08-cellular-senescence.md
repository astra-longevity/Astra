# 08 — Cellular senescence

> "Antagonistic" hallmark (López-Otín et al., *Cell* 2023, doi:10.1016/j.cell.2022.11.001). In plain terms: a damaged cell can "retire" instead of dividing, which protects it from cancer. But these "zombie" cells accumulate with age and poison their neighbours with inflammatory signals.

## 1. Definition

Senescence is a lasting halt of cell division, triggered by stress: telomeres that are too short, DNA damage, an activated oncogene, mitochondrial stress. The first to observe it was Hayflick, in human cells in culture that stopped dividing after a finite number of divisions (Hayflick 1965, doi:10.1016/0014-4827(65)90211-9). Senescent cells do not die: they resist apoptosis and secrete an inflammatory cocktail, the **SASP** (*senescence-associated secretory phenotype*). Useful in the short term (wound healing, tumour suppression), senescence becomes harmful when the cells accumulate and are no longer cleared (Gorgoulis 2019, doi:10.1016/j.cell.2019.10.005).

## 2. Mechanisms and sources

**Two main "brakes".** The **p53 → p21** pathway reacts quickly to DNA damage; the **p16^INK4a → Rb** pathway maintains the arrest over time. p16^INK4a serves as a marker: in blood T lymphocytes, its level rises exponentially with age and with smoking and a sedentary lifestyle (Liu 2009, doi:10.1111/j.1474-9726.2009.00489.x).

**Triggers.** Telomere shortening, DNA damage (radiation, chemotherapy), oncogene activation, ROS and mitochondrial dysfunction, protein stress. Review: Muñoz-Espín & Serrano 2014, doi:10.1038/nrm3823.

**The SASP, a double-edged sword.** It is a mixture of cytokines (IL-6, IL-8, IL-1α), chemokines, growth factors and enzymes that degrade the matrix (Coppé 2008, doi:10.1371/journal.pbio.0060301). It is driven by NF-κB, the inflammasome and IL-1 (Acosta 2013, doi:10.1038/ncb2784), and partly by mTOR (Laberge 2015, doi:10.1038/ncb3195). The SASP can make neighbouring cells senescent: this is called "paracrine" senescence.

**Beneficial roles.** Senescence blocks precancerous cells, helps wound healing (transient senescence via PDGF-AA, Demaria 2014, doi:10.1016/j.devcel.2014.11.012) and takes part in embryonic development. The problem comes from chronic accumulation, when the immune system no longer clears these cells.

**Causal evidence in mice.** Genetically eliminating p16-positive cells (INK-ATTAC model):
- delays age-related pathologies in mice with accelerated aging (Baker 2011, doi:10.1038/nature10600);
- in normal mice, extends median lifespan by about 25 % and protects kidneys, heart and fat (Baker 2016, doi:10.1038/nature16932).
Transplanting a few senescent cells into a young mouse is enough to cause loss of physical function (Xu 2018, doi:10.1038/s41591-018-0092-9).

## 3. Symptoms and clinical manifestations

Senescent cells accumulate in the tissues associated with the following diseases (Childs 2017, doi:10.1038/nrd.2017.116; Gasek 2021, doi:10.1038/s43587-021-00121-8):
- **Idiopathic pulmonary fibrosis**: many senescent cells; removing them reduces fibrosis in mice (Schafer 2017, doi:10.1038/ncomms14532).
- **Osteoarthritis**: local elimination of senescent cells slows post-traumatic osteoarthritis in mice (Jeon 2017, doi:10.1038/nm.4324).
- **Osteoporosis**: removing them prevents age-related bone loss in mice (Farr 2017, doi:10.1038/nm.4385).
- **Atherosclerosis, heart failure, type 2 diabetes, chronic kidney disease, fatty liver**.
- **Neurodegenerative diseases** (Alzheimer's): senescent glial cells.
- **Side effects of chemotherapy** and frailty of cancer survivors.
- **Measurement in humans**: a gene set ("SenMayo") makes it possible to detect senescent cells in transcriptomic data and rises with age in several tissues (Saul 2022, doi:10.1038/s41467-022-32552-1).

## 4. Interactions with the other hallmarks


- **Chronic inflammation (11)**: the SASP is one of its main drivers (*inflammaging*).
- **Telomere attrition (02)** and **genomic instability (01)**: these are the two classic triggers of senescence.
- **Mitochondrial dysfunction (07)**: it induces senescence with a particular SASP; mitochondria are required for the senescent phenotype (Correia-Melo 2016, doi:10.15252/embj.201592862).
- **Deregulated nutrient sensing (06)**: mTOR amplifies the SASP; rapamycin reduces it (Laberge 2015).
- **Stem cell exhaustion (09)**: senescence of stem cells and SASP of the "niche"; navitoclax rejuvenates old haematopoietic stem cells (Chang 2016, doi:10.1038/nm.4010).
- **Altered intercellular communication (10)**: paracrine senescence, distant effects through the blood.
- **Epigenetic alterations (03)**: chromatin remodelling (heterochromatin foci).
- **Disabled macroautophagy (05)** and **loss of proteostasis (04)**: senescent cells use them to sustain their secretion.
- **Dysbiosis (12)**: bacterial products fuel inflammation and the senescent burden (still an indirect link).

## 5. Existing or investigational remedies

Two families: **senolytics** selectively kill senescent cells (intermittent dosing, "hit and run"); **senomorphics** block their SASP without killing them.

| Intervention | Target | Level of evidence | Key result | Source |
|---|---|---|---|---|
| **Dasatinib + quercetin (D+Q)** | Anti-apoptotic pathways (SCAPs) | Preclinical | Discovered by transcriptome analysis; in old mice, improves physical function and extends remaining survival by about 36 % | Zhu 2015, doi:10.1111/acel.12344; Xu 2018, doi:10.1038/s41591-018-0092-9 |
| D+Q in pulmonary fibrosis | — | Open-label human phase 1 trial (n=14) | Feasible; walking distance and speed improved; no change in lung function | Justice 2019, doi:10.1016/j.ebiom.2018.12.052 |
| D+Q in diabetic kidney disease | — | Human phase 1 trial (n=9) | Fewer senescent cells in fat and skin 11 days after 3 days of treatment: first direct evidence in humans | Hickson 2019, doi:10.1016/j.ebiom.2019.08.069 |
| D+Q in postmenopausal osteoporosis | — | Human phase 2 trial (RCT, n=60) | No overall effect on bone; modest benefit in women with a high senescent burden | Farr 2024, doi:10.1038/s41591-024-03096-2 |
| D+Q in mild Alzheimer's | — | Human phase 1 trial (n=5) | Dasatinib reaches the cerebrospinal fluid, quercetin does not; acceptable tolerability | Gonzales 2023, doi:10.1038/s41591-023-02543-w |
| **Fisetin** (flavonoid) | Senolytic | Preclinical; human phase 1-2 trials: knee osteoarthritis completed (n=75, NCT04210986, results posted on ClinicalTrials.gov, not yet published); AFFIRM frailty trials ongoing (NCT03430037, NCT03675724, planned end Nov. 2026) | The most potent of the flavonoids tested; extends mouse life even when started late. No efficacy shown in humans to date | Yousefzadeh 2018, doi:10.1016/j.ebiom.2018.09.015; https://clinicaltrials.gov/study/NCT04210986; https://clinicaltrials.gov/study/NCT03430037; https://clinicaltrials.gov/study/NCT03675724 |
| **Navitoclax (ABT-263)** | BCL-2/BCL-xL | Preclinical (in oncology: human trials) | Rejuvenates old haematopoietic stem cells; toxic to platelets | Chang 2016, doi:10.1038/nm.4010 |
| **UBX0101** (knee injection) | p53/MDM2 interaction | Human phase 2 trial (n=183) | **Failure**: no better than placebo on pain at 12 weeks (August 2020); programme stopped | Unity Biotechnology press release, https://www.globenewswire.com/news-release/2020/08/17/2079116/0/en/unity-biotechnology-announces-12-week-data-from-ubx0101-phase-2-clinical-study-in-patients-with-painful-osteoarthritis-of-the-knee.html |
| **JAK inhibitors** (ruxolitinib) | SASP (senomorphic) | Preclinical; approved for other indications | Reduce the SASP, frailty and fat dysfunction in old mice | Xu 2015, doi:10.1073/pnas.1515386112 |
| **Rapamycin** | SASP via mTOR | Preclinical (for this effect) | Reduces IL-1α translation and the SASP | Laberge 2015, doi:10.1038/ncb3195 |
| **Senolytic CAR-T cells** (against uPAR) | Surface receptor uPAR | Preclinical | Eliminate senescent cells; liver fibrosis corrected in mice | Amor 2020, doi:10.1038/s41586-020-2403-9 |
| **Senolytic vaccine** (against GPNMB) | Surface antigen of senescent cells | Preclinical | Improves age-related phenotypes and extends the life of progeroid mice | Suda 2021, doi:10.1038/s43587-021-00151-2 |

Overview of clinical trials: Chaib 2022, doi:10.1038/s41591-022-01923-y; Kirkland & Tchkonia 2020, doi:10.1111/joim.13141.

## 6. Open questions

1. **No universal marker**: no single marker identifies a senescent cell. p16, p21, β-galactosidase and SenMayo overlap poorly depending on the tissue. A validated blood biomarker to select patients and track the effect is missing.
2. **Disappointing human results so far**: UBX0101 failed, and D+Q gives modest effects limited to subgroups (Farr 2024). Is this a problem of dose, tissue, patient selection or concept?
3. **Heterogeneity**: senescent cells differ from one tissue to another and do not share the same vulnerabilities. A universal senolytic is unlikely.
4. **Useful roles**: eliminating senescent cells can impair wound healing (Demaria 2014) or other physiological functions. What are the long-term effects?
5. **Senolytic or senomorphic?** Is it better to kill the cells or silence their SASP, and for which disease?
6. **Immunotherapies** (CAR-T, vaccines): promising, but cost, safety and duration of effect remain unknown in humans.

## 7. References

- López-Otín C et al. Hallmarks of aging: An expanding universe. *Cell* 2023. doi:10.1016/j.cell.2022.11.001
- Hayflick L. The limited in vitro lifetime of human diploid cell strains. *Exp Cell Res* 1965. doi:10.1016/0014-4827(65)90211-9
- Gorgoulis V et al. Cellular senescence: defining a path forward. *Cell* 2019. doi:10.1016/j.cell.2019.10.005
- Muñoz-Espín D, Serrano M. Cellular senescence: from physiology to pathology. *Nat Rev Mol Cell Biol* 2014. doi:10.1038/nrm3823
- Liu Y et al. Expression of p16INK4a in peripheral blood T-cells is a biomarker of human aging. *Aging Cell* 2009. doi:10.1111/j.1474-9726.2009.00489.x
- Coppé JP et al. Senescence-associated secretory phenotypes reveal cell-nonautonomous functions… *PLoS Biol* 2008. doi:10.1371/journal.pbio.0060301
- Acosta JC et al. A complex secretory program orchestrated by the inflammasome controls paracrine senescence. *Nat Cell Biol* 2013. doi:10.1038/ncb2784
- Laberge RM et al. MTOR regulates the pro-tumorigenic senescence-associated secretory phenotype… *Nat Cell Biol* 2015. doi:10.1038/ncb3195
- Demaria M et al. An essential role for senescent cells in optimal wound healing through secretion of PDGF-AA. *Dev Cell* 2014. doi:10.1016/j.devcel.2014.11.012
- Baker DJ et al. Clearance of p16Ink4a-positive senescent cells delays ageing-associated disorders. *Nature* 2011. doi:10.1038/nature10600
- Baker DJ et al. Naturally occurring p16Ink4a-positive cells shorten healthy lifespan. *Nature* 2016. doi:10.1038/nature16932
- Xu M et al. Senolytics improve physical function and increase lifespan in old age. *Nat Med* 2018. doi:10.1038/s41591-018-0092-9
- Childs BG et al. Senescent cells: an emerging target for diseases of ageing. *Nat Rev Drug Discov* 2017. doi:10.1038/nrd.2017.116
- Gasek NS et al. Strategies for targeting senescent cells in human disease. *Nat Aging* 2021. doi:10.1038/s43587-021-00121-8
- Schafer MJ et al. Cellular senescence mediates fibrotic pulmonary disease. *Nat Commun* 2017. doi:10.1038/ncomms14532
- Jeon OH et al. Local clearance of senescent cells attenuates the development of post-traumatic osteoarthritis. *Nat Med* 2017. doi:10.1038/nm.4324
- Farr JN et al. Targeting cellular senescence prevents age-related bone loss in mice. *Nat Med* 2017. doi:10.1038/nm.4385
- Saul D et al. A new gene set identifies senescent cells and predicts senescence-associated pathways across tissues. *Nat Commun* 2022. doi:10.1038/s41467-022-32552-1
- Correia-Melo C et al. Mitochondria are required for pro-ageing features of the senescent phenotype. *EMBO J* 2016. doi:10.15252/embj.201592862
- Chang J et al. Clearance of senescent cells by ABT263 rejuvenates aged hematopoietic stem cells in mice. *Nat Med* 2016. doi:10.1038/nm.4010
- Zhu Y et al. The Achilles' heel of senescent cells: from transcriptome to senolytic drugs. *Aging Cell* 2015. doi:10.1111/acel.12344
- Justice JN et al. Senolytics in idiopathic pulmonary fibrosis: results from a first-in-human, open-label, pilot study. *EBioMedicine* 2019. doi:10.1016/j.ebiom.2018.12.052
- Hickson LJ et al. Senolytics decrease senescent cells in humans: preliminary report from a clinical trial of dasatinib plus quercetin… *EBioMedicine* 2019. doi:10.1016/j.ebiom.2019.08.069
- Farr JN et al. Effects of intermittent senolytic therapy on bone metabolism in postmenopausal women: a phase 2 randomized controlled trial. *Nat Med* 2024. doi:10.1038/s41591-024-03096-2
- Gonzales MM et al. Senolytic therapy in mild Alzheimer's disease: a phase 1 feasibility trial. *Nat Med* 2023. doi:10.1038/s41591-023-02543-w
- Yousefzadeh MJ et al. Fisetin is a senotherapeutic that extends health and lifespan. *EBioMedicine* 2018. doi:10.1016/j.ebiom.2018.09.015
- Xu M et al. JAK inhibition alleviates the cellular senescence-associated secretory phenotype and frailty in old age. *PNAS* 2015. doi:10.1073/pnas.1515386112
- Amor C et al. Senolytic CAR T cells reverse senescence-associated pathologies. *Nature* 2020. doi:10.1038/s41586-020-2403-9
- Suda M et al. Senolytic vaccination improves normal and pathological age-related phenotypes and increases lifespan in progeroid mice. *Nat Aging* 2021. doi:10.1038/s43587-021-00151-2
- Chaib S, Tchkonia T, Kirkland JL. Cellular senescence and senolytics: the path to the clinic. *Nat Med* 2022. doi:10.1038/s41591-022-01923-y
- Kirkland JL, Tchkonia T. Senolytic drugs: from discovery to translation. *J Intern Med* 2020. doi:10.1111/joim.13141
- Unity Biotechnology. 12-week data from UBX0101 phase 2 study in painful knee osteoarthritis, 17 August 2020. https://www.globenewswire.com/news-release/2020/08/17/2079116/0/en/unity-biotechnology-announces-12-week-data-from-ubx0101-phase-2-clinical-study-in-patients-with-painful-osteoarthritis-of-the-knee.html
