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

## Addendum v1.1, 2026-10-22: compound published elsewhere (A50 ruling via A2)

Committed **before** the Q3 queries run. The v1 run (Q0-Q2) is unchanged.

**Arms:** the 15 compound-new blind arms (rows of `research/metric/v0/split.csv` with part `blind` and no "compound seen in training" flag): B01 dichloroacetate, B02 TPPU, B06 sodium thiosulfate, B07 lactoferrin, B09 trametinib, B10 mianserin, B11 cinnarizine, B12 L-deprenyl, B13 sildenafil, B14 alpelisib, B15 alogliptin, B16 BT2, B17 glucosamine, B18 taurine, B19 urolithin A.

**Question per arm:** is there any published mouse lifespan or survival result for that compound, alone or combined, any cohort or lab (not only ITP), published before the seal? Answer yes / no + location; no number, no effect direction.

**Q3, one per arm** (`ruby search.rb compound`, PubMed + Europe PMC incl. preprints, no lower date limit, up to the run date): compound synonyms AND (mice OR mouse OR murine) AND (lifespan OR "life span" OR longevity OR "life extension" OR ((aging OR ageing OR aged) AND (survival OR mortality))).

**Screening (masked as v1):** outcome sentences are never printed; a Q3 title containing an outcome word is printed as `[title masked]`. The script also tags each record with disease-model tokens found in its non-outcome text (tumour, xenograft, infection, sepsis, injury, ischemia, toxicity, transgenic disease model, progeroid, etc.).

**Classification per record:**
- `yes`: survival or lifespan of mice under normal ageing (any strain, start age, alone or combined).
- `disease-model only`: survival only in a disease, injury, toxicity, infection, tumour or progeroid model. Listed, not a yes.
- `unclear`: can't be told from the masked output; passed to A65 as such.
- `no` for the arm: no record classified `yes` or `unclear`.

**Output:** `results-compound.tsv`: `arm`, `compound`, `pmid`, `doi`, `basis` (normal ageing / disease model / unclear), `location` (from `locate`). A "yes" doesn't move the arm; A65 flags it `effect published elsewhere` in `split.csv` by script.
