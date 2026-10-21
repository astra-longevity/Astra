# M5 reading by the writer (A78), README rule 7a

Source: NPJ Regen Med 2026, doi:10.1038/s41536-026-00490-x, Fig 1 (kidney, C57BL/6J, D+Q from 12 to 20 months). Only the mechanistic marker panels and the Fig 1 legend (for n and the error type) were read. No survival panel was opened.

The image is not committed (publisher figure). It is pinned by `sha256(fig1.png) = 696a7e0a2b7b6187f9f2de10789cb599f1f89a2281538283722f5df6e37413f6`.

## Order and independence

- Base contains #225 (NA4.1, merge `c1b935e`). This reading is committed after it.
- The scan was frozen before any value was computed: `bars3.py` sha256 `85a95d30…`, `scan.json` sha256 `aca3c0a6…`, at 2026-10-21T15:41:51Z.
- Written blind to A66's #211. I did not see his values, scripts or cap rule.

## Method

| File | Role |
| --- | --- |
| `axes.py` | own pixel scan: y-axis, baseline, tick stubs (raw coordinates) |
| `ocrtext.swift`, `labels.py` | OCR of tick-label **text only**, paired with my own tick coordinates |
| `bars3.py` → `scan.json` | own pixel scan: hollow-outline bars (two side lines + top edge) and error-bar caps |
| `coords.json` | committed coordinates: ticks with labels, bar tops, caps, n, panel identity |
| `convert.py` | deterministic conversion: linear axis from the 2 outermost label-matched ticks, SEM = \|cap − bar\|, ratio D+Q / Old, 95 % CI by the delta method on the log ratio with t(0.975, n−1) |

**Cap rule (mine).** Walk up the centre band (±3 px) from the bar's top edge while ink continues (gaps of at most 1 row). Flag rows whose ink run is 18–40 px wide. Data dots are ≤ 13 px and top edges about 50 px, so neither is flagged. The cap is the topmost contiguous block of flagged rows. This rule was set from the pixel structure alone.

Reproduce with `M5_IMAGE=/path/fig1.png python3 convert.py`.

## Result (treated = D+Q, control = Old)

| Panel | n | Old | D+Q | Ratio | 95 % CI | Comparable with #211 |
| --- | --- | --- | --- | --- | --- | --- |
| Fig 1d p16 positive area (%) | 5 | 21.61 ± 0.99 | 4.52 ± 0.68 | **0.209** | [0.135, 0.325] | yes |
| Fig 1f SA-β-gal positive area (%) | 5 | 13.29 ± 1.99 | 7.47 ± 1.23 | **0.562** | [0.303, 1.043] | yes |
| Fig 1d p21 cells per field | 5 | 9.12 ± 1.21 | 1.70 ± 0.51 | 0.187 | [0.075, 0.465] | **no**: A66 has it `no_mapping` (cap not located on his side); no back-filling either way |
| Fig 1b p16 mRNA | 6 | — | — | `no_mapping` | — | no: fewer than 2 tick labels readable |
| Fig 1b p21 mRNA | 6 | — | — | `no_mapping` | — | no: same |

Values are mean ± SEM in axis units. On both comparable panels, the middle tick (15) is predicted exactly by the 2-tick calibration (linearity check).

**Limits, stated for the seal:**
- Only **2 of the 5** listed panels (1d p16 area, 1f SA-β-gal) can be compared with A66. M-005 v2's size rests on those two. This is a limit, not figure-wide agreement.
- In Fig 1f the Young bar's top edge merges with the baseline. Young is not used in any ratio.
- The lower-right panel (ticks 8/6/4/2, marker word unreadable, layout suggests the Klotho panel i) is not a senescence marker and is not on the M5 figure list, so it is not used.
- The ratios are senescence-marker ratios. They become an H8 clearance multiplier only through NA4.1 (`c1b935e`), and that clearance multiplier reaches an organ only through the hallmark mean (m+n−1)/n. That step is applied in M-005 v2, not here.
