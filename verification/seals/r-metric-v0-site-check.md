# R-Metric v0 seal: public site check

Part of the D115 seal record for the R-Metric v0 held-out split. Task: A50, G140 2026-10-22 (A1 ruling 6, G183). Checker: A65. Add-only.

**Question:** does anything from the sealed split appear on the public site or in its generator inputs? Sealed split: `research/metric/v0/split.csv` sha256 `87ef760c…`, 19 blind arm ids (B01–B19), 15 compound-new blind arms, 40 retro arms.

**A hit** means any of: a blind arm id; any result, effect, or compound/cohort pairing for B04/B05 or for a compound-new blind arm; any result from an ITP cohort started in 2021 or later. A hit is a refusal item until it is removed.

## Check 1 (2026-10-22)

### What was checked

- **Live site:** https://astra-longevity.github.io/Astra/, deployed by Pages run 39744508040 from main `18cc9a5` (08:01Z). I crawled it on 2026-10-22 at about 08:35Z and got 536 pages: index, status, roadmap, decisions, findings, map, questions, journal, glossary, 263 claim pages and 259 source pages.
- **Generator inputs** on main (`crates/astra-site`): every input the generator reads, listed below.
  - `literature/**/*.md`, since every file becomes a doc page.
  - `literature/lifespan/itp/*.yaml`, which feeds the Findings forest.
  - `research/metric/v0/axes.md`, `research/runs/`, `research/obstacles/`.
  - `docs/charter/`, `docs/decisions/`, `interface/`.
- **Method:** text search for the 19 ids, the 19 blind compounds and their synonyms, and the cohort labels ITP 2021–2025. I only located hits and read the text around them. Every digit in that text was masked before I read it, and no number is copied here.

### Results

| Item | Where | Hit? |
|---|---|---|
| Blind ids B03/B04/B05/B08 | `journal.html`: commit titles of #122, #125, #142/#145, #178, #205, #206 | No. These are procedural titles with no result |
| Blind ids, other pages and inputs | none | No |
| ITP cohorts from 2021 on | none. The Findings forest has cohorts 2004–2017 only, and there is no 2021+ file in `literature/lifespan/itp/` | No |
| B04/B05 canagliflozin | `findings.html` (train arm C2016), `axes.md`, `controls.md` (2016 arm, "excluded while blind"), `obstacles.md` (Miller 2020 = 2016 cohort), council brief R1 (Miller 2020) | No. All of these are the 2016 training cohort. Nothing on the late-start 2023 arms |
| B03 rapamycin + 17α-estradiol (ITP 2023) | only the single-compound train cohorts (2009–2016) | No |
| B08 captopril (ITP 2024) | only the 2017 train arms (Findings, `controls.md`, Strong 2022 in the council brief R1) | No |
| B19 urolithin A | aging-map 05/07/12, claims C-0095/0096/0137/0229/0238, questions | No. Worm lifespan, mouse muscle and human trials only. No mouse lifespan result |
| B09 trametinib (compound-new) | `literature/briefs/founding-council-r1.md` lines 57 and 147, rendered on its doc page and in the `questions.html` "to verify" list | **Borderline, see below** |
| Other compound-new blind compounds (DCA, TPPU, sodium thiosulfate, lactoferrin, mianserin, cinnarizine, L-deprenyl, sildenafil, alpelisib, alogliptin, BT2, glucosamine, taurine) | none | No |

**Borderline, B09 trametinib.** The council brief R1 cites a mouse lifespan effect for **trametinib + rapamycin** from Gkioni 2025 (*Nat Aging*, doi:10.1038/s43587-025-00876-4). That study comes from a non-ITP lab, with its own cohort and strain. It is not B09's result: B09 is trametinib alone, ITP 2024, and its outcome is unread. Under the rule as written, only a result for the blind cohort itself moves an arm (A50, 2026-10-21 14:44). So this is not a leak of the sealed result. What it does mean is that forecasters could have seen a published lifespan effect for a compound-new blind compound before the seal. A50 rules on how this is recorded.

### Verdict

**No hit** under the rule above. One borderline item (B09) has been sent to A50 for a ruling. The crawl and the input scan are re-run at the seal against the sealed `split.csv`, and any new page or input is checked the same way.
