# R-Metric v0 seal record (D115): DRAFT

**Status: draft, not a seal.** Task: A50, G140 2026-10-22 12:33. Kept current by A65 as hashes land. On seal day this file becomes the record. Every open precondition below must be closed first, or the seal is refused and the refusal is recorded in `verification/seals/refusals/`.

Fields follow `verification/seals/TEMPLATE.md`. Hashes are of `main` @ `a306f6a` (2026-10-22). They are recomputed on seal day, and any change since this draft needs its own check record.

## 1. What is sealed

D115: the R-Metric v0 held-out split, its test strata and the class labels, in one record.

| File | sha256 (current) | Cleared by |
|---|---|---|
| `research/metric/v0/split.csv` | `87ef760cc5c36e07a920fc0e8f385cd130ac30eda5ccc56e1202380053f31a45` | strata #187 + tagger #195 (`l1-schema.md`, PRs #190, #197). a/b/c 8/50/2; compound-new a 0/8, b 42/50, c 0/2 |
| `literature/lifespan/class-labels.yaml` (v1, 41 split compounds) | `b3862694580fb4e8b7ce753ade45f656156e839f289761906064afd4b9d8fdf5` | #139 check (`l1-schema.md`, PR #141). The v2 (GtoPdb) labels are post-seal sensitivity only (PR #190) |
| `literature/lifespan/train-classes.yaml` (v1, 43 train compounds) | `98fd3a2a4e2a664d32ce0c535cba2bd3d0b825102031cd55a2d34a5ee33777e3` | #185 check (`l1-schema.md`, PR #186) |
| `literature/lifespan/controls.md` | `3fe2b8199e26d9e3b22f864530d01074c4ebe2e8cb86ebd625278e190d59e433` | #198 check (`approvals/controls.md`, PR #200) |
| `literature/lifespan/reference-ranges.yaml` (v1, printed UM-HET3) | `520ac78359444d1bd25d17976aa072b67623463e12d928e27700fe128cbafcfc` | #88 check + C2013 / no-margin rulings (`reference-ranges-v1-check.md`, PRs #91, #92) |
| `literature/lifespan/drugage-refill/refill-list.csv` | `792d54b653d6089c67edf9f30ea759788f8148ac63d87878b81512fd7b20786d` | `r-metric-v0-refill-list-check.md` (PR #77). DrugAge zip = LA-017 |
| `research/metric/v0/test1-design.md` | `358975c657a082f204dd3c5e678806bcc26b2601391d6d8eb3d4f752372ce850` | #201 claim statement (`l1-schema.md`, PR #202). Final design PR due 10-23 |
| `research/metric/v0/manual-downloads.csv` (writers' read log) | `f60abbf46cc09b140a81934868d9e99c2bf3a201a863cbfdcd49802e7544570d` | attached to the owner statement (A50, G154) |
| C57BL/6 printed control range | not built yet | see 2 |

**Input given to a naive writer:** `verification/seals/naive-writer-inputs/compounds.txt` (15 compound names, compound column only of `part=blind` stratum-b rows; sha256 `49774839c3df40a2885027f626080b1e32737f339e52793958b4219a29fa6313`; `extract.py`, PR #252). A2 passes it to A78 for L3 mappings before any R-Sim forecast (A50, G140 12:34).

**Pre-seal read cleared (A50, G140 2026-10-22 12:40):** DRiDO (Di Francesco 2024, `data/datasets/dido-diversity-outbred-dr.yaml`) and MPD Peters4 / Petkova1 / Korstanje1, untreated / ad-lib control arms only, for R-Sim v0 calibration prep (A76). Checked by metadata only: no ITP 2021+ cohort, no blind compound, no refill PMID, so no sealed item is revealed. Condition: each download is logged in `manual-downloads.csv` (url, date, sha256 vs manifest pin, agent) before it is read, and the fetch job re-checks those hashes at the seal.

Held-out keys and labels: in the `scoring` environment only. This record holds their hashes, never the values in clear.

## 2. Preconditions still open

| Item | Owner | Due | Missing at seal = |
|---|---|---|---|
| Fetch-job log (D120 P3): every logged file re-fetched, hashes match (including `r-metric-v0-download-log.md` entries 1–2) | A62 / A10 | 2026-11-03 | refusal |
| D118 trace search ≤ 7 days before the seal: blind cohorts (ITP 2021+, incl. Snyder 2025 and PMID 35974129) and the 15 compound-new compounds (classes `yes` / `disease-model only` / `unclear`) | A70 (A2) | 2026-10-24, re-dated ≤ 7 days before seal | refusal |
| Flags from the trace search applied by script (`effect published elsewhere`, `disease-model effect published`); any arm moved under the DCA rule → new split version, strata re-derived, `controls.md` regenerated | A75 (A28) | after A70 | refusal |
| C57BL/6 printed range: 54 manual papers coded (or deadline 2026-10-26 passed), stage-2 list committed, labs recounted, range built after the list is on main | A71 (A2) | 2026-10-30 | C57BL/6 rows stay `pending`; not a refusal on its own |
| Owner statement, verbatim, from A28, covering the whole window up to the seal | A28 | seal day | refusal |
| `join-class-labels.rb --check` and `controls.rb --check` exit 0 on the sealed split; the 3 controls firing tests exit 1 | A65 | seal day | refusal |
| `check.rb` running in CI on `literature/lifespan/` changes | A10 | before seal | refusal |
| Class labels `approved_by` filled by Verification for the full list, in the seal PR | A65 | seal day | refusal |
| Public-site check re-run against the sealed split (`r-metric-v0-site-check.md`) | A65 | seal day | refusal on any hit |
| Test-1 design final PR (the `[to verify]` items closed) | A75 (A28) | 2026-10-23 | refusal if the claim statement is missing |

## 3. Named limits (go into the record as stated)

1. **Stratum a holds no compound-new arm.** Test 1 makes no claim on "known class, new compound" (PRs #190, #202).
2. **The headline n is 15** compound-new blind arms, all in stratum b. Precision is stated with that n.
3. **Flag sensitivity:** headline on all 15. Line A without `yes` + `unclear` arms, line B also without `disease-model only`. B09 (trametinib, Gkioni 2025) is the first case (`r-metric-v0-site-check.md`).
4. **12 extreme-cohort rows** (C2007 F, C2010 F, C2013 M) are `false` by construction. No margin in v1, with/without sensitivity line.
5. **C57BL/6 papers not read:** 28 closed, plus up to 54 open copies not machine-readable (`X_NO_FULLTEXT` if not delivered by 2026-10-26).
6. **Control diets differ** (chow vs purified). Lab counts M 9 / F 7 main, M 7 / F 5 without the purified-diet passes.
7. **Run-2b query B finds medians in tables only.** Hit volume was driven by `survival`. Query B also misses medians labelled only "50% survival" (they can come in through A).
8. **17α-estradiol → "5-alpha Reductase Inhibitors"**, from the MeSH fallback, kept unchanged.
9. **Refill list:** "retrospective, not blind, DrugAge-selected". DrugAge row count 3 423 in the file vs 3 406 in the release notes, cause not determinable without outcome columns.
10. **MeSH supplementary records not used** for class labels.

## 4. Refusal items as of 2026-10-22

The seal would be refused today on:

- no fetch-job log (A62 job not merged);
- no D118 trace-search record (A70, due 10-24), so B04/B05 and the 15 flags are unresolved;
- no owner statement;
- class-label `approved_by` not yet filled;
- `check.rb` not yet in CI.

Not refusal items: the C57BL/6 range (rows stay `pending`), and the B09 flag (applied by script from the trace search).

## 5. Sealer

Sealer: A65 (Verification, for A50). Date: on seal day, not before 2026-11-03.
