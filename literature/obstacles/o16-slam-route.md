# O16: public route to SLAM longitudinal data

Status: route note v0, 2026-10-20 (A70, for A2). All URLs accessed 2026-10-20. No account was registered and no access block was worked around.

## Short answer

A public, per-mouse SLAM deposit exists: Zenodo record 10.5281/zenodo.5167639 (CC BY 4.0), released with Palliyaguru 2021 (Cell Metab). It covers **lifespan, NMR body composition, fasting glucose and body weight** for both strains and both sexes. It does **not** contain CBC, echocardiography or GTT. Those three still need a written request to NIA (X1035, draft due 2026-10-30).

This changes A9's coverage row for SLAM (`data/datasets/coverage/rsim-v0-readouts.md`, "unknown (no public release)", LA-007): part of SLAM is public. A licence ruling for the Zenodo deposit is needed from A66/A50 before use.

## What SLAM is

Study of Longitudinal Aging in Mice, NIA intramural (Translational Gerontology Branch, de Cabo lab). C57BL/6J (JAX 000664) and UM-HET3 mice of both sexes, about 200 mice per cohort every 3 months, followed to death on NIH-31 chow ad libitum (Palliyaguru 2021 J Gerontol A; Palliyaguru 2021 Cell Metab). No treatment arms: every mouse is an untreated control. Planned measures (J Gerontol A): body weight, food intake and temperature every 2 weeks; 6-h fasting glucose, body composition and haematology (ADVIA 120) every 3 months; gait, frailty index and motor tests at longer intervals; echo on a subset; GTT/ITT before euthanasia only.

## Routes checked

| Route | What is there | Per-mouse? | Strains / sexes | Ages | Licence | URL | Verdict |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Zenodo deposit of Palliyaguru 2021 Cell Metab | `Data/slam/`: census, survival with cause of death, NMR, fasting glucose + lactate, body weight/temperature/food | yes | B6 + HET3, F + M | 2 to 38 mo | CC BY 4.0 | https://doi.org/10.5281/zenodo.5167639 | **usable** (see below) |
| Same deposit, other folders | JAX DO glucose phenotypes (`Data/jax/JAC_DO_phenotypes_compiled_v8.csv`), NIA rhesus data, NHANES files | yes | DO; monkeys; humans | — | CC BY 4.0 | same | side source |
| Martinez-Romero 2024 Nat Aging (haematology clock) | CBC from SLAM (and JAX) | — | B6 + HET3 (+ DO) | 12 to 151 wk | — | https://doi.org/10.1038/s43587-024-00728-7 | data "from the lead contact upon reasonable request": **no public route** |
| Palliyaguru 2021 J Gerontol A (SLAM design) | design, measures; promises a SLAM website and public databases "upon publication" | no | — | — | — | https://doi.org/10.1093/gerona/glaa285 | describes data, none deposited |
| SLAM website / database | announced in the design paper | — | — | — | — | none found | not found |
| Mouse Phenome Database | no SLAM or de Cabo project found by search; API returns 403 (same block as O9) | — | — | — | — | https://phenome.jax.org | not found (API blocked, not worked around) |
| Figshare, Dryad | searches for "Study of Longitudinal Aging in Mice" and SLAM + HET3: 0 hits | — | — | — | — | api.figshare.com, datadryad.org | nothing |
| Zenodo search | only the deposit above matches SLAM | — | — | — | — | zenodo.org | — |
| GEO | search "Longitudinal Aging in Mice": no SLAM-labelled series in the top 20 | — | — | — | — | ncbi.nlm.nih.gov/gds | nothing |

## What the Zenodo deposit holds (measured on the files)

Zip sha256 `c6e20eccc88e603dea1c33d27f4a1fd012c9d802968e4ef707049835decacaff` (79 MB). Data cut: deaths to 2019-10-08, glucose to 2019-09-03.

| File | Content | Size |
| --- | --- | --- |
| `census.csv` | one row per mouse: id, tag, cohort 1-13, sex, strain, date of birth | 2 783 mice: B6 F 690, B6 M 688, HET3 F 697, HET3 M 708 |
| `Survival.csv` | date of death + cause | 1 775 records (1 770 match the census) |
| `NMR.csv` | body weight, fat, lean, fluid per scan | 11 860 dated scans, 2 086 mice, 4-32 mo, median 6 scans per mouse (max 12) |
| `Glucose_Lactate.csv` | fasting glucose and lactate per bleed | 13 811 glucose values, 2 394 mice, 2-38 mo, median 6 per mouse (max 11) |
| `BW_Temp_FC_All_2019-11-16.csv` | body weight, temperature, food intake | not yet summarised |

Lifespan: causes include "Found dead" 686, "Culled Per Vet" 200, "DVR or Pathology" 57 (natural or humane-endpoint deaths) but also "End of study, organs removed" 279, "Used in experiment" 268, "Per PI" 138, "Not in room" 54, "Died during procedure" 40 and smaller classes, which must be treated as censored.

### Censoring, per strain × sex

Counts from `census.csv` joined to `Survival.csv` on tag. "Died" = cause "Found dead", "Culled Per Vet" or "DVR or Pathology". "Removed" = any other cause (end of study, used in experiment, per PI, procedure deaths, etc.). "No record" = in the census, no row in `Survival.csv`.

| Strain × sex | Mice | Died | Removed | No record |
| --- | --- | --- | --- | --- |
| B6 F | 690 | 246 | 200 | 244 |
| B6 M | 688 | 235 | 206 | 247 |
| HET3 F | 697 | 217 | 214 | 266 |
| HET3 M | 708 | 244 | 208 | 256 |

The files carry **no censoring flag** and the deposit has **no README**: `Survival.csv` has only tag, date of death and cause, and the Zenodo record has one zip and an abstract. Mice with no record cannot be told apart as alive at the cut or lost; their last NMR or glucose date is the only available last-seen date. Which causes count as deaths is a rule to set (and seal) before any lifespan use; no lifespan analysis is done here.

Per-file sha256 (first 16 hex): census `40bbd8986557dc97`, Survival `76ec762f06a4cfaa`, NMR `fa54986c8e2bee9d`, Glucose_Lactate `886991d04d340fc3`, BW_Temp_FC `798c24aee4c4380d`.

## Gaps per needed readout

Need from A2 / A9 (`rsim-v0-readouts.md` @26336bb): per-mouse lifespan of untreated arms, CBC, NMR body composition, fasting glucose every 3 mo, echo subset, GTT; B6 + UM-HET3, both sexes.

| Readout | Public route | Status |
| --- | --- | --- |
| Lifespan, untreated | Zenodo `Survival.csv` + `census.csv` | **covered**, both strains and sexes; censoring rule needed; data cut 2019 (later cohorts truncated) |
| NMR body composition | Zenodo `NMR.csv` | **covered**, repeated per mouse |
| Fasting glucose, every 3 mo | Zenodo `Glucose_Lactate.csv` | **covered**, repeated per mouse |
| CBC | Martinez-Romero 2024: on request only | **gap** → NIA request |
| Echo subset | none | **gap** → NIA request |
| GTT | none (terminal only in SLAM design) | **gap** → NIA request; even then cross-sectional, not repeated |
| Frailty, grip, rotarod, gait (bonus) | none | **gap** → can be added to the request |

Since lean mass and glucose sit in the same deposit for the same mice, the "GTT + lean" gap in A9's coverage can be partly bridged with fasting glucose per lean mass, but that is not a GTT.

## Next

1. A66/A50: licence ruling for zenodo.5167639 (CC BY 4.0; derived from NIA intramural data).
2. A9: manifest `slam-nia` can point to the Zenodo deposit for the covered readouts.
3. NIA request draft for CBC, echo, GTT (+ frailty, grip, rotarod, gait, and death/censor status after the 2019 cut): `data/requests/nia-slam-access.md`.

The zip was downloaded once, only to measure the files and record hashes; the local copy is deleted. Future fetches go through A9's manifest.

## Sources

- Palliyaguru DL et al. Study of Longitudinal Aging in Mice: presentation of experimental techniques. J Gerontol A 2021;76:552-560. doi:10.1093/gerona/glaa285 (Crossref OK)
- Palliyaguru DL et al. Fasting blood glucose as a predictor of mortality: lost in translation. Cell Metab 2021. doi:10.1016/j.cmet.2021.08.013 (Crossref OK); data doi:10.5281/zenodo.5167639 (Zenodo record, CC BY 4.0)
- Martinez-Romero J et al. A hematology-based clock derived from the Study of Longitudinal Aging in Mice to estimate biological age. Nat Aging 2024. doi:10.1038/s43587-024-00728-7 (Crossref OK)
