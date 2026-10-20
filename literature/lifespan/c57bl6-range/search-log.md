# C57BL/6 control-range search: log

Rule: `search-rule.md`, committed in dee2410 (PR #90) before this run.
No departure from the rule so far.

## Run 1

| | |
|---|---|
| Date | 2026-10-20 (UTC, printed by `search.rb`) |
| Database | PubMed, NCBI E-utilities esearch + efetch |
| Query | read verbatim from `search-rule.md` by `search.rb` |
| Query sha256 (lines joined by spaces) | `d05a86d36c79a972c76c53d38511e739947d674843854733f7b387829987d482` |
| PubMed count | 38 |
| `hits.tsv` | 38 rows, sha256 `83fe5b1891525e664c250d9f2471239d99687865d222fac6910e6abac56d2ca6` |
| `refill-authors.tsv` | 23 refill PMIDs, sha256 `bdf12f40bb60615f95f4ae32f9d4296f49f4c9f3fd3dbb94b92fd87c40820591` |
| `search.rb` sha256 | `6450c043899284288e9170274f72b7d18cfa6b7ede6ede9be2f330f5218cdccb` |

Both TSV files hold PubMed metadata only (PMID, year, journal, authors,
affiliation, title). No lifespan number was read in this step, and no refill
outcome or control column was opened.

Next: screening of all 38 hits into `screening.csv` (stage 1 title + abstract,
stage 2 full text), reason codes from the rule only.

## Screening result (2026-10-20)

- Stage 1 (38 hits): 17 pass, 21 excluded (X_TREATED 9, X_NOT_MOUSE_LIFESPAN 5,
  X_STRAIN 2, X_REFILL_LAB 2, X_REFILL 1, X_HYBRID 1, X_TRUNCATED 1).
- Stage 2 (17 full texts sought via PMC, Unpaywall, publisher): **1 pass**
  (39451245, C57BL/6J males, lab Bollyky), 16 excluded: X_NO_FULLTEXT 9,
  X_NO_PRINTED_MEDIAN 3, X_STRAIN 2, X_TREATED 2.
- Labs passing: **males 1, females 0**, below the required 5 per sex. Per the
  rule, no range PR; reported to A2. Both sexes stay `pending`.
- The 9 X_NO_FULLTEXT papers are paywalled with no open copy; they are not
  dropped silently and can be re-screened if access is obtained.
- No lifespan number has been extracted into any file yet.

Agent: A71

## Stage 1 screening (title + abstract)

All 38 hits screened into `screening.csv` (stage 1), reason codes from the rule
only. Unclear hits pass to stage 2. Abstracts fetched from PubMed (efetch) to a
temporary file, not committed. No lifespan number is taken from an abstract.

| Decision | Hits |
|---|---|
| pass to stage 2 | 17 |
| X_TREATED | 9 |
| X_NOT_MOUSE_LIFESPAN | 5 |
| X_STRAIN | 2 |
| X_REFILL_LAB | 2 |
| X_REFILL | 1 |
| X_HYBRID | 1 (pending) |
| X_TRUNCATED | 1 |

Judgment point for A65: 30289438 (Allison DB) is `X_REFILL_LAB` because the
same person co-authors refill 29921922 at the same university (UAB), in a
different school. Institution was read as the university.
