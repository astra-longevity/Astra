# Conceptual framework of aging

This document defines the concepts shared across the map: what is called "aging", the major theories that explain it, the measurement tools and the organising frameworks (hallmarks and alternatives). The per-hallmark files (`01-…` to `12-…`) refer to it.

---

## 1. Biological definitions

### 1.1 Functional decline and mortality

Biological aging (senescence in the demographic sense) is a **progressive, generalised deterioration of physiological functions**. It leads to lower fertility, a loss of resilience to stress and a risk of death that increases with age. López-Otín et al. describe it as a "time-dependent progressive functional deterioration" that increases vulnerability to disease and death [1, 2].

In adult humans, mortality follows the **Gompertz law** fairly closely: the death rate grows exponentially with age and doubles roughly every 8 years in adulthood [22, 23]. The rhesus monkey has a doubling time close to that of humans; its shorter life is explained mainly by higher mortality at all ages [23]. This demographic signature serves as an operational definition in biogerontology: an "anti-aging" intervention must slow the Gompertz slope, not just lower its intercept (that is, reduce one specific cause of death).

### 1.2 Aging and disease

The relationship between aging and disease is at the centre of the current debate:

- **Geroscience position**: age is the main risk factor for almost all chronic diseases (cancer, cardiovascular and neurodegenerative diseases, type 2 diabetes). Acting on the mechanisms of aging should delay several diseases at once, which the "one disease, one treatment" approach cannot do [3].
- **Aging as a disease**: some authors want to treat it as a disease entity in its own right, with regulatory consequences (therapeutic indication, trial endpoints). The lack of a biomarker validated as a surrogate endpoint is the main obstacle [4].
- **Classic distinction**: aging is universal, intrinsic, progressive and deleterious (Strehler's criteria); a disease affects a subset of individuals [to verify — Strehler 1962, *Time, Cells and Aging*, no DOI]. This boundary fades once one accepts that age-related diseases are accelerated or localised expressions of the same processes [2, 5].

### 1.3 Cellular senescence and organismal senescence

The two meanings of the word must not be confused:

- **Cellular senescence**: a stable arrest of the cell cycle, together with a secretory phenotype (SASP), triggered by telomere shortening, DNA damage, oncogenes or various stresses. It is one of the 12 hallmarks [2] (covered in its own file).
- **Organismal senescence**: the overall decline described above. Cellular senescence contributes to it, but only in part.

---

## 2. Major theories

Theories fall into two families: **evolutionary** theories answer the *why* (why has natural selection not eliminated aging?), **mechanistic** theories the *how*. They are largely complementary.

### 2.1 Evolutionary theories

Shared premise: the **force of natural selection declines with age**. Because extrinsic mortality (predation, accidents, infections) eliminates most individuals before old age, late-acting genetic effects largely escape selection.

| Theory | Author(s) | Central idea | Testable prediction |
|---|---|---|---|
| **Mutation accumulation** | Medawar (1952) [to verify — *An Unsolved Problem of Biology*, lecture, no DOI] | Deleterious alleles with late effects accumulate by drift, because selection does not eliminate them | Additive genetic variance of mortality increasing with age |
| **Antagonistic pleiotropy** | Williams (1957) [6] | The same allele can be beneficial early (fertility, growth) and harmful late; selection favours it | Genetic trade-offs between early reproduction and longevity |
| **Disposable soma** (*disposable soma*) | Kirkwood (1977) [7] | Resources are limited and split between reproduction and somatic maintenance; optimal maintenance remains imperfect | Inverse link between reproductive investment and maintenance; plasticity according to extrinsic risk |

**Value**: these theories explain why aging exists, why longevity varies widely between species (bats, naked mole-rat, bowhead whale), and why some pro-growth pathways (IGF-1, mTOR) favour reproduction while shortening life.

**Limitations**: they do not identify a precise molecular mechanism. Some species with negligible or negative senescence complicate the idea of universal aging: a comparison of 46 species shows very diverse mortality and fertility trajectories, including mortality that is stable or decreasing with age [24].

### 2.2 Mechanistic theories

#### Free radicals and mitochondria
Harman (1956) attributes aging to the cumulative damage caused by free radicals produced by oxidative metabolism [8]. He later refined a mitochondrial version (mitochondria are both source and target of ROS). **Assessment**: antioxidant trials in humans have not extended life, and in mice knocking out some antioxidant enzymes does not always shorten it. ROS are now seen as hormetic **signals** at low doses rather than simple toxins. Mitochondrial dysfunction remains a hallmark in its own right [2].

#### Damage theories (stochastic)
Aging would result from the **random** accumulation of molecular damage (DNA, proteins, lipids) that repair compensates less and less well. López-Otín's "primary" hallmarks (genomic instability, telomere attrition, epigenetic alterations, loss of proteostasis, disabled macroautophagy) are its modern formulation [1, 2].

#### Hyperfunction / quasi-program (Blagosklonny)
Blagosklonny proposes that aging does not come mainly from accumulated damage, but from the **useless continuation, after development, of growth programs** (notably mTOR). This "quasi-program" produces cellular hyperfunction (hypertrophy, hypersecretion, pro-inflammatory signals) that causes age-related diseases [9]. **Value**: the theory predicted that rapamycin (an mTOR inhibitor) would be a geroprotector, which was confirmed in mice. It also fits with antagonistic pleiotropy. **Limitations**: it probably underestimates the share of real damage (somatic mutations, aggregates), and its demonstration in humans remains indirect.

#### Programmed vs stochastic
- **Programmed**: a genetic program, selected for its own sake (for example for the benefit of the group), would cause aging. This strong version is a minority view and hardly compatible with classical evolutionary theory.
- **Stochastic**: aging is an unselected side effect of damage accumulation.
- **Dominant intermediate position**: aging is not programmed in the adaptive sense, but it is **regulated** by conserved pathways (insulin/IGF-1, mTOR, AMPK, sirtuins). Mutations in these pathways extend life in yeast, worms, flies and mice [1, 2]. Blagosklonny's hyperfunction bridges the two positions [9].

#### Information theory (Sinclair)
The *Information Theory of Aging* holds that the main cause is the **loss of epigenetic information**: cell identity erodes because chromatin complexes are diverted to repairing DNA double-strand breaks. The "ICE" mouse model (*Inducible Changes to the Epigenome*) induces breaks without mutations and causes accelerated aging phenotypes as well as an advanced epigenetic clock. Partial reprogramming with OSK (Oct4, Sox2, Klf4) partly reverses these effects [10]. **Value**: the theory suggests that aging is partly **reversible** (via partial reprogramming) and gives epigenetic clocks a plausible causal role. **Limitations**: a single, controversial model (role of mutations, specificity of the phenotypes), and in vivo reprogramming carries a risk of teratoma.

---

## 3. Measures of biological age

Chronological age does not capture the diversity of trajectories. A **biomarker of aging** should ideally: (1) predict mortality and morbidity better than chronological age, (2) respond to interventions, (3) be reproducible, and (4) be validated in independent populations. Moqri et al. (*Biomarkers of Aging* consortium) propose a validation framework that distinguishes **predictive**, **intervention-responsive** and **causal** biomarkers [4].

### 3.1 Epigenetic clocks (DNA methylation)

| Clock | Reference | Training | What it measures | Comment |
|---|---|---|---|---|
| **Horvath** (multi-tissue) | Horvath 2013 [11] | Chronological age, 353 CpGs, ~50 tissues | Pan-tissue "epigenetic" age | 1st generation; weak prediction of mortality once age is accounted for |
| **Hannum** | Hannum 2013 [12] | Chronological age, 71 CpGs, blood | Blood epigenetic age | 1st generation |
| **PhenoAge** | Levine 2018 [13] | "Phenotypic age" (9 clinical biomarkers + age) | Morbidity / mortality | 2nd generation; much better prediction of events |
| **GrimAge** | Lu 2019 [14] | Mortality, via 7 DNAm surrogates of plasma proteins (including PAI-1, GDF15) and of cumulative smoking | Time to death | Among the best predictors of mortality; includes smoking |
| **DunedinPACE** | Belsky 2022 [15] | **Pace** of decline of 19 biomarkers over 20 years (Dunedin cohort, same birth year) | Speed of aging ("biological years per year") | 3rd generation; measures a rate, not a state; probes chosen for reliability |

**Cross-cutting limitations**:
- **Technical reliability**: technical replicates of the same sample can differ by several years (up to ~9 years for some clocks). Higgins-Chen et al. propose versions based on principal components ("PC clocks") that greatly reduce this noise [16].
- **Causality**: most clocks are correlational. It is not known whether the measured CpGs cause aging or are passive markers of it. The ICE model provides a causal argument, still partial [10].
- **Intervention validity**: an effect on a clock does not prove an effect on health. No clock is validated as a regulatory surrogate endpoint [4].
- **Confounding**: blood cell composition, ethnicity, smoking (explicitly built into GrimAge) and measurement platform.

### 3.2 Frailty index (*frailty index*)

The Rockwood and Mitnitski frailty index is calculated as the **proportion of deficits** present on a list (symptoms, signs, functional disabilities, biological abnormalities) [17]. A standard procedure (≥ 30 variables, deficits related to age and health, not saturating too early) allows it to be built from almost any database [18]. **Value**: it is simple, cheap, strongly predicts mortality and institutionalisation, and is clinically interpretable. **Limitations**: it is a downstream (phenotypic) measure, with no mechanistic information, not very sensitive to early stages and dependent on the chosen list of variables.

### 3.3 Proteomic and metabolomic biological age

- **Proteomics**: Argentieri et al. trained a clock on 2,897 plasma proteins (Olink) in ~45,000 UK Biobank participants, then validated it in Chinese (China Kadoorie Biobank) and Finnish (FinnGen) cohorts. The gap between proteomic age and chronological age predicts mortality and the incidence of 18 major chronic diseases [19]. Proteins are closer to function than methylation, and clocks can be built **per organ** from tissue-specific proteins. Oh et al. thus estimated the age of 11 organs in nearly 6,000 people; an organ that is "ahead" is associated with a higher risk of disease of that organ [25].
- **Metabolomics**: Deelen et al. identified, by NMR, 14 blood metabolites associated with all-cause mortality in 44,168 people (5,512 deaths), the basis of the MetaboHealth score. This profile predicts 5- and 10-year mortality better than conventional risk factors (C-statistic 0.84 vs 0.77) [26].
- **Common framework**: Moqri et al. recommend multi-omic approaches and longitudinal validation [4].

### 3.4 Telomeres

Leukocyte telomere length decreases with age, and telomere attrition is a hallmark [1, 2]. As a biomarker, however, its **inter-individual variance is very wide**, its correlation with age modest and its prediction of mortality weak once other factors are accounted for. The measurements (qPCR, TRF Southern blot, Flow-FISH) are poorly comparable with each other. Telomeres are therefore more useful as a mechanism than as a clock.

### 3.5 General limitations of validation

1. **No gold standard**: mortality is the ultimate endpoint, but too slow for trials; clocks serve as surrogates without formal validation.
2. **Choice of training target**: training on chronological age (1st generation) measures age, not aging; training on mortality (GrimAge) or a rate of decline (DunedinPACE) gets closer to the relevant biology.
3. **Weak agreement between clocks**: different clocks applied to the same individual can diverge, which suggests that they capture different dimensions of aging [4].
4. **Population bias**: training cohorts are mostly European [19].

---

## 4. Organising frameworks

### 4.1 Hallmarks of aging (López-Otín et al., 2013 → 2023)

**2013**: 9 hallmarks [1], classified into three categories:
- *Primary* (causes of damage): genomic instability, telomere attrition, epigenetic alterations, loss of proteostasis.
- *Antagonistic* (responses to damage, beneficial at low dose, harmful at high dose): deregulated nutrient sensing, mitochondrial dysfunction, cellular senescence.
- *Integrative* (functional consequences): stem cell exhaustion, altered intercellular communication.

**2023**: 12 hallmarks [2]. Three are added: **disabled macroautophagy** (separated from proteostasis), **chronic inflammation** (*inflammaging*) and **dysbiosis**. The primary / antagonistic / integrative classification is kept.

Criteria for including a hallmark [1, 2]:
1. it appears during normal aging;
2. its experimental aggravation accelerates aging;
3. its experimental attenuation slows, halts or reverses aging.

**Value**: a shared vocabulary, an operational breakdown for research and identifiable pharmacological targets. It is the framework chosen to organise this map (files 01-12).

**Limitations** (see 4.5): criteria unevenly met, little hierarchy of causality between hallmarks, a list that can be extended at will.

### 4.2 Pillars of geroscience (Kennedy et al., 2014)

The *Trans-NIH Geroscience Interest Group* proposes **seven** interconnected **pillars**: adaptation to stress, epigenetics, inflammation, macromolecules (damage), metabolism, proteostasis, stem cells and regeneration [3]. **Difference from the hallmarks**: the emphasis is on **interconnection** (acting on one pillar changes the others) and on the translational goal, namely delaying chronic diseases together. **Value**: it founded the concepts of geroscience and gerotherapeutics. **Limitations**: like the hallmarks, it provides neither a causal hierarchy nor a quantitative measure.

### 4.3 SENS (de Grey et al., 2002)

*Strategies for Engineered Negligible Senescence*: an **engineering** approach that does not try to understand all of metabolism but to **periodically repair** seven categories of accumulated damage (cell loss, senescent and death-resistant cells, nuclear (cancer) and mitochondrial mutations, intracellular and extracellular aggregates, cross-links of the extracellular matrix) [20]. **Value**: a clear action program, forerunner of senolytics and matrix repair. **Limitations**: it ignores regulated processes (hyperfunction, signalling), several targets remain technically out of reach (e.g. mitochondrial allotopic expression), and it was long marginalised by the academic community.

### 4.4 Hallmarks of health (López-Otín & Kroemer, 2021)

This framework reverses the perspective: health is defined positively, by **eight hallmarks grouped into three domains**. The first is *spatial compartmentalisation* (integrity of barriers, containment of local perturbations). The second is *maintenance of homeostasis over time* (recycling and turnover, integration of circuits, rhythmic oscillations). The third covers *adequate responses to stress* (homeostatic resilience, hormetic regulation, repair and regeneration) [21]. **Value**: it emphasises systemic properties (barriers, rhythms, resilience) that the hallmarks of aging handle poorly, and it is useful for defining success criteria for interventions. **Limitations**: more descriptive than operational, hard to measure.

### 4.5 Critiques of the hallmarks (Gems & de Magalhães, 2021)

Gems and de Magalhães make a central critique [5]:
- **Unexamined implicit paradigm**: the hallmarks extend, without saying so, the molecular damage theory. Yet data (e.g. on mTOR, hyperfunction and prolonged developmental programs) indicate that a large part of the pathology of aging comes from biological **programs** continued beyond their usefulness rather than from wear and tear.
- **Heterogeneous list**: it mixes causes, mechanisms and consequences without an explanatory model linking them, and each hallmark gathers disparate phenomena.
- **Weak criteria**: the criterion "aggravation accelerates aging" discriminates poorly, because many lesions cause a pathology that *resembles* aging without being a mechanism of it.
- **Recommendation**: develop an **explanatory theory** (e.g. a framework based on antagonistic pleiotropy and programs) instead of a simple list, and test rival hypotheses.

**Consequence for Astra**: use the hallmarks as an **index** of the literature, not as a causal model. Each hallmark file should state to what extent the three criteria are met in humans, not only in animal models.

### 4.6 Comparative summary

| Framework | Nature | Main strength | Main weakness |
|---|---|---|---|
| Hallmarks 2013/2023 [1, 2] | Mechanistic taxonomy | Shared vocabulary, targets | No causal hierarchy; "damage" bias |
| Geroscience pillars [3] | Interconnected network | Link between aging and chronic diseases | Not very quantitative |
| SENS [20] | Repair engineering | Concrete action plan | Ignores regulation; feasibility |
| Hallmarks of health [21] | Positive definition | Systemic properties, resilience | Hard to measure |
| Hyperfunction [9] | Causal theory | Verified predictions (rapamycin) | Underestimates damage |
| Information theory [10] | Causal theory | Reversibility, causal role of the epigenome | Based on one main model |

---

## 5. Open questions

1. **Single or multiple root cause?** Damage, programs, loss of information: is there a dominant causal level, or is aging an emergent phenomenon of a network with no first cause [2, 5, 9, 10]?
2. **Hierarchy between hallmarks**: which ones are upstream? Can a quantified causal graph be built, for example through Mendelian randomisation or combinatorial interventions?
3. **Validation of biomarkers**: which clock (or multi-omic combination) can serve as a surrogate endpoint in a clinical trial? What size of change is clinically meaningful [4, 16]?
4. **Causal vs correlational clocks**: are the measured methylation changes drivers or markers? Does reversing the clock (reprogramming) really reverse functions [10]?
5. **Translatability**: do interventions that work in short-lived models (worm, fly, mouse) hold for long-lived species, including humans? What can species with exceptional longevity teach us?
6. **Aging by organ**: do organs age at different speeds within the same individual, and does the biologically oldest one determine the prognosis [19]?
7. **Regulatory status**: should aging be classified as a disease (or a targetable "condition") to allow trials of gerotherapeutics [3, 4]?
8. **Ceiling of longevity**: is there a biological limit to human lifespan, or only a modifiable Gompertz slope?

---

## References

1. López-Otín C, Blasco MA, Partridge L, Serrano M, Kroemer G. The hallmarks of aging. *Cell*. 2013;153(6):1194-1217. https://doi.org/10.1016/j.cell.2013.05.039
2. López-Otín C, Blasco MA, Partridge L, Serrano M, Kroemer G. Hallmarks of aging: An expanding universe. *Cell*. 2023;186(2):243-278. https://doi.org/10.1016/j.cell.2022.11.001
3. Kennedy BK, Berger SL, Brunet A, et al. Geroscience: linking aging to chronic disease. *Cell*. 2014;159(4):709-713. https://doi.org/10.1016/j.cell.2014.10.039
4. Moqri M, Herzog C, Poganik JR, et al. Biomarkers of aging for the identification and evaluation of longevity interventions. *Cell*. 2023;186(18):3758-3775. https://doi.org/10.1016/j.cell.2023.08.003
5. Gems D, de Magalhães JP. The hoverfly and the wasp: A critique of the hallmarks of aging as a paradigm. *Ageing Res Rev*. 2021;70:101407. https://doi.org/10.1016/j.arr.2021.101407
6. Williams GC. Pleiotropy, natural selection, and the evolution of senescence. *Evolution*. 1957;11(4):398-411. https://doi.org/10.1111/j.1558-5646.1957.tb02911.x
7. Kirkwood TBL. Evolution of ageing. *Nature*. 1977;270:301-304. https://doi.org/10.1038/270301a0
8. Harman D. Aging: a theory based on free radical and radiation chemistry. *J Gerontol*. 1956;11(3):298-300. https://doi.org/10.1093/geronj/11.3.298
9. Blagosklonny MV. Aging and immortality: quasi-programmed senescence and its pharmacologic inhibition. *Cell Cycle*. 2006;5(18):2087-2102. https://doi.org/10.4161/cc.5.18.3288
10. Yang JH, Hayano M, Griffin PT, et al. Loss of epigenetic information as a cause of mammalian aging. *Cell*. 2023;186(2):305-326. https://doi.org/10.1016/j.cell.2022.12.027
11. Horvath S. DNA methylation age of human tissues and cell types. *Genome Biol*. 2013;14:R115. https://doi.org/10.1186/gb-2013-14-10-r115
12. Hannum G, Guinney J, Zhao L, et al. Genome-wide methylation profiles reveal quantitative views of human aging rates. *Mol Cell*. 2013;49(2):359-367. https://doi.org/10.1016/j.molcel.2012.10.016
13. Levine ME, Lu AT, Quach A, et al. An epigenetic biomarker of aging for lifespan and healthspan. *Aging (Albany NY)*. 2018;10(4):573-591. https://doi.org/10.18632/aging.101414
14. Lu AT, Quach A, Wilson JG, et al. DNA methylation GrimAge strongly predicts lifespan and healthspan. *Aging (Albany NY)*. 2019;11(2):303-327. https://doi.org/10.18632/aging.101684
15. Belsky DW, Caspi A, Corcoran DL, et al. DunedinPACE, a DNA methylation biomarker of the pace of aging. *eLife*. 2022;11:e73420. https://doi.org/10.7554/eLife.73420
16. Higgins-Chen AT, Thrush KL, Wang Y, et al. A computational solution for bolstering reliability of epigenetic clocks: implications for clinical trials and longitudinal tracking. *Nat Aging*. 2022;2:644-661. https://doi.org/10.1038/s43587-022-00248-2
17. Mitnitski AB, Mogilner AJ, Rockwood K. Accumulation of deficits as a proxy measure of aging. *ScientificWorldJournal*. 2001;1:323-336. https://doi.org/10.1100/tsw.2001.58
18. Searle SD, Mitnitski A, Gahbauer EA, Gill TM, Rockwood K. A standard procedure for creating a frailty index. *BMC Geriatr*. 2008;8:24. https://doi.org/10.1186/1471-2318-8-24
19. Argentieri MA, Xiao S, Bennett D, et al. Proteomic aging clock predicts mortality and risk of common age-related diseases in diverse populations. *Nat Med*. 2024;30:2450-2460. https://doi.org/10.1038/s41591-024-03164-7
20. de Grey ADNJ, Ames BN, Andersen JK, et al. Time to talk SENS: critiquing the immutability of human aging. *Ann N Y Acad Sci*. 2002;959:452-462. https://doi.org/10.1111/j.1749-6632.2002.tb02115.x
21. López-Otín C, Kroemer G. Hallmarks of health. *Cell*. 2021;184(1):33-63. https://doi.org/10.1016/j.cell.2020.11.034

22. Gompertz B. On the nature of the function expressive of the law of human mortality, and on a new mode of determining the value of life contingencies. *Phil Trans R Soc*. 1825;115:513-583. https://doi.org/10.1098/rstl.1825.0026
23. Finch CE, Pike MC, Witten M. Slow mortality rate accelerations during aging in some animals approximate that of humans. *Science*. 1990;249(4971):902-905. https://doi.org/10.1126/science.2392680
24. Jones OR, Scheuerlein A, Salguero-Gómez R, et al. Diversity of ageing across the tree of life. *Nature*. 2014;505:169-173. https://doi.org/10.1038/nature12789
25. Oh HSH, Rutledge J, Nachun D, et al. Organ aging signatures in the plasma proteome track health and disease. *Nature*. 2023;624:164-172. https://doi.org/10.1038/s41586-023-06802-1
26. Deelen J, Kettunen J, Fischer K, et al. A metabolic profile of all-cause mortality risk identified in an observational study of 44,168 individuals. *Nat Commun*. 2019;10:3346. https://doi.org/10.1038/s41467-019-11311-9

**Sources marked [to verify]** (books without DOI, not verifiable online): Strehler 1962 (*Time, Cells and Aging*), Medawar 1952 (*An Unsolved Problem of Biology*).
