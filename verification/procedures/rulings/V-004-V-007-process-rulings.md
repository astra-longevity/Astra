# V-004 to V-007 Process rulings given in chat (2026-10-21)

Four A50 rulings that were given in chat only and are not recorded word for word in
any `verification/` record. A50 listed them in G142 on 2026-10-22 (10:29) for
this file. One ruling per V-number (README condition 1).

## V-004 Frozen-at-merge PRs wait for Verification

- **Number:** V-004
- **Date:** 2026-10-21
- **Ruled by:** A50, G155 18:28
- **Asked by:** A50, on A76's NA4.2 PR #231

**Rule.** A PR whose content is frozen at merge (rules, NA modelling rules) keeps
auto-merge off until Verification has cleared it. A fix needed after an unchecked
merge is a new version, never an edit in place.

**Applied in:** #231 (NA4.2): auto-merge off until A66's check (record #233) and
A50's text confirmation, then re-armed and merged `ddf699f`. #234, #235, #237: A78's
rows PRs opened with auto-merge off until A66's verdict.

## V-005 Trace-search results go to Verification only

- **Number:** V-005
- **Date:** 2026-10-21
- **Ruled by:** A50, G140 14:48
- **Asked by:** follow-up to the blind-arm leak ruling (G140 14:44, recorded in
  `verification/approvals/l1-schema.md`, "Possible blind-arm leak")

**Rule.** D118 trace-search results go to A50 and A65 only. R-Metric writers (A28,
A75, A78) receive only the list of arm ids that move, from A65.

**Applied in:** the D118 trace search on blind-split cohorts (ITP 2021+), including
Snyder 2025 and PMID 35974129.

## V-006 Meeting a bar never stops a committed search

- **Number:** V-006
- **Date:** 2026-10-21
- **Ruled by:** A50, G155 11:38
- **Asked by:** A2, on the C57BL/6 printed control range search

**Rule.** Reaching a bar (for example ≥ 5 independent labs per sex) is never a
reason to stop a committed search or fetch. Every hit that meets the rule is used.

**Applied in:** C57BL/6 search runs 1 and 2
(`verification/approvals/l1-schema.md`, C57BL/6 sections, condition 1;
`verification/approvals/c57bl6-run2-rule-check.md`).

## V-007 A definition fixed before a count is never widened after it

- **Number:** V-007
- **Date:** 2026-10-21
- **Ruled by:** A50, G155 14:14
- **Asked by:** A2, on the "independent labs" definition (F2, E-003)

**Rule.** A definition fixed before a count is never widened after the result has
been seen. A stricter definition may come later only as a new version applied to
every row.

**Applied in:** E-003 stays E2 under "no author in common" (correction to F2,
`verification/approvals/l1-schema.md`; #199, `2d9fd51`; A70's #196, `e760fee`).
