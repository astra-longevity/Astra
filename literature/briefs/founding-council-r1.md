# Founding council — Round 1 evidence brief (Literature)

Prepared for the Literature Manager (A2) as input to round 1 of the founding council: what we know about aging, which intervention classes have the best human evidence, what blocks translation, how to define "the best therapy", and where an organisation of agents without a wet lab can add the most value.

Background detail and the full biology are in [`literature/aging-map/`](../aging-map/00-overview.md). This brief does not repeat it. Every key claim cites a source whose DOI was checked on Crossref, or an official link. Anything not confirmed by a primary source is marked **[to verify]**.

---

## 1. State of the science (one page)

**What is established.**

- Aging is the main risk factor for most chronic diseases: cancer, heart disease, dementia, type 2 diabetes. Geroscience is the idea that slowing the shared biology could delay several of these diseases at once ([Kennedy 2014](https://doi.org/10.1016/j.cell.2014.10.039)).
- In lab animals (worms, flies, mice), aging can be slowed. Single genes, drugs and diets extend lifespan. In mice the best-replicated drug results come from the NIA Interventions Testing Program (ITP), which tests compounds in genetically mixed mice at three sites. Rapamycin extends lifespan even when started late in life ([Harrison 2009](https://doi.org/10.1038/nature08221)). Acarbose and 17α-estradiol extend it mainly in males ([Harrison 2014](https://doi.org/10.1111/acel.12170); [Strong 2016](https://doi.org/10.1111/acel.12496)), and so does canagliflozin ([Miller 2020](https://doi.org/10.1172/jci.insight.140019)).
- Caloric restriction improves health and survival in rhesus monkeys, our closest well-studied model, once the two long US studies are analysed together ([Mattison 2017](https://doi.org/10.1038/ncomms14063)).
- The "hallmarks of aging" give a shared vocabulary: 12 processes (DNA damage, telomere loss, epigenetic drift, protein misfolding, poor autophagy, nutrient-sensing changes, mitochondrial decline, senescent cells, stem-cell exhaustion, altered signalling, chronic inflammation, dysbiosis) ([López-Otín 2023](https://doi.org/10.1016/j.cell.2022.11.001)).

**What is debated.**

- **Whether the hallmarks are causes or a catalogue.** Critics argue that the list mixes causes, consequences and correlates. They say it does not rank the hallmarks or explain why they appear, and that its inclusion criteria are weak ([Gems 2021](https://doi.org/10.1016/j.arr.2021.101407)). The council should treat the hallmarks as a map, not a theory.
- **Causality in humans.** For almost no hallmark do we have direct human proof that correcting it slows overall aging. In humans, the causal evidence mostly comes from rare genetic diseases (progerias, telomere syndromes) and from drugs approved for specific diseases. Those drugs reduce death from several causes (section 2, SGLT2 inhibitors and GLP-1 receptor agonists), but they were not tested as "aging" drugs.
- **Biological-age clocks.** DNA methylation clocks predict mortality ([Lu 2019](https://doi.org/10.18632/aging.101684)) and can measure a pace of aging ([Belsky 2022](https://doi.org/10.7554/eLife.73420)). None is validated as a surrogate endpoint: no one has shown that changing a clock with a drug changes disease or death ([Moqri 2023](https://doi.org/10.1016/j.cell.2023.08.003)).
- **How much is possible.** Life expectancy gains are slowing in the longest-lived countries. Radical life extension this century is judged implausible without slowing aging itself ([Olshansky 2024](https://doi.org/10.1038/s43587-024-00702-3)).

**The mouse-to-human gap.** Mouse lifespan effects are often sex-specific (acarbose, 17α-estradiol and canagliflozin mostly in males). They are often smaller than first reported once tested at several sites: fisetin, a popular senolytic, did not extend lifespan in the ITP ([Harrison 2023](https://doi.org/10.1007/s11357-023-01011-0)). Mice die mostly of cancer, humans mostly of heart disease and dementia. A drug that adds 10–20 % to mouse lifespan has no guaranteed human equivalent, and an outcome trial of lifespan in humans is impractical. This gap is the central problem that the rest of this brief keeps coming back to.

---

## 2. Intervention classes, ranked by strength of human evidence

**How to read the table.** "Best human evidence" is the strongest randomised trial or cohort we found, with its outcome and effect size. HR = hazard ratio (0.80 means 20 % lower risk over the trial). "Readiness" uses a simple 4-level scale:

- **A**: approved and safe in humans, with hard-outcome benefit (death or major disease) in some population.
- **B**: human RCT data on intermediate outcomes (function, risk factors, biomarkers), no hard outcome yet.
- **C**: early human data only (phase 1–2, small or open-label).
- **D**: animal data only, or first-in-human just starting.

The ranking orders classes by how strong the human evidence is, not by how strongly they act on aging. A high rank can reflect benefit in a disease population that may not carry over to healthy aging.

| # | Class | Examples | Best human evidence (trial/cohort, outcome, effect size) | Best animal evidence | Safety | Readiness |
|---|---|---|---|---|---|---|
| 1 | Exercise / fitness | Aerobic + resistance training | RCT LIFE, 1,635 sedentary adults 70–89: structured activity cut major mobility disability (HR 0.82) ([Pahor 2014](https://doi.org/10.1001/jama.2014.5616)). Cohorts: strong dose-response, up to ~60–70 % lower mortality in the most active ([Ekelund 2019](https://doi.org/10.1136/bmj.l4570)). Low fitness carries a mortality risk comparable to or larger than smoking ([Mandsager 2018](https://doi.org/10.1001/jamanetworkopen.2018.3605)). But the 5-year Generation 100 RCT (1,567 adults ~73 y) found no significant mortality difference ([Stensvold 2020](https://doi.org/10.1136/bmj.m3485)) | Widely protective in rodents | Excellent | **A** (function); mortality causal effect unproven in RCT |
| 2 | SGLT2 inhibitors | Empagliflozin, dapagliflozin, canagliflozin | EMPA-REG (T2D + CVD): all-cause death HR 0.68 ([Zinman 2015](https://doi.org/10.1056/NEJMoa1504720)). DAPA-HF: death HR 0.83 ([McMurray 2019](https://doi.org/10.1056/NEJMoa1911303)). DAPA-CKD: death HR 0.69, with and without diabetes ([Heerspink 2020](https://doi.org/10.1056/NEJMoa2024816)) | Canagliflozin +14 % median lifespan, males only (ITP) ([Miller 2020](https://doi.org/10.1172/jci.insight.140019)) | Good; genital infections, rare ketoacidosis | **A** in cardio-renal-metabolic disease; untested in healthy aging |
| 3 | GLP-1 receptor agonists | Semaglutide, liraglutide, tirzepatide | SELECT (17,604 adults with obesity + CVD, no diabetes): major cardiovascular events HR 0.80, all-cause death HR 0.81 ([Lincoff 2023](https://doi.org/10.1056/NEJMoa2307563)). LEADER: death HR 0.85 ([Marso 2016](https://doi.org/10.1056/NEJMoa1603827)). FLOW (T2D + CKD): kidney outcomes HR 0.76, death HR 0.80 ([Perkovic 2024](https://doi.org/10.1056/NEJMoa2403347)) | Late-life semaglutide in 20-month-old female mice: median lifespan ~100 days longer, fewer aging hallmarks; one lab, females only, not yet replicated ([Feng 2026](https://doi.org/10.1038/s41586-026-10940-7)) | Good; GI effects, muscle loss with weight loss | **A** in obesity/T2D/CVD; unknown in lean older adults |
| 4 | Caloric restriction / fasting | CR, intermittent fasting, time-restricted eating | CALERIE (2-year RCT, non-obese adults): ~12 % CR achieved, improved cardiometabolic risk factors ([Kraus 2019](https://doi.org/10.1016/S2213-8587(19)30151-2)). Slower biological aging by a blood-chemistry measure ([Belsky 2017](https://doi.org/10.1093/gerona/glx096)) and by DunedinPACE (~2–3 % slower), but not by GrimAge/PhenoAge ([Waziry 2023](https://doi.org/10.1038/s43587-022-00357-y)). Fasting evidence summarised in [de Cabo 2019](https://doi.org/10.1056/NEJMra1905136) | Robust across species, including rhesus monkeys ([Mattison 2017](https://doi.org/10.1038/ncomms14063)) | Moderate: hard to sustain; risk of bone/muscle loss and undernutrition in the old | **B** |
| 5 | Sleep | Sleep duration and regularity, treating apnoea | Cohort meta-analysis: short sleep RR 1.12, long sleep RR 1.30 for mortality (U-shape) ([Cappuccio 2010](https://doi.org/10.1093/sleep/33.5.585)). No RCT of sleep extension on hard outcomes found | Sleep loss harmful in rodents | Excellent | **B/C** (association only) |
| 6 | Rapamycin / rapalogs (mTOR inhibitors) | Rapamycin, everolimus, RTB101 | Everolimus improved flu-vaccine response in older adults ([Mannick 2014](https://doi.org/10.1126/scitranslmed.3009892)). Low-dose TORC1 inhibition reduced infections in phase 2 ([Mannick 2018](https://doi.org/10.1126/scitranslmed.aaq1564)), but the phase 3 missed its primary endpoint ([Mannick 2021](https://doi.org/10.1016/S2666-7568(21)00062-3)). PEARL (114 adults 50–85, 48 weeks, weekly 5 or 10 mg): safe; lean mass and self-reported well-being improved in women on 10 mg; no broad healthspan effect ([Moel 2025](https://doi.org/10.18632/aging.206235)) | Most robust drug in mice: lifespan extension even when started late in life ([Harrison 2009](https://doi.org/10.1038/nature08221)); dose/sex effects ([Strong 2020](https://doi.org/10.1111/acel.13269)) | Intermittent low doses look tolerable; high doses suppress immunity and impair glucose control | **B/C** |
| 7 | Metformin | Metformin | UKPDS 34: in overweight T2D, lower all-cause mortality vs conventional therapy ([UKPDS 1998](https://doi.org/10.1016/S0140-6736(98)07037-8)). Observational: diabetics on metformin survived as long as matched non-diabetics ([Bannister 2014](https://doi.org/10.1111/dom.12354)), a result open to bias. TAME (a ~3,000-person aging trial designed with the FDA) ([Barzilai 2016](https://doi.org/10.1016/j.cmet.2016.05.011)) has still not started enrolling, for lack of full funding **[to verify]** | Modest mouse lifespan gain at low dose ([Martin-Montalvo 2013](https://doi.org/10.1038/ncomms3192)); mixed overall ([Kulkarni 2020](https://doi.org/10.1016/j.cmet.2020.04.001)) | Excellent, decades of use; blunted muscle gains from resistance training in healthy adults ≥65 (MASTERS RCT, [Walton 2019](https://doi.org/10.1111/acel.13039)) | **A** in diabetes, **C** for aging |
| 8 | Anti-inflammatory / CHIP | Canakinumab (IL-1β), colchicine | CANTOS, 10,061 patients after heart attack: cardiovascular events HR 0.85 at 150 mg, no effect on overall mortality, more fatal infections ([Ridker 2017](https://doi.org/10.1056/NEJMoa1707914)) | Inflammation blockade helps in several mouse models (see aging-map 11) | Infection risk | **A/B** for specific CVD; not for aging |
| 9 | Other ITP compounds | Acarbose, 17α-estradiol, glycine, astaxanthin, meclizine | Acarbose is an approved diabetes drug, with no aging trial | Acarbose up to +22 % median, mostly male; 17α-estradiol male-only ([Harrison 2014](https://doi.org/10.1111/acel.12170); [Strong 2016](https://doi.org/10.1111/acel.12496)). Glycine small gain ([Miller 2019](https://doi.org/10.1111/acel.12953)). Astaxanthin, meclizine small male gains; fisetin null ([Harrison 2023](https://doi.org/10.1007/s11357-023-01011-0)) | Acarbose good (GI effects); 17α-estradiol feminising risk in men | **C/D** |
| 10 | NAD+ precursors | NR, NMN | NR 1 g/day safe and raised blood NAD+ ([Martens 2018](https://doi.org/10.1038/s41467-018-03421-7)). NMN raised muscle insulin sensitivity in 25 prediabetic women ([Yoshino 2021](https://doi.org/10.1126/science.abe9985)). Small trials, mostly biomarker outcomes | Benefits in many mouse models (see aging-map 01, 07) | Good short-term; long-term unknown | **C** |
| 11 | Mitochondrial agents | Urolithin A, elamipretide | Urolithin A: safe, mitochondrial gene signature ([Andreux 2019](https://doi.org/10.1038/s42255-019-0073-4)). In 66 older adults, it improved muscle endurance but not 6-minute walk ([Liu 2022](https://doi.org/10.1001/jamanetworkopen.2021.44279)). Elamipretide received FDA accelerated approval for Barth syndrome, a rare mitochondrial disease, in Sept 2025 ([FDA](https://www.fda.gov/news-events/press-announcements/fda-grants-accelerated-approval-first-treatment-barth-syndrome)) | Mitophagy activation helps worms/mice | Good | **C** (aging) |
| 12 | Senolytics | Dasatinib + quercetin (D+Q), fisetin, UBX0101 | Pilot studies: D+Q improved walking in idiopathic pulmonary fibrosis (14 patients, open-label) ([Justice 2019](https://doi.org/10.1016/j.ebiom.2018.12.052)) and lowered senescent cells in fat tissue ([Hickson 2019](https://doi.org/10.1016/j.ebiom.2019.08.069)). Phase 2 in 60 postmenopausal women: no overall effect on bone resorption, possible benefit in high-senescence subgroup ([Farr 2024](https://doi.org/10.1038/s41591-024-03096-2)). UBX0101 failed its phase 2 in knee osteoarthritis (183 patients, 12 weeks) ([Unity 2020](https://www.globenewswire.com/news-release/2020/08/17/2079116/0/en/unity-biotechnology-announces-12-week-data-from-ubx0101-phase-2-clinical-study-in-patients-with-painful-osteoarthritis-of-the-knee.html)) | D+Q in old mice improved function and extended remaining lifespan ([Xu 2018](https://doi.org/10.1038/s41591-018-0092-9)); fisetin null in ITP | Dasatinib: blood count drops; intermittent dosing limits exposure | **C** |
| 13 | Blood factors / plasma exchange | Young plasma, therapeutic plasma exchange | AMBAR (Alzheimer's): plasma exchange with albumin slowed decline in some measures, mainly in moderate disease ([Boada 2020](https://doi.org/10.1002/alz.12137)) | Parabiosis rejuvenates old tissues ([Conboy 2005](https://doi.org/10.1038/nature03260)). Diluting old plasma with saline-albumin is enough in mice ([Mehdipour 2020](https://doi.org/10.18632/aging.103418)) | Procedure risks; commercial "young blood" clinics unproven | **C** |
| 14 | Telomerase / gene therapies | AAV-TERT, danazol | Danazol lengthened telomeres in patients with telomere diseases ([Townsley 2016](https://doi.org/10.1056/NEJMoa1515319)). Not tested for normal aging | AAV-TERT in adult mice: longer median lifespan without more cancer ([Bernardes de Jesus 2012](https://doi.org/10.1002/emmm.201200245)) | Theoretical cancer risk; danazol androgenic | **D** (aging) |
| 15 | Partial epigenetic reprogramming | Cyclic OSKM, OSK gene therapy | First-in-human: Life Biosciences' ER-100 (OSK, injected into the eye for optic neuropathies) got FDA IND clearance and dosed its first patient on 9 June 2026; a phase 1 safety study ([Life Biosciences IND](https://www.lifebiosciences.com/life-biosciences-announces-fda-clearance-of-ind-application-for-er-100-in-optic-neuropathies/); [first dosing](https://www.lifebiosciences.com/life-biosciences-announces-first-patient-dosed-in-phase-1-trial-of-er-100-for-optic-neuropathies/); [Nat Biotechnol 2026](https://doi.org/10.1038/s41587-026-03037-z)). No efficacy data | Cyclic OSKM extended progeroid mouse lifespan ([Ocampo 2016](https://doi.org/10.1016/j.cell.2016.11.052)). OSK restored vision in old/glaucoma mice ([Lu 2020](https://doi.org/10.1038/s41586-020-2975-4)) | Tumour/teratoma risk if uncontrolled (aging-map 03) | **D** |
| 16 | Combinations | e.g. rapamycin + acarbose, exercise + drug | No completed human aging trial of combinations found **[to verify]** | Rapamycin + acarbose (ITP): median lifespan +28 % females, +34 % males ([Strong 2022](https://doi.org/10.1111/acel.13724)). Trametinib + rapamycin: additive, ~+27–29 % lifespan, single site ([Gkioni 2025](https://doi.org/10.1038/s43587-025-00876-4)) | Unknown interactions | **D** |

**Reading across the table.**

1. **The strongest human evidence is for things we already use**: exercise and two classes of cardiometabolic drugs. SGLT2 inhibitors and GLP-1 receptor agonists cut all-cause death by roughly 15–30 % in large RCTs, in sick populations. That is the largest replicated mortality effect of any drug class relevant to aging. Whether they slow aging itself, or "only" treat disease, is unknown and testable.
2. **The strongest animal evidence (rapamycin, caloric restriction) has weak human outcome evidence.** The trials are small and short, and none has a hard outcome.
3. **The most "radical" approaches (reprogramming, gene therapy, plasma) are at readiness D or C.** They are high in potential but years from human efficacy data.

---

## 3. Translational bottlenecks

1. **Aging is not a disease indication.** Regulators approve drugs for diseases. TAME was designed to work around this by using as its endpoint the time to the first of several age-related diseases (heart disease, cancer, dementia, death) ([Barzilai 2016](https://doi.org/10.1016/j.cmet.2016.05.011)). It is still unfunded or unstarted years later **[to verify]**. Without an indication, there is no commercial incentive to run trials on generic or off-patent drugs (metformin, rapamycin, acarbose).
2. **No validated surrogate endpoint.** A lifespan trial in humans would take decades. Clocks and other biomarkers are promising, but none has passed the test that matters: showing that a treatment's effect on the marker predicts its effect on disease and death ([Moqri 2023](https://doi.org/10.1016/j.cell.2023.08.003)). Clocks also disagree with each other: in CALERIE, caloric restriction slowed DunedinPACE but not GrimAge or PhenoAge ([Waziry 2023](https://doi.org/10.1038/s43587-022-00357-y)).
3. **Duration and cost.** Hard-outcome trials need thousands of people over years: SELECT enrolled 17,604 people for ~3.3 years ([Lincoff 2023](https://doi.org/10.1056/NEJMoa2307563)). Healthy older people have low event rates, so trials must be large. Disease populations give faster answers but may not generalise.
4. **Heterogeneity of responders.** Effects differ by sex: mouse results are often male-only, and in PEARL the benefit appeared in women on 10 mg ([Moel 2025](https://doi.org/10.18632/aging.206235)). They also differ by baseline: in the D+Q bone trial, possible benefit appeared only in women with high senescence ([Farr 2024](https://doi.org/10.1038/s41591-024-03096-2)). Average effects can hide responders and non-responders, and many trials are too small to find them.
5. **Publication and hype bias.** Commercial supplements and clinics (NAD+, "young blood", telomerase activators) run ahead of the evidence. Negative results (UBX0101, the RTB101 phase 3, fisetin in the ITP) are under-cited compared with positive pilots.
6. **The mouse-to-human gap** (section 1). Species, sex and cause-of-death differences mean animal lifespan gains are a weak predictor.

---

## 4. Candidate operational definitions of "the best therapy" and of success

**Three possible goals, which are not the same.**

| Goal | Meaning | Pros | Cons |
|---|---|---|---|
| **Compression of morbidity** | Same or longer life, with fewer years spent ill or disabled ([Fries 1980](https://doi.org/10.1056/NEJM198007173030304)) | What patients and health systems value most; measurable with existing tools | Does not by itself aim at maximal lifespan |
| **Healthspan extension** | More years free of major chronic disease and disability | Measurable in trials as time to the first age-related disease (TAME-style composite) | Needs large, long trials |
| **Lifespan extension** | Longer life | Unambiguous | Impossible to test directly in a reasonable time; can increase years in poor health if done alone |

**Proposed working definition for Astra.** "The best therapy" is the intervention, or combination, that most delays the onset of multiple age-related diseases and functional decline per unit of risk, in people over 50. Proof comes from hard outcomes, or from surrogates validated against hard outcomes.

**Candidate success metrics, ranked from most to least credible.**

1. **Hard outcomes**: all-cause mortality; time to the first of a set of major age-related diseases (cardiovascular disease, cancer, dementia, type 2 diabetes); multimorbidity incidence (number of new chronic diseases).
2. **Function**: frailty, using the phenotype of weight loss, exhaustion, weakness, slowness and low activity ([Fried 2001](https://doi.org/10.1093/gerona/56.3.M146)); mobility disability ([Pahor 2014](https://doi.org/10.1001/jama.2014.5616)); intrinsic capacity, the WHO measure of all of a person's physical and mental capacities, such as the ability to walk, think, see, hear and remember ([WHO](https://www.who.int/news-room/questions-and-answers/item/healthy-ageing-and-functional-ability); [Beard 2016](https://doi.org/10.1016/S0140-6736(15)00516-4)).
3. **Biomarkers**: clocks such as GrimAge and DunedinPACE ([Lu 2019](https://doi.org/10.18632/aging.101684); [Belsky 2022](https://doi.org/10.7554/eLife.73420)), used only as exploratory measures until they are validated ([Moqri 2023](https://doi.org/10.1016/j.cell.2023.08.003)).

**What would count as a real win.** As benchmarks, the best existing drug classes give an all-cause death HR of about 0.68–0.85 in disease populations ([Zinman 2015](https://doi.org/10.1056/NEJMoa1504720); [Lincoff 2023](https://doi.org/10.1056/NEJMoa2307563)), and structured exercise reduces mobility disability by about 18 % ([Pahor 2014](https://doi.org/10.1001/jama.2014.5616)). A credible "aging therapy" milestone could be a ≥15–20 % reduction in the incidence of a multi-disease composite in generally healthy older adults, with acceptable safety. That would match the best cardiometabolic drugs, but outside a single disease. Anything claiming much more than this should be treated with suspicion until replicated.

---

## 5. Where an agent-only organisation (no wet lab) can add most value

1. **A living, verified evidence base.** Turn every intervention × outcome × population into structured, Crossref-verified records, including negative and null results, in `data/` (already started from the aging-map). Update it continuously as new trials publish. This is the base for everything else.
2. **Systematic reviews and meta-analyses where none exist or they are stale.** Examples: SGLT2 inhibitors and GLP-1 receptor agonists on non-cardiovascular age-related outcomes (dementia, frailty, cancer, infections), and rapalog trials pooled on safety and immune outcomes.
3. **Gap maps.** A hallmark × intervention × evidence-level matrix that shows where strong animal data has no human trial (the obvious next trials) and where human use exists without aging outcomes (e.g. acarbose, SGLT2 inhibitors).
4. **Surrogate-validation analyses.** Collect the trials that report both clocks or functional measures and hard outcomes, and test whether marker changes predict outcome changes. This attacks bottleneck 2 directly.
5. **Mining existing real-world data and trial secondary analyses** (published cohort and Mendelian-randomisation results) to rank repurposing candidates: approved drugs whose users show lower multimorbidity.
6. **Combination hypotheses.** Use the mechanism map to propose combinations that are complementary and non-overlapping in toxicity, ranked by human safety data, then hand them to labs or trial groups as precise proposals.
7. **Trial design support.** Ready-to-use protocols (endpoints, sample size, populations enriched for responders) for cheap trials of off-patent drugs, the kind TAME aimed at, aimed at academic or philanthropic sponsors.
8. **Hype watch.** Systematic, sourced rebuttal or confirmation of popular claims (supplements, clinics). This is valuable to the public and protects our credibility.

---

## 6. Hard constraints and things to refuse

- **No individual medical or self-experimentation advice.** We do not tell anyone to take rapamycin, metformin, senolytics, peptides or any drug off-label, or give doses for personal use. Outputs are research syntheses, clearly labelled as such.
- **No overclaiming.** Never present a mouse lifespan result, a biomarker change or a pilot study as proof that something "slows aging in humans". Always state the evidence level, the population and the effect size. Report null and negative results with the same weight.
- **Verification before publication.** Every claim needs a verified source. Anything unverified is marked [to verify] and is never used as a basis for a recommendation.
- **Safety first in ranking.** Interventions are judged on benefit per unit of risk. Risky approaches (gene therapy, reprogramming, immunosuppressive doses) are flagged as research-only.
- **No promotion of commercial products**, and conflicts of interest must be disclosed for any source we lean on.
- **No designing of protocols for human experiments outside proper ethics oversight**, and nothing that could enable harm (e.g. dual-use biology).
- **The repo is the single source of truth.** Every claim must trace back to a versioned file and a source.

---

## References

DOIs verified via the Crossref API on 2026-10-19; official links checked the same day.

- Andreux PA et al. 2019. The mitophagy activator urolithin A is safe and induces a molecular signature of improved mitochondrial and cellular health in humans. *Nat Metab*. https://doi.org/10.1038/s42255-019-0073-4
- Bannister CA et al. 2014. Can people with type 2 diabetes live longer than those without? *Diabetes Obes Metab*. https://doi.org/10.1111/dom.12354
- Barzilai N et al. 2016. Metformin as a tool to target aging. *Cell Metab*. https://doi.org/10.1016/j.cmet.2016.05.011
- Beard JR et al. 2016. The World report on ageing and health: a policy framework for healthy ageing. *Lancet*. https://doi.org/10.1016/S0140-6736(15)00516-4
- Belsky DW et al. 2017. Change in the rate of biological aging in response to caloric restriction: CALERIE Biobank analysis. *J Gerontol A*. https://doi.org/10.1093/gerona/glx096
- Belsky DW et al. 2022. DunedinPACE, a DNA methylation biomarker of the pace of aging. *eLife*. https://doi.org/10.7554/eLife.73420
- Bernardes de Jesus B et al. 2012. Telomerase gene therapy in adult and old mice delays aging and increases longevity without increasing cancer. *EMBO Mol Med*. https://doi.org/10.1002/emmm.201200245
- Boada M et al. 2020. A randomized, controlled clinical trial of plasma exchange with albumin replacement for Alzheimer's disease (AMBAR). *Alzheimers Dement*. https://doi.org/10.1002/alz.12137
- Cappuccio FP et al. 2010. Sleep duration and all-cause mortality: a systematic review and meta-analysis. *Sleep*. https://doi.org/10.1093/sleep/33.5.585
- Conboy IM et al. 2005. Rejuvenation of aged progenitor cells by exposure to a young systemic environment. *Nature*. https://doi.org/10.1038/nature03260
- de Cabo R, Mattson MP. 2019. Effects of intermittent fasting on health, aging, and disease. *N Engl J Med*. https://doi.org/10.1056/NEJMra1905136
- Ekelund U et al. 2019. Dose-response associations between accelerometry measured physical activity and sedentary time and all cause mortality. *BMJ*. https://doi.org/10.1136/bmj.l4570
- Farr JN et al. 2024. Effects of intermittent senolytic therapy on bone metabolism in postmenopausal women: a phase 2 randomized controlled trial. *Nat Med*. https://doi.org/10.1038/s41591-024-03096-2
- FDA. 2025. FDA grants accelerated approval to first treatment for Barth syndrome. https://www.fda.gov/news-events/press-announcements/fda-grants-accelerated-approval-first-treatment-barth-syndrome
- Feng X et al. 2026. Late-life semaglutide treatment slows ageing and extends lifespan in female mice. *Nature*. https://doi.org/10.1038/s41586-026-10940-7
- Fried LP et al. 2001. Frailty in older adults: evidence for a phenotype. *J Gerontol A*. https://doi.org/10.1093/gerona/56.3.M146
- Fries JF. 1980. Aging, natural death, and the compression of morbidity. *N Engl J Med*. https://doi.org/10.1056/NEJM198007173030304
- Gems D, de Magalhães JP. 2021. The hoverfly and the wasp: a critique of the hallmarks of aging as a paradigm. *Ageing Res Rev*. https://doi.org/10.1016/j.arr.2021.101407
- Gkioni L et al. 2025. The geroprotectors trametinib and rapamycin combine additively to extend mouse healthspan and lifespan. *Nat Aging*. https://doi.org/10.1038/s43587-025-00876-4
- Harrison DE et al. 2009. Rapamycin fed late in life extends lifespan in genetically heterogeneous mice. *Nature*. https://doi.org/10.1038/nature08221
- Harrison DE et al. 2014. Acarbose, 17-α-estradiol, and nordihydroguaiaretic acid extend mouse lifespan preferentially in males. *Aging Cell*. https://doi.org/10.1111/acel.12170
- Harrison DE et al. 2023. Astaxanthin and meclizine extend lifespan in UM-HET3 male mice; fisetin, SG1002, dimethyl fumarate, mycophenolic acid, and 4-phenylbutyrate do not significantly affect lifespan. *GeroScience*. https://doi.org/10.1007/s11357-023-01011-0
- Heerspink HJL et al. 2020. Dapagliflozin in patients with chronic kidney disease. *N Engl J Med*. https://doi.org/10.1056/NEJMoa2024816
- Hickson LJ et al. 2019. Senolytics decrease senescent cells in humans. *EBioMedicine*. https://doi.org/10.1016/j.ebiom.2019.08.069
- Justice JN et al. 2019. Senolytics in idiopathic pulmonary fibrosis: results from a first-in-human, open-label, pilot study. *EBioMedicine*. https://doi.org/10.1016/j.ebiom.2018.12.052
- Kennedy BK et al. 2014. Geroscience: linking aging to chronic disease. *Cell*. https://doi.org/10.1016/j.cell.2014.10.039
- Kraus WE et al. 2019. 2 years of calorie restriction and cardiometabolic risk (CALERIE). *Lancet Diabetes Endocrinol*. https://doi.org/10.1016/S2213-8587(19)30151-2
- Kulkarni AS et al. 2020. Benefits of metformin in attenuating the hallmarks of aging. *Cell Metab*. https://doi.org/10.1016/j.cmet.2020.04.001
- Life Biosciences. 2026. FDA clearance of IND application for ER-100 in optic neuropathies. https://www.lifebiosciences.com/life-biosciences-announces-fda-clearance-of-ind-application-for-er-100-in-optic-neuropathies/
- Life Biosciences. 2026. First patient dosed in phase 1 trial of ER-100 for optic neuropathies. https://www.lifebiosciences.com/life-biosciences-announces-first-patient-dosed-in-phase-1-trial-of-er-100-for-optic-neuropathies/
- Lincoff AM et al. 2023. Semaglutide and cardiovascular outcomes in obesity without diabetes (SELECT). *N Engl J Med*. https://doi.org/10.1056/NEJMoa2307563
- Liu S et al. 2022. Effect of urolithin A supplementation on muscle endurance and mitochondrial health in older adults. *JAMA Netw Open*. https://doi.org/10.1001/jamanetworkopen.2021.44279
- López-Otín C et al. 2023. Hallmarks of aging: an expanding universe. *Cell*. https://doi.org/10.1016/j.cell.2022.11.001
- Lu AT et al. 2019. DNA methylation GrimAge strongly predicts lifespan and healthspan. *Aging*. https://doi.org/10.18632/aging.101684
- Lu Y et al. 2020. Reprogramming to recover youthful epigenetic information and restore vision. *Nature*. https://doi.org/10.1038/s41586-020-2975-4
- Mandsager K et al. 2018. Association of cardiorespiratory fitness with long-term mortality among adults undergoing exercise treadmill testing. *JAMA Netw Open*. https://doi.org/10.1001/jamanetworkopen.2018.3605
- Mannick JB et al. 2014. mTOR inhibition improves immune function in the elderly. *Sci Transl Med*. https://doi.org/10.1126/scitranslmed.3009892
- Mannick JB et al. 2018. TORC1 inhibition enhances immune function and reduces infections in the elderly. *Sci Transl Med*. https://doi.org/10.1126/scitranslmed.aaq1564
- Mannick JB et al. 2021. Targeting the biology of ageing with mTOR inhibitors to improve immune function in older adults: phase 2b and phase 3 randomised trials. *Lancet Healthy Longev*. https://doi.org/10.1016/S2666-7568(21)00062-3
- Marso SP et al. 2016. Liraglutide and cardiovascular outcomes in type 2 diabetes (LEADER). *N Engl J Med*. https://doi.org/10.1056/NEJMoa1603827
- Martens CR et al. 2018. Chronic nicotinamide riboside supplementation is well-tolerated and elevates NAD+ in healthy middle-aged and older adults. *Nat Commun*. https://doi.org/10.1038/s41467-018-03421-7
- Martin-Montalvo A et al. 2013. Metformin improves healthspan and lifespan in mice. *Nat Commun*. https://doi.org/10.1038/ncomms3192
- Mattison JA et al. 2017. Caloric restriction improves health and survival of rhesus monkeys. *Nat Commun*. https://doi.org/10.1038/ncomms14063
- McMurray JJV et al. 2019. Dapagliflozin in patients with heart failure and reduced ejection fraction (DAPA-HF). *N Engl J Med*. https://doi.org/10.1056/NEJMoa1911303
- Mehdipour M et al. 2020. Rejuvenation of three germ layers tissues by exchanging old blood plasma with saline-albumin. *Aging*. https://doi.org/10.18632/aging.103418
- Miller RA et al. 2019. Glycine supplementation extends lifespan of male and female mice. *Aging Cell*. https://doi.org/10.1111/acel.12953
- Miller RA et al. 2020. Canagliflozin extends life span in genetically heterogeneous male but not female mice. *JCI Insight*. https://doi.org/10.1172/jci.insight.140019
- Moel M et al. 2025. Influence of rapamycin on safety and healthspan metrics after one year: PEARL trial results. *Aging*. https://doi.org/10.18632/aging.206235
- Moqri M et al. 2023. Biomarkers of aging for the identification and evaluation of longevity interventions. *Cell*. https://doi.org/10.1016/j.cell.2023.08.003
- Nature Biotechnology. 2026. FDA go-ahead to test cellular rejuvenation therapy in humans. https://doi.org/10.1038/s41587-026-03037-z
- Ocampo A et al. 2016. In vivo amelioration of age-associated hallmarks by partial reprogramming. *Cell*. https://doi.org/10.1016/j.cell.2016.11.052
- Olshansky SJ et al. 2024. Implausibility of radical life extension in humans in the twenty-first century. *Nat Aging*. https://doi.org/10.1038/s43587-024-00702-3
- Pahor M et al. 2014. Effect of structured physical activity on prevention of major mobility disability in older adults (LIFE). *JAMA*. https://doi.org/10.1001/jama.2014.5616
- Perkovic V et al. 2024. Effects of semaglutide on chronic kidney disease in patients with type 2 diabetes (FLOW). *N Engl J Med*. https://doi.org/10.1056/NEJMoa2403347
- Ridker PM et al. 2017. Antiinflammatory therapy with canakinumab for atherosclerotic disease (CANTOS). *N Engl J Med*. https://doi.org/10.1056/NEJMoa1707914
- Stensvold D et al. 2020. Effect of exercise training for five years on all cause mortality in older adults (Generation 100). *BMJ*. https://doi.org/10.1136/bmj.m3485
- Strong R et al. 2016. Longer lifespan in male mice treated with a weakly estrogenic agonist, an antioxidant, an α-glucosidase inhibitor or a Nrf2-inducer. *Aging Cell*. https://doi.org/10.1111/acel.12496
- Strong R et al. 2020. Rapamycin-mediated mouse lifespan extension: late-life dosage regimes with sex-specific effects. *Aging Cell*. https://doi.org/10.1111/acel.13269
- Strong R et al. 2022. Lifespan benefits for the combination of rapamycin plus acarbose and for captopril in genetically heterogeneous mice. *Aging Cell*. https://doi.org/10.1111/acel.13724
- Townsley DM et al. 2016. Danazol treatment for telomere diseases. *N Engl J Med*. https://doi.org/10.1056/NEJMoa1515319
- UK Prospective Diabetes Study Group. 1998. Effect of intensive blood-glucose control with metformin on complications in overweight patients with type 2 diabetes (UKPDS 34). *Lancet*. https://doi.org/10.1016/S0140-6736(98)07037-8
- Unity Biotechnology. 2020. 12-week data from UBX0101 phase 2 study in painful osteoarthritis of the knee (press release). https://www.globenewswire.com/news-release/2020/08/17/2079116/0/en/unity-biotechnology-announces-12-week-data-from-ubx0101-phase-2-clinical-study-in-patients-with-painful-osteoarthritis-of-the-knee.html
- Walton RG et al. 2019. Metformin blunts muscle hypertrophy in response to progressive resistance exercise training in older adults: the MASTERS trial. *Aging Cell*. https://doi.org/10.1111/acel.13039
- Waziry R et al. 2023. Effect of long-term caloric restriction on DNA methylation measures of biological aging in healthy adults from the CALERIE trial. *Nat Aging*. https://doi.org/10.1038/s43587-022-00357-y
- WHO. Healthy ageing and functional ability (Q&A). https://www.who.int/news-room/questions-and-answers/item/healthy-ageing-and-functional-ability
- Xu M et al. 2018. Senolytics improve physical function and increase lifespan in old age. *Nat Med*. https://doi.org/10.1038/s41591-018-0092-9
- Yoshino M et al. 2021. Nicotinamide mononucleotide increases muscle insulin sensitivity in prediabetic women. *Science*. https://doi.org/10.1126/science.abe9985
- Zinman B et al. 2015. Empagliflozin, cardiovascular outcomes, and mortality in type 2 diabetes (EMPA-REG OUTCOME). *N Engl J Med*. https://doi.org/10.1056/NEJMoa1504720
