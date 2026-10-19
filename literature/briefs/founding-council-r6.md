# Founding council: Round 6 evidence brief (Literature), challenge and convergence

Author: Literature (A2). Drafted by worker W-Council-R6. In Round 6 the red team (A50, Verification) attacks R1–R5 as an outside reviewer would. In this brief Literature attacks its own evidence plan first, so the council does not hear these points for the first time from a journal referee.

**Numbering.** A10 used D68–D69, A50 D70–D77 and A28 D78–D79, so this brief uses **D80–D89**. The chair may renumber.

**Sources.** Every DOI was checked on Crossref on 2026-10-19. The results are in `founding-council-r6.check.tsv` (16 resolved and 1 failed, which is not cited). Trial counts come from the R4 brief (ClinicalTrials.gov API v2). Claims I could not check are marked [to verify].

---

## 0. Summary

- **Five reviewer attacks** (§1). None is fatal if we label honestly. Two of them would be fatal if we did not:
  - selling the flagship's non-cardiovascular serious adverse event (SAE) counts as efficacy evidence;
  - showing a ranking built on evidence rated low or very low without saying so.
  
  Three can be fixed before the first extraction: an outcome-completeness check, a single primary outcome per PICO question, and a novelty check against existing reviews.
- **Six internal contradictions** between decided items (§2). The most serious one: the ranking rule (D33/R3) can rank interventions whose evidence the GRADE gate says must not be pooled.
- **Reversal Literature wants** (§3): G5's "≥ 4 of the 16 classes *estimated* at L2-sealed in year 1". The likely result is 0–1, so the target would make us lower the gate. Replace it with "16/16 classes mapped, each with its trust level; estimated count reported, no target".
- **Missing items** (§4): medical-advice and legal exposure of a public ranking, conflicts of interest, patient and public involvement, licences of the extracted data, and a named clinician reviewer.
- **Charter outline** (§5): ten sections, so the president can run Astra without reading G60.
- **Decision items D80–D89** (§6).

---

## 1. Self-attack: what a systematic-review referee would say

### A1. "Your flagship pools SAE tables that were never designed to measure dementia, cancer or fracture"

*Attack.* In the 27 trials, dementia, cancer, fracture and infection appear only as serious adverse event rows. These events were not adjudicated or pre-specified, and they were reported under a seriousness threshold. Dementia appears in 21 of 27 trials, with very small counts: 0–6 rows per trial, as counted in R4 §2.1. Diagnosis of dementia in an SAE listing depends on hospital admission and on the investigator's choice of term. That is outcome measurement, not chance error. RoB 2 domain 4, "measurement of the outcome" ([Higgins 2011](https://doi.org/10.1136/bmj.d5928) for the original tool), would be rated high or "some concerns" for every trial. With small counts, the choice of pooling method changes the answer ([Bradburn 2007](https://doi.org/10.1002/sim.2528)).

*Verdict: acceptable with a label, fatal without one.* The flagship can say "no signal of harm or benefit on non-cardiovascular SAEs, GRADE low or very low". It cannot say "SGLT2 inhibitors reduce dementia". Already decided: Mantel–Haenszel or Peto pooling for rare events (R4), and every effect labelled extrapolated. **Add:** a fixed sentence on every flagship non-CV output, "event counts from safety reporting, not adjudicated outcomes", and a ban on using these effects in the ranking unless their GRADE rating is moderate or higher (D80).

### A2. "Outcome reporting bias: the SAE rows you see are the ones that were reported"

*Attack.* Outcome reporting bias is common and changes conclusions. [Kirkham 2010](https://doi.org/10.1136/bmj.c365) found it suspected in at least one trial in about a third of Cochrane reviews. In a share of meta-analyses, it was enough to change a significant result [exact proportions to verify]. [Dwan 2013](https://doi.org/10.1371/journal.pone.0066844) confirms the pattern across cohorts of trials. For harms, publications under-report compared with registries and unpublished sources:
- [Riveros 2013](https://doi.org/10.1371/journal.pmed.1001566) found SAEs more completely reported on ClinicalTrials.gov than in journals [proportions to verify];
- [Golder 2016](https://doi.org/10.1371/journal.pmed.1002127) found that many adverse events are missing from the published record.

Our own R4 data contain an example: EXSCEL's posted table lists 18 infection terms where similar trials list about 100–250.

*Verdict: fixable.* Using ClinicalTrials.gov as the main source is the right choice, and the literature supports it. **Fix:** before pooling, run a mechanical **completeness check** per trial and per organ class. Flag any trial whose term count or event rate is below a tenth of the median of trials of similar size. A flagged trial gets its numbers from the paper or supplement, or is excluded from that outcome, and the exclusion is reported (D81). This generalises the R4 EXSCEL rule.

### A3. "Multiplicity: 27 trials × 8 components × 16 classes × subgroups. Something will be significant"

*Attack.* We plan many outcomes, many classes, two age bands, a direct column and an extrapolated column. A reviewer will count the comparisons and expect false positives ([Bender 2008](https://doi.org/10.1016/j.jclinepi.2008.03.004)). The G1 rule "at most one finding per PICO, nulls counted" helps. But a PICO with eight components still has eight chances.

*Verdict: fixable.* **Fix:** each PICO registers one **primary** outcome before extraction (default: the D33 composite where it can be estimated, else all-cause death). The other components are secondary and labelled as such. A G1 finding counts only on its primary outcome (D82). The ranking already uses posterior shrinkage (van Zwet–Gelman, D13/D22), which partly answers multiplicity for effect sizes. It does not answer "how many of your findings are false", so we report the number of comparisons beside every finding.

### A4. "Your 'original value' is a rerun of existing meta-analyses"

*Attack.* Many published systematic reviews are redundant, misleading or conflicted ([Ioannidis 2016](https://doi.org/10.1111/1468-0009.12210)). SGLT2 inhibitor outcome trials have already been pooled ([Zelniker 2019](https://doi.org/10.1016/S0140-6736(18)32590-X)). We use that review on purpose as our calibration target (L3), which proves the overlap. GLP-1 RA trials have been pooled several times as well.

*Verdict: fixable, and the main risk to the president's "new value" mandate.* The value we can defend:
1. non-CV outcomes from posted registry SAE tables across both classes and 27 trials;
2. a transported 10-year RMST in healthy 70–79-year-olds, which no trial reports;
3. sealed ledger forecasts.

**Fix:** before any mission starts, a **novelty check**: a dated search for existing reviews of the same PICO, recorded in the mission file. The mission is stopped or reframed if one exists and adds nothing (D83). Literature owns the search. Verification audits a sample.

### A5. "Your headline number rests on reconstructed survival data and low-grade evidence"

*Attack.* The north star is an RMST gain ([Royston & Parmar 2013](https://doi.org/10.1186/1471-2288-13-152)). It needs Kaplan–Meier curves rebuilt from printed figures ([Guyot 2012](https://doi.org/10.1186/1471-2288-12-9)). For the composite, only ASPREE prints a curve close to it: death, dementia or persistent physical disability ([McNeil 2018](https://doi.org/10.1056/NEJMoa1800722)). The flagship can give curves for mortality only (R4 §2.2). Under GRADE ([Guyatt 2008](https://doi.org/10.1136/bmj.39489.470347.AD); [Balshem 2011](https://doi.org/10.1016/j.jclinepi.2010.07.015)), indirect populations, unadjudicated outcomes and imprecision each take a level off. Most non-CV effects start low and end very low.

*Verdict: acceptable with labels. It makes G5 unrealistic (§3).* The labels are already decided: the "not estimable" column and the extrapolated column. **Add:** the public table shows the GRADE rating next to every number, in the same font size (D84).

---

## 2. Internal contradictions

| # | Items | Contradiction | Proposed resolution |
|---|---|---|---|
| C1 | R3 ranking (posterior median among P(net>0) ≥ 0.8) vs "no pooled number without GRADE" (R2/R4) | The ranking rule has no GRADE floor. A very-low-GRADE effect with a tight posterior could be ranked first. The GRADE gate allows the number to be shown, not ranked. | Ranking requires GRADE moderate or higher on the primary outcome. Below that the entry is listed "unranked: evidence too weak" (D80). |
| C2 | D21 event list vs D33 v0 composite | D21 adds COPD, CKD 4–5 and hip fracture. D33 v0 is death, dementia and persistent disability (the ASPREE-like core). G5 says "full D33 composite". Workers will not know which definition a number uses. | One versioned outcome-definition file in `data/`. Every number cites its version (D85). The clerk has flagged a related D20/D33 label issue. |
| C3 | G5 "≥ 4 classes estimated" vs R4 finding that the flagship cannot estimate the composite | The only funded scored mission cannot fill a G5 "estimated" slot. G5 counts on missions that are not on the roadmap. | §3. |
| C4 | D63 (5% error on a rolling 3-month pool) vs D65/A50 count rule (≥ 6/60 = incident) | 6/60 is 10%, not 5%. Two thresholds for the same signal. | Use the count rule (A50's version, which is testable). Delete the 5% wording (D86). |
| C5 | Extrapolated column (R3) vs G1 "L2-sealed findings" | Can an extrapolated effect count as one of the 12 G1 findings? This was never stated. The R3 research brief expects 1–2 flagship findings, and those are extrapolated. | It counts only if the finding's text says "extrapolated" and transport rule D32 passed. Otherwise it does not count (D87). |
| C6 | D61 (dual sign-off for A1-customer missions) vs D2 (signature precedence) | The clerk flagged this as point 7. If D2 lets A1 sign alone for the president, D61 is empty. | D61 wins for audited missions. The chair confirms in the charter. |

---

## 3. The decision Literature most wants reversed: G5's "≥ 4 classes estimated"

**Decided.** G5 says every one of the 16 classes has an entry, and at least 4 are *estimated* at L2-sealed (full D33 composite, 70–79) in year 1.

**Steelman of keeping it.** A map with no numbers is a literature review, and the president asked for new value. A hard target forces the platform (RMST engine, transport rule, Guyot pipeline) to work end to end on real interventions within a year. Four out of sixteen is already modest. Possible candidates: exercise (multi-domain trials), statins or blood-pressure lowering (large trials with mortality curves), metformin (TAME has not reported, so it would be extrapolated), and the SGLT2/GLP-1 flagship.

**Why it should be reversed.**
1. *Evidence supply.* "Estimated" for the full composite needs, per class, a randomised trial with curves or counts for death, dementia and disability in or near 70–79. ASPREE is the only trial we know that reports this composite directly, and its intervention (aspirin) showed no benefit. Every other class reaches the composite only through D32 transport from sick populations, and GRADE removes at least one level for that.
2. *Pipeline timing.* A10 puts the first score at weeks 6–8 at best, with coloc, RMST and Guyot reference cases still unconfirmed (A10 R4 §2.5). A28 expects 1–2 flagship findings in year 1. Neither brief shows four classes.
3. *Goodhart.* A count target on "estimated" pushes toward lowering the transport rule or the GRADE floor in month 10. This is the drift D65 is meant to catch. We would be building a target that sets off our own alarm.
4. *Honest result.* "16 of 16 classes mapped, 1 estimated, 15 not estimable for stated reasons" is a publishable and useful result. "4 estimated" reached by stretching the rules is the kind of number A50 told us we would be embarrassed by.

**Proposal (D88).** G5 becomes "16/16 classes mapped with best evidence, trust level and the specific missing piece; the count of classes estimated is reported monthly with no target". The pressure toward new value moves to G1 (findings) and G2/G3 (ledger), which carry their own anti-gaming rules. If the chair keeps a number, it should be "≥ 1 estimated" with a written review at month 6.

---

## 4. Missing risks and roles (evidence side)

1. **Medical-advice and legal exposure.** A public "best anti-aging intervention" ranking will be read as advice. Proposal: a fixed disclaimer, no dosing and no individual recommendations; entries for prescription drugs say "discuss with a doctor; evidence in sick populations". We should ask the president whether a legal review is needed before the first public ranking [to verify: jurisdiction and need for counsel].
2. **Conflicts of interest.** We have no funders yet. But agents may cite sponsor-written trials, and future sponsors (D74) may have products in the ranking. Proposal: a public COI register (funders, sponsors, any person reviewing), and a sponsor whose product class is under review is excluded from that review.
3. **Patient and public involvement.** None is planned. Outcome choice (is disability worse than dementia? how much harm per year gained is acceptable?) is a value judgement. Reviewers expect lay input, reported with GRIPP2 ([Staniszewska 2017](https://doi.org/10.1136/bmj.j3453)). Proposal: a small lay panel (≥ 3 people aged 65+) reviews the D33 composite and the harm display once in year 1. The president recruits them (it needs people, a resource request).
4. **Licences of extracted data.** CT.gov records are public. Journal tables and digitised curves may be under publisher copyright. Extracted numbers are generally treated as facts, but bulk republication of supplements is not [to verify]. Proposal: store only numbers plus a source location, never copied figures or tables. Verification's licence check (D67) covers this.
5. **Named clinician reviewer.** G10 is an outside methods reviewer. Nobody checks that a "disability" or "dementia" mapping makes clinical sense. Proposal: G10's reviewer pool includes one geriatrician.
6. **Primary data.** If Astra ever moves to new human data (surveys, the "mice" resource line, or partnerships), ethics approval and consent rules apply ([WMA 2013](https://doi.org/10.1001/jama.2013.281053)). Proposal: the charter forbids any primary human data collection without a president decision and external ethics approval.

---

## 5. Charter outline (`docs/charter/`)

One page per section, plain English, each pointing to the decision numbers it rests on.

1. **Mission and north star.** What Astra measures: 10-year RMST free of death, dementia and disability, ages 70–79. What "not estimable" means.
2. **Organisation.** President, VP (A1), managers and their scopes, Verification's independence, clerk. Who can create whom; depth and size caps.
3. **How decisions are made.** Council and meeting rules, dissent and steelman requirements, the decision log, appeal to the president.
4. **Year-1 goals.** G1–G10 with targets, failure thresholds, owners, and how each is measured from the repository.
5. **Evidence rules.** Levels L1–L2, dual extraction, risk of bias, GRADE gate, ranking rule, transport rule, extrapolated column, harms display, primary outcome per PICO.
6. **The ledger.** What is forecast, sealing, base rates, scoring, exits (D62).
7. **Quality and incidents.** Verification audits, planted errors, S1–S3 incidents, add-only corrections, retraction scan, drift report.
8. **Repository and tools.** Repo as single source of truth, PR-only `main`, protection rules nobody touches, languages, CI checks.
9. **Public outputs and limits.** What is published, disclaimers, COI register, licences, no medical advice, no primary human data without approval.
10. **Resources and calendar.** Agent budget, Verification ≥ 15%, president resource requests, first ten missions and dates, monthly report to the president.

---

## 6. Proposed decision items

| # | Proposal | Alternatives | Owner |
|---|---|---|---|
| **D80** | An intervention is ranked only if its primary outcome is GRADE moderate or higher. Otherwise it is listed "unranked: evidence too weak", with its numbers shown. | (a) Rank everything with the GRADE rating shown, which is easy to misread; (b) GRADE high only, which leaves the ranking nearly empty | A28 (rule), Literature (GRADE) |
| **D81** | Outcome-completeness check per trial and organ class before any pooling. Below a tenth of the median of size-matched trials, take the numbers from the paper or supplement, or exclude and report. | (a) Case by case, as in R4 for EXSCEL, which does not scale; (b) no check | Literature, A10 (CI) |
| **D82** | One primary outcome registered per PICO before extraction. G1 counts the primary outcome only. The number of comparisons is shown beside every finding. | (a) All components count, which inflates G1; (b) formal multiplicity correction across PICOs, which is too conservative | Literature, A28 |
| **D83** | Dated novelty check for existing reviews, recorded before a mission starts. The mission is stopped or reframed if one exists and adds nothing. | (a) No check; (b) Verification does it, which takes its budget | Literature (search), Verification (sample) |
| **D84** | The GRADE rating is printed next to every public number, same size. | Footnote only, which is ignored | A11 |
| **D85** | One versioned outcome-definition file (D21 events and D33 composite versions). Every number cites its version. | Keep the definitions in briefs, which is ambiguous | A9 |
| **D86** | Audit incident threshold = A50's count rule. Remove the "5%" wording from D63. | Keep both, which contradict each other | Verification |
| **D87** | An extrapolated effect counts for G1 only if labelled and D32 passed. | (a) Never counts, which removes the flagship from G1; (b) always counts | A28 |
| **D88** | G5 = 16/16 classes mapped with trust level and missing piece. The estimated count is reported, with no target (or "≥ 1" with a month-6 review). | Keep "≥ 4 estimated" | Chair, Literature |
| **D89** | Evidence-side guardrails in the charter: disclaimer and no medical advice, public COI register, lay panel once in year 1 (GRIPP2), numbers stored without copied figures, one geriatrician in the G10 pool, no primary human data without president decision and external ethics approval. | Defer to after the first public ranking, which is too late | Chair (charter), president (lay panel, legal) |

---

## References

- Balshem H et al. GRADE guidelines: 3. Rating the quality of evidence. *J Clin Epidemiol* 2011. https://doi.org/10.1016/j.jclinepi.2010.07.015
- Bender R et al. Attention should be given to multiplicity issues in systematic reviews. *J Clin Epidemiol* 2008. https://doi.org/10.1016/j.jclinepi.2008.03.004
- Bradburn MJ et al. Much ado about nothing: a comparison of the performance of meta-analytical methods with rare events. *Stat Med* 2007 (Crossref issue date 2006, online first). https://doi.org/10.1002/sim.2528
- Dwan K et al. Systematic review of the empirical evidence of study publication bias and outcome reporting bias, an updated review. *PLoS One* 2013. https://doi.org/10.1371/journal.pone.0066844
- Golder S et al. Reporting of adverse events in published and unpublished studies of health care interventions: a systematic review. *PLoS Med* 2016. https://doi.org/10.1371/journal.pmed.1002127
- Guyatt GH et al. GRADE: an emerging consensus on rating quality of evidence and strength of recommendations. *BMJ* 2008. https://doi.org/10.1136/bmj.39489.470347.AD
- Guyot P et al. Enhanced secondary analysis of survival data: reconstructing the data from published Kaplan-Meier survival curves. *BMC Med Res Methodol* 2012. https://doi.org/10.1186/1471-2288-12-9
- Higgins JPT et al. The Cochrane Collaboration's tool for assessing risk of bias in randomised trials. *BMJ* 2011. https://doi.org/10.1136/bmj.d5928
- Ioannidis JPA. The mass production of redundant, misleading, and conflicted systematic reviews and meta-analyses. *Milbank Q* 2016. https://doi.org/10.1111/1468-0009.12210
- Kirkham JJ et al. The impact of outcome reporting bias in randomised controlled trials on a cohort of systematic reviews. *BMJ* 2010. https://doi.org/10.1136/bmj.c365
- McNeil JJ et al. Effect of aspirin on disability-free survival in the healthy elderly. *N Engl J Med* 2018. https://doi.org/10.1056/NEJMoa1800722
- Riveros C et al. Timing and completeness of trial results posted at ClinicalTrials.gov and published in journals. *PLoS Med* 2013. https://doi.org/10.1371/journal.pmed.1001566
- Royston P, Parmar MKB. Restricted mean survival time: an alternative to the hazard ratio for the design and analysis of randomized trials. *BMC Med Res Methodol* 2013. https://doi.org/10.1186/1471-2288-13-152
- Staniszewska S et al. GRIPP2 reporting checklists: tools to improve reporting of patient and public involvement in research. *BMJ* 2017. https://doi.org/10.1136/bmj.j3453
- World Medical Association. Declaration of Helsinki. *JAMA* 2013. https://doi.org/10.1001/jama.2013.281053
- Zelniker TA et al. SGLT2 inhibitors for primary and secondary prevention of cardiovascular and renal outcomes in type 2 diabetes. *Lancet* 2019. https://doi.org/10.1016/S0140-6736(18)32590-X
