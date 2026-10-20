# C57BL/6 printed control-lifespan range: search rule

Owner: Literature (A71). Written and committed on 2026-10-20 **before the search
was run and before any lifespan number was read** (A50 condition 1, PR #83).
Nothing in this file changes after the search runs. Any departure is listed in
`search-log.md` with its reason and reported to A50/A65.

## Purpose

Build `reference-ranges.yaml` entries for strain `C57BL/6` (per sex) from
control median lifespans printed in papers that are strictly outside the
R-Metric v0 refill list. Same use as the UM-HET3 v1 range: flag
`control_in_ref_range`, leave-one-source-out, label "printed, not per-mouse".

## Search (fixed)

- **Database:** PubMed only, via NCBI E-utilities `esearch` (db=pubmed),
  run by `search.rb` in this folder. No other database, no hand-added papers.
- **Query (verbatim):**

  ```
  ("C57BL/6"[tiab] OR "C57BL/6J"[tiab] OR "C57BL/6N"[tiab] OR "C57BL6"[tiab] OR "B6 mice"[tiab])
  AND ("lifespan"[tiab] OR "life span"[tiab] OR "longevity"[tiab])
  AND ("median"[tiab] OR "survival curve"[tiab] OR "Kaplan-Meier"[tiab])
  AND "Mice"[MeSH Terms]
  AND ("2000/01/01"[dp] : "2024/12/31"[dp])
  AND english[la]
  NOT ("review"[pt] OR "systematic review"[pt] OR "meta-analysis"[pt])
  ```
- **Output:** every PMID returned, stored in `hits.tsv` with PubMed metadata only
  (PMID, year, title, last author, affiliation of last author, journal). The
  query count and run date go in `search-log.md`.
- Every hit is screened. No hit is dropped except by a reason code below.

## Screening (two stages, every decision in `screening.csv`)

Stage 1, title + abstract; stage 2, full text. One line per PMID:
`pmid, stage, decision, reason_code, note`. Full text is read only for hits
that pass stage 1.

### Inclusion (all required)

- **I1 Strain.** A control group of C57BL/6 mice (any sub-strain: J, JNia, N,
  JRj, NCrl, NHsd, unstated...). Wild-type littermates of a mutant line count
  only if the paper states the line is congenic on C57BL/6 (backcrossed ≥ N6 or
  "congenic"). Not B6 hybrids (B6:FVB F1, B6C3F1, B6D2F1...): those are
  recorded as `X_HYBRID` and stay `pending`.
- **I2 Untreated.** Control = no compound (vehicle allowed), no genetic
  modification, standard chow ad libitum. Not high-fat diet, calorie
  restriction, irradiation, infection, surgery or any other insult.
- **I3 Printed per-sex median and n.** Median lifespan for at least one sex,
  printed as a number (text or table, not read from a figure), with the n of
  that group printed. Days, weeks or months (converted: week = 7 d,
  month = 30.44 d; the printed unit is kept).
- **I4 Full natural lifespan.** The median is reached in the data (study not
  stopped before half the group died) and the mice entered the study at
  ≤ 12 months of age.
- **I5 Primary data.** The paper reports its own cohort.

### Exclusion reason codes

| Code | Reason |
|---|---|
| X_NOT_MOUSE_LIFESPAN | no lifespan study in mice (stage 1) |
| X_STRAIN | no C57BL/6 control group (I1) |
| X_HYBRID | B6 hybrid only; logged, `pending` |
| X_TREATED | control not untreated / not standard ad lib chow (I2) |
| X_NO_PRINTED_MEDIAN | median only in a figure, or n not printed (I3) |
| X_TRUNCATED | median not reached, or entry age > 12 months (I4) |
| X_NOT_PRIMARY | data from another paper (I5); the source paper is screened if it is a hit |
| X_REFILL | PMID is one of the 23 refill PMIDs |
| X_REFILL_LAB | same lab as a refill paper (see below) |
| X_REFILL_COHORT | same mice / control group as a refill paper |
| X_DUPLICATE | same control group already counted from another hit (earliest paper kept) |
| X_NO_FULLTEXT | full text not reachable (logged; reported, not silently dropped) |

## Independence from the refill (A50 condition 2)

- Checked on refill **metadata only**: the 23 PMIDs of
  `literature/lifespan/drugage-refill/refill-list.csv` and their PubMed author
  lists and affiliations (esummary/efetch). No refill outcome or control column
  is ever read.
- **Lab** = last author. A hit whose last author is the last author of any
  refill paper is `X_REFILL_LAB`; the same applies when the hit's last author
  is a co-author of a refill paper *and* the hit lists the same institution.
- **Cohort** = the same group of mice, identified from the hit's methods
  (cites a refill paper as the source of its animals or survival data).

## Labs and values (A50 condition 3)

- One value per control group per sex. A lab (last author) may give several
  values; labs are counted, not values.
- Required: **≥ 5 independent labs per sex**. If fewer, A71 reports to A2
  before any range PR; the sex stays `pending`.
- Recorded per value: PMID, DOI, lab, sub-strain (as printed), sex, n,
  median (printed unit and days), diet, housing site, entry age, source
  (table/text).
- The range is `[min, max]` of the values of the *other* labs
  (leave-one-lab-out), bounds inclusive; its width is stated in the file.

## Freeze (A50 conditions 4-5)

- The C57BL/6 entries enter `reference-ranges.yaml` in one PR, with the file's
  sha256 in the PR body, before any refill outcome or control column is read
  by anyone. That sha256 goes in the D115 seal.
- B6:FVB F1 and B6C3F1 stay `pending`.
- When per-mouse data replace printed values (v2), the list of
  `test2_main` flips is published.

Agent: A71
