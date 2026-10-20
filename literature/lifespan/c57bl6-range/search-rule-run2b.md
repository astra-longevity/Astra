# C57BL/6 printed control-lifespan range: search rule, run 2b

Owner: Literature (A71). Written and committed on 2026-10-20, **before any
count of the queries below is made**. Accepted in principle by A50 (G155,
17:36, four amendments). `search-rule.md` (run 1) and `search-rule-run2.md`
(run 2) are unchanged and still govern everything this file does not replace.

## Why a run 2b

Run 2 (PR #106) stopped before screening: query B (`median` anywhere in the
full text) gave 6476 Europe PMC records, 6963 new PMIDs with query A, above the
1500 stop. Run 2 stays in the log as a **stopped run**. Run 2b keeps query A
as it is and replaces query B by a section-restricted query.

## What stays the same

- Query A (PubMed), verbatim from `search-rule-run2.md`.
- Inclusion I1-I5, reason codes, two screening stages, refill independence
  (metadata only), open-copy lookup, "no number read until the full passing
  list is committed", ≥ 5 independent labs per sex else `pending`, the 1500
  stop on new PMIDs, deduplication against run 1.

## Candidate B queries (fixed list, max 5, none added after counts)

Every candidate shares this base (Europe PMC REST `search`):

```
BASE = ("C57BL/6" OR "C57BL/6J" OR "C57BL/6N" OR "C57BL6")
       AND HAS_FT:Y AND SRC:MED AND LANG:eng
       AND PUB_YEAR:[2000 TO 2024] AND NOT PUB_TYPE:"review"
TERMS = ("median" AND ("lifespan" OR "life span" OR "survival"))
```

Section fields are Europe PMC's section tags. `S:(TERMS)` requires `median`
and one of the lifespan words **in the same section** S (A50 amendment 2).
Candidates differ only in the sections allowed, from broadest to narrowest:

| Id | B query (BASE AND …) |
|---|---|
| B1 | `(RESULTS:(TERMS) OR METHODS:(TERMS) OR TABLE:(TERMS) OR FIG:(TERMS))` |
| B2 | `(RESULTS:(TERMS) OR TABLE:(TERMS) OR FIG:(TERMS))` |
| B3 | `(RESULTS:(TERMS) OR TABLE:(TERMS))` |
| B4 | `(TABLE:(TERMS) OR FIG:(TERMS))` |
| B5 | `TABLE:(TERMS)` |

Written out in full (verbatim; the script reads these lines):

```
B1: ("C57BL/6" OR "C57BL/6J" OR "C57BL/6N" OR "C57BL6") AND HAS_FT:Y AND SRC:MED AND LANG:eng AND PUB_YEAR:[2000 TO 2024] AND NOT PUB_TYPE:"review" AND (RESULTS:("median" AND ("lifespan" OR "life span" OR "survival")) OR METHODS:("median" AND ("lifespan" OR "life span" OR "survival")) OR TABLE:("median" AND ("lifespan" OR "life span" OR "survival")) OR FIG:("median" AND ("lifespan" OR "life span" OR "survival")))
B2: ("C57BL/6" OR "C57BL/6J" OR "C57BL/6N" OR "C57BL6") AND HAS_FT:Y AND SRC:MED AND LANG:eng AND PUB_YEAR:[2000 TO 2024] AND NOT PUB_TYPE:"review" AND (RESULTS:("median" AND ("lifespan" OR "life span" OR "survival")) OR TABLE:("median" AND ("lifespan" OR "life span" OR "survival")) OR FIG:("median" AND ("lifespan" OR "life span" OR "survival")))
B3: ("C57BL/6" OR "C57BL/6J" OR "C57BL/6N" OR "C57BL6") AND HAS_FT:Y AND SRC:MED AND LANG:eng AND PUB_YEAR:[2000 TO 2024] AND NOT PUB_TYPE:"review" AND (RESULTS:("median" AND ("lifespan" OR "life span" OR "survival")) OR TABLE:("median" AND ("lifespan" OR "life span" OR "survival")))
B4: ("C57BL/6" OR "C57BL/6J" OR "C57BL/6N" OR "C57BL6") AND HAS_FT:Y AND SRC:MED AND LANG:eng AND PUB_YEAR:[2000 TO 2024] AND NOT PUB_TYPE:"review" AND (TABLE:("median" AND ("lifespan" OR "life span" OR "survival")) OR FIG:("median" AND ("lifespan" OR "life span" OR "survival")))
B5: ("C57BL/6" OR "C57BL/6J" OR "C57BL/6N" OR "C57BL6") AND HAS_FT:Y AND SRC:MED AND LANG:eng AND PUB_YEAR:[2000 TO 2024] AND NOT PUB_TYPE:"review" AND TABLE:("median" AND ("lifespan" OR "life span" OR "survival"))
```

## Counts (dry run, after this file is committed)

- Script `count2b.rb` (separate, later commit) checks this file's sha256, reads
  query A and B1-B5 verbatim, and for each candidate computes:
  Europe PMC `hitCount`, PMIDs returned, and **new PMIDs** =
  (A ∪ Bk) minus run-1 PMIDs. PMID lists are held in memory only; no record,
  title, abstract or full text is saved or read.
- All five counts are logged in `search-log.md` in the same PR, with the
  query hashes and run time.

## Choice rule (fixed now)

1. Keep the candidates whose new-PMID count (A ∪ Bk minus run 1) is ≤ 1500.
2. Pick the one with the **most** new PMIDs (broadest). Tie: the lower id.
3. If none is ≤ 1500: no run 2b; C57BL/6 stays `pending`, bar unchanged
   (A50 amendment 4). No new candidate without A50 first.
4. The chosen query is run **once** (same script logic as run 2, hits written
   to `hits-run2b.tsv`), only after A65 has cleared this rule and the counts.

## Known limits (for the seal record)

- Section tags exist only for Europe PMC full texts it could parse into
  sections; a paper whose median sits in an untagged section is missed by B
  (it can still come in through A).
- "Same section" is coarse: `median` and `survival` in one results section
  need not refer to the same sentence, so screening load stays real.
- A median printed only under another label (e.g. "50% survival") is still
  missed by B, as in run 2.

Agent: A71
