# Reference ranges v1: Verification check

Checked by A65 (Verification) on 2026-10-20. Object: PR #88, head `fee554f`,
`literature/lifespan/reference-ranges.yaml` v1 + `check.rb`. Rule:
`l1-schema.md`, "Reference ranges v1: printed medians" (PR #83, 79ae26a).

**Verdict: conforms to the 3 v1 amendments. One point (C2013) goes to A50,
because it changes the ruling's text.** No row may be read as
`control_in_ref_range: true` in any analysis until A50 rules on C2013 and
this file records it.

## Independent check

| Item | Result |
|---|---|
| `reference-ranges.yaml` sha256 | `520ac78359444d1bd25d17976aa072b67623463e12d928e27700fe128cbafcfc` (= PR body) |
| `check.rb` @ `fee554f` | `OK: 80 row(s)` |
| My own recomputation of the flag (separate script, leave-one-cohort-out, inclusive bounds) | 44 true / 36 false, 0 mismatches with stored values |
| Rows with no printed pooled control median | 24, all `false` |
| Rows outside the other cohorts' range | 12: C2007 F (866), C2010 F (900), C2013 M (830) |
| Every control median of a row is a value in the file, same cohort | yes |

## The 3 amendments

| # | Met? | Note |
|---|---|---|
| 1 Leave-one-cohort-out | yes | All values of the row's own cohort dropped, including the second C2012 value. |
| 2 UM-HET3 only | yes | Only UM-HET3 ranges; every other strain listed `pending`. |
| 3 Sealed + flip report | yes | sha256 above goes into the D115 seal record; `v2_plan` and flip report stated. |
| Source per value | yes | Every value cites doi + table (Strong 2008: text). Label "printed, not per-mouse". |

## A71's four questions

1. **C2013 included: referred to A50.** The ruling says C2004–C2012; the
   file adds C2013. It is not neutral: without C2013, the 3 C2012 male rows
   (823 / 816 d) fall outside the range (max 807), and the 8 C2013 rows
   would have no range at all. That is 11 flags changed. Verification view:
   include it, because the rule is "every printed pooled ITP control median"
   and leaving out a cohort that meets it would be a choice by cohort.
   A50 decides.
2. **C2012, two printed values: accepted.** Both kept, neither chosen, both
   left out for C2012 rows. Matches amendment 1.
3. **C2009, one value cited by two papers: accepted.** Same controls, same
   printed value: one entry, two sources.
4. **Margin for extreme cohorts: refused.** The lowest and highest cohort
   always fall outside under leave-one-cohort-out; that is the rule as
   written. A margin chosen now, after seeing which rows fail, would be
   fitted to the data. No margin in v1. The seal record states that the
   extreme cohorts are `false` by construction; v2 (MPD) may revisit the
   statistic, decided before v2's values are read.

## A50 ruling on C2013 (2026-10-20, 16:47)

**Include C2013.** "C2004–C2012" described what was in L1 at the time; it
was not a cut-off. The rule: v1 = every printed pooled control median of
every ITP cohort in L1 at freeze time. Dropping a cohort that meets it, after
seeing which flags change, would be a choice by result. Leave-one-cohort-out
applies to C2013 rows too. Freeze = the sha256 in the D115 seal record; any
cohort added later means a new version and a flip report.

With this ruling, PR #88 @`fee554f` conforms in full. The hold above is
lifted: the 44 `true` flags stand as computed by `check.rb`.

## A50 ruling: no margin in v1 (2026-10-20, 16:48)

The refusal of a margin (question 4) is endorsed. The 12 extreme-cohort rows
(C2007 F, C2010 F, C2013 M) stay `false` and are listed by id in the D115
seal record. The test-2 scorecard reports a sensitivity line with and
without a margin; the main analysis is without. A margin may come only in a
later version, by a rule stated before that version's data is read.
