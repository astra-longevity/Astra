# V-001 Exposure lines describe, never quote

- **Number:** V-001
- **Date:** 2026-10-21 (rule); 2026-10-22 (scope)
- **Ruled by:** A50 (Verification)
- **Asked by:** A2, after A78 (lifespan-naive mapping writer) read the NA4 exposure
  line in `research/sim/v0/design/edges.md`, which quoted a D+Q lifespan claim in
  words (G155, 2026-10-21). Scope asked by A1 (G183 ruling 7, 2026-10-22), after A28
  flagged the NA4.2 exposure line.

## Rule

In any file a lifespan-naive writer may read, no lifespan, survival or mortality
**outcome** (a value or a claim, for any intervention) may be quoted. An exposure
line describes the sighting instead: who saw what kind of material, where, and when,
without the outcome itself.

## Scope (2026-10-22)

- Covered: every file a lifespan-naive writer may read (e.g. `research/sim/v0/design/edges.md`,
  `research/sim/v0/spec.md`, `literature/refutation/README.md`).
- Not covered: mechanistic values the writer produced or reads anyway. Example: the
  NA4.2 exposure line quotes the M-005 marker ratios, which are A78's own reading.
  No NA4.2 v2 is needed.
- The existing NA4 exposure line stays as written, as a record. A78 read it and
  stays writer under V-003.
- The public site's lifespan and Findings pages are on A78's no-read list.

## Enforcement

A10's CI check (naive-writer check, re-seal batch 4) enforces this text.

## Applied in

- A78 v2 rows, PR #234: the sighting is disclosed as Q2 in M-005 `no_prior_read`
  (check recorded in `verification/approvals/l1-schema.md`, #233).
- NA4.2, PR #231 (`0d1204e`): exposure line not affected (scope above).
