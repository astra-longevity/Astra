# Audits

How Verification audits produced work after the fact. Every rule cites the
founding-council decision it comes from
(`docs/decisions/founding-council-log.md`). Rules with no decision are listed
at the end under "Proposed, needs a decision" and do not apply yet.

Verification sets severities and blocks; only the president lifts a block or
overrules a finding, in writing (D47, D64). Auditors rotate every 2 months
(D64). Verification never audits as red team its own findings (D55).

## 1. Extraction audit (D3)

What: re-extract a sample of extracted outcomes from the source and compare.

- Sample size: at least 20 outcomes per producing unit per month. If a unit
  extracted fewer than 20, audit all of them (D3).
- Doubled sample: in a unit's first 3 months, and after any S1 incident in
  that unit (D63).
- The error rate feeds the D3 growth signals (D3).

## 2. Rolling count rule (D63)

Applied to the extraction audit pool of each unit, over a rolling 3 months
(about 60 audited items):

| Errors in the pool | Result |
|---|---|
| ≥ 6 of 60 | incident (severity set under section 6) |
| 3 to 5 | next sample doubled |
| ≤ 2 | pass |

The same count rule is the audit trigger for the D65 monthly drift report
(D65). The pool size is counted in items, not as a percentage (D63, D3).

## 3. Citation-support audit (D51)

What: check that a cited source supports the claim, by re-reading the cited
locator (page, table, figure, line).

- Sample: at least 30 citations per month; if fewer than 30 were made, all
  of them (D51).
- Each citation gets exactly one verdict: **supported**, **not supported**,
  or **unverifiable** (D51).
- Unverifiable is never a pass (D51).
- More than 20% unverifiable blocks the dependent findings from the score
  (D51).

## 4. Accuracy against known truth (D71)

What: compare our values with values whose truth is known in advance.

- Truth sets: M7 values, D66 planted errors, serious-adverse-event rows
  cross-checked with CT.gov structured results (D71).
- Reported per field type (D71).
- Every mismatch is adjudicated against the locator and given a cause. Only
  "our error" counts in the accuracy rate; source discrepancies are
  published as a separate rate (D71).
- Public label: "same-model agreement" (D71).

### Planted errors (D66)

- At least 20 planted errors a quarter, scored with a rolling count rule
  whose cut-offs A10 and A50 fix first (D66).
- Until those cut-offs exist: detection below 80% = "unreliable"
  (provisional, D66).
- Error types are public; which items are planted stays hidden. The key is
  kept in a president-approved environment, otherwise labelled "planted (key
  not isolated)". A producer never writes a plant (D66).

## 5. G10 judgement sample (D102, D84)

- 5 sign-offs a quarter, drawn across Literature and Research, go to the
  outside reviewer (D102).
- Novelty calls (D84) are part of that pool (D84).
- Until an outside reviewer exists, every monthly report shows "G10
  uncovered" as a red line (D102).

## 6. Severity of findings (D54)

| Severity | Flag | Fix |
|---|---|---|
| S1 | within 24 h, by a scheduled job | by the producer within 72 h; a missed fix goes to the president |
| S2 | — | within 2 weeks |
| S3 | — | next release |

- Verification sets severity; the chair cannot downgrade it (D54).
- Corrections are add-only, recorded in `docs/incidents/` (D54).
- A negative-control result with P(net > 0) ≥ 0.8 opens an S2 (D113).
- S1 incidents go to the president the same day (D47).

## 7. Reporting (D47)

- A generated monthly report goes to the president through the direct
  channel (D47).
- It includes the D3 signals, the D63 and D66 counts, the D51 unverifiable
  share, the D71 rates (ours and source), and the G10 status (D3, D51, D63,
  D66, D71, D102).

## Sampling method

The decisions fix sample sizes but not how items are drawn. Proposed method
(P1 below): list the eligible items, sort them by a stable id, draw with a
seeded pseudo-random generator, and publish the seed only after the audit is
done, so producers cannot predict the sample.

## Proposed, needs a decision

These rules are not in the decision log. They do not bind anyone until a
numbered decision adopts them.

- **P1. Seeded sampling.** Eligible items listed and sorted by stable id;
  drawn with a seeded generator; seed and draw script committed to
  `verification/` after the audit closes; anyone can re-run the draw.
- **P2. Who holds the seed before publication.** Same environment as the
  D66 plant key, or a different one.
- **P3. "Error" in an extraction audit.** What counts as one error (any
  field mismatch vs. a mismatch that changes the effect estimate), and
  whether rounding differences count.
- **P4. Unit of the D51 20% threshold.** Per month, per finding, or per
  rolling window, and when a block is lifted.
- **P5. "Unit" in D3/D63.** Whether a unit is a manager's team, a mission,
  or a single worker.
- **P6. Pool when fewer than 60 items.** How the D63 thresholds (6, 3–5, 2)
  scale when a unit's 3-month pool is smaller than 60.
- **P7. Auditor independence.** An auditor never audits work they produced
  or co-signed (D55 covers only red team).
- **P8. Audit records.** One file per audit in `verification/audits/`
  (sample, seed, verdicts, locators) as the evidence base for the monthly
  report.
