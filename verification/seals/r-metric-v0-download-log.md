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
| Kept? | **No.** A10 (via A50, G140 17:06): hash only, no unzip, 0 artifacts, no cache entry. The run failed on the MPD 403s and the runner VM was deleted |
| Ruling (A50, 17:06) | A later Actions cache entry of the zip is not a read and needs no re-seal, because SLAM has no blind-split arm. Any unzip or parse of the zip before the seal gets its own line in this log first |
| Seal impact | None expected. The deposit holds untreated NIA SLAM colony mice (C57BL/6J, UM-HET3, cut 2019), not ITP blind-split arms (ITP 2021+). Same file and sha as A70's 2026-10-20 O16 download, already in `manual-downloads.csv` (line 39) |

At the seal: the fetch job re-fetches this URL, and the sha256 must match the one above.

### 2. A9 verify run 39622203341 (2026-10-21 15:04Z)

| Field | Value |
|---|---|
| Who | A9 (Database Manager), GitHub Actions run 39622203341 on `main` @ `b31d1be` |
| Dataset | `slam-zenodo-5167639` (LA-021), second CI download |
| URL | same as entry 1 |
| sha256 | `c6e20eccc88e603dea1c33d27f4a1fd012c9d802968e4ef707049835decacaff`: **matches** LA-021 and the manifest |
| Size | 79 062 609 B: **matches** |
| Purpose | Verify the pinned hash and size only. No unzip |
| Kept? | **Nothing.** A10 (via A9, G157 18:30; forwarded by A50, G140 18:32): no artifact (`fetch.yml` has no upload step), no Actions cache entry (the run failed on the MPD files and the cache saves only on success), no runner copy (VM deleted). Hash only, no unzip |
| Seal impact | None expected (same deposit as entry 1, no blind-split arm) |
