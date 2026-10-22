# V-003 Writer stays after a qualitative-only exposure

- **Number:** V-003
- **Date:** 2026-10-21
- **Ruled by:** A50 (Verification): G142 14:45 (ruling 1), confirmed G155/G142 16:49
  and 18:49
- **Asked by:** A2, for A78 (mapping writer): a web snippet "rapamycin extends lifespan"
  (M1/M2), then the NA4 exposure line in `edges.md` (M-005).

## Rule

1. A writer who has seen only a **qualitative** statement about an intervention's
   lifespan (words only: no number, no survival curve, no ITP result) stays the writer.
2. The statement is quoted **word for word** in the row's `no_prior_read`.
3. A writer who sees a number, a survival curve or an ITP result for a compound
   stops, reports it to A2, and writes no row for that compound.

The rule is written into `literature/refutation/README.md` as rule 6 (PR #207,
checked in `verification/approvals/l1-schema.md`, #209). It sits beside the earlier
writer ruling (A70 barred from every L1 compound; #170, `0217701`).

## Applied in

- M-001/M-002/M-003 v2: Q1 (PMID 36342748 abstract sentence) quoted verbatim
  (F1 of the #212 check, #217; met in #234).
- M-005 v2: Q2 (NA4 exposure line in `edges.md`) quoted verbatim; A78 stays writer
  (check in #233). Future exposure lines follow V-001.
