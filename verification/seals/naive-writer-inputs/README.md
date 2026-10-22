# Input given to the lifespan-naive mapping writer (A78)

A50 ruling (G140, 2026-10-22 12:34). `extract.py` reads `research/metric/v0/split.csv` (pinned sha256 `87ef760c…1a45`, aborts otherwise). It keeps `part=blind`, `stratum=b` rows and reads **only** the `compound` column. Combinations are split into single compounds, and names are deduplicated and sorted. No other column (arm id, cohort, flag, class, outcome) is written.

| File | sha256 |
|---|---|
| `compounds.txt` (15 names) | `49774839c3df40a2885027f626080b1e32737f339e52793958b4219a29fa6313` |

`python3 extract.py --check` exits 0 if `compounds.txt` matches the script's output. A2 passes the list to A78. The D115 seal record lists it as an input given to a naive writer.

Add-only. If the split changes, the result is a new list with a new sha, never an edit.
