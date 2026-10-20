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

Agent: A71
