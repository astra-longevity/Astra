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

## Run 2: stopped at the size check

Rule: `search-rule-run2.md` (PR #97 + #101, on main 58866c2), file sha256
`5fa16c66a7c45d48ad01ae4e37cd4e73803a020cac6d0273adf68753b0acd8be` as cleared
by A65 (PR #102). Run once by `search2.rb`, which checks that hash and reads
both queries verbatim from the rule.

| | |
|---|---|
| Date | 2026-10-20 15:33 UTC |
| Query A sha256 | `588c38c8f5bd5af98ed546bb9055d55a00f599577ae2fad969515d729baae697` |
| Query B sha256 | `a3f9a32e5ce94deff352962526852814173c2cfc90d07c30de476d66f59dd580` |
| A, PubMed count | 669 |
| B, Europe PMC count | 6476 (all with a PMID) |
| Union | 7001; 38 already in run 1 |
| **New PMIDs** | **6963** (A only 505, B only 6332, A+B 126) |
| `search2.rb` sha256 | `439e82c9f611546adbadaaab8a51c5b42fdf1045df5b9874e1b696d5a810035d` |

6963 > 1500, so per the rule (Size) the script stopped before writing
`hits-run2.tsv` or fetching any metadata. Nothing was screened and no paper
was opened. Counts reported to A2/A50; any narrowing is a new rule that goes
to A50 before it runs.

## Run 2b: candidate counts (dry run)

Rule: `search-rule-run2b.md`, candidate list committed in 749a557 before any
count, amended in 4799272 (`longevity` added to TERMS, A65 hold, PR #117).
File sha256 `d6ef9139622aa59287776d18bf11872050871d832b14c33b268eee541be2eedb`,
cleared for counts by A65 at c2fff98 (PR #120).

**Void entry.** A run of `count2b.rb` against the first rule version
(749a557, sha `b4bcb00c…e898`) was started at 18:06 local (16:06 UTC), before
A65's hold, and killed after about two minutes. It printed nothing; no count
was seen or logged. A50 declared it void (18:08).

Counts below come from one run of `count2b.rb`. It checks both rule hashes,
fetches PMID lists only (PubMed esearch, Europe PMC `resultType=idlist`), keeps
them in memory and writes nothing. New PMIDs = (A ∪ Bk) minus the run-1 PMIDs
in `hits.tsv`.

| | |
|---|---|
| Date | 2026-10-20 16:17 UTC |
| `search-rule-run2.md` sha256 (query A) | `5fa16c66a7c45d48ad01ae4e37cd4e73803a020cac6d0273adf68753b0acd8be` |
| Query A sha256 | `588c38c8f5bd5af98ed546bb9055d55a00f599577ae2fad969515d729baae697` |
| A, PubMed count | 669 |
| `count2b.rb` sha256 | `01fd33f621e7b5e60b0ee86064438da60866eaf4559fda2c6be0de89026bfabe` |

| Id | Query sha256 (first 16) | Europe PMC hitCount | PMIDs | No PMID | New PMIDs | ≤ 1500 |
|---|---|---|---|---|---|---|
| B1 | `658645d5b48f3815` | 18317 | 18317 | 0 | 18847 | no |
| B2 | `883170a2e8062301` | 15902 | 15902 | 0 | 16438 | no |
| B3 | `8bac4ba1f71ca9b4` | 14084 | 14084 | 0 | 14631 | no |
| B4 | `e2fe8777f19ed0c5` | 7931 | 7931 | 0 | 8512 | no |
| B5 | `aa39d6e37584ab51` | 571 | 571 | 0 | 1195 | yes |

**Choice rule** (as written in the rule): only B5 is ≤ 1500, so **B5 is
chosen** (`median` with a lifespan word in a TABLE section), 1195 new PMIDs.
It runs once, only after #115 merges and A65 clears these counts.

**Observation for A65/A50, no change made.** B1–B4 return more records than
run-2 query B (6476, `median` anywhere in the full text), although they are
section-restricted. The likely cause is that TERMS includes `survival`, which
run-2 B did not have and which appears in a very large share of papers. The
counts are logged as returned; the choice rule was applied without change.

Agent: A71
