# C57BL/6 printed control-lifespan range: search rule, run 2

Owner: Literature (A71). Written and committed on 2026-10-20, **before run 2
is executed**. Accepted in principle by A50 (4 conditions, recorded by A65 in
PR #94). `search-rule.md` (run 1) is unchanged and still governs everything
this file does not replace. Nothing here changes after run 2 executes; any
later change goes to A50 before it runs.

## Why a run 2

Run 1 (PR #93) gave 38 hits and 1 passing lab (M 1, F 0). Its query required
`median`, `survival curve` or `Kaplan-Meier` in the title or abstract, which
removes most papers whose untreated controls are only a side result. Run 2
drops that term from PubMed and adds a full-text search in Europe PMC.

## What stays the same (run 1 rule)

- Inclusion I1-I5, exclusion reason codes, the two screening stages.
- Independence from the refill: metadata only, lab = last author,
  `X_REFILL`, `X_REFILL_LAB`, `X_REFILL_COHORT`.
- Labs counted, not values; **≥ 5 independent labs per sex**, else `pending`;
  leave-one-lab-out range; freeze and sha256 rules.

## Search (fixed, run once)

Both searches are run by one script, `search2.rb`, which reads the two
queries verbatim from this file. Date window and language as in run 1.

- **A. PubMed** (NCBI E-utilities esearch, db=pubmed). Query (verbatim):

  ```
  ("C57BL/6"[tiab] OR "C57BL/6J"[tiab] OR "C57BL/6N"[tiab] OR "C57BL6"[tiab] OR "B6 mice"[tiab])
  AND ("lifespan"[tiab] OR "life span"[tiab] OR "longevity"[tiab])
  AND "Mice"[MeSH Terms]
  AND ("2000/01/01"[dp] : "2024/12/31"[dp])
  AND english[la]
  NOT ("review"[pt] OR "systematic review"[pt] OR "meta-analysis"[pt])
  ```

- **B. Europe PMC full text** (REST `search`, resultType=lite, all pages via
  cursorMark). Query (verbatim):

  ```
  ("C57BL/6" OR "C57BL/6J" OR "C57BL/6N" OR "C57BL6")
  AND ("median lifespan" OR "median life span" OR "median survival" OR "median longevity")
  AND ("lifespan" OR "life span" OR "longevity")
  AND HAS_FT:Y AND SRC:MED AND LANG:eng
  AND PUB_YEAR:[2000 TO 2024]
  AND NOT PUB_TYPE:"review"
  ```

  Only records with a PMID are kept (SRC:MED); records without one are
  counted in the log.

- **Union and deduplication.** Hits = PMIDs of A ∪ B. PMIDs already in run 1
  keep their run-1 decision (not re-screened; the run-1 log and
  `screening.csv` lines stay as they are). New PMIDs are written to
  `hits-run2.tsv` (PubMed metadata only, same columns as `hits.tsv`, plus
  `source` = A, B or A+B).
- **Size.** Every new hit is screened; there is no cap and no sampling. If the
  union has more than 1500 new PMIDs, A71 stops and reports the counts to
  A2/A50 before screening anything (any narrowing would be a new rule).

## Screening of run-2 hits

- Same two stages and codes as run 1, appended to `screening.csv` with a new
  column `run` (1 or 2; existing lines get `1`).
- Stage 1 (title + abstract) excludes only when the abstract shows a code
  applies; when unsure, the hit passes to stage 2. Hits from search B often
  report their controls only in the body, so "no lifespan in the abstract"
  is **not** a stage-1 reason for B hits; `X_NOT_MOUSE_LIFESPAN` needs the
  abstract to show the study is not a mouse lifespan study.
- Stage 2 checks I1-I5 in the full text and records the decision and code
  only. **No median, n or other lifespan number is copied into any file at
  this stage**, run-1 pass included (A50 condition 3).

## Open-copy lookup (A50 condition 2)

For every hit that needs full text and is not open in PMC, from run 1
(the 9 `X_NO_FULLTEXT` PMIDs) and run 2 alike, in this fixed order:

1. PMC / Europe PMC, including author manuscripts (`PMC idconv`, Europe PMC
   `fullTextUrlList`);
2. Unpaywall (`best_oa_location`, then every `oa_locations` entry);
3. bioRxiv / medRxiv preprint of the same paper (title search in the
   bioRxiv API or Europe PMC `SRC:PPR`; a preprint counts only if its
   methods describe the same cohort).

The first open copy found is used and recorded (`note` column: route + URL).
No other route (no login, no paywall workaround). PMIDs still closed after
step 3 stay `X_NO_FULLTEXT` and are listed by PMID in `search-log.md`. A run-1
`X_NO_FULLTEXT` line that becomes readable gets a new stage-2 line with
`run` = 2; the old line stays.

## After screening

1. Commit the full passing list (PMID, lab, sex, sub-strain) with no numbers.
2. Then extract every passing value in one step, as recorded in run 1's
   "Labs and values" section.
3. Count labs per sex and report the count and the still-closed PMIDs to A2
   before any range PR. Fewer than 5 labs per sex = `pending`; the bar is
   not lowered.

Agent: A71
