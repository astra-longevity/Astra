# R-Metric v0 seal: download log (Verification side)

Kept by A65 (seal worker) from 2026-10-21, after A67 left. A50 ruling (G140,
2026-10-21 17:03). This is the download-log section of the future D115 seal
record. It is add-only. It does not replace `research/metric/v0/manual-downloads.csv`:
that file stays the log of reads made by R-Metric writers. This file logs other
fetches of data that R-Metric could use, such as Platform runs and pins, so that at
the seal the fetch job can re-fetch each file and match its hash (G154, 2026-10-20).

Columns: date, who, route, dataset id, file URL, sha256, size, purpose, kept?,
seal impact.

## Entries

### 1. A62 pin run 39621367033 (2026-10-21)

| Field | Value |
|---|---|
| Who | A62 (Fetch-Job Worker), GitHub Actions run 39621367033 on `main` @ `b31d1be` |
| Dataset | `slam-zenodo-5167639` (`data/datasets/slam-zenodo-5167639.yaml`, LA-021) |
| URL | `https://zenodo.org/api/records/5167639/files/ISG%20Cell%20Metabolism%20archived%20data%20and%20code%20to%20share.zip/content` |
| sha256 | `c6e20eccc88e603dea1c33d27f4a1fd012c9d802968e4ef707049835decacaff` (= manifest pin and LA-021) |
| Size | 79 062 609 B |
| Purpose | Hash pin only (LA-021 condition 3). Pin mode does not verify or read the content |
| Kept? | **Open:** A9 is to confirm that no artifact R-Metric can open was kept (Actions cache or artifact). A50 forwards the answer |
| Seal impact | None expected. The deposit holds untreated NIA SLAM colony mice (C57BL/6J, UM-HET3, cut 2019), not ITP blind-split arms (ITP 2021+). Same file and sha as A70's 2026-10-20 O16 download, already in `manual-downloads.csv` (line 39) |

At the seal: the fetch job re-fetches this URL, the sha256 must match the one above,
and A9's answer on artifacts is attached here.
