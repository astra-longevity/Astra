# Trace search: ITP blind-split cohorts (D118)

Status: search rule v1, 2026-10-21 (writer A70, for A2; ruling A50). Committed **before any query is run**. Results go to A50 and A65 only (via A2, G155), never to A28, A75, A76 or A78.

## What is searched

Public traces of a survival, lifespan or mortality result for:

1. Snyder 2025, *GeroScience*, doi:10.1007/s11357-025-01741-3 (ITP end-of-life pathology, late-start canagliflozin).
2. PMID 35974129 (flagged by A65 with Snyder 2025 as a possible blind-cohort paper; not opened).
3. Any paper on an arm of the blind part of the R-Metric v0 split: the 19 in-progress ITP arms B01-B19 (cohorts 2021-2025), listed in `research/metric/v0/split-proposal.md` ("Blind part"). Compound names are taken from that table.

## Sources and queries

Run by `search.rb` (one run, date written to `search-log.md`):

| Source | How | Covers |
| --- | --- | --- |
| PubMed | E-utilities `esearch`, publication date 2021-01-01 to run date | journal papers |
| Europe PMC | REST `search`, same dates, all sources | journal papers, bioRxiv/medRxiv preprints (`SRC:PPR`), PMC-indexed meeting abstract supplements |
| NIA ITP "Supported Interventions" page | status of each blind compound, read by hand, date written | result posted by NIA |

Queries (exact strings in `search.rb`):

- Q0: Snyder 2025 by DOI; PMID 35974129 by PMID. Both are always located (step below), whatever the screening says.
- Q1, one per blind compound: compound name or synonym AND (`"Interventions Testing Program"` OR `UM-HET3` OR `"genetically heterogeneous mice"`).
- Q2, catch-all: `"Interventions Testing Program"` from 2022-01-01.

Supplements are covered by the location step (below), which reads the supplementary-material list of each included paper.

## Screening (masked)

The script prints title, journal, date, PMID/DOI, the matched compound and any cohort-year tokens (`C2021`, `2021 cohort`, `started in 2021`, …). Every abstract sentence that contains an outcome word (survival, lifespan, life span, mortality, death, longevity, Kaplan, hazard, median) is **not printed**; only a flag says that one exists. No number next to an outcome word is ever printed.

A paper is **included** if it reports animals from an ITP cohort started 2021 or later (cohort tokens, or a blind compound tested in the ITP with no cohort year before 2021). If the cohort can't be told from the masked output, it is included as `unclear` for A65.

## Location step

For each included paper, `search.rb locate PMID ...` reads the Europe PMC full-text XML when open and prints only the **ids and labels** of sections, figures, tables and supplementary files whose title or caption contains an outcome word (e.g. `Fig. 3`, `Table S2`, `Results > Survival`). No caption text, no numbers. When no open full text exists, the location is `abstract flag only, full text not open`.

## Output

`results.tsv`, one row per included paper: `pmid`, `doi`, `arms`, `cohort_basis`, `outcome_present` (yes / no / abstract-only), `location`. No number copied, no effect described.

## Rules

- This file and `search.rb` are committed before the run. A change after the run is a new version with a new dated run.
- The writer reads no result: only the masked script output.
- A paper found with a result moves its arm out of the blind part (split-proposal, "Timing risk"): that decision is A65's, not this file's.
