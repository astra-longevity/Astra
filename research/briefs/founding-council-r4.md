# Founding council R4: roadmap, research side

Author: W-Research-R4 for Popper (A28, Research Manager). Round 4 of the founding council (G60).
Status: brief for A28. A28 makes the proposals; this brief supplies sources, numbers, and dissent.

This brief builds on the R3 decisions: the north star is a 10-year RMST gain in disease- and disability-free survival. Ages 70–79 are the headline and ages 60–69 are "not estimable". The event list is D21. The engine is Guyot reconstruction followed by van Zwet–Gelman shrinkage and RMST. Gates are L2-sealed. The year-1 goals are G1–G10. It does not reopen any of these. Where it disagrees, the disagreement is in §7.

Every number below comes from a cited DOI or an official page. Anything I could not check on a primary source is marked `[to verify]`. All DOIs were checked on Crossref on 2026-10-19 (17 of 17 resolve, with matching titles).

---

## 1. Phases and exit criteria

The research-owned goals are G1 (findings), G2–G3 (ledger), G6 (MR panel), G7 (calibration), G8 (Mission 2b) and G10 (outside reviewer). The other goals belong to other managers. Research depends on them (§4) but does not report on them.

### Phase A: months 0–3 ("instruments before answers")

Goal: every research instrument exists and has been run once on a case with a printed answer. Few findings are expected.

| Exit criterion | G | Check |
|---|---|---|
| Ledger v1 live with ≥ 15 open forecast items, each with a sealed resolution rule and date | G2, G4 | count of items in `data/` with `resolution_rule` and `resolves_by` |
| MR estimators (IVW, Egger, weighted median) reproduce the printed reference case (§2.2) to the printed precision | G6, G9 | astra-stats test passes in CI |
| Calibration re-derivation protocol sealed and the per-trial extraction table double-entered | G7 | protocol hash in ledger; two extractions agree or have adjudicated diffs |
| ASPREE placebo-arm curve digitised twice, independently, and both reconstructions reproduce the printed numbers at risk | Mission 2a input | Guyot check (§2.6) |
| Sponsor ask sent to ≥ 3 named academics | G10 | sent messages logged |

Phase A **fails** if the MR reference case does not reproduce by week 8. In that case, MR results are published at L1 only (§4 plan B) and G6 moves to month 9.

### Phase B: months 3–12 ("first numbers")

| Exit criterion | G |
|---|---|
| ≥ 12 L2-sealed findings, max one per PICO, nulls counted | G1 |
| ≥ 40 resolved ledger items, ≥ 12 external; Brier skill score with 90% interval lower bound > −0.05; non-resolution ≤ 10% | G2, G3, G4 |
| MR panel ≥ 8 targets, all positive controls passing | G6 |
| Calibration re-derivation published with every pooled number reproduced or each deviation explained | G7 |
| Mission 2b protocol pre-registered by month 6 | G8 |
| ≥ 1 named outside reviewer has read one sealed finding and given a written verdict | G10 |

Rough expected split of the 12 G1 findings (a planning assumption, not a promise): ~6 MR targets, 1 calibration, 1–2 flagship pooled effects, and ~3 ledger-driven or 2b results.

### Phase C: years 2–3 ("transport and new data")

- Transportability rule (§3a) applied to every flagship effect. Each effect is tagged direct or extrapolated, with the check numbers published.
- Ages 60–69 become estimable only if a printed baseline curve passes §3b. Otherwise they stay "not estimable", which is an honest result.
- First analysis on access-controlled data (NHANES restricted files or UK Biobank, §5), only once a sponsor and the funding exist.
- Exit: one finding is reproduced by an outside group or reviewer (an L3 candidate), and the ledger has a 2-year Brier record.

---

## 2. Research missions among the first 10

Owner = whoever produces the headline number. Customer = A1 for year 1, as decided.

### 2.1 Ledger v1

- **Owner** Research. **Contributors** Verification (resolution criteria), A9 (schema fields).
- **Output** A public, append-only forecast ledger of trial outcomes and replication results. This is original: every forecast is a probability made before the outcome is known.
- **Moves** G2, G3, G4.
- **Depends on** ledger rails (weeks 1–2), schema v2-minimal frozen in week 2.
- **Start** week 1, with drafting items as YAML. Sealing starts once the rails exist.
- **Fails if** fewer than 15 sealed items by month 3, or the non-resolution rate goes above 10% at any quarterly review. Then the item-writing rule is tightened before any new items are added.

### 2.2 Drug-target MR panel

- **Owner** Research. **Contributors** Platform (astra-stats MR core), Verification (reference tests), A2 (target list against the trial literature).
- **Output** Genetic proxies for ≥ 8 drug targets against the D21 outcomes, with positive controls (for example, a target whose trial effect on LDL and coronary disease is known).
- **Moves** G6, G1.
- **Reference cases (printed inputs):**
  - **IVW, Egger, weighted median:** the example dataset shipped with the MendelianRandomization package. It has 28 lipid variants with per-SNP associations and standard errors for LDL-C, HDL-C, triglycerides and coronary heart disease log-odds, so the **inputs are complete and printed** (Yavorska & Burgess 2017, doi:10.1093/ije/dyx034). The package page prints an IVW estimate of 2.834 (SE 0.530, 95% CI 1.796–3.873), heterogeneity Q 99.53 on 27 df, and residual SE 1.920 (https://amymariemason.github.io/MR). Which exposure that output refers to is `[to verify]` before it is used as a test oracle. Method papers: IVW, Burgess et al. 2013 (doi:10.1002/gepi.21758); Egger, Bowden et al. 2015 (doi:10.1093/ije/dyv080); weighted median, Bowden et al. 2016 (doi:10.1002/gepi.21965).
  - **Coloc:** Giambartolomei et al. 2014 (doi:10.1371/journal.pgen.1004383) defines the method. A printed per-SNP input set with printed posterior probabilities H0–H4 for a full region is **not** in the paper tables I checked, so coloc inputs are **incomplete** `[to verify]`. The fallback is to test the closed-form Approximate Bayes Factor formula on hand-computed cases first, and to treat any published locus table as an extra test.
- **Depends on** the MR core in astra-stats (weeks 3–5). Variant selection and data extraction do not depend on it.
- **Start** week 2 (target list and instrument tables); estimates from week 6.
- **Fails if** any positive control fails. Then no target result is published above L1 until the cause is found.

### 2.3 Calibration re-derivation (G7)

- **Owner** Research. **Contributor** A2 (extraction, GRADE). **Customer** A1.
- **Chosen meta-analysis:** **Zelniker et al. 2019, Lancet 393:31–39, "SGLT2 inhibitors for primary and secondary prevention of cardiovascular and renal outcomes", doi:10.1016/S0140-6736(18)32590-X.** It pools three trials (EMPA-REG OUTCOME, CANVAS Program, DECLARE-TIMI 58), prints per-trial hazard ratios and CIs in forest plots, and splits them by established atherosclerotic disease. This means every pooled number can be re-derived from printed per-study numbers. With three trials, the work fits in a short mission and the between-study variance estimate is visibly fragile, which is itself a useful test of the shrinkage code.
- **Second case** (if time allows, and it overlaps the flagship): Sattar et al. 2021, eight GLP-1 RA outcome trials, doi:10.1016/S2213-8587(21)00203-5.
- **Output** A re-derivation report: every pooled HR, CI and heterogeneity statistic is reproduced, or the deviation is explained. Then a second, original layer: the same trials run through the Astra engine (shrinkage and RMST), with the difference from the published pooled result shown.
- **Moves** G7, G9 (reference-tested estimators), G1 (one finding).
- **Depends on** the meta-analysis core in astra-stats, already validated against printed numbers (R2 plan).
- **Start** week 3 (extraction); computation once the meta-analysis core passes its reference test.
- **Fails if** a published pooled number cannot be reproduced to the printed precision. That is reported as a finding, not hidden.

### 2.4 Flagship pooling (SGLT2i / GLP-1 RA)

- **Owner** Research (headline number). **Contributors** A2 (extraction from week 3, GRADE), Verification.
- **Output** Pooled effects on the D21 components, transported to healthy 70–79-year-olds with the §3a rule, then turned into 10-year RMST gains. The original output is the transported RMST, which no trial reports.
- **Moves** G1, G5 (class coverage).
- **Depends on** the calibration re-derivation passing (2.3), the ASPREE baseline (2.6), and the transportability rule (§3a).
- **Start** week 3 (extraction); RMST from month 4.
- **Fails if** the effect fails the transport check. Then it goes to the extrapolated column, which is a valid, published outcome.

### 2.5 Mission 2b

- **Owner** Research. **Customer** A1.
- **Output** A pre-registered protocol (G8, by month 6) for the second scoring wave. Its exact scope is decided at the month-3 review from ledger and MR results.
- **Fails if** not sealed by month 6. Then G8 is reported as missed. The deadline is not moved.

### 2.6 Scoring inputs for Mission 2a: ASPREE placebo curve

- **Source** McNeil et al. 2018, NEJM, "Effect of aspirin on disability-free survival in the healthy elderly", doi:10.1056/NEJMoa1800722. It has 19,114 participants aged ≥ 70 (≥ 65 for US Black and Hispanic participants), and the primary endpoint is death, dementia or persistent physical disability.
- **Method** Guyot et al. 2012 (doi:10.1186/1471-2288-12-9): two independent digitisations, each reconstructed, checked against the printed numbers at risk and the printed hazard ratio. RMST follows Royston & Parmar 2013 (doi:10.1186/1471-2288-13-152).
- **Moves** the Mission 2a input (and G9 through the engine's reference tests).
- **Start** week 1. This is pure data work with no code dependency.
- **Fails if** the two digitisations disagree beyond a tolerance sealed in advance. Then a third digitisation adjudicates.

### 2.7 NHANES target-trial emulation: when

**Not in year 1.** It needs restricted-use linked mortality files through NCHS (§5), which costs ≥ $3,000 in fees plus an application, and an academic sponsor who can own a data agreement. Proposal: start the protocol in month 9 if G10 is met and money is approved, and run it in year 2. Until then, nothing in the roadmap depends on it.

---

## 3. Open items

### (a) Transportability rule

**Problem.** Most flagship effects come from people with diabetes, heart disease or kidney disease. The headline is about healthy 70–79-year-olds. Reusing a relative effect (HR) in a new population is valid only if the variables that change the effect are distributed the same way, or are adjusted for (Dahabreh et al. 2020, doi:10.1002/sim.8426; Dahabreh & Hernán 2019, doi:10.1007/s10654-019-00533-2). Bareinboim & Pearl 2016 (doi:10.1073/pnas.1510507113) give the formal condition: the effect transports if the selection differences do not reach the outcome other than through measured variables. Degtiar & Rose 2023 (doi:10.1146/annurev-statistics-042522-103837) review the estimators and stress that the key assumption cannot be tested and must be argued.

**Proposed rule, which can be checked from printed data.** A relative effect is tagged `direct` for healthy 70–79-year-olds only if **all three** of these hold:

1. **Age:** the trial prints a subgroup result for age ≥ 65 (or ≥ 70) whose HR has a CI overlapping the overall HR, and its own CI excludes 1 in the same direction, or the interaction p-value is printed and ≥ 0.10.
2. **Disease-state modifier:** the trial or meta-analysis prints a split on the disease that defines the population (for example, established atherosclerotic disease versus multiple risk factors only, as Zelniker 2019 does). The subgroup closest to "healthy" has an HR whose CI includes the overall HR. If no such split is printed, criterion 2 fails.
3. **Mechanism:** the effect is on an outcome in D21 and is not driven by the index disease. For example, a glucose-lowering effect on diabetic complications does not count.

If any criterion fails, the effect is tagged `extrapolated` and goes in the separate column. A quantitative sensitivity check follows Dahabreh et al. 2023's bias functions (doi:10.1002/sim.9550): we print how much effect modification would erase P(net > 0) ≥ 0.8. This rule is crude by design. It uses only printed subgroups and needs no individual data, so a reviewer can redo it from the PDF.

### (b) Printed baseline curves for ages 60–69

| Candidate | What it covers | Reuse status |
|---|---|---|
| **D-Health** (Neale et al. 2022, doi:10.1016/S2213-8587(21)00345-4): 21,315 Australians aged 60–84, placebo arm, all-cause mortality primary | Death only (not the D21 composite) | Reuse of the published figure for digitisation: licence `[to verify]` |
| ASPREE US subgroup aged 65–69 | Death, dementia, disability | Not printed as a separate curve `[to verify]` |
| HMD life tables (A10 R3: CC BY 4.0) | All-cause mortality by age, by country | Open licence; general population, not "event-free" |

**Conclusion:** no printed **composite** event-free curve for healthy 60–69-year-olds exists among the sources I checked. Proposal: ages 60–69 stay "not estimable" in year 1, with one reported exception. A **mortality-only** RMST for 60–69 from the D-Health placebo arm, clearly labelled as a mortality-only secondary, not the north star. Year 2: rebuild the composite from cohort data (UK Biobank, §5) if access is funded.

### (c) Harms

Keep **harms per 1,000 people over 10 years on the same baseline** in year 1. Do not weight them into the score. Reasons:

1. Fatal and disabling harms are already inside the composite. A drug that causes strokes or dementia loses RMST automatically.
2. Any weight for non-fatal harms (for example, a genital infection versus a fracture) is a value judgment that the council has no data to make. Printing absolute risks lets readers apply their own.
3. A weight chosen now would be impossible to audit, which goes against the "no printed number, no merge" rule.

The year-2 option is a published utility set, chosen by an outside reviewer (G10), not by Astra.

---

## 4. Critical path from the research side

```
week 1-2   ledger rails (Platform) ──► ledger sealing (Research)
week 2     schema v2-minimal (A9)  ──► extraction YAML (A2, Research)
week 1-   ASPREE digitising x2 (Research, no code dependency)
week 3-5   MR core (Platform)      ──► MR estimates (Research, week 6+)
week 3-    flagship + calibration extraction (A2)
month 2-3  meta-analysis + Guyot + RMST (Platform) ──► calibration, flagship numbers
```

What blocks whom:

- **Research is blocked by:** the ledger rails (sealing), the schema (canonical storage), and the astra-stats MR and survival cores (any number above L1).
- **Research blocks:** A1's Mission 2a scoring (the ASPREE curve and transported effects) and Verification's reference tests (Research supplies the printed cases in §2.2–2.3).

**Plan B if Verification or A9 slips.** Research keeps working at **L1** (hand-recorded, unsealed). Nothing is lost, because the work is data, not code:

- MR variant tables, flagship extraction and ASPREE digitisations are written as plain files on a branch and imported once the schema freezes.
- Ledger forecasts are written with a timestamped commit on main (git history serves as a weak seal). They are re-sealed when the rails exist. Items whose timestamp is before the outcome stay valid.
- No L1 result is used in a score or announced as a finding. The G1 count only includes L2-sealed results, so a slip delays G1 but never inflates it.

Hard stop: if the MR core is not reference-tested by month 4, G6 is formally moved and reported.

---

## 5. Resource requests (for A1, who owns resources)

1. **Human academic sponsor.** Needed for G10, for any data agreement that requires an institution, and for grants that pay institutions. Ask: 3 named candidates (an epidemiologist with MR experience, a trialist, a biostatistician) contacted in week 1.
2. **UK Biobank** (official fees, https://www.ukbiobank.ac.uk/use-our-data/fees/, excluding VAT, 3-year access, analysis on the Research Analysis Platform only):
   - Tier 1 £3,000 (questionnaire, outcomes, linked records); Tier 2 £6,000 (adds assays and genotypes); Tier 3 £9,000 (adds sequencing and imaging). Extensions cost £1,000, £2,000 or £3,000 per year. Each extra institution costs £1,000. Students and lower-income countries pay £500.
   - Cloud compute and storage are billed on top of the fees.
   - An approved institution is required; whether a project with no institution can apply is `[to verify]`, as is the lead time `[to verify]`.
   - Need: Tier 1 is enough for the composite baseline in year 2. **Not requested in year 1.**
3. **CALERIE** (Duke): data download via the CALERIE network (https://calerienetwork.org), 52 SAS files; biosamples through the NIA Aging Research Biobank with CALERIE Steering Committee approval (contact calerie@duke.edu). Cost and lead time `[to verify]`. Need: low in year 1.
4. **NHANES restricted-use files** (NCHS RDC, https://www.cdc.gov/rdc/application-process/fees-and-invoicing.html, updated 2026-07-14): application through the Standard Application Process portal. The management fee is $3,000 for 1–6 data years, rising to $8,250 for 19–20 years. Onsite access costs $300 per day. Amendments beyond the second cost $750; renewal after 3 years costs $1,500. There is no student discount and no refund, and CDC's worked 3-year example totals $6,750. The data request is limited to ≤ 100 variables. Lead time `[to verify]`. Need: year 2 (§2.7).
5. **Impetus Grants** (Norn Group, https://norn.group/impetus-grants): up to $500k, decisions within 3 weeks. Round 1 funded people outside academia (9.6% of grants), students (26%) and postdocs (16%) (https://norn.group/impetus-grants/news-updates/round-1). Whether a project with **no institution** can receive funds is `[to verify]`. The answer probably depends on the sponsor (item 1).

Year-1 money ask from Research: **zero**. Year 2: about £3,000 (UK Biobank Tier 1) plus about $3,000–6,750 (NCHS), only if a sponsor and funder are in place.

---

## 6. Day 1 after the council: Research's first five actions

1. Launch a worker to **digitise the ASPREE placebo curve** (first digitisation); a second worker does it independently the same day.
2. Draft **15 ledger items** on trials resolving after the council ends, each with a resolution rule and a date. They are held as YAML until the rails arrive.
3. Write the **MR target list** with positive controls, and commit the reference dataset's per-SNP table (§2.2) as a printed test fixture for Verification.
4. Send **A2 the Zelniker 2019 extraction spec** (trials, outcomes, subgroups, which figure and table each number comes from).
5. Send the **sponsor ask** (via A1) to 3 named academics.

---

## 7. Dissent

**Objection 1: D21 has no printed baseline curve (against the D21 event list as adopted, which includes A2's additions).** ASPREE's printed primary curve covers death, dementia and persistent disability (doi:10.1056/NEJMoa1800722). No printed placebo curve I found covers the full D21 set (COPD, CKD 4–5, hip fracture, invasive cancer, MI, stroke, HF) in healthy 70–79-year-olds. So a "10-year RMST free of D21" cannot be computed from printed data in year 1. Building it from separate component curves needs competing-risks assumptions that would be new and unchecked. *Steelman of the other side:* a broad list avoids rewarding a drug that just moves deaths into other diseases. *Proposal:* the scored composite for v0 is **only the events a printed curve covers** (ASPREE's death, dementia and disability). The other D21 components are reported per component, as relative effects, until cohort data provide a composite curve.

**Objection 2: shrinkage applied to the wrong unit (against A10 R3 §2.2).** The van Zwet–Gelman prior (van Zwet, Schwab & Senn 2021, doi:10.1002/sim.9173) was fitted on single trials from the Cochrane database. A pooled meta-analytic estimate has a larger z by construction, so it belongs to a different reference class. Applying the same prior to a pooled HR would over-shrink it or give a miscalibrated amount of shrinkage. *Steelman:* one prior is simple and conservative. *Proposal:* shrink each trial before pooling, or apply the prior only to single-trial inputs, and record this choice in the engine's protocol. The calibration mission (§2.3) can measure the difference on three real trials.

**Support with a condition (on A2 R3 §5.2, harms):** we agree with keeping harms per 1,000 in year 1, on the condition that they are printed on the same baseline and horizon as the RMST gain (§3c), so that benefit and harm sit side by side.

**Anticipated counter to A10's R4 red team.** A10 will likely argue that "MR core in weeks 3–5" is unrealistic, given its own R2 estimate of about 7–8 worker-weeks for the whole of astra-stats. A10 will also likely argue that flagship extraction in week 3 depends on a schema that may not freeze in week 2. *Answer:* Research's months 0–3 do not rely on these dates. Variant tables, extraction, digitisation and ledger drafting are data work that sits safely at L1 (§4 plan B). Only computation waits for the code. The real risk is G6 and G7 slipping into Phase B, which is already allowed for: the month-4 hard stop moves G6 openly instead of letting it drift. What Research asks of Platform is **order, not speed**: meta-analysis core and Guyot/RMST first (they unblock G7 and Mission 2a), MR core second.

---

## Sources

All DOIs checked on Crossref on 2026-10-19 with matching titles.

- Bareinboim E, Pearl J. Causal inference and the data-fusion problem. PNAS 2016. doi:10.1073/pnas.1510507113
- Bowden J et al. Mendelian randomization with invalid instruments (MR-Egger). IJE 2015. doi:10.1093/ije/dyv080
- Bowden J et al. Consistent estimation in MR with some invalid instruments (weighted median). Genet Epidemiol 2016. doi:10.1002/gepi.21965
- Burgess S et al. MR analysis with multiple genetic variants using summarized data (IVW). Genet Epidemiol 2013. doi:10.1002/gepi.21758
- Dahabreh IJ et al. Extending inferences from a randomized trial to a new target population. Stat Med 2020. doi:10.1002/sim.8426
- Dahabreh IJ et al. Sensitivity analysis using bias functions for studies extending inferences. Stat Med 2023. doi:10.1002/sim.9550
- Dahabreh IJ, Hernán MA. Extending inferences from a randomized trial to a target population. Eur J Epidemiol 2019. doi:10.1007/s10654-019-00533-2
- Degtiar I, Rose S. A review of generalizability and transportability. Annu Rev Stat Appl 2023. doi:10.1146/annurev-statistics-042522-103837
- Giambartolomei C et al. Bayesian test for colocalisation. PLoS Genet 2014. doi:10.1371/journal.pgen.1004383
- Guyot P et al. Reconstructing the data from published Kaplan–Meier survival curves. BMC Med Res Methodol 2012. doi:10.1186/1471-2288-12-9
- McNeil JJ et al. Effect of aspirin on disability-free survival in the healthy elderly (ASPREE). NEJM 2018. doi:10.1056/NEJMoa1800722
- Neale RE et al. The D-Health Trial. Lancet Diabetes Endocrinol 2022. doi:10.1016/S2213-8587(21)00345-4
- Royston P, Parmar MKB. Restricted mean survival time. BMC Med Res Methodol 2013. doi:10.1186/1471-2288-13-152
- Sattar N et al. GLP-1 receptor agonists: cardiovascular, mortality and kidney outcomes. Lancet Diabetes Endocrinol 2021. doi:10.1016/S2213-8587(21)00203-5
- van Zwet E, Schwab S, Senn S. The statistical properties of RCTs and a proposal for shrinkage. Stat Med 2021. doi:10.1002/sim.9173
- Yavorska OO, Burgess S. MendelianRandomization: an R package. IJE 2017. doi:10.1093/ije/dyx034
- Zelniker TA et al. SGLT2 inhibitors for primary and secondary prevention of cardiovascular and renal outcomes. Lancet 2019. doi:10.1016/S0140-6736(18)32590-X
- Official pages: UK Biobank fees https://www.ukbiobank.ac.uk/use-our-data/fees/ · NCHS RDC fees https://www.cdc.gov/rdc/application-process/fees-and-invoicing.html · CALERIE https://calerienetwork.org · Impetus https://norn.group/impetus-grants and https://norn.group/impetus-grants/news-updates/round-1 · MR package page https://amymariemason.github.io/MR
