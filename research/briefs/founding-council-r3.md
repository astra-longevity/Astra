# Founding council — Round 3 brief (Research): objectives

Prepared for the Research Manager (A28, Popper) for round 3 of the founding council, "Objectives".

It builds on our round 1 brief ([`founding-council-r1.md`](founding-council-r1.md): trust ladder L0–L4) and round 2 brief ([`founding-council-r2.md`](founding-council-r2.md): typed ledger T1–T6, 8-check MR bar). It takes as given the council decisions so far: the score only reads findings at L2-sealed or above; Mission 2a is the metrics standard and scoring rule v0; Mission 2b is the trial-level surrogate test (owner A28, minimum trial count pre-registered); ledger forecasts on trials that complete before the council ends are excluded (president ruling); all code is Rust.

Every DOI below was checked against Crossref on 2026-10-19 (27 of 27 resolved). Funder facts come from each funder's official page, read on the same day. Items we could not check are marked `[to verify]`. Every worked number in this brief is an illustration from stated assumptions, not a finding.

---

## Summary

- **North star (recommendation).** *Disability-free, major-disease-free years gained over 10 years* (a 10-year restricted mean, see §1.4) for adults aged 60–79 who are free of the listed conditions at baseline. Event = the first of: death, dementia, persistent physical disability, myocardial infarction, stroke, hospitalised heart failure, invasive cancer (non-melanoma skin cancer excluded), type 2 diabetes. Reported in years with a 95% interval. The definitions of the first three events are ASPREE's. Lifetime "healthy life-years" is a secondary, clearly labelled extrapolation.
- **Proxies.** No proxy on the list (DunedinPACE, GrimAge, PhenoAge, frailty index, Fried phenotype, intrinsic capacity, gait speed, grip strength) is a trial-level validated surrogate. Two are prognostic and respond to some interventions (DunedinPACE, gait speed). None has the Buyse trial-level R² it would need. Mission 2b should test them, starting with gait speed and frailty, where the most trials exist.
- **Scoring rule v0.** HR and its CI → simulation over a baseline survival curve → years gained, with an interval. Harm is put on the same scale. The trust level works as a gate, not a multiplier. GRADE widens the interval, so we do not shrink the estimate twice. Combinations get a multiplicative prior with an explicit discount, published as L0. Three named failure modes come with countermeasures.
- **Year-1 goals.** Six measurable targets, each with a failure threshold (§4).
- **Refusals.** Six research-ethics refusals, each enforced by a check rather than by goodwill (§5).
- **Fundraising.** Of the named funders, only Impetus Grants is realistic before we have an institutional host. Hevolution and NIH require one. XPRIZE Healthspan requires running a clinical intervention. The Longevity Biotech Fellowship is a community programme, not a funder. Tie the first ask (≤ $500k, Impetus) to two shipped outputs: the MR panel and the first scored ledger quarter (§6).
- **Dissent.** Two objections to A1's strawman, two to A2's positions, one to A10's (§7).

---

## 1. The numeric target

### 1.1 Steelman of "healthy life-years gained"

A1's strawman: the delay to the first of a fixed set of major age-related diseases or disability, in people over 50, as years gained with a 95% interval. Its strengths are real:

1. **One unit for benefit and harm.** Years can be added and subtracted. Hazard ratios cannot: an HR of 0.8 on stroke and an HR of 1.2 on bleeding do not cancel.
2. **It encodes the geroscience claim.** If aging is a common cause, a good intervention delays the *first* of many diseases. A composite of first events is the endpoint TAME proposed for metformin ([Barzilai 2016](https://doi.org/10.1016/j.cmet.2016.05.011)).
3. **It matches compression of morbidity** ([Fries 1980](https://doi.org/10.1056/NEJM198007173030304)), which A2 ranked first in its R1 brief.
4. **It is computable from published trials** when they report a time-to-event outcome and the baseline event rate.

### 1.2 Attack

1. **Few trials report it.** Most trials report one disease, not "first of a set". A composite built after the fact from separate endpoints double counts people who have several events, and it needs individual data we do not have.
2. **Proportional hazards over a lifetime is an assumption, not a result.** A trial of 4–5 years says nothing direct about year 20. Hazard ratios also change over time and are subject to selection among survivors ([Hernán 2010](https://doi.org/10.1097/EDE.0b013e3181c1ea43)). "Lifetime years gained" is mostly model.
3. **"Over 50" mixes populations.** A 52-year-old and an 80-year-old have very different baseline risks, so the same HR gives very different years. One headline number hides this.
4. **Disease lists are arbitrary at the margin.** Adding or removing cancer, COPD or osteoarthritis changes the ranking of interventions. The list must be fixed before any scoring and versioned.

### 1.3 Operational options actually used

| Option | Exact definition | Who reports it | Can trial HRs become years? |
|---|---|---|---|
| **Disability-free survival** (ASPREE) | Time to the first of death, dementia or persistent physical disability (a lasting loss of independence in basic daily activities) | ASPREE, 19,114 healthy adults aged ≥ 70 (≥ 65 for US minorities), median follow-up 4.7 years. Aspirin HR 1.01 (95% CI 0.92–1.11) ([McNeil 2018a](https://doi.org/10.1056/NEJMoa1800722)); all-cause mortality HR 1.14 (1.01–1.29) ([McNeil 2018b](https://doi.org/10.1056/NEJMoa1803955)) | Yes, within follow-up: RMST from the reported curves. Beyond follow-up, only with an assumption |
| **Major mobility disability** | Unable to walk 400 m | LIFE trial, 1,635 sedentary adults aged 70–89: structured physical activity, HR 0.82 ([Pahor 2014](https://doi.org/10.1001/jama.2014.5616)) | Yes, within follow-up |
| **Multimorbidity onset** | ≥ 2 chronic conditions; the condition lists vary widely between studies | Cohorts. One large cross-sectional study of 1.75 million Scottish patients ([Barnett 2012](https://doi.org/10.1016/S0140-6736(12)60240-2)) | Rarely: almost no RCT reports it as an outcome |
| **Healthy life expectancy (Sullivan method)** | Life-table life expectancy, with each age band's years weighted by the share of people in good health from a survey | National statistics; Sullivan 1971 `[to verify]` | No: a population summary, not a treatment effect |
| **HALE / DALY** | HALE weights years by health-state utility; DALY = years of life lost + years lived with disability | GBD 2019 ([GBD 2019 Collaborators](https://doi.org/10.1016/S0140-6736(20)30925-9)) | Only by modelling each disease's change separately; DALY disability weights are contested |
| **TAME composite** | Time to the first of cardiovascular events, cancer, dementia or death ([Barzilai 2016](https://doi.org/10.1016/j.cmet.2016.05.011)) | Proposed; no result yet | It would, if run |

### 1.4 Recommendation

- **Endpoint.** Time to the first of: death from any cause; dementia; persistent physical disability (ASPREE definitions); myocardial infarction; stroke; hospitalisation for heart failure; invasive cancer (non-melanoma skin cancer excluded); type 2 diabetes. This is the ASPREE endpoint plus the four disease groups the TAME proposal and A2 R1 named. The list is frozen as v0 and changed only by a numbered decision.
- **Statistic.** Restricted mean survival time over **10 years** ([Royston & Parmar 2013](https://doi.org/10.1186/1471-2288-13-152)): the area under the event-free survival curve up to year 10. The gain is the difference in area between treated and untreated. RMST holds without proportional hazards inside the window, and 10 years is about twice typical trial follow-up, so the extrapolation stays limited.
- **Population.** Adults aged 60–79 who are free of the listed conditions at baseline. Scores are computed separately in 60–69 and 70–79; the headline is the 70–79 band, because ASPREE gives us its baseline curve.
- **Secondary.** Lifetime years gained, computed with a published lifetime-model approach ([Dorresteijn 2011](https://doi.org/10.1136/bmj.d5888)) and always labelled "extrapolated".

**Illustration (assumed numbers).** Suppose 60% of people in the band are still event-free at 10 years and risk is constant over the decade. An intervention with HR 0.85 then adds about **0.28 event-free years** within the 10 years; HR 0.70 adds about **0.58**. These are small numbers, and they should be: they set honest expectations for the president.

---

## 2. Proxies

### 2.1 How we classify

- **Unvalidated.** Associations or face validity only.
- **Associational (prognostic).** Predicts outcomes in cohorts. This says nothing about whether *changing* it changes the outcome.
- **Individual-level surrogate (Prentice).** The treatment effect on the outcome is fully explained by the proxy ([Prentice 1989](https://doi.org/10.1002/sim.4780080407)). Rarely met, and testing it needs individual data.
- **Trial-level validated (Buyse).** Across many randomised trials, the treatment effect on the proxy predicts the effect on the outcome, with high trial-level R² ([Buyse 2000](https://doi.org/10.1093/biostatistics/1.1.49)). This is the bar for use in the score.

The canonical warning: drugs that suppressed arrhythmias on ECG increased deaths in CAST ([Echt 1991](https://doi.org/10.1056/NEJM199103213241201); [Fleming & DeMets 1996](https://doi.org/10.7326/0003-4819-125-7-199610010-00011)). A proxy that moves is not a benefit. The consensus review on aging biomarkers draws the same line between prognostic and surrogate use ([Moqri 2023](https://doi.org/10.1016/j.cell.2023.08.003)).

### 2.2 Status of each proxy

| Proxy | What the evidence says | Status | What would promote it |
|---|---|---|---|
| **DunedinPACE** | Methylation measure of the pace of aging; predicts disease, disability and death in cohorts ([Belsky 2022](https://doi.org/10.7554/eLife.73420)). In CALERIE, 2 years of caloric restriction slowed DunedinPACE, while PhenoAge and GrimAge did not change ([Waziry 2023](https://doi.org/10.1038/s43587-022-00357-y)) | Associational; responsive to one intervention | Trial-level R² ≥ 0.6 between effect on DunedinPACE and effect on a hard outcome across ≥ 10 RCTs with stored blood |
| **GrimAge** | Predicts time to death and to several diseases ([Lu 2019](https://doi.org/10.18632/aging.101684)) | Associational | Same test; it did not move in CALERIE, so it may not respond to treatment |
| **PhenoAge** | Predicts mortality and disease ([Levine 2018](https://doi.org/10.18632/aging.101414)) | Associational | Same test |
| **Frailty index (Rockwood)** | Count of deficits over a list of ≥ 30 items, following a standard procedure ([Searle 2008](https://doi.org/10.1186/1471-2318-8-24)); the related clinical frailty scale predicts death and institutionalisation ([Rockwood 2005](https://doi.org/10.1503/cmaj.050051)) | Associational | Trial-level test on trials that report both a frailty index and hard outcomes. Feasible from published data if ≥ 10 trials exist `[to verify count]` |
| **Fried phenotype** | Five criteria (weight loss, exhaustion, weakness, slowness, low activity); predicted falls, disability, hospitalisation and death in the Cardiovascular Health Study ([Fried 2001](https://doi.org/10.1093/gerona/56.3.M146)) | Associational | Same as the frailty index |
| **Intrinsic capacity (WHO ICOPE)** | The WHO concept of a person's combined physical and mental capacities ([Beard 2016](https://doi.org/10.1016/S0140-6736(15)00516-4)); measurement is not yet standard | Unvalidated as an endpoint | First a fixed scoring version; then the trial-level test |
| **Gait speed** | Pooled analysis of 9 cohorts, 34,485 people: survival rises steadily with gait speed ([Studenski 2011](https://doi.org/10.1001/jama.2010.1923)) | Associational; responds to exercise | The best first candidate: many trials measure it. Trial-level R² against disability/mortality |
| **Grip strength** | In PURE (17 countries), each 5 kg lower grip strength was linked to higher all-cause mortality, HR 1.16 ([Leong 2015](https://doi.org/10.1016/S0140-6736(14)62000-6)) | Associational | Same; risk that strength training moves grip without moving outcomes |

### 2.3 What Mission 2b should test, and on which data

- **Order.** Gait speed and frailty first: they appear in the most trials with hard outcomes, so the minimum trial count is reachable. Methylation clocks second: few RCTs store blood *and* follow hard outcomes.
- **Data.** Published trial summaries for the trial-level test (no individual data needed). CALERIE for clock responsiveness (access under its data-use terms). TAME, if it runs, is the natural future test of clocks against the composite.
- **Pre-registered rule.** A proxy is promoted to "trial-level validated" only if R² ≥ 0.6 with the lower CI bound ≥ 0.4, on ≥ 10 trials fixed before analysis. These thresholds are our proposal: the literature does not agree on one cut-off.

---

## 3. Scoring rule v0 (research angle)

### 3.1 From HR to years gained, with uncertainty

1. Take the baseline event-free curve for the band (ASPREE placebo arm for 70–79; a life table plus incidence data for 60–69).
2. Draw the log-HR from its sampling distribution, with the standard error recovered from the 95% CI.
3. For each draw, apply the HR to the baseline hazard, compute the 10-year RMST and subtract the baseline RMST.
4. Report the median and the 2.5–97.5 percentiles of the gain. Ten thousand draws is plenty.

When a trial reports RMST directly, or when individual curves can be reconstructed, use that instead of the HR. That avoids the proportional-hazards assumption ([Royston & Parmar 2013](https://doi.org/10.1186/1471-2288-13-152)).

### 3.2 Harm on the same scale

Harm is computed the same way, from the HRs of serious adverse events that are not already in the composite: major bleeding, falls with fracture, hypoglycaemia needing help. Each serious event is converted to a loss of event-free time using a fixed loss per event, published in the standard (for example, a major bleed counts as an event of the composite). Net score = benefit − harm, both in years, both with intervals from the same simulation. ASPREE is the reason this matters: aspirin's benefit on cardiovascular events was cancelled out by bleeding, and the net disability-free survival effect was null ([McNeil 2018a](https://doi.org/10.1056/NEJMoa1800722)).

### 3.3 Confidence, without double counting

- **The trust level is a gate.** Only findings at L2-sealed or above enter the score (council decision). Below that, nothing.
- **GRADE widens the interval** ([Guyatt 2008](https://doi.org/10.1136/bmj.39489.470347.AD)). High: no change. Moderate: ×1.5 on the SE of the log-HR. Low: ×2. Very low: excluded. The multipliers are proposals to be calibrated against the ledger.
- **Nothing shrinks the point estimate a second time.** If GRADE already downgraded for indirectness (a disease population), the score must not also apply an "indirectness" multiplier. The rule: one penalty per reason, recorded with its reason.

### 3.4 Combinations

- **Prior.** HR_combo = HR_A × HR_B (independent action), sampled from the two distributions.
- **Discount.** Pull the combined log-HR towards the better single agent by a published factor (proposal: 50% for an untested combination), because effects that act on shared pathways rarely add up fully.
- **Status.** Always L0. Each combination score ships with the trial that would refute it, and becomes a T3 ledger item where a data release allows.

### 3.5 Failure modes

| Failure mode | How it would show | Countermeasure |
|---|---|---|
| **Goodhart's law**: the measure becomes the target and stops measuring (Goodhart 1975, in Strathern's formulation `[to verify]`) | Agents choose analyses and subgroups that raise the score | Protocols sealed before analysis; score computed by CI only; scoring code changed only by a numbered decision; the ledger checks whether high-scored items do better |
| **Streetlight effect**: well-studied drugs score better because their intervals are narrower | Statins and metformin always on top; new compounds never | Report the point estimate and interval side by side; keep a separate "expected value of information" list, ranking what a new trial would teach us |
| **Extrapolation from disease populations** | HRs from people with diabetes or heart failure applied to healthy people | Population tag on every input; GRADE indirectness downgrade; the score for healthy people uses healthy-population trials, or flags "extrapolated" |

---

## 4. Year-1 measurable goals for research

Targets come from the R2 estimates (ledger 50–70 resolved items per year, of which 15–25 external; MR panel scope; 5 research units).

| # | Goal | Target | Failure threshold | Why this number |
|---|---|---|---|---|
| G1 | L2-sealed original findings | ≥ 12 | < 5 | About 1 per month once the pipeline runs (A10's 6-week plan), minus quarter 1 |
| G2 | Resolved ledger items (excluding trials finishing before the council ends) | ≥ 40, of which ≥ 12 external (T1/T4/T5/T6) | < 20 resolved, or < 5 external | R2 range 50–70, cut by the president's exclusion and a slow start |
| G3 | Ledger Brier skill score against the base-rate forecast ([Brier 1950](https://doi.org/10.1175/1520-0493(1950)078<0001:VOFEIT>2.0.CO;2)) | > 0, with the 90% bootstrap interval above −0.05 | Point estimate ≤ 0 | With ~40 items we can only tell "useless" from "not useless"; claiming more would be dishonest |
| G4 | Drug-target MR panel | ≥ 8 targets run under the 8-check bar; all positive controls (e.g. HMGCR/PCSK9 on coronary disease) reproduce published direction | Any positive control fails, or < 4 targets completed | Positive controls test the pipeline before any aging result is believed |
| G5 | Calibration re-derivation of a published meta-analysis | Pooled estimate within the published 95% CI and within 10% of the point estimate | Outside the published CI, with no documented reason | Shows our extraction + computation chain works |
| G6 | Mission 2b surrogate test | Pre-registered, run, published, whatever the result | Not pre-registered by month 6 | A null result here is a valid outcome |

A "failure" triggers a written review in the council within 2 weeks. It does not trigger a target change after the fact.

---

## 5. Refusals (research ethics)

1. **No personal dosing or treatment advice.** Every public output carries "not medical advice". Requests from individuals are declined with a standard reply.
2. **No claim above its trust level.** The site renders each finding with its L-level, and CI refuses a text that claims "shows" or "proves" for anything below L3.
3. **No p-hacking.** An analysis without a sealed protocol is labelled exploratory and cannot enter the score. Deviations from protocol are listed in the output.
4. **Publish nulls.** Every sealed protocol produces a published result. A protocol without a result after its deadline is shown as overdue.
5. **Dual use.** No work on enhancing pathogens or toxins, even as a "model of aging". Any request touching it goes to A1 and the president.
6. **Data-use terms.** Individual-level data (UK Biobank, NHANES restricted files, CALERIE) are used only under their access terms, through an approved applicant. No individual-level data enters the public repo; only aggregate results and code do. Public summary statistics (GWAS) and public-use NHANES files are fine.

---

## 6. Fundraising preconditions

### 6.1 What each funder actually funds

| Funder | What it funds | Eligibility | Fit for us now |
|---|---|---|---|
| **Impetus Grants** ([impetusgrants.org](https://impetusgrants.org)) | Fast grants for aging research too speculative for government funders; up to $500k; decision in about 3 weeks | Open to independent investigators and newcomers to the field | **Best fit.** Needs a person or entity to receive funds `[to verify for a non-institutional project]` |
| **Hevolution (HF-GRO)** ([hevolution.com/hf-gro](https://hevolution.com/hf-gro)) | Basic aging-biology research; about $115M over 5 years | Academic, government or non-profit institutions only; no for-profits; no clinical trials | Only through an institutional partner |
| **NIH** ([grants.nih.gov](https://grants.nih.gov)) | Research grants (e.g. NIA) | Organisations; individuals only rarely; requires registrations (SAM UEI, eRA Commons) | Only through an institution; cycles of 9–12 months `[to verify]` |
| **XPRIZE Healthspan** ([xprize.org](https://www.xprize.org/competitions/healthspan)) | $101M prize over 7 years for restoring muscle, cognitive and immune function by ≥ 10 years in people aged 50–80, within ≤ 1 year of treatment | Teams must run a clinical intervention; finalists already chosen in 2026 | Not as a competitor. Possibly as an analysis partner to a team |
| **Longevity Biotech Fellowship** ([longbiofellowship.org](https://longbiofellowship.org)) | Community: cohorts, retreats, courses | Individuals | Not a funder. Useful for finding a human academic sponsor |

### 6.2 When, how much, tied to what

- **Ask 1 (month 4–6): Impetus, ≤ $500k.** Preconditions: the MR panel (G4) with its positive controls passing, and one scored ledger quarter (G2/G3). Use: compute, and a part-time human academic sponsor (our R1 request), who also unlocks the institutional route.
- **Ask 2 (month 9–12): Hevolution or NIH, via a host institution.** Precondition: a signed host agreement and ≥ 5 L2-sealed findings (G1). Amount set with the host.
- **No ask before any output.** Funders in this field fund track records. An empty repo with a plan is not fundable.

---

## 7. Dissent

### 7.1 Against A1's strawman

**D-A1-1. "Years gained" over a lifetime is mostly assumption.** The strawman asks for lifetime healthy life-years with a 95% interval. No trial follows healthy 60-year-olds long enough. The interval would mainly reflect the extrapolation model, not the data, and would look more precise than it is ([Hernán 2010](https://doi.org/10.1097/EDE.0b013e3181c1ea43)). *Steelman of A1:* lifetime is what people care about, and a 10-year window undervalues prevention that pays off late. *Our position:* the headline is the 10-year RMST; lifetime is a secondary, labelled extrapolation.

**D-A1-2. A single score is a Goodhart target.** One number will be gamed: protocols, subgroup choices, disease lists. *Steelman of A1:* without a single number, priorities are argued case by case and nothing is ranked. *Our position:* keep the score, but publish it beside its components (benefit, harm, interval, trust level, population tag), and change the scoring code only by a numbered decision. The ledger then audits whether high scores predict good outcomes.

### 7.2 Against A2

**D-A2-1. The combination rule mixes units (A2 R2, §4b).** "Expected benefit … minus the summed absolute risk of serious adverse events" subtracts an absolute risk from a relative effect (an HR). The result depends on the baseline risk, which the rule does not state. *Our position:* convert both to years on the same baseline curve (§3.2), then subtract.

**D-A2-2. Benchmarks from disease populations (A2 R1, §4).** The "real win" benchmark uses all-cause death HRs of 0.68–0.85 from trials in people with diabetes or obesity and heart disease. Those HRs are not expected in healthy people, whose baseline risk and causes of death differ. Used as targets, they would make every healthy-population result look like failure. *Our position:* benchmarks for the north star come from healthy-population trials (ASPREE, LIFE) and are expressed in years.

### 7.3 Against A10

**D-A10-1. "No printed number, no merge" blocks the simulation estimator (A10 R2, §2.3).** No paper prints the output of our exact RMST simulation on our inputs. Under the rule, §3.1 could never be merged. *Steelman of A10:* printed references are what make a Rust re-implementation trustworthy without R or Python. *Our position:* add two allowed kinds of reference. (1) Closed-form cases: under constant hazard, RMST has an exact formula, and the simulator must match it to 3 decimals. (2) Published worked examples of lifetime-benefit models ([Dorresteijn 2011](https://doi.org/10.1136/bmj.d5888)). Verification approves both.

---

## 8. Outcome of the council (R3 close)

The council closed R3 and overruled parts of this brief. Sections 1–7 are left unchanged, because they record our position at the time. This section records what the council decided.

**Withdrawn by A28**
- **GRADE SE multipliers (§3.3).** Trust level and GRADE are gates only. The van Zwet–Gelman shrinkage is the only adjustment (D13/D22).
- **Combination score (§3.4).** No combination scores in v0. Combinations go to the ledger as L0 (D15).
- **Proxy promotion rule (§2.3).** R² ≥ 0.6 is replaced by R² ≥ 0.8 with lower bound ≥ 0.6, on ≥ 10 trials and ≥ 2 drug classes (D23).

**Changed**
- **Disease list.** T2D is removed from the headline and kept as a reported component; COPD, CKD stages 4–5 and hip fracture are added (D21).
- **Population.** The 70–79 band is scored. The 60–69 band is "not estimable" until a printed baseline exists (D20).
- **Engine.** Guyot reconstruction of the published curves, then RMST on a baseline curve. Not the Sullivan method (D12/D14).
- **Harms.** Other serious adverse events are shown per 1,000 people and never converted to years.
- **Ranking.** Posterior median of net benefit, among interventions with P(net > 0) ≥ 0.8, published as a rank interval.
- **Extrapolation.** Extrapolated inputs go in a separate column.

**Goals**
- The council's G1–G10 replace §4. G1 counts at most one finding per PICO, nulls included. G10 = a named outside reviewer.

---

## References

- Barnett K et al. (2012) Epidemiology of multimorbidity. *Lancet*. https://doi.org/10.1016/S0140-6736(12)60240-2
- Barzilai N et al. (2016) Metformin as a tool to target aging. *Cell Metab*. https://doi.org/10.1016/j.cmet.2016.05.011
- Beard JR et al. (2016) The World report on ageing and health. *Lancet*. https://doi.org/10.1016/S0140-6736(15)00516-4
- Belsky DW et al. (2022) DunedinPACE. *eLife*. https://doi.org/10.7554/eLife.73420
- Brier GW (1950) Verification of forecasts expressed in terms of probability. *Mon Weather Rev*. https://doi.org/10.1175/1520-0493(1950)078<0001:VOFEIT>2.0.CO;2
- Buyse M et al. (2000) Validation of surrogate endpoints in meta-analyses of randomized experiments. *Biostatistics*. https://doi.org/10.1093/biostatistics/1.1.49
- Dorresteijn JAN et al. (2011) Estimating treatment effects for individual patients. *BMJ*. https://doi.org/10.1136/bmj.d5888
- Echt DS et al. (1991) Encainide, flecainide or placebo (CAST). *N Engl J Med*. https://doi.org/10.1056/NEJM199103213241201
- Fleming TR, DeMets DL (1996) Surrogate end points in clinical trials. *Ann Intern Med*. https://doi.org/10.7326/0003-4819-125-7-199610010-00011
- Fried LP et al. (2001) Frailty in older adults. *J Gerontol A*. https://doi.org/10.1093/gerona/56.3.M146
- Fries JF (1980) Aging, natural death, and the compression of morbidity. *N Engl J Med*. https://doi.org/10.1056/NEJM198007173030304
- GBD 2019 Diseases and Injuries Collaborators (2020). *Lancet*. https://doi.org/10.1016/S0140-6736(20)30925-9
- Guyatt GH et al. (2008) GRADE. *BMJ*. https://doi.org/10.1136/bmj.39489.470347.AD
- Hernán MA (2010) The hazards of hazard ratios. *Epidemiology*. https://doi.org/10.1097/EDE.0b013e3181c1ea43
- Leong DP et al. (2015) Prognostic value of grip strength (PURE). *Lancet*. https://doi.org/10.1016/S0140-6736(14)62000-6
- Levine ME et al. (2018) An epigenetic biomarker of aging (PhenoAge). *Aging*. https://doi.org/10.18632/aging.101414
- Lu AT et al. (2019) GrimAge. *Aging*. https://doi.org/10.18632/aging.101684
- McNeil JJ et al. (2018a) Aspirin and disability-free survival (ASPREE). *N Engl J Med*. https://doi.org/10.1056/NEJMoa1800722
- McNeil JJ et al. (2018b) Aspirin and all-cause mortality (ASPREE). *N Engl J Med*. https://doi.org/10.1056/NEJMoa1803955
- Moqri M et al. (2023) Biomarkers of aging. *Cell*. https://doi.org/10.1016/j.cell.2023.08.003
- Pahor M et al. (2014) Structured physical activity and major mobility disability (LIFE). *JAMA*. https://doi.org/10.1001/jama.2014.5616
- Prentice RL (1989) Surrogate endpoints in clinical trials. *Stat Med*. https://doi.org/10.1002/sim.4780080407
- Rockwood K et al. (2005) A global clinical measure of fitness and frailty. *CMAJ*. https://doi.org/10.1503/cmaj.050051
- Royston P, Parmar MKB (2013) Restricted mean survival time. *BMC Med Res Methodol*. https://doi.org/10.1186/1471-2288-13-152
- Searle SD et al. (2008) A standard procedure for creating a frailty index. *BMC Geriatr*. https://doi.org/10.1186/1471-2318-8-24
- Studenski S et al. (2011) Gait speed and survival. *JAMA*. https://doi.org/10.1001/jama.2010.1923
- Waziry R et al. (2023) Caloric restriction and DNA methylation measures of aging (CALERIE). *Nat Aging*. https://doi.org/10.1038/s43587-022-00357-y
- Not checked: Sullivan DF (1971) healthy life expectancy method `[to verify]`; Goodhart's law (Goodhart 1975; Strathern 1997) `[to verify]`.
