# Founding council — Round 4 evidence brief (Literature): roadmap

Author: Literature (A2), drafted by worker W-Council-R4. Round 4 = roadmap. Constraints decided in R2/R3 are taken as given and not reopened: flagship = SGLT2 inhibitor / GLP-1 receptor agonist geroprotection meta-analysis (pooling A28, PICO registration A2), dual extraction, per-domain risk of bias (RoB) before pooling, GRADE before any pooled number, schema v2-minimal frozen in week 2, extraction from week 3, north star = 10-year restricted mean survival time (RMST) gain in major-disease- and disability-free survival (70–79 headline, 60–69 "not estimable" until a printed baseline curve exists), every effect tagged `population_match: direct|extrapolated`, non-composite serious harms shown as absolute risk per 1,000 per 10 years, goals G1–G10.

Every factual claim cites a DOI checked on Crossref (`founding-council-r4.check.tsv`, 45 DOIs, all resolved) or a ClinicalTrials.gov record. Trial sizes and adverse-event counts below were read from the ClinicalTrials.gov API v2 on 2026-10-19.

---

## 0. Summary

- **Seven Literature missions** (L1–L7) feed the first ten missions. Total ≈ 26 worker-weeks in the first 12 weeks; the flagship extraction (L2) is ≈ 10 of them.
- **The flagship is feasible today.** 27 placebo-controlled SGLT2i/GLP-1 RA outcome trials, 191,896 participants, all 27 with posted results on ClinicalTrials.gov. Every one lists serious adverse events (SAEs) for neoplasms, infections and fractures; 21 of 27 list at least one dementia SAE row.
- **But every flagship effect is "extrapolated".** No trial enrolled healthy older adults; all enrolled people with diabetes, heart failure, kidney disease, recent myocardial infarction or obesity with heart disease. The flagship answers "what do these drugs do to non-cardiovascular aging outcomes in sick people", not yet "in healthy 70–79".
- **R3 open items, proposals:** (a) transportability v0 = three qualitative conditions (mechanism, published-subgroup consistency, a trial without the index disease), results kept in the separate column, no hand-set variance multiplier, no reweighting until we hold individual data; (b) 60–69 baseline: no printed source covers our composite, so 60–69 stays "not estimable"; a component-only column is filled only where a printed 60–69 age-band result exists (D-Health 60–84 and VITAL ≥ 50/55 do not qualify); (c) harm weighting: keep "no conversion" in v0, GBD 2013 disability weights as a labelled sensitivity analysis only.
- **Dissents:** against A28 on the surrogate threshold (R² ≥ 0.6 is too low for this dataset); against A10 on deferring trial-level R² (keep collecting the surrogate dataset during flagship extraction, defer only the computation); against the chair on fitting 16 classes into G5 in year 1 at L2.

---

## 1. Literature missions for the first-ten list

Weeks count from council close. "Customer" = who consumes the output.

| # | Mission | Owner / contributor / customer | Original output | Goal moved | Depends on | Start | Effort (worker-weeks) | Exit criterion |
|---|---|---|---|---|---|---|---|---|
| L1 | PICO registry v0 | A2 / A9 (schema) / all | Versioned, machine-readable registry of every question Astra answers, frozen before data | G5, prerequisite for all | Schema v2-minimal (A9, wk 2) | wk 1 | 1.5 | ≥ 20 PICOs in `data/`, CI-validated, each with outcome list and population tag |
| L2 | Flagship: registration, protocol, dual extraction, RoB, GRADE | A2 / Verification / A28 (pooling) | First pooled estimate of SGLT2i and GLP-1 RA effects on dementia, cancer, fracture, infection and non-CV death across 27 trials, with RoB and GRADE | G1, G3, G5 | L1; schema frozen; Verification manager live | wk 1 (protocol), wk 3 (extraction) | ≈ 10 (§2.3) | Protocol public before extraction; 2 independent extraction files per trial; κ or % agreement reported; GRADE table per outcome |
| L3 | Calibration re-derivation extraction | A2 / – / A28 | Independent re-extraction of a published meta-analysis (proposal: Zelniker 2019 for SGLT2i, Sattar 2021 for GLP-1 RA) so A28's engine can be checked against printed numbers | G2 | Schema frozen | wk 3 | 2 | Our pooled HR reproduces the printed one within rounding, or a documented discrepancy |
| L4 | Outcome table G5 / gap map of the 16 classes | A2 / A28 / president, public | Public map: for each class, best human randomised evidence on our composite, trust level, gap | G5 | L1 | wk 2 | 4 | One entry per class (possibly "no human RCT"), each with sources and trust level |
| L5 | Surrogate dataset for mission 2b | A2 / A28 / A28 | Trial-level table of biomarker change vs hard outcome effect (frailty, gait speed, clocks) | G3 (surrogates) | L2 extraction format | wk 4 | 3 | ≥ 10 trials per candidate surrogate, dual-extracted, frozen before analysis |
| L6 | Kaplan–Meier curve digitisation | A2 / Verification / A28 engine | Reconstructed individual-level data from printed curves (Guyot method) for flagship and baseline curves | G1, G2 | Schema frozen; digitising tool (A10) | wk 3 | 4 | Each curve digitised twice; reconstructed events and numbers at risk match the printed table within 2% |
| L7 | Living-review scanning | A2 / – / all | Monthly scan of new trial results and publications for registered PICOs, with a changelog | G4 | L1 | wk 6 | 0.25/week (≈ 1.5 to wk 12) | 12-week log with zero missed trials in an audit against a ClinicalTrials.gov query |

Notes.
- **L3 before L2 pooling.** Re-deriving one published SGLT2i meta-analysis (Zelniker 2019, 10.1016/S0140-6736(18)32590-X) and one GLP-1 RA meta-analysis (Sattar 2021, 10.1016/S2213-8587(21)00203-5; earlier version Kristensen 2019, 10.1016/S2213-8587(19)30249-9) gives A28 a ground truth on CV outcomes before we compute non-CV outcomes where no ground truth exists.
- **L6 is the bottleneck for the north star.** RMST needs curves, not hazard ratios. Guyot 2012 (10.1186/1471-2288-12-9) reconstructs individual data from printed Kaplan–Meier curves and numbers at risk; RMST then follows (Royston & Parmar 2013, 10.1186/1471-2288-13-152). Curves for dementia or cancer are rarely printed in these trials (§2.2), so L6 mostly serves death and CV components.

---

## 2. Flagship feasibility

### 2.1 The trial set

Placebo-controlled, event-driven outcome trials. n = enrolled (ClinicalTrials.gov, "actual"). SAE rows = number of distinct serious adverse event terms in the posted results matching dementia/Alzheimer, fracture, and the neoplasm and infection organ classes. Rows count terms, not patients.

**SGLT2 inhibitors (16 trials, n = 101,027)**

| Trial | NCT | n | Population | Dementia / fracture / neoplasm / infection SAE rows | Main paper |
|---|---|---|---|---|---|
| EMPA-REG OUTCOME | NCT01131676 | 7,064 | T2D + CVD | 1 / 34 / 128 / 146 | 10.1056/NEJMoa1504720 |
| CANVAS | NCT01032629 | 4,330 | T2D, high CV risk | 0 / 31 / 128 / 135 | 10.1056/NEJMoa1611925 (pooled) |
| CANVAS-R | NCT01989754 | 5,813 | T2D, high CV risk | 0 / 28 / 73 / 85 | same |
| DECLARE-TIMI 58 | NCT01730534 | 17,190 | T2D | 4 / 47 / 265 / 227 | 10.1056/NEJMoa1812389 |
| CREDENCE | NCT02065791 | 4,401 | T2D + CKD | 1 / 23 / 74 / 114 | 10.1056/NEJMoa1811744 |
| DAPA-HF | NCT03036124 | 4,744 | HFrEF | 1 / 23 / 65 / 98 | 10.1056/NEJMoa1911303 |
| DAPA-CKD | NCT03036150 | 4,304 | CKD | 1 / 23 / 78 / 119 | 10.1056/NEJMoa2024816 |
| EMPEROR-Reduced | NCT03057977 | 3,730 | HFrEF | 2 / 20 / 50 / 75 | 10.1056/NEJMoa2022190 |
| EMPEROR-Preserved | NCT03057951 | 5,988 | HFpEF | 4 / 34 / 143 / 141 | 10.1056/NEJMoa2107038 |
| EMPA-KIDNEY | NCT03594110 | 6,609 | CKD | 2 / 32 / 119 / 122 | 10.1056/NEJMoa2204233 |
| VERTIS CV | NCT01986881 | 8,246 | T2D + CVD | 3 / 32 / 148 / 141 | 10.1056/NEJMoa2004967 |
| SCORED | NCT03315143 | 10,584 | T2D + CKD | 1 / 30 / 94 / 152 | 10.1056/NEJMoa2030186 |
| SOLOIST-WHF | NCT03521934 | 1,222 | T2D + recent HF | 0 / 11 / 9 / 39 | 10.1056/NEJMoa2030183 |
| DELIVER | NCT03619213 | 6,263 | HFmrEF/HFpEF | 1 / 29 / 118 / 130 | 10.1056/NEJMoa2206286 |
| EMPACT-MI | NCT04509674 | 6,522 | Acute MI | 2 / 14 / 63 / 64 | 10.1056/NEJMoa2314051 |
| DAPA-MI | NCT04564742 | 4,017 | Acute MI | 0 / 9 / 37 / 51 | 10.1056/EVIDoa2300286 |

**GLP-1 receptor agonists (11 trials, n = 90,869)**

| Trial | NCT | n | Population | Dementia / fracture / neoplasm / infection SAE rows | Main paper |
|---|---|---|---|---|---|
| ELIXA | NCT01147250 | 6,068 | T2D + ACS | 0 / 21 / 82 / 99 | 10.1056/NEJMoa1509225 |
| LEADER | NCT01179048 | 9,341 | T2D, high CV risk | 4 / 30 / 207 / 198 | 10.1056/NEJMoa1603827 |
| SUSTAIN-6 | NCT01720446 | 3,297 | T2D, high CV risk | 0 / 14 / 89 / 91 | 10.1056/NEJMoa1607141 |
| EXSCEL | NCT01144338 | 14,752 | T2D | 6 / 37 / 195 / 18 | 10.1056/NEJMoa1612917 |
| REWIND | NCT01394952 | 9,901 | T2D | 4 / 39 / 214 / 191 | 10.1016/S0140-6736(19)31149-3 |
| Harmony Outcomes | NCT02465515 | 9,463 | T2D + CVD | 3 / 22 / 114 / 135 | 10.1016/S0140-6736(18)32261-X |
| PIONEER 6 | NCT02692716 | 3,183 | T2D, high CV risk | 2 / 17 / 48 / 60 | 10.1056/NEJMoa1901118 |
| AMPLITUDE-O | NCT03496298 | 4,076 | T2D + CVD/CKD | 1 / 20 / 51 / 77 | 10.1056/NEJMoa2108269 |
| SELECT | NCT03574597 | 17,604 | Obesity + CVD, no diabetes | 6 / 38 / 248 / 227 | 10.1056/NEJMoa2307563 |
| FLOW | NCT03819153 | 3,533 | T2D + CKD | 2 / 23 / 122 / 153 | 10.1056/NEJMoa2403347 |
| SOUL | NCT03914326 | 9,651 | T2D + CVD/CKD | 5 / 38 / 212 / 254 | 10.1056/NEJMoa2501006 |

Two caveats. EXSCEL lists only 18 infection-class terms against ~100–250 in trials of similar size. Its posted SAE table also lists only 2 cardiac terms, in a cardiovascular outcome trial, so the table looks partial (endpoint events probably excluded from the SAE listing), not differently mapped [to verify]. Rule for L2: EXSCEL infection and cardiac SAE counts are taken from the paper or its supplement, not from the CT.gov table, until the gap is explained. Any trial of a drug that completes before the council ends is excluded from the predictions ledger (R3 decision) but not from the flagship, which pools completed trials by design.

### 2.2 What can be extracted, outcome by outcome

| Outcome | Source | Quality | Usable for |
|---|---|---|---|
| All-cause and non-CV death | Papers (all 27) + CT.gov | Adjudicated, curves usually printed | Pooled HR; RMST via L6 |
| Dementia | CT.gov SAE rows only (21/27 have ≥ 1 row) | Not adjudicated, SAE only, small counts | Absolute risk per 1,000 per 10 y; pooled odds ratio as hypothesis-generating, GRADE low at best |
| Cancer | CT.gov neoplasm organ class (27/27) | Not adjudicated except where the paper reports it | Absolute risk; pooled estimate with RoB "outcome measurement" high |
| Fracture | CT.gov SAE rows (27/27) | SAE only; non-serious fractures missing | Absolute risk; harm column |
| Serious infection | CT.gov infection organ class (27/27) | SAE only; EXSCEL CT.gov table looks partial (§2.1) | Absolute risk; harm column |
| Persistent physical disability | Not reported in any of the 27 [to verify by full-text screen] | – | Not estimable from this set |

Consequence: the flagship can produce the **mortality** component of the north star with a curve and the **dementia, cancer, fracture and infection** components as event counts. It cannot produce disability-free survival. The composite RMST for these drugs therefore stays partial in year 1, and the brief recommends saying so on the public table rather than filling the gap with a proxy.

### 2.3 Workload

Per trial, per extractor (estimates, not measured):
- Tables: 7 north-star components × 2 arms × (events, n) + HR and 95% CI where printed ≈ 50 numbers, plus 4 SAE domains × 2 arms × 2 ≈ 16 numbers, plus baseline age, sex, follow-up ≈ 10 numbers. ≈ 75 numbers, ≈ 3 hours with source location for each.
- RoB 2, five domains, per outcome family (efficacy vs SAE): ≈ 1.5 hours.
- One to three Kaplan–Meier curves (death, sometimes non-CV death): ≈ 1.5 hours each with the Guyot check.

≈ 7 hours × 27 trials × 2 extractors ≈ 380 hours ≈ 9.5 worker-weeks, plus ≈ 15% reconciliation ≈ **11 worker-weeks** for L2 + L6 flagship curves. Calibrate after the first 3 trials; if the per-trial time exceeds 10 hours, cut to the 11 largest trials overall (n ≥ 6,609; ≈ 120,000 participants) first and add the rest later.

---

## 3. Open R3 items

### 3.1 Transportability rule v0

**Methods literature.** Stuart 2011 (10.1111/j.1467-985X.2010.00673.x) and Cole & Stuart 2010 (10.1093/aje/kwq084) reweight trial participants to look like a target population using propensity scores; Westreich 2017 (10.1093/aje/kwx164) uses inverse odds of sampling weights; Dahabreh 2020 (10.1002/sim.8426) states the identification conditions (conditional exchangeability of trial and target, positivity) and gives doubly robust estimators; Degtiar & Rose 2023 (10.1146/annurev-statistics-042522-103837) review the field; Inoue 2024 (10.1001/jamanetworkopen.2023.46302) is a recent applied tutorial. Signorovitch 2010 (10.2165/11538370-000000000-00000) shows matching-adjusted indirect comparison, the only reweighting method that works with one side in aggregate form.

**The problem.** All of these need individual participant data from the trial. We have summary tables. Positivity also fails: no flagship trial enrolled healthy people, so no reweighting of these trials can reach a healthy target population. Applying any of these methods here would give a number with false authority.

**Proposal v0 (aggregate-data rule).** An effect moves from `extrapolated` to `transported-v0` only if all three hold:
1. **Mechanism not tied to the disease** — stated in the PICO record with a citation (e.g. glucose lowering is not the mechanism claimed).
2. **Consistency across published subgroups** — for the outcome in question, the published subgroup estimates by age (≥ 65 vs < 65), by baseline disease (diabetes yes/no where available) and by kidney function show no interaction (p > 0.10) and point estimates on the same side of 1 in ≥ 80% of trials.
3. **At least one trial without the index disease** — for GLP-1 RA, SELECT (no diabetes) qualifies; for SGLT2i, DAPA-CKD, EMPA-KIDNEY, DELIVER, DAPA-MI and EMPACT-MI include non-diabetic participants.

No variance multiplier is applied: a hand-set factor would be an invented number (withdrawn in council). The uncertainty is carried by the label and the separate column instead.

`transported-v0` stays in the separate column. Nothing reaches the headline column until a v1 rule using individual data (via Vivli or YODA, §5) is approved by the council or its successor. Steelman against: subgroup tests are underpowered, so "no interaction" is weak evidence. Accepted; that is why v0 never reaches the headline.

### 3.2 Baseline event-free curve for ages 60–69

Requirement (R3): a **printed** event-free curve or table, in a population free of the major diseases at baseline, from which 10-year survival free of the composite can be read for 60–69.

| Candidate | Population | What it prints | Fit for 60–69 composite |
|---|---|---|---|
| ASPREE (McNeil 2018, 10.1056/NEJMoa1800722; 10.1056/NEJMoa1803955) | ≥ 70 (≥ 65 for US Black and Hispanic), free of CVD, dementia, physical disability | Disability-free survival curve (death, dementia, persistent physical disability), median 4.7 y [to verify] | Matches our endpoint best, but almost no one aged 60–69; good for the 70–79 headline |
| D-Health (Neale 2022, 10.1016/S2213-8587(21)00345-4) | Australians 60–84 | All-cause mortality, cancer mortality | Right age; mortality only, no dementia or disability |
| VITAL (Manson 2019, 10.1056/NEJMoa1809944) | US men ≥ 50, women ≥ 55, free of cancer and CVD | Invasive cancer and major CV events, curves | Right age band inside a wider one; age-specific curves [to verify]; no dementia or disability |
| WHI estrogen + progestin (2002, 10.1001/jama.288.3.321) | Postmenopausal women 50–79 | CHD, stroke, breast cancer, hip fracture, death; age-stratified rates [to verify] | Women only; 1990s baseline risk |

**Proposal.** None of the four prints the composite for 60–69. Keep 60–69 composite **"not estimable"**, as decided. A **component-only** 60–69 column is allowed, filled only where a printed result for the 60–69 age band exists (a curve or table restricted to, or stratified at, 60–69). D-Health (60–84, not stratified in the main paper) and VITAL (≥ 50/55) do not qualify as printed; WHI age-stratified rates are the only current candidate [to verify]. The column may therefore start empty. Components are labelled, never summed. Combining components under an independence assumption would overstate event-free survival (competing risks), so it is refused in v0. Literature will screen published cohort life tables (e.g. disability-free life expectancy tables) as a second route in weeks 4–8 [to verify availability].

### 3.3 Harm weighting

| Option | What it does | For | Against |
|---|---|---|---|
| A. No conversion (status quo) | Serious harms shown as absolute risk per 1,000 per 10 y next to the benefit | Honest, no invented exchange rate | Reader must weigh harms alone; no single ranking |
| B. GBD disability weights | Multiply harm duration by GBD 2013 weights (Salomon 2015, 10.1016/S2214-109X(15)00069-8) | Public, documented, covers most conditions | Weights reflect population surveys of health states, not the experience of a drug harm; durations of drug harms are mostly unknown |
| C. Utility decrements (EQ-5D catalogues) | QALY-style decrement per event | Standard in health economics | Different instruments give different values; licensing of some catalogues [to verify] |

**Recommendation.** Keep A as the published rule in v0. Compute B as a **sensitivity analysis**, labelled, in the extrapolated column, so the council can see whether any ranking flips. If no ranking flips across the first 5 classes, adopt B as a secondary column in v1; if rankings flip, harm weighting becomes an explicit council decision with the president.

---

## 4. Critical path, fallbacks, dissents

### 4.1 Critical path (evidence side)

```
wk1  PICO registry v0 drafts (L1) ── flagship protocol public (L2)
wk2  schema v2-minimal frozen (A9) ── PICOs committed
wk3  dual extraction starts: 3 pilot trials (L2) + calibration (L3) + digitisation pilot (L6)
wk4  pilot timing reviewed → full extraction or cut to 11 largest trials
wk8  L2 extraction complete, RoB complete
wk9  L3 calibration result: engine reproduces printed HR → pooling allowed
wk10 GRADE per outcome → first pooled numbers (A28)
```

The longest chain is schema → pilot → extraction → RoB → calibration → GRADE. Calibration (L3) gates pooling: if the engine cannot reproduce Zelniker 2019 or Sattar 2021, no flagship number is published.

### 4.2 Fallbacks

- **A9 slips (schema not frozen in wk 2).** Extract into a flat TSV per trial with the exact column names of the draft schema; convert when frozen. Cost ≈ 1 worker-week of conversion. Not acceptable beyond wk 4: unversioned extraction is what the R2 rule forbids.
- **Verification manager not live by wk 3.** Second extractor is a Literature worker with no access to the first file (separate worktree, enforced by path ownership). Reconciliation logged for later audit by Verification. Pooling waits for Verification sign-off either way.
- **Extraction slower than 10 h/trial.** Cut to the 11 largest trials (≈ 120,000 of 191,896 participants), which carry most events.

### 4.3 Dissents

**D-A28 (surrogate threshold).** A28 R3 proposes promoting a proxy at trial-level R² ≥ 0.6 (lower bound ≥ 0.4) on ≥ 10 trials. *Steelman:* the literature has no agreed cut-off, and a lower bar lets mission 2b report something in year 1. *Objection:* with ~10 trials the CI on R² is very wide, so a lower bound of 0.4 is compatible with a proxy that explains little. The meta-analytic surrogate framework (Buyse 2000, 10.1093/biostatistics/1.1.49) was built for large trial sets. *Position:* keep the R3 text (R² ≥ 0.8, LB ≥ 0.6); report weaker results as "not validated, estimate X", which still counts as an output.

**D-A10 (deferring trial-level R²).** A10 R4 (docs/rust/briefs/founding-council-r4.md, summary point 6) proposes to "drop or defer trial-level R², Steiger and Hartung-Knapp until a reference is in hand". *Steelman:* an estimator with no reference result cannot be validated, and unvalidated code should not produce numbers. *Objection:* the cost of the surrogate test is the data, not the code. The L5 dataset (biomarker change and hard-outcome effect per trial) can be extracted during flagship extraction at small marginal cost; collecting it later means re-opening the same papers. *Position:* defer the R² computation as A10 asks, but keep L5 extraction in the flagship window and freeze the dataset before any analysis. Agreement with A10 on the rest: no curve-based number before Guyot is validated on ≥ 3 published curves (A10 D31), extraction from week 4 if the one-schema-per-week calendar (A10 D30) is adopted.

**D-chair (G5 scope).** G5 says all 16 classes get an entry, ≥ 4 estimated at L2-sealed. *Steelman:* breadth shows the public what is and is not known. *Objection:* §2 shows that even the best-studied class lacks disability outcomes; "estimated" for 4 classes will mean partial composites. *Position:* keep 16 entries, but count a class as "estimated" only if ≥ 2 north-star components carry a pooled estimate with GRADE; publish component coverage next to each entry.

---

## 5. Resource requests

1. **Full-text access** for supplements and appendices of the 27 trials: most main papers are NEJM/Lancet; several supplements are free, some are not [to verify per trial]. Request: an institutional library proxy or a named academic sponsor (aligned with G10) — no cost estimate yet.
2. **Individual participant data platforms** (Vivli, YODA) for transportability v1: applications require a named academic investigator and a protocol [to verify current terms]. Not needed before wk 12; flag now because approval takes months.
3. ClinicalTrials.gov API and Crossref are free and sufficient for everything else. No compute request.

---

## 6. Literature day-1 actions after the council

1. Open L1: commit the PICO template and the first 5 PICOs (flagship SGLT2i, flagship GLP-1 RA, aspirin from ASPREE, vitamin D from D-Health/VITAL, metformin).
2. Draft the flagship protocol (L2) from §2, including the EXSCEL mapping check and the 11-trial fallback.
3. Pick the 3 pilot trials (proposal: EMPA-REG OUTCOME, LEADER, SELECT: largest SAE tables, one non-diabetic).
4. Ask A9 for the extraction fields for curve reconstruction metadata so schema v2-minimal includes them.
5. Ask A1 to start the sponsor search (resource 1–2).
6. Brief two extractor workers with separate worktrees; no extraction before schema freeze.

---

## References

Trials (ClinicalTrials.gov records, `https://clinicaltrials.gov/study/<NCT>`): NCT01131676, NCT01032629, NCT01989754, NCT01730534, NCT02065791, NCT03036124, NCT03036150, NCT03057977, NCT03057951, NCT03594110, NCT01986881, NCT03315143, NCT03521934, NCT03619213, NCT04509674, NCT04564742, NCT01147250, NCT01179048, NCT01720446, NCT01144338, NCT01394952, NCT02465515, NCT02692716, NCT03496298, NCT03574597, NCT03819153, NCT03914326.

Trial papers: Zinman 2015 10.1056/NEJMoa1504720; Neal 2017 10.1056/NEJMoa1611925; Wiviott 2019 10.1056/NEJMoa1812389; Perkovic 2019 10.1056/NEJMoa1811744; McMurray 2019 10.1056/NEJMoa1911303; Heerspink 2020 10.1056/NEJMoa2024816; Packer 2020 10.1056/NEJMoa2022190; Anker 2021 10.1056/NEJMoa2107038; EMPA-KIDNEY 2023 10.1056/NEJMoa2204233; Cannon 2020 10.1056/NEJMoa2004967; Bhatt 2021 10.1056/NEJMoa2030186; Bhatt 2021 10.1056/NEJMoa2030183; Solomon 2022 10.1056/NEJMoa2206286; Butler 2024 10.1056/NEJMoa2314051; James 2024 10.1056/EVIDoa2300286; Pfeffer 2015 10.1056/NEJMoa1509225; Marso 2016 10.1056/NEJMoa1603827; Marso 2016 10.1056/NEJMoa1607141; Holman 2017 10.1056/NEJMoa1612917; Gerstein 2019 10.1016/S0140-6736(19)31149-3; Hernandez 2018 10.1016/S0140-6736(18)32261-X; Husain 2019 10.1056/NEJMoa1901118; Gerstein 2021 10.1056/NEJMoa2108269; Lincoff 2023 10.1056/NEJMoa2307563; Perkovic 2024 10.1056/NEJMoa2403347; McGuire 2025 10.1056/NEJMoa2501006.

Meta-analyses: Zelniker 2019 10.1016/S0140-6736(18)32590-X; Kristensen 2019 10.1016/S2213-8587(19)30249-9; Sattar 2021 10.1016/S2213-8587(21)00203-5.

Methods: Dahabreh 2020 10.1002/sim.8426; Degtiar & Rose 2023 10.1146/annurev-statistics-042522-103837; Westreich 2017 10.1093/aje/kwx164; Stuart 2011 10.1111/j.1467-985X.2010.00673.x; Cole & Stuart 2010 10.1093/aje/kwq084; Signorovitch 2010 10.2165/11538370-000000000-00000; Inoue 2024 10.1001/jamanetworkopen.2023.46302; Guyot 2012 10.1186/1471-2288-12-9; Royston & Parmar 2013 10.1186/1471-2288-13-152; Buyse 2000 10.1093/biostatistics/1.1.49; Salomon 2015 10.1016/S2214-109X(15)00069-8.

Baseline sources: McNeil 2018 10.1056/NEJMoa1800722; McNeil 2018 10.1056/NEJMoa1803955; Neale 2022 10.1016/S2213-8587(21)00345-4; Manson 2019 10.1056/NEJMoa1809944; WHI Writing Group 2002 10.1001/jama.288.3.321.
