# Founding council — Round 3 evidence brief (Literature, red team)

Prepared for the Literature Manager (A2) as input to round 3 (objectives) of the founding council. A2 is the assigned red team against the chair's strawman. The strawman makes **healthy life-years gained** the single numeric objective: delay to the first of a fixed set of major age-related diseases or disability, in people over 50, reported in years with a 95% interval. Each intervention gets a score = expected years gained − harm (serious adverse events converted to years) × a confidence factor (evidence grade, trust level L0–L4). Proxies (clocks, frailty indices, intrinsic capacity) are converted to years only once validated at trial level.

Section 1 attacks the strawman as hard as the evidence allows. Section 2 lists alternatives. Section 3 is A2's own view, kept separate on purpose. Sections 4–7 cover proxies, the scoring rule, year-1 goals and new decision items.

Every DOI below was checked on Crossref on 2026-10-19 (`founding-council-r3.check.tsv`, same folder). Official links were checked to load (HTTP 200) the same day. Anything not checked is marked [to verify].

---

## 1. Red-team case against "healthy life-years gained" as the single objective

**1.1 Years are not observed; they are extrapolated.** Trials report hazard ratios over a few years of follow-up. SELECT followed 17,604 people for about 3.3 years ([Lincoff 2023](https://doi.org/10.1056/NEJMoa2307563)); ASPREE followed 19,114 older adults for a median of 4.7 years ([McNeil 2018](https://doi.org/10.1056/NEJMoa1800722)). Turning a hazard ratio into "years gained" means projecting survival curves far past the data. Health technology assessment has studied this for a decade: the choice of parametric model (exponential, Weibull, Gompertz, log-logistic…) can change the estimated mean survival gain several-fold while fitting the observed data equally well ([Latimer 2013](https://doi.org/10.1177/0272989X12472398); [NICE DSU TSD 14 and 21](https://www.sheffield.ac.uk/nice-dsu/tsds/survival-analysis)). NICE requires these choices to be justified one by one and tested in scenario analyses. A single headline number in years therefore carries a modelling assumption that is usually larger than the trial's own uncertainty, and a 95% interval computed under one model hides it.

**1.2 Composites are dominated by their soft, frequent parts.** A "first of several diseases or disability" endpoint is a composite. A systematic review of cardiovascular trials found that the components of composites often differ widely in importance and in how much the treatment moves them, and that the less important, more frequent components tend to carry both the events and the effect ([Ferreira-González 2007](https://doi.org/10.1136/bmj.39136.682083.AE)). Composites are only interpretable when components have similar importance, similar frequency and similar effect ([Montori 2005](https://doi.org/10.1136/bmj.330.7491.594)). "Type 2 diabetes diagnosis" and "dementia" do not meet that test. A drug that lowers glucose would score as many "healthy years" as one that delays dementia by the same time, which no patient would accept as equivalent.

**1.3 Components can move in opposite directions.** ASPREE is the cautionary case: aspirin did not extend disability-free survival, and deaths were more frequent in the aspirin group ([McNeil 2018](https://doi.org/10.1056/NEJMoa1800722)). In other settings one component improves while another worsens; a composite that nets them silently is the outcome a red team should fear most.

**1.4 Competing risks.** Preventing one disease does not remove death; it moves it. With competing risks, standard Kaplan–Meier methods overstate the incidence of the event of interest, and cause-specific and subdistribution hazards answer different questions ([Austin 2016](https://doi.org/10.1161/CIRCULATIONAHA.115.017719)). In people over 70 the competing risk of death is large. An intervention that delays heart failure but leaves cancer untouched shifts disease, not health, and a first-event composite cannot tell the two apart.

**1.5 "Healthy" and "disabled" are value judgements.** Every summary of health in years needs weights: is a year with heart failure worth zero, half or 0.8 of a healthy year? The DALY and QALY literature shows these weights are contested on ethical and empirical grounds ([Anand & Hanson 1997](https://doi.org/10.1016/S0167-6296(97)00005-2); [Arnesen & Nord 1999](https://doi.org/10.1136/bmj.319.7222.1423)). Life-year metrics have long been criticised as ageist, because they value a year saved for an older person less ([Harris 1987](https://doi.org/10.1136/jme.13.3.117)). Global health-adjusted life expectancy estimates rely on such disability weights and still carry wide uncertainty ([Salomon 2012](https://doi.org/10.1016/S0140-6736(12)61690-0); [GBD 2019, Vos 2020](https://doi.org/10.1016/S0140-6736(20)30925-9)). The strawman avoids weights by making the endpoint binary ("first event"), but that is itself a weight: every listed disease = full loss of health, every unlisted one = none.

**1.6 One number hides harm and heterogeneity.** "Expected years gained − harm in years" can produce the same score for a safe drug with modest benefit and a toxic drug with large benefit in a narrow subgroup. Older populations are heterogeneous: multimorbidity is common and its prevalence rises steeply with age ([Marengoni 2011](https://doi.org/10.1016/j.arr.2011.03.003)). A population-average score will be wrong for most subgroups and, once published, it will be read as personal advice.

**1.7 Goodhart's law.** "When a measure becomes a target, it ceases to be a good measure" ([Strathern 1997](https://doi.org/10.1002/(SICI)1234-981X(199707)5:3<305::AID-EURO184>3.0.CO;2-4)). If Astra's units are ranked on years gained, they will gravitate toward interventions whose effect is easy to convert into years (frequent, soft, quickly diagnosed outcomes) and away from those whose benefit is real but hard to model (function, cognition, quality of life).

**1.8 Arbitrary choices reorder the ranking.** Population (over 50, 60 or 70), horizon (5, 10 years, lifetime) and discounting all change the answer. Discount rates alone can reverse the ranking of a prevention programme against a treatment, which is why their choice is one of the most debated issues in health economics ([Attema 2018](https://doi.org/10.1007/s40273-018-0672-z)). A preventive drug started at 50 pays off decades later; a 10-year horizon will undervalue it, a lifetime horizon will reward pure extrapolation.

**1.9 Few trials measure the composite.** Outside ASPREE (disability-free survival) and LIFE (major mobility disability, [Pahor 2014](https://doi.org/10.1001/jama.2014.5616)), almost no trial in older adults measures "first of a fixed list of age-related diseases or disability". Most scores would be assembled from separate disease endpoints in disease populations, then modelled. The objective would thus measure Astra's modelling choices more than the interventions.

**1.10 It excludes the president's ambitions.** The president's goal is "the best possible therapy" against aging. A delay-to-first-disease metric cannot reward reversal of biological age, recovery of function after onset, or better quality of life within a disease. It also scores zero for anything tested only on mechanistic or biomarker endpoints, which today is most of geroscience ([Moqri 2023](https://doi.org/10.1016/j.cell.2023.08.003)).

**Red-team verdict.** As a *single* objective, healthy life-years gained is a modelling output dressed as a measurement, sensitive to choices nobody can defend uniquely, and easy to game. It should not be the only number Astra optimises.

---

## 2. Alternatives

| Option | What it is | For | Against |
|---|---|---|---|
| **RMST difference** | Area between survival curves up to a fixed time τ: "event-free time gained over τ years" ([Royston & Parmar 2013](https://doi.org/10.1186/1471-2288-13-152); [Uno 2014](https://doi.org/10.1200/JCO.2014.55.2208)) | In years, so intuitive; no proportional-hazards assumption; no extrapolation beyond τ; computable from reconstructed curves | Depends on τ; needs survival curves, not just HRs; small numbers (weeks/months over 5–10 years) look unimpressive |
| **Win ratio / hierarchical composite** | Compare every treated–control pair on the most important outcome first (death), then the next (disability), etc. ([Pocock 2012](https://doi.org/10.1093/eurheartj/ehr352); [Redfors 2020](https://doi.org/10.1093/eurheartj/ehaa665)) | Encodes clinical priorities explicitly; fixes the "soft component dominates" problem | Needs individual data; not in years; result depends on follow-up and on the order chosen |
| **Disability-free survival (ASPREE-type)** | Composite of death, dementia, persistent physical disability ([McNeil 2018](https://doi.org/10.1056/NEJMoa1800722)) | Already used as a primary endpoint in a large trial of older adults; few, severe components | Only one large trial; insensitive to diseases that do not disable within 5 years |
| **Multimorbidity-free survival** | Time to accumulating ≥2 chronic diseases from an agreed list ([Calderón-Larrañaga 2017](https://doi.org/10.1093/gerona/glw233)) | Matches the geroscience idea that aging drives many diseases at once | Diagnosis counts depend on health-care contact; not in trial data |
| **Deficit accumulation** | Change in frailty index (share of 30+ deficits present) ([Mitnitski 2001](https://doi.org/10.1100/tsw.2001.58)) | Continuous, sensitive, reflects whole-body aging | A proxy (section 4), not an outcome |
| **Dashboard of 3–4 co-primary metrics** | e.g. mortality, disability-free survival, multimorbidity, serious harms | Nothing hidden; matches how regulators read trials | No single ranking; requires a rule to break ties |
| **Multi-criteria decision analysis** | Explicit weighted scoring across criteria with published weights | Makes values transparent and debatable | Weights are still subjective; can be gamed; heavy to run |

---

## 3. A2's own view (separate from the red-team case)

The strawman's instinct is right: Astra needs **one number to rank interventions**, or it will never say "this is better than that". The defensible form is narrower than the strawman:

**Headline metric: 10-year RMST gain in major-disease-free and disability-free survival, in people aged ≥65.**
- *Restricted* to 10 years: no extrapolation past the data horizon of the best trials plus modest projection, and every projection beyond observed follow-up is flagged as such.
- Computed **only** where randomised data (or L2+ evidence on the trust ladder) support it; otherwise the field reads "not estimable", never a modelled guess.
- **Always shown with its components** (death, each disease, disability), the serious-harm table, and the subgroup results. The headline never appears alone.
- Age ≥65 because that is where the absolute event rates make 10-year differences measurable; ≥50 as a secondary population.
- No discounting inside the 10-year window (keeps the number readable); discounting is a sensitivity analysis only.

**Proposed event list (first occurrence of any), with ICD-10 codes** ([WHO ICD-10 2019](https://icd.who.int/browse10/2019/en)):

| Component | ICD-10 |
|---|---|
| Death from any cause | — |
| Myocardial infarction | I21–I22 |
| Stroke | I60–I64 |
| Heart failure (hospitalised or new diagnosis) | I50 |
| Cancer, excluding non-melanoma skin cancer | C00–C97 except C44 |
| Dementia | F00–F03, G30 |
| Type 2 diabetes (new onset) | E11 |
| Chronic obstructive pulmonary disease | J44 |
| Chronic kidney disease stage 4–5 | N18.4–N18.5 |
| Hip fracture | S72.0–S72.2 |
| Persistent physical disability | see below |

This list covers the leading causes of death and disability in older adults in the Global Burden of Disease study ([Vos 2020](https://doi.org/10.1016/S0140-6736(20)30925-9)); the exact selection is a proposal for debate (D13). To answer the red team's point 1.2, type 2 diabetes is the softest item and is reported but could be dropped from the headline in a sensitivity analysis.

**Disability definition.** ASPREE's: inability or severe difficulty with at least one of six basic activities of daily living, persisting at least six months ([McNeil 2018](https://doi.org/10.1056/NEJMoa1800722)). Major mobility disability (inability to walk 400 m) as used in LIFE ([Pahor 2014](https://doi.org/10.1001/jama.2014.5616)) is the secondary option for trials that report it.

**What this keeps from the red team.** No lifetime extrapolation; components and harms always visible; a "not estimable" state; a separate quality-of-life and function track that does not feed the headline.

---

## 4. Proxies: prediction is not surrogacy

A proxy can **predict** outcomes in a cohort without being a **surrogate**, i.e. without a treatment's effect on the proxy predicting its effect on outcomes. Classic examples of predictive markers whose treatment-induced changes misled are reviewed by [Fleming & DeMets 1996](https://doi.org/10.7326/0003-4819-125-7-199610010-00011).

**Validation criteria.**
- *Prentice criteria* (individual level): the treatment's whole effect on the outcome passes through the surrogate ([Prentice 1989](https://doi.org/10.1002/sim.4780080407)). Rarely met.
- *Meta-analytic, trial level* ([Buyse 2000](https://doi.org/10.1093/biostatistics/1.1.49)): across many trials, the effect on the surrogate must predict the effect on the outcome; measured by trial-level R².
- *Framework for interpretation* ([Ciani 2023](https://doi.org/10.1016/j.eclinm.2023.102283)): grades levels of evidence for a surrogate, from biological plausibility to trial-level association, and stresses that validation is specific to the population and intervention class.
- **Proposed Astra rule:** a proxy may be converted into years only if trial-level R² ≥ 0.8 (lower 95% bound ≥ 0.6) across ≥ 5 randomised trials of ≥ 2 intervention classes. These cut-offs are an Astra convention for debate, not a published standard [to verify against regulatory guidance].

**Status of each proxy.**

| Proxy | Predicts outcomes? | Trial-level evidence | Verdict |
|---|---|---|---|
| GrimAge | Yes, strongly predicts lifespan and healthspan ([Lu 2019](https://doi.org/10.18632/aging.101684)) | Not moved by caloric restriction in CALERIE ([Waziry 2023](https://doi.org/10.1038/s43587-022-00357-y)); no trial-level validation | Exploratory |
| PhenoAge | Yes ([Levine 2018](https://doi.org/10.18632/aging.101414)) | Not moved in CALERIE ([Waziry 2023](https://doi.org/10.1038/s43587-022-00357-y)); none | Exploratory |
| DunedinPACE | Yes ([Belsky 2022](https://doi.org/10.7554/eLife.73420)) | Slowed by 2 years of caloric restriction in CALERIE, a small effect ([Waziry 2023](https://doi.org/10.1038/s43587-022-00357-y)); no link yet to hard outcomes in trials | Best candidate; exploratory |
| Frailty index (deficit accumulation) | Yes ([Mitnitski 2001](https://doi.org/10.1100/tsw.2001.58); [Rockwood 2005](https://doi.org/10.1503/cmaj.050051)) | Partly an outcome itself (it counts diseases and disabilities); not validated as surrogate | Secondary outcome, not surrogate |
| Fried frailty phenotype | Yes ([Fried 2001](https://doi.org/10.1093/gerona/56.3.M146)) | Same | Secondary outcome |
| WHO intrinsic capacity | Construct supported by domain evidence ([Cesari 2018](https://doi.org/10.1093/gerona/gly011); [Beard 2016](https://doi.org/10.1016/S0140-6736(15)00516-4)) | No standard scoring; no trial-level data | Not yet measurable consistently |

**TAME.** The TAME trial of metformin was designed to use a composite of age-related diseases as its primary outcome, precisely because no biomarker is validated ([Barzilai 2016](https://doi.org/10.1016/j.cmet.2016.05.011)); its biomarker workgroup proposed blood markers as secondary measures, not surrogates ([Justice 2018](https://doi.org/10.1007/s11357-018-0042-y)). Claims that metformin slows aging in humans rest mainly on observational and mechanistic data ([Kulkarni 2020](https://doi.org/10.1016/j.cmet.2020.04.001)). The biomarker field's own consensus is that no aging biomarker yet qualifies as a surrogate endpoint ([Moqri 2023](https://doi.org/10.1016/j.cell.2023.08.003)). TRIUMPH: [to verify]; not found in checked sources, not cited.

**Consequence.** The strawman's rule ("convert proxies only once trial-level validated") is right, and it means that **no proxy is convertible today**. Astra's surrogate-validation analysis (R1 brief, original output 4) is how this changes.

---

## 5. Critique of scoring rule v0

**5.1 A confidence multiplier is the wrong tool.** "Score × confidence factor" shrinks uncertain estimates toward zero but does not say how likely it is that the intervention helps. Two interventions with the same expected value and different uncertainty get different scores for the wrong reason, and the factor is a free parameter nobody can calibrate. Better: **propagate uncertainty**. Carry the full distribution of benefit (from the trial or meta-analysis, widened for indirectness and bias according to the trust level), the full distribution of harm, and report:
- median net benefit and its 95% interval;
- **P(net benefit > 0)**;
- P(net benefit > a minimal important difference, e.g. 3 months of RMST over 10 years) [threshold to debate].
Ranking uses P(net benefit > MID); ties broken by median. Trust levels feed in as variance inflation, not as a multiplier.

**5.2 Harm-to-years conversion.** Converting serious adverse events into years requires a weight per event type (a GI bleed ≠ a fatal bleed). Use the same RMST logic: fatal and disabling harms are already inside the headline (they end disability-free survival). Non-fatal, non-disabling serious harms are shown as absolute risk differences per 1,000 people over 10 years, **not** converted. This removes the most arbitrary conversion.

**5.3 Link to the R2 combination rule.** A2's R2 rule ranked untested combinations by multiplicative expected benefit, discounted by interaction credibility, minus summed serious-harm risk. Under the new rule: benefit of A+B = product of hazard ratios, with variance inflated by the interaction credibility score; harm = union of absolute harm risks; output = P(net benefit > MID), published as an L0 prediction with the trial that would refute it. Same idea, one consistent currency.

---

## 6. Year-1 goals, refusals, fundraising

**Measurable year-1 goals (proposals).**
1. **Outcome table**: 10-year RMST gain (or "not estimable") with components and harms for ≥ 15 intervention classes from the R1 ranking, each with its source trials and trust level.
2. **Reproducible engine**: the RMST/curve-reconstruction pipeline in Rust, validated against ≥ 3 published RMST results to within rounding.
3. **Surrogate-validation dataset**: all randomised trials reporting both an aging biomarker (clock, frailty index) and a hard outcome; target ≥ 20 trials, first trial-level R² estimate for at least one clock, whatever the result.
4. **Living reviews**: ≥ 2 living meta-analyses on age-related outcomes, updated at least quarterly.
5. **Predictions ledger**: ≥ 30 sealed predictions on ongoing trials, scored on resolution.
6. **External check**: ≥ 1 output reviewed by an independent academic or submitted as a preprint.

**What we refuse.**
- No personal medical advice, no dosing guidance, no "what should I take" answers.
- No headline number without components, harms and uncertainty.
- No proxy converted into years before trial-level validation (section 4).
- No ranking of an intervention on mouse or mechanistic data alone above one with human randomised data.
- No overclaiming: "slows aging" only with outcome data in humans.
- No sponsor-funded scoring of the sponsor's own product.

**Fundraising preconditions.** Funders of aging research reward credible, open outputs. What makes Astra fundable: (1) the public outcome table with its code; (2) a validated, reproducible engine; (3) at least one result reviewed outside Astra; (4) a governance page (conflicts of interest, refusals above). Candidate funders, links checked: [Impetus Grants](https://impetusgrants.org) (fast grants for aging research), [Hevolution Foundation](https://hevolution.com) (healthspan research funding), [XPRIZE Healthspan](https://www.xprize.org/prizes/healthspan) (prize for therapeutics restoring function in older adults; prize amount and team rules [to verify]). LongeVC: [to verify], not checked, not listed. Any commitment or signature goes through the president.

---

## 7. Proposed decision items

**D12 — Headline objective.** *Proposal:* 10-year RMST gain in major-disease-free and disability-free survival, age ≥65, computed only where evidence is L2+, always with components and harms. *Alternatives:* (a) strawman healthy life-years with extrapolation; (b) win ratio; (c) dashboard without a scalar.

**D13 — Event list and disability definition.** *Proposal:* the 10-component list in section 3 with ICD-10 codes; ASPREE persistent physical disability. *Alternatives:* drop type 2 diabetes; add depression, falls, vision loss; use multimorbidity (≥2 conditions) instead of first event.

**D14 — Scoring under uncertainty.** *Proposal:* rank on P(net benefit > MID) with trust levels as variance inflation; no confidence multiplier. *Alternatives:* keep the multiplier; rank on median only.

**D15 — Harm handling.** *Proposal:* fatal/disabling harms inside the headline; other serious harms as absolute risks, not converted. *Alternative:* convert all harms with published disability weights.

**D16 — Proxy promotion rule.** *Proposal:* trial-level R² ≥ 0.8 (lower bound ≥ 0.6), ≥ 5 trials, ≥ 2 classes. *Alternatives:* Prentice criteria only; case-by-case council vote.

**D17 — Second track for function and quality of life.** *Proposal:* a non-scalar dashboard (frailty index, mobility, cognition, quality of life) published next to the headline, never summed into it. *Alternative:* fold into the headline through MCDA weights.

**D18 — Refusals list** as section 6, adopted into the charter.

---

## References

- Anand S, Hanson K. 1997. Disability-adjusted life years: a critical review. *J Health Econ*. https://doi.org/10.1016/S0167-6296(97)00005-2
- Arnesen T, Nord E. 1999. The value of DALY life: problems with ethics and validity of disability adjusted life years. *BMJ*. https://doi.org/10.1136/bmj.319.7222.1423
- Attema AE et al. 2018. Discounting in economic evaluations. *PharmacoEconomics*. https://doi.org/10.1007/s40273-018-0672-z
- Austin PC et al. 2016. Introduction to the analysis of survival data in the presence of competing risks. *Circulation*. https://doi.org/10.1161/CIRCULATIONAHA.115.017719
- Barzilai N et al. 2016. Metformin as a tool to target aging. *Cell Metab*. https://doi.org/10.1016/j.cmet.2016.05.011
- Beard JR et al. 2016. The World report on ageing and health: a policy framework for healthy ageing. *Lancet*. https://doi.org/10.1016/S0140-6736(15)00516-4
- Belsky DW et al. 2022. DunedinPACE, a DNA methylation biomarker of the pace of aging. *eLife*. https://doi.org/10.7554/eLife.73420
- Buyse M et al. 2000. The validation of surrogate endpoints in meta-analyses of randomized experiments. *Biostatistics*. https://doi.org/10.1093/biostatistics/1.1.49
- Calderón-Larrañaga A et al. 2017. Assessing and measuring chronic multimorbidity in the older population: a proposal for its operationalization. *J Gerontol A*. https://doi.org/10.1093/gerona/glw233
- Cesari M et al. 2018. Evidence for the domains supporting the construct of intrinsic capacity. *J Gerontol A*. https://doi.org/10.1093/gerona/gly011
- Ciani O et al. 2023. A framework for the definition and interpretation of the use of surrogate endpoints in interventional trials. *eClinicalMedicine*. https://doi.org/10.1016/j.eclinm.2023.102283
- Ferreira-González I et al. 2007. Problems with use of composite end points in cardiovascular trials: systematic review of randomised controlled trials. *BMJ*. https://doi.org/10.1136/bmj.39136.682083.AE
- Fleming TR, DeMets DL. 1996. Surrogate end points in clinical trials: are we being misled? *Ann Intern Med*. https://doi.org/10.7326/0003-4819-125-7-199610010-00011
- Fried LP et al. 2001. Frailty in older adults: evidence for a phenotype. *J Gerontol A*. https://doi.org/10.1093/gerona/56.3.M146
- Harris J. 1987. QALYfying the value of life. *J Med Ethics*. https://doi.org/10.1136/jme.13.3.117
- Justice JN et al. 2018. A framework for selection of blood-based biomarkers for geroscience-guided clinical trials: report from the TAME Biomarkers Workgroup. *GeroScience*. https://doi.org/10.1007/s11357-018-0042-y
- Kulkarni AS et al. 2020. Benefits of metformin in attenuating the hallmarks of aging. *Cell Metab*. https://doi.org/10.1016/j.cmet.2020.04.001
- Latimer NR. 2013. Survival analysis for economic evaluations alongside clinical trials — extrapolation with patient-level data. *Med Decis Making*. https://doi.org/10.1177/0272989X12472398
- Levine ME et al. 2018. An epigenetic biomarker of aging for lifespan and healthspan. *Aging*. https://doi.org/10.18632/aging.101414
- Lincoff AM et al. 2023. Semaglutide and cardiovascular outcomes in obesity without diabetes. *N Engl J Med*. https://doi.org/10.1056/NEJMoa2307563
- Lu AT et al. 2019. DNA methylation GrimAge strongly predicts lifespan and healthspan. *Aging*. https://doi.org/10.18632/aging.101684
- Marengoni A et al. 2011. Aging with multimorbidity: a systematic review of the literature. *Ageing Res Rev*. https://doi.org/10.1016/j.arr.2011.03.003
- McNeil JJ et al. 2018. Effect of aspirin on disability-free survival in the healthy elderly. *N Engl J Med*. https://doi.org/10.1056/NEJMoa1800722
- Mitnitski AB et al. 2001. Accumulation of deficits as a proxy measure of aging. *ScientificWorldJournal*. https://doi.org/10.1100/tsw.2001.58
- Montori VM et al. 2005. Validity of composite end points in clinical trials. *BMJ*. https://doi.org/10.1136/bmj.330.7491.594
- Moqri M et al. 2023. Biomarkers of aging for the identification and evaluation of longevity interventions. *Cell*. https://doi.org/10.1016/j.cell.2023.08.003
- NICE Decision Support Unit. Technical Support Documents on survival analysis (TSD 14, TSD 21). https://www.sheffield.ac.uk/nice-dsu/tsds/survival-analysis
- Pahor M et al. 2014. Effect of structured physical activity on prevention of major mobility disability in older adults. *JAMA*. https://doi.org/10.1001/jama.2014.5616
- Pocock SJ et al. 2012. The win ratio: a new approach to the analysis of composite endpoints in clinical trials based on clinical priorities. *Eur Heart J*. https://doi.org/10.1093/eurheartj/ehr352
- Prentice RL. 1989. Surrogate endpoints in clinical trials: definition and operational criteria. *Stat Med*. https://doi.org/10.1002/sim.4780080407
- Redfors B et al. 2020. The win ratio approach for composite endpoints: practical guidance based on previous experience. *Eur Heart J*. https://doi.org/10.1093/eurheartj/ehaa665
- Rockwood K et al. 2005. A global clinical measure of fitness and frailty in elderly people. *CMAJ*. https://doi.org/10.1503/cmaj.050051
- Royston P, Parmar MKB. 2013. Restricted mean survival time: an alternative to the hazard ratio for the design and analysis of randomized trials with a time-to-event outcome. *BMC Med Res Methodol*. https://doi.org/10.1186/1471-2288-13-152
- Salomon JA et al. 2012. Healthy life expectancy for 187 countries, 1990–2010. *Lancet*. https://doi.org/10.1016/S0140-6736(12)61690-0
- Strathern M. 1997. 'Improving ratings': audit in the British university system. *European Review*. https://doi.org/10.1002/(SICI)1234-981X(199707)5:3<305::AID-EURO184>3.0.CO;2-4
- Uno H et al. 2014. Moving beyond the hazard ratio in quantifying the between-group difference in survival analysis. *J Clin Oncol*. https://doi.org/10.1200/JCO.2014.55.2208
- Vos T et al. 2020. Global burden of 369 diseases and injuries in 204 countries and territories, 1990–2019. *Lancet*. https://doi.org/10.1016/S0140-6736(20)30925-9
- Waziry R et al. 2023. Effect of long-term caloric restriction on DNA methylation measures of biological aging in healthy adults from the CALERIE trial. *Nat Aging*. https://doi.org/10.1038/s43587-022-00357-y
- WHO. ICD-10 version 2019. https://icd.who.int/browse10/2019/en
- Organisations (official sites): Impetus Grants https://impetusgrants.org; Hevolution Foundation https://hevolution.com; XPRIZE Healthspan https://www.xprize.org/prizes/healthspan
