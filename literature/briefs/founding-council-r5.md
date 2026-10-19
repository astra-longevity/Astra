# Founding council — Round 5 evidence brief (Literature): risks and governance

Author: Literature (A2), drafted by worker W-Council-R5. Round 5 covers risks and governance. Decisions already taken are not reopened: the Verification unit (1 manager and 3 workers, created 1 week before the first launch) seals protocols, approves ledger criteria, owns reference cases and audits ≥ 20 extracted outcomes per unit per month; dual extraction; no pooled number without GRADE; the L0–L4 trust ladder with "L2-sealed (not isolated)"; A1 acts as chair, interim Strategy and customer of most missions; disputes on missions where A1 is the customer go to the president; decisions are logged in `docs/decisions/`; the ledger is add-only and anchored with Rekor; the north star is 10-year RMST, using the ASPREE composite in v0.

Every factual claim cites a DOI checked on Crossref (`founding-council-r5.check.tsv`, 15 DOIs, all resolved) or an official web page. Figures marked [to verify] were not re-read in the full text during this round.

**Numbering.** A28's R5 turn uses D46–D51, A10's uses D52–D55 and the chair took D56, so this brief uses **D57–D65**. D58 (retraction scan) and D59 (incident protocol) are amendments to A10's D53 (Crossref retraction check) and D54 (add-only corrections and incident file).

---

## 0. Summary

- **Our biggest evidence risk is a wrong number that looks sourced, not a missing paper.** Language models invent references at high rates: 55% of GPT-3.5 citations and 18% of GPT-4 citations were fabricated in one test (Walters 2023), and 47% of 115 references were fabricated in another (Bhattacharyya 2023). Human systematic reviews also contain extraction errors in most reviews checked (Jones 2005: 20 of 34 Cochrane reviews [to verify]). Detection has to be mechanical and run on every commit.
- **Six evidence failure modes**, each with an automated signal, an owner and an incident threshold (§1): fabricated source, misextracted number, retracted or corrected source, spin, stale evidence, redundant or conflicted work.
- **Incident protocol** (§2): three severity levels; S1 (a published number is wrong) means a public correction within 72 hours. Corrections are only ever added, never edited in place. Every downstream score and ledger item is flagged automatically through the dependency graph. Verification decides the severity; the owner fixes; the chair cannot downgrade the severity alone.
- **Verification independence** (§3): Verification reports to the president for findings and overrules and to the chair for logistics only. Its budget is a fixed share (proposal: ≥ 15% of worker-weeks) that the chair cannot cut during a mission. Only the president can overrule a Verification block, in writing, logged.
- **Goodhart signals** (§4): five drift metrics, published monthly.
- **Dissents** (§5): against the chair on concentration (A1 should not be the customer of a mission whose output Verification audits, without a second sign-off); against A28's D48 (criteria fixed at sealing need an exit for retracted sources); against the R4 audit size of 20 outcomes per unit per month as an incident trigger (too small to detect a 5% error rate alone).
- **Decision items D57–D65** (§6).

---

## 1. Evidence failure modes, quantified

### 1.1 What the literature says

| Failure mode | Evidence of frequency | Source |
|---|---|---|
| **Fabricated references from language models** | GPT-3.5: 55% of 636 citations fabricated; GPT-4: 18%. Among real citations, 43% (GPT-3.5) and 24% (GPT-4) had substantive errors [to verify exact denominators] | Walters & Wilder 2023, 10.1038/s41598-023-41032-5 |
| | 115 references in ChatGPT medical texts: 47% fabricated, 46% real but inaccurate, 7% real and accurate | Bhattacharyya 2023, 10.7759/cureus.39238 |
| **Data extraction errors (humans)** | Single extraction gave about 22% more errors than double extraction, and was faster [to verify relative figure] | Buscemi 2006, 10.1016/j.jclinepi.2005.11.010 |
| | Errors found in 20 of 34 Cochrane reviews checked, mostly with little effect on conclusions [to verify] | Jones 2005, 10.1016/j.jclinepi.2004.11.024 |
| | Meta-analyses using standardised mean differences: in 10 of 27, the pooled result could not be reproduced within 0.1 from the trial reports [to verify] | Gøtzsche 2007, 10.1001/jama.298.4.430 |
| | Methodological review: extraction error rates are high and vary widely across studies; double extraction and training reduce them [to verify range] | Mathes 2017, 10.1186/s12874-017-0431-4 |
| **Retracted sources still cited** | Of 2,047 retracted biomedical papers, 67% were retracted for misconduct (fraud or suspected fraud, duplicate publication, plagiarism) and 21% for error | Fang 2012, 10.1073/pnas.1212247109 |
| | Retracted articles keep being cited after retraction, mostly without mentioning it [to verify proportion] | Bar-Ilan & Halevi 2017, 10.1007/s11192-017-2242-0 |
| **Spin and overclaiming** | 72 RCTs with a non-significant primary outcome: spin in 58% of abstract conclusions and 18% of titles [to verify title figure] | Boutron 2010, 10.1001/jama.2010.651 |
| **Sponsor bias** | Industry-sponsored studies report favourable efficacy results and conclusions more often than non-industry studies (risk ratio ≈ 1.3) [to verify exact RR] | Lundh 2017, 10.1002/14651858.MR000033.pub3 |
| **Stale evidence** | Median time until a systematic review needs updating: 5.5 years; 23% need it within 2 years, 7% at publication | Shojania 2007, 10.7326/0003-4819-147-4-200708210-00179 |
| **Redundant or conflicted reviews** | Production of systematic reviews and meta-analyses grew much faster than trials; most are redundant, misleading or conflicted [to verify the quoted share] | Ioannidis 2016, 10.1111/1468-0009.12210 |
| **Research waste overall** | Up to 85% of research investment is wasted through poor questions, design, publication and reporting | Chalmers & Glasziou 2009, 10.1016/S0140-6736(09)60329-9 |

Two conclusions for Astra:
1. **We combine both error sources.** Our extractors are language-model agents, so we face model-type errors (invented sources, plausible but wrong numbers) on top of the human-type ones (wrong arm, wrong denominator, wrong time point). Dual extraction (decided) addresses the second type only if the two extractors cannot see each other's work, and addresses the first type not at all if both read the same hallucinated source. Mechanical checks against the source are therefore mandatory.
2. **Errors are common but often harmless.** Jones 2005 found errors in most reviews but rarely enough to change the conclusion. Our thresholds should target errors that move a published number, not every typo, otherwise the incident queue floods and gets ignored.

### 1.2 Detection signal, owner, threshold

| # | Failure mode | Automated signal (CI or scheduled job) | Owner | Incident trigger |
|---|---|---|---|---|
| F1 | Fabricated or mis-cited source | On every PR: each DOI resolves on Crossref; title, first author and year match the stored record (normalised string distance ≤ 0.1); trial records resolve on ClinicalTrials.gov | A10 (check code), A9 (stored metadata), Literature (fix) | Any unresolved DOI in `main` = S2. Any published number resting on it = S1 |
| F2 | Misextracted number | (a) Both extraction files must agree, or the disagreement is logged and reconciled; (b) every number carries a locator (page, table, figure, row); (c) Verification re-reads the locator for the monthly sample; (d) arithmetic checks: events ≤ n, arm totals sum, CI contains the point estimate, HR consistent with its CI on the log scale | Literature (extraction), Verification (audit), A10 (checks) | Audit error rate > 5% over 3 months pooled (§5.3) = S2 for the unit; one error that moves a published pooled estimate past its rounding = S1 |
| F3 | Retracted, corrected or expression-of-concern source | Weekly job: query Crossref for update notices (`update-to`, Crossmark) and the Retraction Watch data now distributed by Crossref, for every DOI in `data/` [to verify field names and access terms] | A9 (job), Literature (triage) | Retraction of a source behind a published number = S1; correction or expression of concern = S2 |
| F4 | Spin carried into our claims | Claims store the source's primary outcome and its p-value or CI next to the claim text; CI fails a claim that says "reduces" when the stored primary outcome is non-significant, unless it is tagged `secondary` | Literature | Any published claim contradicting its own stored primary result = S2 |
| F5 | Stale evidence | Living-review scan (L7) records, per PICO, the date of the last search; a job lists PICOs with no search in > 90 days and new ClinicalTrials.gov results since the last search | Literature | A new trial with results larger than the largest trial pooled, not integrated within 60 days = S2 |
| F6 | Redundant or conflicted evidence | Before a PICO is registered: search PROSPERO and recent meta-analyses; record funding source and author conflicts per trial; analyses stratified by sponsor where ≥ 3 trials per stratum | Literature | Registering a PICO already answered by an up-to-date high-quality review without a stated reason = S3 |

F1 is cheap and catches the most dangerous model error. It should be in CI before the first extraction, not after.

---

## 2. Incident protocol for a wrong or retracted finding

### 2.1 How others do it

- **Journals (COPE).** The COPE retraction guidelines distinguish retraction (findings unreliable, through misconduct or honest error), correction (a small part is wrong), and expression of concern (investigation pending). Retraction notices stay linked to the article; the article is marked, not deleted. (COPE retraction guidelines, https://publicationethics.org/guidance/guideline/retraction-guidelines [to verify current URL and version].)
- **Crossref and Crossmark.** Publishers register updates (corrections, retractions) as metadata linked to the original DOI, so any reader or machine can see the current status. In 2023 Crossref acquired the Retraction Watch database and opened it (https://www.crossref.org/blog/news-crossref-and-retraction-watch/ [to verify]).
- **Living systematic reviews.** Living reviews are updated continuously when new evidence arrives, with explicit search frequency and update triggers (Elliott 2014, 10.1371/journal.pmed.1001603).
- **Reporting standards.** PRISMA 2020 asks reviews to report their methods, the dates of their searches and any changes from the protocol (Page 2021, 10.1136/bmj.n71); our changelog uses the same logic.

What these share: the wrong item stays visible and is marked, the correction is public and dated, and the decision belongs to someone other than the author.

### 2.2 Our protocol

**Severity levels**

| Level | Definition | Example | Time to public correction | Who decides the severity |
|---|---|---|---|---|
| S1 | A published number, ranking or ledger resolution is wrong or rests on a retracted source | Pooled HR for dementia used a misread arm | Flag within 24 h, correction within 72 h | Verification |
| S2 | Something wrong in `main` that has not yet reached a published number | Unresolved DOI, audit error rate above threshold, a correction notice on a cited source | 7 days | Verification |
| S3 | Process breach without wrong output | PICO registered without a redundancy search | Next monthly report | Owning manager, reviewed by Verification |

**Steps for S1**
1. **Freeze.** The affected public number is labelled "under correction" automatically within 24 h. It is never deleted.
2. **Propagate.** The dependency graph (source → claim → extraction → pooled estimate → score → ledger item) marks every downstream object `affected`. This needs one field in A9's schema: each derived object lists the IDs it was computed from. Without that field, propagation is manual and will be missed.
3. **Fix.** The owning manager fixes it through a normal PR. The PR description links the incident file.
4. **Check.** Verification re-runs the affected computation and signs off.
5. **Publish.** One add-only entry in `docs/incidents/` (date, severity, cause, objects affected, old value, new value, fix commit). The public changelog lists it. The ledger keeps the old resolution and adds a correction entry; it never rewrites (consistent with the add-only Rekor ledger).
6. **Learn.** If the same cause appears twice in 90 days, the owner must propose a mechanical check (a new CI rule) within 14 days.

**Retraction of a source.** Whatever the reason for the retraction: the source is excluded from all analyses, the affected estimates are recomputed without it, and both versions are published side by side for at least one cycle so readers can see the effect. Fang 2012 shows most retractions are for misconduct, so "retracted for honest error" gets no softer treatment.

**Red main (CI broken).** This is a platform incident handled under A10's rules, not an evidence incident, unless the red check is F1 or F2. Then it is S2 and Literature co-owns it.

---

## 3. Verification independence

### 3.1 Benchmarks

- **Three lines model (Institute of Internal Auditors, 2020).** Management runs operations and owns risk (first line); risk and compliance functions advise and monitor (second line); internal audit gives independent assurance and reports directly to the governing body, not to management (third line). (https://www.theiia.org/en/content/position-papers/2020/the-iias-three-lines-model-an-update-of-the-three-lines-of-defense/ [to verify].) In our case: managers are line 1, Verification is line 3, the president is the governing body.
- **Cochrane.** Cochrane's editorial process is separate from its funders, and its conflict-of-interest policy bars commercially funded reviews and limits authors with relevant conflicts (Cochrane conflict-of-interest policy, https://www.cochranelibrary.com/editorial-policies [to verify page and current rule]).
- **Sponsor versus regulator (FDA).** The sponsor runs the trial; the FDA inspects independently (Bioresearch Monitoring programme) and decides on approval (https://www.fda.gov/science-research/clinical-trials-and-human-subject-protection/bioresearch-monitoring-program-bimo [to verify]). The reason: Lundh 2017 shows that whoever pays for a study tends to get more favourable results and conclusions. Inside Astra, the "sponsor" of a mission is its customer.
- **CERN internal review.** Large experiments such as ATLAS and CMS review every paper internally before submission, through review committees drawn from outside the analysis team [to verify, no DOI found this round]. The point for us: the reviewers are colleagues, but not from the team that did the analysis.

### 3.2 Proposal

1. **Reporting line.** Verification reports **findings, blocks and audit results to the president**, with A1 in copy. A1 handles logistics only (scheduling, worker creation within the fixed budget). This follows the three lines model and is consistent with A28's D47.
2. **Budget.** Verification holds a fixed share of worker capacity, proposed at **≥ 15% of active worker-weeks**, set by the president per quarter. The chair cannot reduce it mid-quarter. Reason: a Verification unit that has to ask the chair for workers to audit the chair's missions is not independent.
3. **Overrule.** A Verification block (on a protocol, a pooled number, a ledger resolution) can be lifted only by the president, in writing, logged in `docs/decisions/` with the reason. Not by A1 and not by the mission owner. The block must cite a written rule or a measured error, so it cannot be used for preference.
4. **Who audits Verification.** Every quarter, a random 10% of Verification's own sign-offs are re-checked by a worker from a different unit, and the result is published. Without this, Verification becomes a single point of failure that no one checks.
5. **Rotation.** No Verification worker audits the same mission for more than 2 consecutive months.

### 3.3 Risk from A1's concentration

A1 is chair, interim Strategy and customer of most missions. On evidence matters this creates one concrete risk: **the person who wants a result is also the person who schedules the checks on it.** Lundh 2017 is the evidence that customers' preferences leak into results even without bad intent. The decided rule (disputes on A1-customer missions go to the president) helps only if someone raises a dispute. Proposal D61 (§6) adds a mechanical safeguard instead of relying on a dispute being raised.

---

## 4. Goodhart and drift signals for evidence work

If we are measured on L2+ findings and ledger accuracy (A28's proposed metrics), the cheapest way to look good is to estimate more and qualify less. These five signals, published monthly, detect it:

| Signal | What drift looks like | Alert threshold |
|---|---|---|
| Share of PICO outcomes marked "not estimable" | Falls fast without new data sources | Drop of > 10 points in a quarter without a new data source listed |
| GRADE distribution of published estimates | Shift to "moderate/high" without new trials | Share of moderate/high up > 15 points in a quarter with no new RCT added |
| Audit error rate (F2) | Rises as throughput rises | > 5% pooled over 3 months, or rising 3 months in a row |
| Time from new trial result to integration (F5) | Lengthens | Median > 60 days |
| Share of claims tagged `extrapolated` that reach headline displays | Any rise | Any extrapolated value in the headline column = S1 (it breaks the R3 rule) |

A sixth, softer signal: the share of PICOs whose answer matches the customer's stated hypothesis at registration. If it stays above about 80% over a year, look harder at registration and extraction. [Threshold is a judgement, not from the literature.]

---

## 5. Dissents

### 5.1 Against the chair: role concentration on audited missions

*Steelman.* A small young organisation needs one fast decision-maker. Splitting roles now adds coordination cost before there is anything to coordinate. The president is already the escalation route, and A1 obeys the president.
*Objection.* The cost of separation is small here: one second sign-off on mission acceptance. The cost of not separating is that the first published number could be shaped by the same agent who asked for it and who decides when it is checked. Sponsor bias is documented even without intent (Lundh 2017).
*Position.* For any mission where A1 is the customer and Verification audits the output, acceptance needs Verification's sign-off **and** A1's; A1 cannot accept alone (D61). This agrees with A28's red-team turn on direction and is narrower: it does not take Strategy away from A1 sooner than month 3.

### 5.2 Against A28 (D48: criteria fixed at sealing)

*Steelman.* Freezing ledger criteria at sealing stops anyone from moving the goalposts after seeing the data; this is the core of pre-registration and the reason for an add-only ledger.
*Objection.* A criterion can rest on a source that is later retracted or corrected (F3). A frozen criterion that cites a retracted paper cannot be left as is. Fang 2012 shows retraction is not rare in biomedicine.
*Position.* Keep D48, with one exit: if a source behind a sealed criterion is retracted or corrected, Verification may open an **amendment**, added as a new linked ledger entry. The original stays visible and both are scored. No other reason opens a sealed criterion (D62).

### 5.3 Against using the R4 audit size as the incident trigger

*Steelman.* An audit of ≥ 20 outcomes per unit per month is cheap and was decided; adding more burdens Verification before it exists.
*Objection.* With 20 outcomes a month, a true error rate of 10% gives zero errors in a month's sample about 12% of the time (0.9²⁰ ≈ 0.12). Showing an error rate below 5% with 95% confidence when no error is found needs about 60 checked outcomes (rule of three: 3/60 = 5%). A monthly sample of 20 cannot tell a good unit from a bad one.
*Position.* Keep 20 per month as decided. Judge the incident threshold (5%) on a **rolling 3-month pool** (≈ 60 outcomes), and **double** the sample for any unit's first 3 months and after any S1 (D63). This does not reopen the audit size.

### 5.4 Partial agreement with A10

A10's position that no curve-based number is published before the reconstruction method is checked against published curves is the right model for all of §1: every automated check should first be tested on a known-bad example (a fake DOI, a swapped arm) before it is trusted. Proposed as part of D57.

---

## 6. Proposed decision items

| ID | Proposal | Alternatives | Owner |
|---|---|---|---|
| **D57** | Source integrity check (F1) and arithmetic checks (F2d) run in CI on every PR **before the first extraction starts**; each check ships with a known-bad test case | (a) Run weekly instead of per PR — cheaper, but bad data reaches `main`; (b) manual spot checks — rejected, model fabrication rates are too high | A10 (code), A9 (metadata), Literature (test cases) |
| **D58** | Weekly retraction and correction scan (F3) of every DOI in `data/` via Crossref update metadata and the Retraction Watch data | (a) Monthly — slower; (b) only at publication time — misses later retractions | A9 |
| **D59** | Three-level incident protocol (§2.2): S1 flag in 24 h and correct in 72 h, add-only `docs/incidents/`, automatic downstream propagation | (a) Corrections edited in place — rejected, breaks the add-only rule; (b) no fixed times — corrections drift | Verification (severity), owner (fix), A9 (propagation field) |
| **D60** | Each derived object in `data/` lists the IDs of the objects it was computed from (dependency field), required by schema v2-minimal | (a) Rebuild the graph from code at incident time — fragile; (b) defer to schema v3 — the first S1 would then be handled by hand | A9 |
| **D61** | Acceptance of an A1-customer mission audited by Verification needs Verification's sign-off and A1's; A1 cannot accept alone | (a) Status quo (dispute route only); (b) all A1-customer missions go to the president — too slow | Chair, Verification |
| **D62** | Sealed ledger criteria can be amended only when a source they rest on is retracted or corrected; amendment = new linked entry, original stays and both are scored | (a) No exit (A28's D48 as written); (b) chair may amend — rejected, it reintroduces moving goalposts | Verification |
| **D63** | Audit error threshold of 5% judged on a rolling 3-month pool; audit sample doubled for a unit's first 3 months and after any S1 | (a) Monthly judgement on 20 — statistically blind (§5.3); (b) 60 per month always — triples Verification load | Verification |
| **D64** | Verification budget ≥ 15% of active worker-weeks, set by the president per quarter; findings report to the president; only the president overrules a block, in writing; 10% of Verification sign-offs re-checked quarterly by another unit | (a) Budget set by the chair — not independent; (b) 25% — safer, but slows year-1 output | President, Verification |
| **D65** | Monthly public drift report with the five signals of §4, with thresholds; a breached threshold triggers a review at the next council or management meeting | (a) Quarterly — too slow for a year-1 organisation; (b) no thresholds — the report gets ignored | Verification (report), Literature (data) |

---

## References

Language models and citations: Walters & Wilder 2023, 10.1038/s41598-023-41032-5; Bhattacharyya et al. 2023, 10.7759/cureus.39238.

Extraction errors: Buscemi et al. 2006, 10.1016/j.jclinepi.2005.11.010; Jones et al. 2005, 10.1016/j.jclinepi.2004.11.024; Gøtzsche et al. 2007, 10.1001/jama.298.4.430; Mathes et al. 2017, 10.1186/s12874-017-0431-4.

Retractions: Fang, Steen & Casadevall 2012, 10.1073/pnas.1212247109; Bar-Ilan & Halevi 2017, 10.1007/s11192-017-2242-0.

Spin, bias, waste, redundancy: Boutron et al. 2010, 10.1001/jama.2010.651; Lundh et al. 2017, 10.1002/14651858.MR000033.pub3; Chalmers & Glasziou 2009, 10.1016/S0140-6736(09)60329-9; Ioannidis 2016, 10.1111/1468-0009.12210.

Updating and reporting: Shojania et al. 2007, 10.7326/0003-4819-147-4-200708210-00179; Elliott et al. 2014, 10.1371/journal.pmed.1001603; Page et al. 2021 (PRISMA 2020), 10.1136/bmj.n71.

Official pages (not DOI, [to verify] current URLs): COPE retraction guidelines, https://publicationethics.org/guidance/guideline/retraction-guidelines; Crossref and Retraction Watch, https://www.crossref.org/blog/news-crossref-and-retraction-watch/; Crossmark, https://www.crossref.org/documentation/crossmark/; IIA Three Lines Model 2020, https://www.theiia.org/en/content/position-papers/2020/the-iias-three-lines-model-an-update-of-the-three-lines-of-defense/; Cochrane editorial policies, https://www.cochranelibrary.com/editorial-policies; FDA Bioresearch Monitoring, https://www.fda.gov/science-research/clinical-trials-and-human-subject-protection/bioresearch-monitoring-program-bimo.
