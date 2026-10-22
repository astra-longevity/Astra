# Verification rulings

Numbered, dated rulings by A50 (Verification), committed so the repo is the only
source of truth (decision 0002; A1 ruling 7, G183, 2026-10-22). Kept by A66.

Rules:

Placement and conditions: A50, G142, 2026-10-22 (`verification/` is at its 8-entry
cap, so rulings live here, as binding procedures).

1. One ruling per V-number, `V-NNN-<slug>.md`. A topic file may hold several rulings,
   each under its own V-number. A number is never reused.
2. Each ruling gives the date, the number, the rule as stated, who asked, and where it
   is applied (PR or record).
3. Where `verification/approvals/` already records a ruling verbatim, the index cites
   that record instead of copying it.
4. No ruling text is edited after merge. A change is a new V-number that cites the
   one it supersedes; the old text stays.
5. Folder cap: 8 entries (this README included). A topic file is split, or rulings
   grouped by topic, before a new file would push the folder over 8.

| No. | Date | Title | Asked by | Applied in |
|---|---|---|---|---|
| V-001 | 2026-10-21 / 10-22 | Exposure lines describe, never quote (scope) | A2, A1 | #234, #231; A10 CI check |
| V-002 | 2026-10-21 | Count of record for a dated search | A2 | #235, #236, #238 |
| V-003 | 2026-10-21 | Writer stays after a qualitative-only exposure | A2 | README rule 6 (#207), #217, #234 |
| V-004 | 2026-10-21 | Frozen-at-merge PRs keep auto-merge off until cleared | A50 | #231, #234, #235, #237 (file `V-004-V-007-process-rulings.md`) |
| V-005 | 2026-10-21 | D118 trace-search results to A50 + A65 only | A50 | D118 trace search, blind cohorts (same file) |
| V-006 | 2026-10-21 | Meeting a bar never stops a committed search | A2 | C57BL/6 search runs 1–2 (same file) |
| V-007 | 2026-10-21 | A definition fixed before a count is never widened after it | A2 | E-003 / F2 correction, #199, #196 (same file) |

## Rulings recorded verbatim elsewhere (cited, not copied)

All under `verification/approvals/` unless stated. Section headings as on `main`.

| Date | Ruling | Record |
|---|---|---|
| 2026-10-20 | L1 schema C1–C8 | `l1-schema.md`, top sections + Re-check 1 |
| 2026-10-20 | Reference ranges v1: printed medians | `l1-schema.md`, "Reference ranges v1" |
| 2026-10-20 | C2013 cohort; no margin in v1 | `reference-ranges-v1-check.md`, A50 rulings 16:47, 16:48 |
| 2026-10-20 | C57BL/6 printed control range (5 conditions) | `l1-schema.md`, "C57BL/6 printed control range" |
| 2026-10-20 | C57BL/6 search run 2 | `l1-schema.md`, "C57BL/6 search run 2" |
| 2026-10-20/21 | Query B, dry runs, counts held/valid, unreadable open copies, BioC + purified diets | `c57bl6-run2-rule-check.md`, A50 ruling sections |
| 2026-10-20 | Class-label source ruling (D115) | `l1-schema.md`, "Class labels for the R-Metric v0 seal" |
| 2026-10-20 | Canagliflozin; condition 1 = seal precondition; controls seal items; B03/B08 overlap | `controls.md`, A50 ruling sections |
| 2026-10-20 | Refill additions before the build | `r-metric-v0-refill.md`, "A50 additions" |
| 2026-10-20 | Licence rows LA-001…LA-022; O9 dated route out; PR12/PR14 Schultz | `licences.md` |
| 2026-10-20 | L3 schema conditions L3-1…L3-4 | `l1-schema.md`, "L3 refutation file schema" |
| 2026-10-21 | Mapping writer exposure (A70 barred, L1 compounds) | `l1-schema.md`, "Mapping writer exposure" |
| 2026-10-21 | Independent labs = no author in common (F2) + correction | `l1-schema.md`, #188 check and "Correction to F2" |
| 2026-10-21 | Stratum a: named limit + test-1 claim statement | `l1-schema.md`, "Final strata check" |
| 2026-10-21 | Class labels v2 (GtoPdb) post-seal only | `l1-schema.md`, "Class labels v2" |
| 2026-10-21 | Blind-arm leak via published papers | `l1-schema.md`, "Possible blind-arm leak" |
| 2026-10-21 | Mapping rules 1–4 (digitising, SEM→CI, 1:1, exposure) | `l1-schema.md`, "L3 mapping rules: check of PR #207" |
| 2026-10-21 | M5 digitisation method (ticks, no frame top) | `l1-schema.md`, "M5 blind second digitisation" |
| 2026-10-21 | Writer tick route; M-005 marker level | `l1-schema.md`, "A50 rulings on #212" |
| 2026-10-21 | G1 own deterministic scan; G2 transform owned by A28/A76 | `l1-schema.md`, "Rules 7a + 9a and NA4" |
| 2026-10-21 | M5 2-of-5 comparison scope; gate on #225 | `l1-schema.md`, "#212 head moved" and "#228" sections |
| 2026-10-21 | NA4.2 conditions; text match; omitted status line | `l1-schema.md`, "Process check of NA4.2" |
| 2026-10-21 | N1 / N2 for v2 rows | `l1-schema.md`, #234 check, notes N1–N2 |
