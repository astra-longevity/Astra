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

## Run 2b: B5 search (run once)

Run once with `search2b.rb` (committed in 84a5168, before the run), after #115
merged (b734f00) and A50's ruling (PR #126). The script checks both rule
hashes, reads query A from `search-rule-run2.md` and B5 from
`search-rule-run2b.md` verbatim, fetches PMID lists, then PubMed metadata only
(no abstract, no full text) for new PMIDs, and writes `hits-run2b.tsv`.

| | |
|---|---|
| Date | 2026-10-20 18:02 UTC |
| `search-rule-run2.md` sha256 | `5fa16c66a7c45d48ad01ae4e37cd4e73803a020cac6d0273adf68753b0acd8be` |
| `search-rule-run2b.md` sha256 | `d6ef9139622aa59287776d18bf11872050871d832b14c33b268eee541be2eedb` |
| Query A sha256 | `588c38c8f5bd5af98ed546bb9055d55a00f599577ae2fad969515d729baae697` |
| Query B5 sha256 | `aa39d6e37584ab51a4cea1403d90761771f2d643b48fa5ee238b890b9a736a07` |
| A, PubMed count | 669 |
| B5, Europe PMC count | 571 (all with a PMID) |
| Union | 1233; 38 already in run 1 |
| **New PMIDs** | **1195** (A only 624, B5 only 564, both 7), ≤ 1500 |
| `hits-run2b.tsv` | 1195 rows, sha256 `1969324167314c4632939ab0e566ef233c36e40b37b1298304878e97683cc379` |
| `search2b.rb` sha256 | `bc6e1bb5eecf1f3a8123ed77d939242f2e7243d0804a85fd1e2134d0e434125e` |

Counts equal the dry run (A 669, B5 571, new 1195). Nesting B1 ≥ … ≥ B5 holds.
Nothing screened, no paper opened, no number read. Next: screening of the 1195
hits under I1–I5, refill/lab independence on metadata, open-copy lookup for
paywalled hits; no number read until the full passing list is committed.

Seal limits (A50, PR #126): medians found via TABLE sections only; volume of
B1–B4 driven by `survival`.

Agent: A71

## Run 2b, stage 1 (title + abstract)

| | |
|---|---|
| Date | 2026-10-20 |
| Hits screened | 1195 / 1195 (`hits-run2b.tsv`), one stage-1 line each in `screening.csv`, `run` = `2b` |
| Abstracts | `abstracts2b.rb` (PubMed efetch, local JSONL, not committed: publisher text); 24 records have no abstract |
| pass | 170 |
| X_NOT_MOUSE_LIFESPAN | 987 |
| X_STRAIN | 21 (UM-HET3 / ITP, recombinant inbred) |
| X_HYBRID | 3 (`pending`) |
| X_REFILL | 5 |
| X_REFILL_LAB | 9 |

- `screening.csv` gains the column `run` (run 1 lines = `1`), as `search-rule-run2.md` says.
- Stage-1 rule: exclude only when the abstract shows a code applies; when unsure the hit
  passes. B5 hits (median in a table) with a knockout or phenotype design pass unless the
  abstract shows a non-lifespan design (infection, tumour, acute insult, irradiation,
  in vitro). Conference supplements with no abstract pass unless the meeting is clinical
  only (human ICU, transplant, gastro, urology, rheumatology congresses: excluded).
- Refill independence on metadata only (`refill-authors.tsv` plus PubMed affiliations of
  the refill co-authors). Two judgment points flagged to A65: 21873593 (Ikeno Y, the hit's
  last-author affiliation is not printed) and 25711530 (Mitchell SJ, same). One co-author
  match kept as a pass: 33675469 (Hine C; refill affiliation Harvard, hit Cleveland Clinic).
- No full text opened, no lifespan number read. Next: open-copy lookup for the 170 passes,
  then stage 2 (decision and code only).

Agent: A71

## Open-copy lookup (run 1 X_NO_FULLTEXT + run 2b stage-1 passes)

| | |
|---|---|
| Date | 2026-10-20 |
| Script | `oa2b.rb` sha256 `ab12a85a1896be1dbc4bdce82e2db0255f8c64c59345c07091adfeb30c17e449` |
| Result | `oa-lookup-run2b.tsv` (179 rows) sha256 `1e356a31f3aa85abe755218e5e3bf8a0fe4e75fc9a64c171cff2ecc3f669527e` |
| Order | PMC / Europe PMC → Unpaywall → preprint (Europe PMC `SRC:PPR` title search), first open copy wins |

| Run | PMIDs | pmc | europepmc | unpaywall | preprint | closed |
|---|---|---|---|---|---|---|
| 1 (`X_NO_FULLTEXT`) | 9 | 1 | 2 | 1 | 0 | 5 |
| 2b (stage-1 pass) | 170 | 135 | 7 | 5 | 0 | 23 |

- Metadata only: the script records URLs; no full text was downloaded or read.
- The script was run once, then committed unchanged with its output.
- Run 1: 4 of the 9 paywalled papers now have an open copy (17416441, 18772469,
  24336818, 26774117). They get a new stage-2 line with `run` = 2; the old lines stay.
- Still closed after step 3 (stay or become `X_NO_FULLTEXT` at stage 2):
  - run 1 (5): 10870527, 12230882, 20305363, 20370440, 27455862
  - run 2b (23): 10978857, 10996667, 11166357, 11462771, 11726538, 16024047, 16510136,
    17092545, 17129217, 17516143, 17921417, 18854208, 20558214, 20818929, 23092318,
    25193556, 26271539, 34085306, 35914953, 35916733, 39261412, 39312071, 39471831
- No lifespan number read. Next: stage 2 on the 151 open papers (147 run 2b + 4 run 1).

Agent: A71

## Run 2b, stage 2 (full text) and passing list

| | |
|---|---|
| Date | 2026-10-21 |
| Set | the 151 open papers from the lookup (147 run 2b + 4 run 1), one decision each |
| Output | `screening.csv`, 151 lines with `stage` = 2 and `run` = `2b` (the 4 run-1 papers re-screened here also get `run` = `2b`; their old run-1 stage-2 lines stay) |
| Note field | `lab=<last author (institution)>; sex=<sexes with a usable control arm>; substrain=<…>; <reason>` |
| Method | full texts cached outside the repo, read through a viewer that masks every digit; the cache is deleted after extraction |

| Code | n |
|---|---|
| pass | 11 |
| X_NO_FULLTEXT | 55 |
| X_NO_PRINTED_MEDIAN | 27 |
| X_STRAIN | 22 |
| X_NOT_MOUSE_LIFESPAN | 10 |
| X_TREATED | 9 |
| X_TRUNCATED | 7 |
| X_NOT_PRIMARY | 6 |
| X_HYBRID | 4 |

### Passing list (no number read yet)

| PMID | Lab (last author) | Sex | Sub-strain |
|---|---|---|---|
| 39451245 (run 1) | Bollyky PL (Stanford) | M | J |
| 19627267 | Paigen B (JAX) | M, F | J |
| 20981551 | Ruf T (Vetmeduni Vienna) | F | not stated (Harlan NL) |
| 22132081 | Richardson A (UTHSCSA Barshop) | M, F | not stated |
| 22265987 | Swerdlow RH (U Kansas) | M | not stated (JAX) |
| 28761067 | de Angelis MH (Helmholtz Munich) | M | J |
| 29849149 | Levine B (UT Southwestern) | M, F | J |
| 34050173 | Cohen HY (Bar-Ilan) | M, F | JOlaHsd |
| 35511946 | Takahashi JS (UT Southwestern) | M | J |
| 38446314 | Kato Y (Kanazawa U) | M | J |
| 39374394 | Speakman JR (Aberdeen/Shenzhen) | F | J |
| 39662934 | Niida S (NCGG Obu) | M, F | NCrSlc and J |

Independent labs per sex (one institution counted once, so the two UT Southwestern
papers count as one lab): **males 9, females 7**. Both are ≥ 5.

### Judgment points (for A65)

1. 33675469 (Hine C): stage-1 pass reversed to X_NOT_MOUSE_LIFESPAN (weeks-long
   fasting study, no lifespan median).
2. WT littermates of a knockout count as C57BL/6 only if the paper states congenic
   or ≥ N6 backcrossing; otherwise X_STRAIN (e.g. Klf5, S6K1, Polr3b, p53 ≥ N5).
3. Control arms on purified diets (Kato TestDiet 5755, Speakman AIN-93G/M, Swerdlow
   AIN-93G) are counted as untreated ad-lib controls.
4. Speakman 37354128: the 12AL control is fed in the dark phase only → X_TREATED.
   Nagy 38850387: the only ad-lib arm is housed at thermoneutrality → X_TREATED.
5. Levine and Takahashi (both UT Southwestern) merged into one lab.
6. Richardson 22132081 shares an institution (Barshop) with refill author Ikeno Y but
   Richardson is not a refill author, so it is not X_REFILL_LAB.
7. Niida 39662934: the median is printed as "age at which survival fell below 50%".
8. Paigen 19627267 is an interim report; Ruf 20981551 prints one median for all
   three diet groups, control included.
9. Frailty trio (30562163, 30958526, 31355774): no printed median.
10. During stage 2 I first coded 34240807 and 34579167 X_REFILL_LAB because a refill
    author was a middle author. The rule matches on the hit's last author, so they
    were recoded by their own reasons: 34240807 X_TREATED, 34579167 X_TRUNCATED.
11. 55 X_NO_FULLTEXT: open per the lookup, but not readable by script. 40 PMC
    copies give front matter only (no XML body from E-utilities, captcha on the
    HTML page, 403 on the Europe PMC PDF); 12 publisher/repository URLs return 403;
    3 routes are not the paper. No block was circumvented. Together with the 28
    closed PMIDs above, they form the library-request list.

### Disclosure

For 25048265, a temporary entry-age extractor printed one trailing digit of a
masked value before it was fixed. The paper is excluded (X_TRUNCATED, entry at
15 months), so nothing it printed is used.

No lifespan number is in any file. Extraction starts only after this list is on main.

Agent: A71

## Manual download of the 54 unreadable open copies (A50 ruling)

| | |
|---|---|
| File | `manual-download.tsv` (54 rows: pmid, run_origin, doi, route, url, failure) sha256 `e4d7c49eeae4607521823e63fde78fd236b4b84d837c71c2029d9499a1206809` |
| Who | a human (president or library, via A1); no script, no captcha bypass |
| Deadline | **2026-10-26** |
| Fallback | any PMID not delivered by the deadline stays `X_NO_FULLTEXT` (tooling) and is listed in the seal as a known limit, next to the 28 closed PMIDs |

- Failures: 39 PMC copies where the publisher blocks machine download, 12 publisher/repository 403, 3 routes that are not the paper (a person should find the paper by DOI).
- 23795901 (PMC3772962) was dropped from the list (55 → 54): A65 found it readable via the NCBI BioC endpoint (`.../BioC_json/PMC3772962/unicode`), even though Europe PMC `fullTextXML` and the PMC page both failed. I screened it the same masked way: the only printed median/max lifespan is the Wijnhoven et al. longevity cohort (plus a confirmatory survival cohort), not this paper's own primary outcome (its own cohorts are cross-sectional, sacrificed at fixed ages), so it is **X_NOT_PRIMARY** (I5), not a pass. I re-tested the other 39 PMC copies on BioC: only this one returns a body, so the remaining 54 stand.
- The downloader only downloads: files stay outside the repo; each file goes in A67's download log (PMID, URL, date, sha256, who).
- A71 screens the delivered files with the same digit-masking viewer, I1–I5 and reason codes, and deletes them after extraction.
- No median is read (the 12 current passes included) until the 55 are coded or the deadline has passed and the full stage-2 list is committed. Then labs are recounted and all medians extracted at once.
- The `oa2b.rb` all-locations fix (#168) is already on main (99637cd via #175).

Agent: A71
