# Lifespan table: row schema

Mission L1 (charter part 6, `docs/charter/charter.md` @566e350). Specs: T5-A2-2 (standard extraction), T1-A2-2 (negative controls), T1-A2-3 (filter flags), T1-A2-5 (standard error source), T2-A2-4 (class labels), in `arcanum/session-1/topics/`.

**Status: draft for Verification (A50) approval.** A50 approves the filter (flags), the class labels and the control lists. Once approved, the filter and the class labels are sealed with the test-2 split (T1-A10-1); changing them afterwards makes the affected items exploratory.

## One file per arm

An **arm** is one treated group compared with its own controls: one compound, one sex, one dose, one start age, one cohort year. Each arm is one YAML file:

```
literature/lifespan/itp/<cohort_year>-<compound>-<dose>-<start>-<sex>.yaml
```

Example: `itp/2005-rapamycin-14ppm-600d-f.yaml`. Non-ITP studies (second L1 worker, later) go in `literature/lifespan/other/` with the same schema.

The pooled-sites result is the main row. Per-site results go in the `sites` list of the same file, because the T1-A2-3 filter is checked per site.

## Rules

1. **No invented values.** A number not printed in the paper (or its supplement, or a public dataset we cite) is `null`. A missing standard error stays `null`.
2. **Every number has a source location** (`table 1`, `fig 2`, `text p. 393`, `supp table S1`).
3. **Anything not checked against the paper itself** is listed in `verification.to_verify`. A row with a non-empty `to_verify` cannot enter test 2.
4. **Raw data is never committed** if it is over 10 MB. Survival data from the Mouse Phenome Database is referenced by its dataset id in `curve_source`; Platform fetches it by manifest (`data/datasets/`).
5. **Days, not months**, for every age and lifespan.

## Fields

| Field | Type | Meaning |
|---|---|---|
| `id` | string | Same as the file name without `.yaml`. |
| `program` | `ITP` \| `other` | ITP = NIA Interventions Testing Program. |
| `publication.doi` | string | Checked on Crossref (`verification.doi_checked`). |
| `publication.citation` | string | First author, year, journal, volume:pages. |
| `cohort_year` | int | ITP cohort (year the mice were born, e.g. `C2005`→2005). |
| `compound` | string | Lower-case name, as in the paper. |
| `role` | `extends` \| `null` \| `shortens` \| `mixed` | Observed direction, pooled, at p < 0.05 (log-rank). Used for the test-2 proportions (T1-A2-4). `mixed` = significant in one direction at some sites, not others. |
| `strain` | string | `UM-HET3` for ITP. |
| `sex` | `F` \| `M` | One sex per row. |
| `start_age_days` | int | Age at first dose. |
| `dose.value`, `dose.unit`, `dose.route` | number, string, string | As printed: e.g. `14`, `ppm`, `diet, encapsulated`. |
| `n.treated`, `n.control` | int \| null | Mice entering analysis. |
| `lifespan.treated` / `lifespan.control` | block | `median_days`, `mean_days`, `p90_days` (age at 90th percentile of deaths, ITP's "maximum lifespan"), each `{value, source}`. |
| `effect.median_pct` | number \| null | (treated − control) / control × 100, from printed medians. |
| `effect.p90_pct` | number \| null | Same for the 90th percentile. |
| `effect.se`, `effect.se_scale` | number \| null, string | Standard error of the effect and its scale (e.g. `log hazard ratio`, `median_pct`). |
| `effect.se_source` | `printed` \| `from_curve` \| `from_p_value` \| null | T1-A2-5. `from_curve` counts in the main test-2 run only if the curve was digitised twice and passed the agreement check (T5-A50-3, D93); a single digitisation is `sensitivity` only. `from_p_value` is sensitivity only. |
| `effect.digitisations` | int | Number of independent digitisations behind a `from_curve` SE (0 otherwise). |
| `p_values` | list | `{test, value, source}`: log-rank, Wang-Allison 90th percentile test, etc., as printed (`< 0.0001` kept as a string). |
| `curve_source` | block | `figure` (where the survival curve is), `mpd_id` (Mouse Phenome Database dataset, or null). |
| `flags.control_in_ref_range` | `true` \| `false` \| `pending` | T1-A2-3 (a): control median within the strain-and-sex reference range. `reference` cites the range used (ITP pooled controls / MPD). |
| `flags.itp_or_replicated` | bool | T1-A2-3 (b). Always `true` for ITP rows. |
| `flags.test2_main` | bool | Derived: both flags true, `to_verify` empty, and SE usable in the main run. Rows that fail stay in the table and feed the sensitivity runs only. |
| `class_label.proposed`, `class_label.source` | string, string | T2-A2-4: class by primary molecular target (e.g. `mTORC1 inhibitor`), with the reference for the target. `class_label.approved` set by A50. |
| `sites` | list | Per-site `{site, median_days, p90_days, effect_p90_pct, p_values}`; sites are `TJL`, `UM`, `UT`. |
| `notes` | string | Caveats the authors state (e.g. pre-treatment survival differences). |
| `extraction` | block | `by` (agent id), `date`; `second_by`, `second_date` when the row is in the double-extraction sample (T5-A2-2, D51/D63 audit, sample drawn by A50). |
| `verification` | block | `doi_checked` (date), `numbers_checked` (`paper` \| `secondary`), `to_verify` (list of field paths still unchecked). |

## Control lists

`controls.md` (next PR) lists, from these rows:

- **negative controls** (T1-A2-2): ITP arms with `role: null`, per strain, sex and dose, with their observed interval;
- **positive controls** (T5-A2-3): arms with `role: extends` (rapamycin, acarbose, 17α-estradiol, NDGA, canagliflozin…), with their interval. Lifespan-shortening controls are mostly non-ITP and come with the second L1 worker.
