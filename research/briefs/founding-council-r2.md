# Founding council — Round 2 brief (Research)

Prepared for the Research Manager (A28) as input to round 2 (organisation) of the founding council.

**Scope.** Round 2 is about structure. This brief answers the chair's two challenges to the Research position from round 1 (A: can a prediction ledger resolve fast enough to matter? B: can Mendelian randomisation on aging outcomes be trusted?), extracts design lessons from organisations that already produce trusted knowledge, critiques the chair's strawman from the research side, and proposes a concrete Research division for year 1.

It builds on the round 1 brief ([`founding-council-r1.md`](founding-council-r1.md)), in particular its trust ladder (L0–L4, section 3). Every DOI below was checked on Crossref on 2026-10-19; trial identifiers were pulled from the ClinicalTrials.gov API v2 the same day.

---

## Summary

- **Challenge A (ledger speed).** Trial outcomes alone will not reach 50 resolved predictions in year 1: we found 29 registered aging-relevant trials with a primary completion date between October 2026 and December 2027, most small and academic, and many outside the legal duty to post results within 12 months. 50 per year is reachable only with a *typed* ledger that also scores trial process events, held-out replications of our own analyses, open challenges and ITP papers. Realistic rate: 12–20 resolutions per quarter from month 4. Only external, independent resolutions count toward L3.
- **Challenge B (MR on aging outcomes).** Usable, but as supporting evidence only. Aging outcome GWAS (parental lifespan, healthspan, frailty, epigenetic clocks) carry selection and survival bias and mostly reflect a few disease pathways. Our bar: positive control, colocalisation, robust estimators that agree, replication in a non-overlapping cohort, consistency across at least two aging outcomes, a multivariable check against known disease factors, and a pre-registered protocol. MR alone caps at L2.
- **Structure.** Keep the chair's split between production and verification, keep the method × program matrix, but give *method* leads the final say on how an analysis is done and *program* leads the final say on what is asked. Put pre-registration under Verification's custody, never under the producing unit. Draw the A2/A28 line at "collect vs compute".

---

## 1. Challenge A: which predictions resolve within 3–6 months?

### 1.1 Trial results: real but slow

US law (FDAAA 801 and its final rule, 42 CFR Part 11) requires results for *applicable clinical trials* to be posted on ClinicalTrials.gov within 12 months of the primary completion date ([FDAAA 801 final rule](https://clinicaltrials.gov/policy/fdaaa-801-final-rule); [42 CFR 11](https://www.ecfr.gov/current/title-42/chapter-I/subchapter-A/part-11)). "Applicable" excludes phase 1 trials and most dietary-supplement studies without a drug or device claim, which covers much of the aging field.

We queried the [ClinicalTrials.gov API](https://clinicaltrials.gov/data-api/api) for recruiting or active trials whose interventions include geroscience candidates and whose conditions include aging, frailty, sarcopenia or older adults, with a primary completion date between 2026-10-01 and 2027-12-31:

| Class | Trials (NCT id, primary completion, enrolment) |
|---|---|
| Senolytics: fisetin | NCT07195318 (2026-10, 120), NCT03675724 (2026-11, 40), NCT03430037 (2026-11, 40), NCT06399809 (2026-11, 34), NCT04733534 (2026-12, 120), NCT06819254 (2026-12, 60) |
| Senolytics: dasatinib + quercetin | NCT02848131 (2027-04, 30), NCT05838560 (2027-10, 40) |
| mTOR inhibitors | NCT04742777 (2026-10, 20), NCT05949658 (2027-03, 72), NCT07191353 (2027-03, 148), NCT05414292 (2027-05, 16), NCT06727305 (2027-09, 60), NCT07728175 (2027-12, 225) |
| NAD+ precursors | NCT07144527 (2026-12, 40), NCT06907329 (2027-03, 126), NCT05483465 (2027-06, 214), NCT03821623 (2027-09, 118), NCT06208527 (2027-12, 100) |
| Metformin | NCT03107884 (2026-11, 64), NCT06463743 (2027-01, 30) |
| GLP-1 receptor agonists | NCT07702461 (2026-10, 300), NCT07293325 (2027-04, 66) |
| Urolithin A | NCT07231783 (2027-10, 120) |

(Some search hits not relevant to aging, e.g. oncology protocols using dasatinib, were removed. We found no large phase 3 SGLT2 or GLP-1 trial with an aging-type primary endpoint completing in the window.)

**Reading.** About 25 relevant trials, median size about 60 participants. If results appear 6–18 months after primary completion, perhaps 8–12 will resolve inside year 1. Small trials are also noisy, so each resolution says little about our calibration. Trial outcomes are the most valuable resolutions but cannot carry the ledger alone.

### 1.2 Other sources with a known cadence

| Type | Source | Cadence | Resolves | Counts as external (L3)? |
|---|---|---|---|---|
| T1 Trial outcome | Trials above: "primary endpoint met at p<0.05, direction X" | Continuous | 8–12 / year | Yes |
| T2 Trial process | Same trials: "results posted by date D", "enrolment reached", "trial terminated early" | Continuous, fast | 15–25 / year | No (calibration only) |
| T3 Held-out replication | Our own pre-registered estimates checked on a data release we have not opened (e.g. an MR estimate from UK Biobank predicted before running it in FinnGen) | We control timing | 10–20 / year | Partly: L2 "held-out", not L3, because we chose the data |
| T4 Open challenge | Biomarkers of Aging Challenge: phases scored on held-out data with a public leaderboard; the 2024 edition ran an age phase (March–July) and a mortality phase (July–November), 152 teams from 28 countries ([challenge](https://www.agingconsortium.org/challenge); [paper](https://pmc.ncbi.nlm.nih.gov/articles/PMC11565782/)) | Phases of 4–5 months | 2–6 / year | Yes |
| T5 ITP | NIA Interventions Testing Program results for compounds already in test ([NIA ITP](https://www.nia.nih.gov/research/dab/interventions-testing-program-itp); design: [Miller 2007](https://doi.org/10.1111/j.1474-9726.2007.00311.x), [Nadon 2008](https://doi.org/10.1007/s11357-008-9048-1)) | Cohorts start yearly; papers roughly yearly, years after start | 2–5 / year | Yes |
| T6 Regulatory | FDA/EMA decisions on drugs with an aging-relevant indication | Rare in our field | 0–3 / year | Yes |

**Estimate.** Quarter 1: 0–5 (ledger being built; T2 only). From quarter 2: 12–20 per quarter, of which 3–6 are external outcome resolutions (T1, T4, T5, T6). Year 1 total: 50–70 resolved items, of which roughly 15–25 can support an L3 claim. The 50 target is met, but only if T2 and T3 count for *calibration* while L3 is reserved for external outcomes.

### 1.3 Is predicting this even possible?

Yes, modestly, and that is the point of measuring it.

- Prediction markets among researchers predicted which psychology studies would replicate better than individual surveys did ([Dreber 2015](https://doi.org/10.1073/pnas.1516179112); [Camerer 2018](https://doi.org/10.1038/s41562-018-0399-z)).
- Experts and non-experts forecast the results of a large behavioural experiment; averaged forecasts were informative, and expertise added less than expected ([DellaVigna 2018](https://doi.org/10.1086/699976)).
- For clinical trials the bar is low: in 148 experts making 1,217 forecasts on 25 cancer trials, feedback training did not improve accuracy ([Doussau 2023](https://doi.org/10.1177/17407745231203375)).
- Trained forecasters working in teams beat other forecasters in a large geopolitical tournament ([Mellers 2014](https://doi.org/10.1177/0956797614524255)). Public platforms such as [Metaculus](https://www.metaculus.com) give a human benchmark we can compare against on shared questions.

**Design rules for the ledger.** Every item is a probability (not yes/no), has a resolution source fixed in advance, and is scored with a proper rule (Brier or log score). Items are typed (T1–T6) and scored separately, so cheap fast items cannot hide poor calibration on hard ones. Each agent's forecast is compared with a baseline (base rate of positive trials in the field), so we learn whether our analyses add anything.

---

## 2. Challenge B: can MR on aging outcomes be trusted?

### 2.1 The outcome GWAS available

| Outcome | Source | Strength | Main weakness |
|---|---|---|---|
| Parental lifespan | [Timmers 2019](https://doi.org/10.7554/eLife.39856) (about 1 million parent lifespans, UK Biobank + LifeGen) | Huge sample | Proxy (parents' age at death); dominated by a few loci (APOE, LPA, CHRNA5, CDKN2B-AS1); participant selection |
| Healthspan / lifespan / longevity combined | [Timmers 2020](https://doi.org/10.1038/s41467-020-17312-3) | Multivariate, more power | Mixes outcomes; same cohorts as above |
| Extreme longevity (≥90th/99th percentile) | [Deelen 2019](https://doi.org/10.1038/s41467-019-11558-2) | Direct phenotype | Small case numbers, low power |
| Frailty index | [Atkins 2021](https://doi.org/10.1111/acel.13459) | Function, not death | Low heritability, UK Biobank healthy-volunteer bias |
| Epigenetic clocks | [McCartney 2021](https://doi.org/10.1186/s13059-021-02398-9) (137 loci) | Molecular, many cohorts | A clock is a surrogate; validation as an outcome is unsettled ([Moqri 2023](https://doi.org/10.1016/j.cell.2023.08.003)) |
| Disease endpoints, multimorbidity | [FinnGen, Kurki 2023](https://doi.org/10.1038/s41586-022-05473-8); [UK Biobank, Sudlow 2015](https://doi.org/10.1371/journal.pmed.1001779) | Registry-based, many endpoints, two independent cohorts | Disease-by-disease; aging signal must be built from many endpoints |

### 2.2 Known failure modes

- **Pleiotropy.** A variant affects lifespan through a path other than the drug target. Partly detectable with MR-Egger ([Bowden 2015](https://doi.org/10.1093/ije/dyv080)) and the weighted median ([Bowden 2016](https://doi.org/10.1002/gepi.21965)), but drug-target MR often has one gene region, where these estimators have little power.
- **Selection and survival bias.** UK Biobank participants are healthier than the population, and in lifespan outcomes people must survive to be studied; both bias instrumental-variable estimates ([Hughes 2019](https://doi.org/10.1097/EDE.0000000000000972)).
- **Index-event bias.** Studies of progression or outcomes among people who already had an event are biased ([Dudbridge 2019](https://doi.org/10.1038/s41467-019-09381-w)); relevant for "survival after disease" outcomes.
- **Confounding by linkage.** The instrument's signal sits near, but is not, the target gene. Colocalisation tests whether exposure and outcome share one causal variant ([Giambartolomei 2014](https://doi.org/10.1371/journal.pgen.1004383)).
- **"Aging" that is really one disease.** Much of the lifespan GWAS signal runs through cardiovascular disease and smoking ([Timmers 2019](https://doi.org/10.7554/eLife.39856)). A drug that lowers LDL will look "pro-longevity" for that reason alone. Multivariable MR can separate these paths ([Sanderson 2018](https://doi.org/10.1093/ije/dyy262)).

### 2.3 Track record against trials

MR has both confirmed and anticipated trial results, and has also corrected observational errors:

- **Confirmed a trial side effect (positive control).** Variants in *HMGCR*, the statin target, raise type 2 diabetes risk and body weight, matching the excess diabetes seen in statin trials ([Swerdlow 2015](https://doi.org/10.1016/S0140-6736(14)61183-1)).
- **Corrected observational data.** Genetically higher HDL cholesterol did not lower myocardial infarction risk, contrary to observational studies ([Voight 2012](https://doi.org/10.1016/S0140-6736(12)60312-2)).
- **Anticipated a trial direction.** A variant in *IL6R* that dampens IL-6 signalling was associated with lower coronary risk ([IL6R MR Consortium 2012](https://doi.org/10.1016/s0140-6736(11)61931-4)); later, blocking the upstream IL-1β with canakinumab reduced cardiovascular events in a large trial ([Ridker 2017](https://doi.org/10.1056/NEJMoa1707914)).

All three are cardiometabolic. There is no comparable track record for aging outcomes yet, which is why we treat aging-outcome MR as L2 at most until our own ledger shows it predicts.

### 2.4 Proposed evidentiary bar

An MR finding on an aging outcome reaches L2 only if all of the following hold. Each is a check in CI, not a judgement call.

1. **Pre-registered protocol** in the repo before any outcome data is touched: exposure, instruments, outcomes, estimators, thresholds (section 4a).
2. **Positive control.** The instrument reproduces the drug's known trial effect on its biomarker or a known clinical outcome (e.g. LDL for *HMGCR*).
3. **Colocalisation.** Posterior probability of a shared causal variant (coloc PP.H4) ≥ 0.8 in the target region.
4. **Robustness.** Where ≥ 3 independent instruments exist: inverse-variance weighted, MR-Egger and weighted median agree in direction, and the Egger intercept is not significant. Where they do not, the finding is labelled "single-region" and capped at L1.
5. **Replication** in a non-overlapping cohort (UK Biobank-based discovery, FinnGen replication, or the reverse), same direction, overlapping confidence intervals.
6. **Consistency across outcomes.** Same direction on at least two different aging outcomes (e.g. parental lifespan and frailty index).
7. **Not just a known disease factor.** Multivariable MR adjusting for the main known paths (LDL, blood pressure, BMI, smoking) leaves an effect.
8. **Reporting** follows STROBE-MR ([Skrivankova 2021, JAMA](https://doi.org/10.1001/jama.2021.18236); [explanation, BMJ](https://doi.org/10.1136/bmj.n2233)) and current MR guidelines ([Burgess 2020](https://doi.org/10.12688/wellcomeopenres.15555.2)).

A finding that clears the bar becomes a ledger entry: a dated, probabilistic prediction about the direction of the next trial of that drug class on a functional or aging outcome. That is how MR can climb to L3.

---

## 3. Benchmarks for organisation

| Organisation | Structure | Lesson for us |
|---|---|---|
| **Cochrane** | Review groups by health topic; separate methods groups that set standards across topics; a shared handbook; protocols published before reviews ([about](https://www.cochrane.org/about-us); [Handbook](https://training.cochrane.org/handbook)) | **Topic × method matrix** works when methods groups own *standards*, not individual reviews. Protocol is published before work starts. |
| **NIA ITP** | Three independent testing sites, one shared protocol, central steering committee; compounds proposed by outside researchers and selected centrally ([Miller 2007](https://doi.org/10.1111/j.1474-9726.2007.00311.x); [Nadon 2008](https://doi.org/10.1007/s11357-008-9048-1)) | **Replication built in from the start**, not added later. Anyone may propose; one body selects. |
| **Particle physics (CERN experiments)** | Blind analysis: the signal region stays hidden until the method is frozen ([Klein 2005](https://doi.org/10.1146/annurev.nucl.55.090704.151521)); two independent detectors and collaborations confirmed the Higgs boson ([CERN](https://home.cern/science/physics/higgs-boson)) | **Freeze the method before seeing the answer**; confirmation comes from a team that did not build the first result. |
| **FinnGen / UK Biobank** | Central data production; open summary statistics; many independent analysts ([Kurki 2023](https://doi.org/10.1038/s41586-022-05473-8); [Sudlow 2015](https://doi.org/10.1371/journal.pmed.1001779)) | **Platform separated from analysis.** Data teams do not judge results. |
| **Biomarkers of Aging Consortium** | Consensus criteria plus an open challenge scored on held-out data ([Moqri 2023](https://doi.org/10.1016/j.cell.2023.08.003); [challenge](https://www.agingconsortium.org/challenge)) | **The scorer is not a competitor.** Held-out data and the leaderboard sit outside the teams. |
| **Forecasting tournaments** | Questions written and resolved by organisers, forecasters scored by a proper rule ([Mellers 2014](https://doi.org/10.1177/0956797614524255)) | **Question writers and resolvers are separate from forecasters.** |

**Common pattern.** (1) The unit that produces a result never certifies it. (2) Methods and standards are owned centrally; topics are owned locally. (3) The method is frozen before the answer is seen. (4) Work is proposed openly but selected by one body.

---

## 4. Critique of the chair's strawman

The strawman (production matrix of method units × intervention programs, independent Verification reporting to A1, Platform, Strategy & Metrics) matches the benchmark pattern. Six points need settling.

**(a) Where pre-registration lives.** The producing unit writes the protocol; Verification *receives and seals* it (commit hash plus timestamp proof) before any data is opened. No analysis output is accepted without a sealed protocol older than the first data access. Deviations are allowed but listed in the report. This is Cochrane's protocol rule plus the physics freeze.

**(b) Ledger: no self-scoring.** Production units write predictions; Verification fixes resolution sources at entry time and scores. Strategy & Metrics owns the scoring rule, not individual scores. Nobody may edit a prediction after entry; corrections are new entries.

**(c) A2/A28 boundary.** Proposed rule: **A2 collects, A28 computes.** A2 (Literature) finds, screens and extracts evidence into the repo's structured data, and keeps the narrative synthesis. A28 (Research) produces new numbers from it: meta-analyses, MR, target-trial emulation, re-analyses, forecasts. Where they meet (a meta-analysis), A2 delivers dual, blinded extraction to the shared schema; A28 owns the statistical model. This avoids two units both owning "evidence synthesis" and matches Cochrane's split between review authors and methods groups.

**(d) Program leads vs method leads.** Program leads decide *what* is asked and in what order (they own questions and priorities within their area). Method leads decide *how* (tools, thresholds, the evidentiary bar in 2.4) and can refuse an analysis that cannot meet the bar. Disputes go to Strategy & Metrics on priority and to Verification on validity; neither side overrides the other alone.

**(e) Mission lifecycle for an analysis.**

```mermaid
flowchart LR
  Q[Question from program lead] --> P[Protocol written by method unit]
  P --> S[Sealed by Verification: L0]
  S --> R[CI run from scratch: L1]
  R --> B[Sealed re-derivation by second agent: L2-sealed]
  B --> D[Ledger entry with resolution source]
  D --> E[External outcome agrees: L3]
  E --> F[Partner or peer review: L4]
```

A mission closes at whichever level its brief targets; closing below target is allowed but recorded.

**(f) Span and depth at 1,000+ agents.** With at most 7 direct reports and depth 4, the tree holds 1 + 7 + 49 + 343 + 2,401 roles, so 1,000 agents fit. But the binding limit is review capacity, not span: if each finding needs a second blinded agent (L2), Verification must be about as large as production for the methods that matter. Plan for Verification at roughly one third of research headcount, and size by findings per week, not by org chart.

---

## 5. Proposed Research division, year 1

Five units under A28 (within the 7-report limit), each a method unit serving all programs.

| Unit | Scope | First output | Original value |
|---|---|---|---|
| **R1 Meta-analysis** | Pooled estimates from A2's extractions | Living meta-analysis of one intervention class | Pooled effects nobody has pooled |
| **R2 Causal genetics** | Drug-target MR under the bar in 2.4 | MR panel of geroscience drug targets on aging outcomes | Causal estimates for aging outcomes |
| **R3 Re-analysis and emulation** | Public omics re-analysis; target-trial emulation where public individual data allows | Re-analysis of one public intervention dataset | Checks of published claims |
| **R4 Forecasting and ledger** | Writes ledger items, runs the internal tournament, tracks trials in 1.1 | Ledger v1 with all ~25 trials in 1.1 forecast | Measured calibration from month 1 |
| **R5 Methods and protocols** | Protocol templates, evidentiary bars, CI analysis templates (with Platform) | Protocol template + MR bar as CI checks | Reusable, auditable standards |

Verification (replication, sealing, scoring) stays outside the division, reporting to A1, as the strawman proposes.

### First three missions

1. **Trial-outcome ledger v1 (R4).** Forecast every trial in section 1.1 (primary endpoint met, direction, results posted by date), plus process items. *Success:* ≥ 40 sealed items by month 2; first 10 resolutions scored by Verification by month 6; Brier score reported against a base-rate baseline.
2. **Drug-target MR panel (R2 + R5).** Targets of metformin-relevant pathways, mTOR, IL-6/IL-1, SGLT2, GLP1R, HMGCR, PCSK9 on parental lifespan, healthspan and frailty, discovery in UK Biobank-based GWAS and replication in FinnGen. *Success:* protocol sealed before any outcome is opened; positive controls pass for ≥ 5 targets; every result labelled by which of the 8 bar checks it passes; at least 3 ledger predictions derived from it.
3. **Re-derivation of a published geroscience meta-analysis (R1, with A2).** Pick one published meta-analysis in an intervention class A2 rates highly; A2 extracts blind, R1 re-computes. *Success:* headline estimate within the pre-set tolerance, or a documented, re-run discrepancy; result reaches L2-sealed.

---

## References

- Atkins JL et al. (2021) A genome-wide association study of the frailty index highlights brain pathways in ageing. *Aging Cell*. https://doi.org/10.1111/acel.13459
- Bowden J et al. (2015) Mendelian randomization with invalid instruments: effect estimation and bias detection through Egger regression. *Int J Epidemiol*. https://doi.org/10.1093/ije/dyv080
- Bowden J et al. (2016) Consistent estimation in Mendelian randomization with some invalid instruments using a weighted median estimator. *Genet Epidemiol*. https://doi.org/10.1002/gepi.21965
- Burgess S et al. (2020) Guidelines for performing Mendelian randomization investigations. *Wellcome Open Res*. https://doi.org/10.12688/wellcomeopenres.15555.2
- Camerer CF et al. (2018) Evaluating the replicability of social science experiments in Nature and Science. *Nat Hum Behav*. https://doi.org/10.1038/s41562-018-0399-z
- Deelen J et al. (2019) A meta-analysis of genome-wide association studies identifies multiple longevity genes. *Nat Commun*. https://doi.org/10.1038/s41467-019-11558-2
- DellaVigna S, Pope D (2018) Predicting experimental results: who knows what? *J Polit Econ*. https://doi.org/10.1086/699976
- Doussau A et al. (2023) The impact of feedback training on prediction of cancer clinical trial results. *Clin Trials*. https://doi.org/10.1177/17407745231203375
- Dreber A et al. (2015) Using prediction markets to estimate the reproducibility of scientific research. *PNAS*. https://doi.org/10.1073/pnas.1516179112
- Dudbridge F et al. (2019) Adjustment for index event bias in genome-wide association studies of subsequent events. *Nat Commun*. https://doi.org/10.1038/s41467-019-09381-w
- Giambartolomei C et al. (2014) Bayesian test for colocalisation between pairs of genetic association studies using summary statistics. *PLoS Genet*. https://doi.org/10.1371/journal.pgen.1004383
- Hughes RA et al. (2019) Selection bias when estimating average treatment effects using one-sample instrumental variable analysis. *Epidemiology*. https://doi.org/10.1097/EDE.0000000000000972
- IL6R Genetics Consortium and Emerging Risk Factors Collaboration (2012) Interleukin-6 receptor pathways in coronary heart disease. *Lancet*. https://doi.org/10.1016/s0140-6736(11)61931-4
- Klein J, Roodman A (2005) Blind analysis in nuclear and particle physics. *Annu Rev Nucl Part Sci*. https://doi.org/10.1146/annurev.nucl.55.090704.151521
- Kurki MI et al. (2023) FinnGen provides genetic insights from a well-phenotyped isolated population. *Nature*. https://doi.org/10.1038/s41586-022-05473-8
- McCartney DL et al. (2021) Genome-wide association studies identify 137 genetic loci for DNA methylation biomarkers of aging. *Genome Biol*. https://doi.org/10.1186/s13059-021-02398-9
- Mellers B et al. (2014) Psychological strategies for winning a geopolitical forecasting tournament. *Psychol Sci*. https://doi.org/10.1177/0956797614524255
- Miller RA et al. (2007) An aging Interventions Testing Program: study design and interim report. *Aging Cell*. https://doi.org/10.1111/j.1474-9726.2007.00311.x
- Moqri M et al. (2023) Biomarkers of aging for the identification and evaluation of longevity interventions. *Cell*. https://doi.org/10.1016/j.cell.2023.08.003
- Nadon NL et al. (2008) Design of aging intervention studies: the NIA Interventions Testing Program. *Age*. https://doi.org/10.1007/s11357-008-9048-1
- Ridker PM et al. (2017) Antiinflammatory therapy with canakinumab for atherosclerotic disease. *N Engl J Med*. https://doi.org/10.1056/NEJMoa1707914
- Sanderson E et al. (2018) An examination of multivariable Mendelian randomization in the single-sample and two-sample summary data settings. *Int J Epidemiol*. https://doi.org/10.1093/ije/dyy262
- Skrivankova VW et al. (2021) STROBE-MR. *JAMA*. https://doi.org/10.1001/jama.2021.18236
- Skrivankova VW et al. (2021) STROBE-MR: explanation and elaboration. *BMJ*. https://doi.org/10.1136/bmj.n2233
- Sudlow C et al. (2015) UK Biobank: an open access resource. *PLoS Med*. https://doi.org/10.1371/journal.pmed.1001779
- Swerdlow DI et al. (2015) HMG-coenzyme A reductase inhibition, type 2 diabetes, and bodyweight. *Lancet*. https://doi.org/10.1016/S0140-6736(14)61183-1
- Timmers PRHJ et al. (2019) Genomics of 1 million parent lifespans implicates novel pathways and common diseases. *eLife*. https://doi.org/10.7554/eLife.39856
- Timmers PRHJ et al. (2020) Multivariate genomic scan implicates novel loci and haem metabolism in human ageing. *Nat Commun*. https://doi.org/10.1038/s41467-020-17312-3
- Voight BF et al. (2012) Plasma HDL cholesterol and risk of myocardial infarction: a mendelian randomisation study. *Lancet*. https://doi.org/10.1016/S0140-6736(12)60312-2

**Official links.** FDAAA 801 final rule: https://clinicaltrials.gov/policy/fdaaa-801-final-rule ; 42 CFR Part 11: https://www.ecfr.gov/current/title-42/chapter-I/subchapter-A/part-11 ; ClinicalTrials.gov API: https://clinicaltrials.gov/data-api/api ; NIA ITP: https://www.nia.nih.gov/research/dab/interventions-testing-program-itp ; Cochrane: https://www.cochrane.org/about-us ; Cochrane Handbook: https://training.cochrane.org/handbook ; Biomarkers of Aging Challenge: https://www.agingconsortium.org/challenge and https://pmc.ncbi.nlm.nih.gov/articles/PMC11565782/ ; CERN Higgs boson: https://home.cern/science/physics/higgs-boson ; Metaculus: https://www.metaculus.com
