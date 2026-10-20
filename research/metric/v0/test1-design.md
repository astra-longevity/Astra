# R-Metric v0: test 1 design (coverage with proof)

Status: **draft proposal by Research (A67 for A28)**. Not sealed. No data has
been read or scored for this file.
Cites: D114 (T1-A28-3 test 1, T1-A2-1, T1-A50-1, T1-A28-6, T1-A1-3); D118
(T5-A2-3, T5-A28-3); charter Part 1. Axis list: `axes.md`.

## What test 1 must show

For **each axis** in `axes.md`:

1. it moves toward "younger" under one intervention known to slow ageing in
   that system (right-way control), and
2. it moves toward "older" under one life-shortening intervention
   (wrong-way control).

An axis that lacks either result is published as a **blind spot, by name**
("unmeasured"). It is never dropped and never reported as "no change"
(D114). Only axes that pass test 1 can carry a "confirmed worsening" later
(T1-A1-3).

## Per-axis procedure

| Step | What | Who |
|---|---|---|
| 1 | Fix the axis readout (dataset, variable, tissue, age window) and its **expected direction** for each control, from mechanistic evidence, before any control data is read | Research (A67), cited sources |
| 2 | Fix the comparison: control-treated vs matched untreated mice, same strain, sex, age, site and batch | Research |
| 3 | Seal steps 1–2 plus the statistic below in the metric version manifest (T1-A28-5) | Verification (A50) seals, not Research |
| 4 | Fetch the data through a manifest (T5-A10-1); until A62's fetch job exists, every hand download goes in `manual-downloads.csv` | Research |
| 5 | Compute the statistic, publish pass/fail per axis, failures included | Research computes; Verification recomputes independently (T1-A50-4) |

## Statistic

For axis *k* and control *c*:

- **Effect**: standardised difference between treated and matched control
  mice on the axis reading, signed so that "older" is positive.
  Hedges' g with its 95% interval, or, for cross-sectional single-cell data
  (TMS), a pseudo-bulk per-mouse score so that the unit is the mouse, not the
  cell (cells from one mouse are not independent).
- **Noise**: every reading carries test-retest plus model uncertainty
  (T1-A50-1). The interval uses both.
- **Pass on one control**: the interval excludes 0 **in the expected
  direction**, under the family-wise rule A50 fixes at sealing (T1-A28-6 (b)).
  The number of tests is 30 axes × 2 controls, so an uncorrected 5% rule
  would pass about 3 axis-controls by chance. Proposal for A50: Holm across
  all axis × control pairs of the version. A50 decides.
- **Pass on the axis**: both controls pass.
- **Wrong-sign result** (the axis moves the wrong way under a control): the
  axis fails and an issue is opened (commandment 8). It is not averaged with
  the other control.

## Sealed baselines

An axis must show more than a generic "sick or treated mouse" signal. For
each passing axis, the same statistic is computed on the sealed baselines of
T1-A50-2 that apply at control level:

- **chronological age** (does the axis just read age? Controls are age-matched,
  so this must give 0; a non-zero result flags a matching error),
- **body weight** (does the axis just read weight? Several controls change
  weight, e.g. glycine lowers it in females, doi:10.1111/acel.12953).

Report: the axis effect, and the effect **after adjusting for body weight**.
An axis whose control effect disappears after weight adjustment is labelled
"reads body weight" and does not pass. Proposal; A50 fixes the rule.

## Choice of controls

- Right-way: prefer ITP arms (rapamycin doi:10.1038/nature08221, acarbose
  doi:10.1111/acel.12170, canagliflozin doi:10.1172/jci.insight.140019,
  glycine doi:10.1111/acel.12953), because their lifespan effect is
  replicated across 3 sites.
- Wrong-way: progeroid models (Ercc1−/Δ, Lmna G609G; refs in `axes.md`),
  high-fat diet and irradiation (`[to verify]`).
- A progeroid model is a valid wrong-way control for an axis only if the axis
  is part of its documented phenotype (issue I-3 in `axes.md`). Otherwise the
  pairing is labelled "weak control" on the scorecard.
- Controls used in test 1 are **removed from the test-2 test set** or
  labelled "used in test 1" there, so the same result is not counted twice.
  Rapamycin and acarbose will be in test 1, so they cannot be blind test-2
  items anyway (T1-A50-3).

## Data reality for v0 (honest status)

- Most per-axis effects of controls are **not in the public datasets** named
  in `axes.md`. TMS has no treated mice; FRL is untreated ageing; ITP gives
  lifespan and body weight. Per-axis control data must come from published
  papers (figures, digitised twice per T5-A50-3) or from GEO series of
  treated mice found by L2 (A70).
- Expected outcome of v0 test 1: **most axes listed as blind spots**. This is
  the correct result under commandment 6 and is the work list for L2 and for
  future mouse requests (commandment 10: proof of the gap first).

## Output (scorecard section)

One row per axis: readout, right-way control and effect (interval),
wrong-way control and effect (interval), weight-adjusted effects, pass /
fail / unmeasured, "weak control" flag, source of each number
(`dataset` | `from_figure` digitised twice). Failures are published.

## Open issues

- T1-I-1: per-axis control data mostly absent from public datasets (above).
- T1-I-2: the family-wise rule and the weight-adjustment rule are proposals
  for A50.
- T1-I-3: single-cell readouts need a per-mouse unit; TMS mice per age group
  are few `[to verify]`, so intervals will be wide.
