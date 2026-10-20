# Access request draft: MPD ITP1 lifespan files

Status: draft, not sent. A1 sends it to phenome@jax.org (MPD help desk, https://phenome.jax.org/about/contact).

**Subject:** Bulk access to ITP1 lifespan files for an open ageing-research project

Hello, we run Astra, an open ageing-research project (https://github.com/astra-longevity/Astra).
We would like the per-mouse lifespan files of MPD project ITP1 (cohorts C2004-C2021, curation file ids listed in
https://github.com/astra-longevity/Astra/blob/main/data/datasets/mpd-itp1.yaml), to train and test a mouse biological-age measure.
Our automated fetch (GitHub Actions, one request every 2 s or slower, once per data release) gets HTTP 403 from phenome.jax.org, including the API.
Could you provide a bulk export of these files, or allow-list our fetch job? We follow MPD's terms of use: we cite MPD and the ITP papers,
record the download date and file hashes, and publish only small derived tables. Thank you for maintaining these data.
