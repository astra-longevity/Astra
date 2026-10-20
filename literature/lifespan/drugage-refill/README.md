# R-Metric v0 retrospective refill list (DrugAge)

**Label: retrospective, not blind, DrugAge-selected.** DrugAge curates studies that report lifespan effects and may hold fewer null results (rule, condition 3).

Rule: `verification/approvals/r-metric-v0-refill.md` (PR #62), applied as written, **with no other filter**. Licence: LA-017 in `verification/approvals/licences.md`. Built by A71 on 2026-10-20.

## Source pin

| | |
|---|---|
| Source | DrugAge Build 5 (HAGR), release date 2024-11-29, 3 406 entries (release notes) |
| URL | https://genomics.senescence.info/drugs/dataset.zip |
| Downloaded | 2026-10-20, one bulk download, no scraping |
| zip sha256 | `d12f59717de60a207748f53bdbcdb484ed07e30d8608eb8e4a101e2a08d7fa80` (45 365 bytes, matches LA-017) |

The zip is not committed. `build.rb` refuses any other file. Note: `drugage.csv` in this zip has 3 423 data rows, not the 3 406 entries in the release notes (the notes say current stats may differ from the build). The pin is the file hash, not the count.

## Files

- `build.rb`: the build. It reads only `compound_name, species, strain, dosage, gender, ITP, pubmed_id`. Outcome columns are never read or written.
- `pubmed-years.tsv`: the PubMed year (`pubdate`) of every mouse PMID in Build 5, from NCBI E-utilities esummary on 2026-10-20. The year comes from PubMed, not from DrugAge (rule).
- `exclusions.csv`: PMIDs excluded, with the source line and how it was mapped to a PMID:
  - every DOI, PMC, PubMed or publisher URL in `research/metric/v0/manual-downloads.csv` @ origin/main, 2026-10-20 (read = train). The NIA compounds page and the seven "G158 A2 message" lines have no PMID; they are ITP content, already covered by the ITP exclusion.
  - the ITP publications (MPD ITP1 reference list).
- `refill-list.csv`: the output. Columns `arm_id, pmid, year, compound, sex, strain, dose`. No lifespan column.

## Steps (build.rb)

1. Keep *Mus musculus* rows.
2. Keep studies (one PMID = one study) with PubMed year 2015 to 2022 inclusive.
3. Drop a study if any of its arms has DrugAge `ITP = Yes`, or if its PMID is in `exclusions.csv`.
4. Keep every arm of the remaining studies, whatever its result.

## Result

| | |
|---|---|
| Mouse arms / studies in Build 5 | 372 / 88 |
| Studies, PubMed year 2015-2022 | 30 |
| Excluded: ITP flag | 7 |
| Excluded: `exclusions.csv` (after ITP) | 0 |
| **Kept** | **23 studies, 40 arms** |
| `refill-list.csv` sha256 | `792d54b653d6089c67edf9f30ea759788f8148ac63d87878b81512fd7b20786d` |
| `build.rb` sha256 | `ea277d5f78dfd2844b4954dd3e2bf69f35b7e3676cbc1a40577dba7cca81f382` |

Rebuild: `ruby literature/lifespan/drugage-refill/build.rb <dataset.zip>` (Ruby stdlib + `unzip`). The output must match the sha256 above.

## Attribution (CC BY 3.0, LA-017)

Adapted from DrugAge, Human Ageing Genomic Resources (https://genomics.senescence.info/drugs/). Cite: Barardo D. et al. (2017) "The DrugAge database of ageing-related drugs." Aging Cell 16(3):594-597, doi:10.1111/acel.12585. Licence: CC BY 3.0 Unported, https://creativecommons.org/licenses/by/3.0/ (HAGR conditions: https://genomics.senescence.info/legal.html).

Changes made: filtered to *Mus musculus*, to studies with PubMed year 2015-2022, ITP and already-read studies removed; all outcome columns removed (lifespan change, significance, weight change, age at initiation, treatment duration); columns renamed; arm ids added.
