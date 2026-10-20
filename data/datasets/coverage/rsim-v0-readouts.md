# R-Sim v0 readout coverage (untreated mice)

Need (research/sim/v0/identifiability.md §6.4): organ readouts in untreated
mice, at least 10 mice per age per sex at 12, 18, 24 and 30 months, repeated on
the same mice if possible. Priority readouts: echocardiography, immune (flow
cytometry, vaccine response), complete blood count (CBC), kidney (GFR,
albuminuria), glucose tolerance test (GTT) with lean mass.

Screened 2026-10-20 by A74 on official pages and papers. Ages are in months.
"n" is mice per age per sex.

## Table

| Source | Readout | Strain | Sexes | Ages | n | Design | Access | Licence | URL | Verdict |
|---|---|---|---|---|---|---|---|---|---|---|
| DRiDO, ad libitum arm (manifest `dido-diversity-outbred-dr`) | echo (EF, FS, LV mass, cardiac output) | Diversity Outbred | F only | ~10, 22, 34 | 182, 123, 17 | longitudinal | public | MIT, approved LA-014 | https://github.com/calico/DRiDO_Paper_Release | partial: females only, ages off the 12/18/24/30 grid, n drops to 17 at 34 mo |
| DRiDO, ad libitum arm | blood flow cytometry (T, B, NK, myeloid subsets) | Diversity Outbred | F only | ~5, 16, 28 | 187, 166, 72 | longitudinal | public | MIT, LA-014 | same | partial: females only, no 30 mo |
| DRiDO, ad libitum arm | CBC | Diversity Outbred | F only | ~11, 23, 35 | 182, 113, 14 | longitudinal | public | MIT, LA-014 | same | partial: females only |
| DRiDO, ad libitum arm | fasting glucose; body composition (DEXA: BMD, fat/lean ratio) | Diversity Outbred | F only | glucose ~5, 16, 28; DEXA ~10, 22, 34 | glucose 185, 164, 68 | longitudinal | public | MIT, LA-014 | same | partial: no GTT, no lean mass in grams |
| DRiDO, ad libitum arm | kidney: urine voiding spot assay | Diversity Outbred | F only | ~10, 22 | not counted | longitudinal | public | MIT, LA-014 | same | no: not GFR or albuminuria |
| MPD Peters4 (manifest `mpd-peters4`) | CBC (ADVIA 120) | 30 inbred strains | M + F | 6, 12, 18, 24 | ~7 | design not stated | public (403 from Actions) | MPD terms, pending | https://phenome.jax.org/projects/Peters4 | partial: n 7, no 30 mo |
| MPD Petkova1 (manifest `mpd-petkova1`) | blood flow cytometry + CBC | 32 inbred strains | M + F | 6, 12, 18, 24 | 7-8 | design not stated | public (403 from Actions) | MPD terms, pending | https://phenome.jax.org/projects/Petkova1 | partial: n 7-8, no 30 mo, no vaccine |
| MPD Korstanje1 (manifest `mpd-korstanje1`) | urine albumin/creatinine ratio | 30 inbred strains | M + F | 12, 18, 24 (page procedure text says 6, 12, 20: to check in the data) | ~7 | design not stated | public (403 from Actions) | MPD terms, pending | https://phenome.jax.org/projects/Korstanje1 | partial: albuminuria only, no GFR, n 7 |
| SLAM, NIA (manifest `slam-nia`) | CBC, fasting glucose, NMR body composition (every 3 mo); frailty, grip, rotarod, open field (every 6 mo) | C57BL/6J, UM-HET3 | M + F | lifelong, to death | not published per age | longitudinal + cross-sectional arm | unknown (no public release) | refused LA-007 | https://doi.org/10.1093/gerona/glaa285 | would meet CBC, body comp; no data route |
| SLAM | echo, CLAMS, MRI | same | M + F | every 6 mo, subset only | subset | longitudinal | unknown | LA-007 | same | partial at best |
| SLAM | GTT/ITT | same | M + F | before euthanasia only | n/a | cross-sectional | unknown | LA-007 | same | partial at best |
| JAX NSC DO longitudinal study | blood at 6, 12, 18 mo; other readouts not listed | Diversity Outbred | 300 F + 300 M | 6, 12, 18, then to death | ~300 | longitudinal | unknown (JAX NSC data portal) | unknown | https://www.agingmice.org/home/animal-phenotyping-core/diversity-outbred-longitudinal-study/ | unknown: readout list and route to confirm |
| NIA Aged Rodent Colonies | none (sells aged C57BL/6J mice and tissues) | C57BL/6J | M + F | to ~30 | n/a | n/a | request | n/a | https://www.nia.nih.gov/research/dab/aged-rodent-colonies-handbook | no data; route for our own measurements |
| Other MPD echo sets (Jaxwest1, GMC12, Eumorphia6, Jaxpheno8); IMPC late adult | echo | various | M + F | one young age (IMPC ~15 mo) | various | cross-sectional | public | MPD / IMPC terms | phenome.jax.org | no: no ageing series |

Tabula Muris Senis (manifest exists) is cross-sectional single-cell, not an organ-function readout; not counted.

## Verdict per priority readout

| Readout | Best source | Verdict |
|---|---|---|
| echocardiography | DRiDO (F, ~10/22/34 mo) | partial: no males |
| immune | DRiDO flow (F) + MPD Petkova1 (M + F, n 7-8) | partial: no 30 mo, no vaccine response |
| CBC | DRiDO (F) + MPD Peters4 (M + F, n 7) | partial |
| kidney | MPD Korstanje1 (albuminuria, M + F, n 7) | partial: no GFR |
| GTT + lean mass | none (DRiDO fasting glucose + DEXA; SLAM GTT terminal only) | gap |

## Gaps (no source covers)

1. GTT repeated on the same mice with lean mass.
2. GFR (any design).
3. Vaccine response by age.
4. Echocardiography in ageing males.
5. 30-month point with n ≥ 10 per sex for any readout (DRiDO ~28-35 mo has n 14-72 in females only).
6. Lung function (expected: terminal assay).

## Routes out

| Obstacle | Route | Who to ask |
|---|---|---|
| MPD 403 (Peters4, Petkova1, Korstanje1, and mpd-itp1) | Ride on the MPD access request sent by the president on 2026-10-20; ask MPD to allow these four projects' CSV downloads from GitHub Actions | MPD support (phenome.jax.org contact) |
| SLAM has no public release (O16) | Written request for per-mouse CBC, body composition, fasting glucose, echo subset and GTT data, untreated arms, both strains and sexes | NIA Intramural, de Cabo lab (SLAM PI), via the president |
| JAX NSC DO longitudinal (600 mice) | Ask for the readout list, release date and data terms of the DO longitudinal study | JAX Nathan Shock Center (agingmice.org contact) |
| GTT longitudinal, GFR, vaccine response, male echo | No public set found: these are candidate first measurements on our own mice (NIA aged C57BL/6J colonies), when the president approves mice | Research (A28) to decide, president for mice |
| Korstanje1 age inconsistency | Check the ages in the CSV once MPD access works; otherwise ask the contributor | ron.korstanje@jax.org |

## Licences for Verification

- `mpd-peters4`, `mpd-petkova1`, `mpd-korstanje1`: pending. Same MPD terms of use as `mpd-itp1` (LA-006); Verification to confirm the conditions extend.
- DRiDO is already approved (LA-014); SLAM stays refused (LA-007) until NIA answers.
