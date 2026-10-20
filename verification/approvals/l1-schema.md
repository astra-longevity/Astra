# L1 lifespan table schema: Verification ruling

Owner: Verification (A50); ruling by A65. Reviewed: PR #42,
`literature/lifespan/schema.md` and the example row
`itp/2005-rapamycin-14ppm-600d-f.yaml`, head `1fa852c`. Date: 2026-10-20.

**Verdict: approve with changes.** The draft may merge as a draft. No row
enters test 2, and no class label enters the D115 seal, until changes C1–C8
below are on `main` and re-checked by Verification.

## 1. Filter flags (T1-A2-3)

| Flag | Mechanical as written? | Change required |
|---|---|---|
| `control_in_ref_range` | **No.** "Within the reference range" names no range, statistic or version, so two extractors can disagree. | **C1.** Add `literature/lifespan/reference-ranges.yaml`: one entry per strain × sex with the source (ITP control cohorts or MPD dataset ids), the statistic (e.g. min–max of pooled control medians), bounds inclusive or not, and a `version`. The row's `reference` cites `reference-ranges.yaml@<version>`. A row's own cohort is left out of the range it is tested against. State whether the pooled or per-site median is tested (the README says per site). `pending` counts as `false` for `test2_main`. |
| `itp_or_replicated` | Yes for ITP (`program: ITP` ⇒ `true`). No for other studies. | **C2.** For `program: other`: `true` only if a second, independent lab tested the same compound in the same sex with the same direction at p < 0.05; the row names it in `replicated_by: <row id>`. |
| `test2_main` | Almost. | **C3.** Write the formula in the schema: `test2_main = control_in_ref_range == true AND itp_or_replicated == true AND to_verify == [] AND (se_source == printed OR (se_source == from_curve AND curve_agreement == pass))`. It is computed by a check, never set by hand. |

## 2. Class labels (T2-A2-4)

Rule: a class is fixed from mechanistic evidence, before any lifespan result
is seen, with a source per label. These labels go into the R-Metric v0 /
R-Sim seal record (D115), so this part binds the seal.

The example fails it: its `class_label.source` is the lifespan paper itself
(Harrison 2009 discussion and fig 3B). Changes:

- **C4. Source independent of lifespan.** Each label cites a mechanism
  source that reports no lifespan outcome: a pharmacology paper, or a
  ChEMBL / DrugBank / IUPHAR mechanism entry with its snapshot date. A
  lifespan paper may not be the source.
- **C5. Fixed vocabulary.** A closed list of classes, each defined by its
  primary molecular target or pathway (e.g. `mTORC1 inhibitor`,
  `alpha-glucosidase inhibitor`, `SGLT2 inhibitor`, `estrogen receptor
  agonist`). A compound with several targets gets the target named first by
  the mechanism source; a tie is labelled `multi-target` and the tie is
  written down. New classes are added before the split, never after.
- **C6. One label per compound, in one file.** Move labels out of the arm
  rows into `literature/lifespan/class-labels.yaml` (`compound`, `class`,
  `source`, `source_date`, `proposed_by`, `approved_by`, `approved_date`).
  All arms of a compound share it. This file is what the seal hashes, so a
  correction to an arm row (n, medians) does not change the seal hash.
  `approved_by` is filled by Verification (A65 for this seal), not by L1.

## 3. Standard-error source (T1-A2-5)

The rule in the schema is correct: `from_curve` counts in the main run only
after two digitisations that passed the agreement check (T5-A50-3, D93);
one digitisation or `from_p_value` = sensitivity only. Changes:

- **C7. Agreement recorded, not inferred.** `digitisations: 2` does not
  prove agreement. Add `effect.curve_agreement: pass | fail | null` and
  `effect.curve_check_ref` (path or CI run of the build's agreement check).
- **C8. Usable p-values and scales.** `from_p_value` needs an exact p-value;
  a bound such as `< 0.0001` gives no SE and the field stays `null`. Fix the
  list of allowed `se_scale` values (`log hazard ratio`, `median_pct`,
  `p90_pct`) and the conversion used for each.

## Re-check

Verification re-checks the follow-up PR against C1–C8 and records the result
here (add-only, below this line).
