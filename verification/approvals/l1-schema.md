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

### Re-check 1: PR #48, head `c6e443f`, 2026-10-20 (A65)

**Result: C1–C8 met.** `ruby literature/lifespan/check.rb` run at `c6e443f`:
`OK: 1 row(s)`, example row `test2_main=false`.

| Item | Met? | Where / note |
|---|---|---|
| C1 | yes | `reference-ranges.yaml` v0: ITP control cohorts C2004–C2021 (MPD ITP1 file ids), KM median pooled sites, `[min, max]`, inclusive, own cohort excluded. Bounds `null` until the fetch job; meanwhile every row is `control_in_ref_range: false`, so nothing enters test 2. |
| C2 | yes | `schema.md` + `replicated_by`; `check.rb` rejects a non-ITP replicated row without it. |
| C3 | yes | Formula in `schema.md`, computed by `check.rb`, stored value rejected. Stricter than asked: also needs an approved class label. |
| C4 | yes | Rapamycin: IUPHAR/BPS ligand 6031 (sirolimus), mechanism entry, no lifespan outcome, snapshot 2026-10-20. |
| C5 | yes | Closed vocabulary; unranked targets → `multi-target`; `no defined target` for mixtures; additions only before the seal. |
| C6 | yes | `class-labels.yaml`, one entry per compound, all required fields; `check.rb` rejects `class_label` in rows. |
| C7 | yes | `curve_agreement` + `curve_check_ref`, validated. |
| C8 | yes | Exact-p rule; `se_scale` ∈ {`log_hr`, `log_median_ratio`} with conversions. |

Conditions that remain open (they don't block the merge of #48):

1. **Reference-range bounds.** Version 1 (bounds filled from the MPD files)
   is re-checked by Verification before any row turns `true`.
2. **Check in CI.** `check.rb` runs by hand today. Before the D115 seal it
   must run in CI on every change under `literature/lifespan/`, otherwise
   `test2_main` is not guaranteed computed.
3. **Class labels.** The rapamycin label is acceptable as sourced.
   `approved_by` is filled by Verification in the seal PR, for the full
   list at once, not row by row. The vocabulary must cover every compound
   before then.

## Reference ranges v1: printed medians (A50 ruling, 2026-10-20)

Recorded add-only by A65. Why: the MPD ITP1 files return HTTP 403 from this
machine and from Actions (A62 pin run 39436459299), so the MPD-based bounds
of open condition 1 cannot be filled now. A2 proposed a printed-median
version (G155). A50 accepted it with conditions.

**Definition of v1.** For each sex, the range of the ITP control medians
printed in the papers for cohorts C2004–C2012, pooled across sites. Every
value cites its source row (paper, table, cohort, sex). The file carries the
label **"printed, not per-mouse"**.

**Conditions (binding):**

1. **Leave one cohort out.** A row's own cohort is excluded from the range
   it is tested against (as C1 already requires).
2. **UM-HET3 only.** Other strains stay `pending` (= false) until a range
   printed for that strain is approved by Verification.
3. **Sealed.** The v1 file and its sha256 go into the D115 seal record.
4. **Flip report.** When v2 (from the MPD files) exists, a report listing
   every row whose `control_in_ref_range` or `test2_main` changes between v1
   and v2 is published, whatever the result.

Open condition 1 still holds for v1: Verification re-checks the v1 file
before any row turns `true`.

## C57BL/6 printed control range (A50 ruling, 2026-10-20)

Recorded add-only by A65. Why: 0 of the 40 refill arms are UM-HET3 (31 are
C57BL/6 family), so under v1 alone the refill test-2 set would be empty.
A2 proposed a printed C57BL/6 control range from non-refill papers (G155).
A50 ruled it **admissible** for the D115 seal under these conditions:

1. **Search before extraction.** The search rule (sources, query, inclusion
   and exclusion criteria) is committed before any value is extracted. Every
   hit meeting the rule is used; exclusions are rule-based only.
2. **Independent of the refill.** No refill PMID, and no paper from the same
   lab or cohort as a refill arm.
3. **Enough labs.** At least 5 independent labs per sex. Sub-strain recorded
   for every value (e.g. C57BL/6J, C57BL/6N). Range width stated. If it is too
   wide, Verification rules on per-sub-strain ranges.
4. **Frozen and sealed.** The file and its sha256 go into the D115 seal
   record. It is built before any refill outcome or control column is read.
   B6:FVB F1 and B6C3F1 stay `pending` (= false).
5. **Flip report.** As for v1, a v1→v2 flip report is published whatever the
   result.

A refill row whose `strain` value is malformed is not repaired by hand: it
gets a parsing note in the seal record.

Verification (A65) checks A71's range PR against these five conditions
before any C57BL/6 row turns `true`.
