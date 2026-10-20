# R-Metric v0 refill list: Verification check

Checked by A65 (Verification) on 2026-10-20. Object: PR #75, head `8c82e77`,
`literature/lifespan/drugage-refill/`. Rule: `r-metric-v0-refill.md`
(PR #62, conditions 1–5; PR #64, conditions 6–7).

**Verdict: conforms.** The list may enter the split proposal and later the
D115 seal record.

## Independent rebuild

I downloaded `https://genomics.senescence.info/drugs/dataset.zip` myself and
ran `build.rb` @ `8c82e77`, alone, on my copy. Only the script's printed counts
were read. No outcome column was opened.

| Item | Value | Matches PR |
|---|---|---|
| zip sha256 | `d12f59717de60a207748f53bdbcdb484ed07e30d8608eb8e4a101e2a08d7fa80` | yes (= LA-017) |
| `build.rb` sha256 | `ea277d5f78dfd2844b4954dd3e2bf69f35b7e3676cbc1a40577dba7cca81f382` | yes |
| `pubmed-years.tsv` sha256 | `bf2bc3340b41dde64559ccfcded689f446ef28099eed3b946d8811f44f3b074a` | n/a |
| `exclusions.csv` sha256 | `1910c7c19c082bdc74b38d7837cbd2ffcfc6c431a7f0879a99996e07f55586a6` | n/a |
| `refill-list.csv` sha256 | `792d54b653d6089c67edf9f30ea759788f8148ac63d87878b81512fd7b20786d` | yes, byte-identical rebuild |
| Mouse arms / studies | 372 / 88 | yes |
| Studies, PubMed year 2015–2022 | 30 | yes |
| Dropped: ITP flag / exclusions.csv | 7 / 0 | yes |
| Kept | 23 studies (PMIDs), 40 arms | yes |

## Conditions

| # | Met? | Note |
|---|---|---|
| 1 Pinned source | yes | Build 5, URL, date, zip sha256 in README; script refuses any other zip. |
| 2 Outcome-stripped | yes | Script reads 7 metadata columns only; output columns `arm_id, pmid, year, compound, sex, strain, dose`. |
| 3 Bias label | yes | README: "retrospective, not blind, DrugAge-selected". |
| 4 Licence | yes | LA-017 (PR #65); attribution in README. |
| 6 Script build, no pruning | yes | Filters: species, PubMed year, ITP flag, `exclusions.csv` (rule exclusions: ITP + logged reads). Nothing removed after the build. |
| 7 Bias measured | open | Applies at scoring: T1-A2-3 filters and the refill vs ITP null/shortening share go in the scorecard. |

Two arms of the list repeat on all metadata columns. They are separate
DrugAge rows (they differ in columns we don't read) and stay in, as the rule
says "every arm, whatever its result".

## Row count: 3 423 vs 3 406

`drugage.csv` has 3 423 physical lines = 3 423 parsed records: no multi-line
fields. 413 records repeat on the 7 metadata columns, but they differ in
columns we may not read, so they cannot be called duplicates without
reading outcomes. The +17 is not explained from metadata. The release notes
say the current stats may differ from the build.

**Ruling: not blocking.** The pin is the zip sha256, not the count. The seal
record states both counts (3 423 rows in the file, 3 406 in the release
notes) and this finding. Nobody reads outcome columns to explain the gap.
