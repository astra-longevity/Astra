# M5 blind second digitisation — NPJ Regen Med 2026, Fig 1 (A66)

Owner: Verification (A66, for A50's G142 ruling of 2026-10-21). This is the
**blind second digitisation** required by rule 7 of A78's mapping rules
(`literature/refutation/README.md`, PR #207, on main `d76be3b`): a figure-only
effect is digitised by A78's committed script and, independently, here, blind
to A78's values. Done before A78 opens the M5 rows PR.

## Source

- Paper: *Multi-omics profiling reveals systemic rejuvenation of the aged
  kidney through senolytic therapy* (D + Q), NPJ Regenerative Medicine 2026,
  doi `10.1038/s41536-026-00490-x`. C57BL/6J, dasatinib + quercetin.
- Mapping: **M5**, senolytic → H8.
- Image: Fig 1 only, fetched from the article's `figures/1` page
  (`media.springernature.com/.../41536_2026_490_Fig1_HTML.png`). **Not committed**
  (not ours to redistribute; licence not checked for this image — a figure read
  as a literature extraction, not a dataset).
- Image fingerprint: `fig1.png` **sha256
  `696a7e0a2b7b6187f9f2de10789cb599f1f89a2281538283722f5df6e37413f6`**. The
  scripts assert this hash before running; a different image aborts.
- Panels read: **1b** (p16, p21 mRNA, qRT-PCR), **1d** (p16/p21 IHC positive
  area and positive cells per field), **1f** (SA-β-gal positive cells). Series
  in each: young / old (vehicle) / old + D + Q. Caption: N = 6 (b), 5 (d, f),
  mean ± SEM. **No survival/KM panel or page was opened** (none is in Fig 1).

## Method (A50 ruling, G142 2026-10-21, option 1)

1. **Calibration on ticks, never the frame.** `ticks.py` scans each y-axis for
   the axis line, the baseline and the tick marks by pixel row/column profile.
   Each tick centre (raw y) is matched to its printed label; `digitise.py` fits
   `value = α + β·row` by least squares and prints the max residual
   (`ticks-scan.txt` is the raw scan output). "Frame top = axis max" is refused.
2. **Every coordinate is traceable.** `charts.json` holds, per chart, the tick
   rows with their labels, the baseline row, and each group's bar walls (from a
   baseline column scan). `digitise.py` prints the raw pixel coordinates of each
   bar top and error-bar cap it uses (`out.json`), with the image sha256.
3. **Bars are hollow outlines**, so ticks, bar tops and caps are all measured at
   **line centres**.
4. **Only old (control) and old + D + Q (treated) are digitised** — rule 8's
   ratio is treated/control. Young is not in the ratio and is not read.
5. **Fallback per panel.** If a bar or cap cannot be located under the rule, that
   panel's size = `no_mapping`, direction kept (rule 7). No other judgement.

## Cap rule history (disclosed; all written before any A78 value existed)

The error-bar cap is the measurement that sets the SEM, and in this figure data
dots overlap the caps. The cap-location rule was revised twice by inspecting
**pixel structure only** (ASCII dumps of the colour classes), never A78's
values, which do not yet exist:

- **v1** (solid + isolated row): rejected every cap, because a data dot touches
  each one → all panels `no_mapping`. Too strict.
- **v2** (contiguous run through the whisker column, bounded above and below):
  returned the first bounded run, which is sometimes a dot, not the cap.
- **v3 (current):** among all 2–5 row bounded runs (extent 15–70 % of bar width,
  centred ±15 %, the run through the centre on the rows just above and below < 50 %
  of the cap extent), take the **widest** (ties: uppermost). A dot run is
  narrower than the cap, so the cap wins; if no bounded run qualifies → `no_mapping`.

## OCR disclosure

While fetching the image, the Datalab OCR engine emitted its own approximate bar
values in prose. They were **not used, not committed, and not passed to A78**;
this reading comes only from the pixel scans above. Recorded here per A50's
ruling condition 4.

## Result (committed; compared to A78's reading under the rules-commit tolerance)

Ratios are D + Q / old (treated/control); interval = delta method on the log
ratio, mean ± t(0.975, n−1)·SEM, n from the caption.

| Panel | Result | ratio D+Q/old | 95% interval | max tick residual |
|---|---|---|---|---|
| 1b p16 mRNA | ok | 0.248 | 0.163 – 0.377 | ≤ 0.004 |
| 1b p21 mRNA | ok | 0.509 | 0.379 – 0.683 | ≈ 0 |
| 1d p16/p21 area % | ok | 0.204 | 0.130 – 0.321 | ≤ 0.013 |
| 1d positive cells/field | **no_mapping** (cap D+Q not located) | — | — | ≤ 0.008 |
| 1f SA-β-gal area % | ok | 0.557 | 0.297 – 1.044 | ≤ 0.023 |

Direction in every panel: D + Q lowers the senescence/area marker vs old
(ratio < 1), as the paper reports. `1d positive cells/field` is `no_mapping` on
size only; its direction is kept.

## Files

- `digitise.py` — the digitiser (asserts the image hash, converts coords to values).
- `ticks.py` — the tick/axis/baseline scanner. `ticks-scan.txt` — its raw output + the image sha256.
- `charts.json` — the per-chart inputs (tick rows+labels, baseline, bar walls).
- `out.json` — the full output: every raw pixel coordinate, calibration, means, SEMs, ratios.
- `barscan.py` — the audit scanner used to read the bar/cap pixel structure (ASCII).

Re-run: put the pinned `fig1.png` (sha256 above) in this folder and
`python digitise.py charts.json`.
