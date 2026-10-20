# Lifespan table: row schema

Mission L1 (charter part 6, `docs/charter/charter.md` @566e350). Specs: T5-A2-2 (standard extraction), T1-A2-2 (negative controls), T1-A2-3 (filter flags), T1-A2-5 (standard error source), T2-A2-4 (class labels), in `arcanum/session-1/topics/`. Verification ruling: `verification/approvals/l1-schema.md` (C1–C8), applied here.

**Status:** schema approved with changes by Verification; this version applies C1–C8 and awaits re-check. Once approved, the filter, `reference-ranges.yaml` and `class-labels.yaml` are sealed with the test-2 split (T1-A10-1, D115); changing them afterwards makes the affected items exploratory.

## Files

| File | Content |
|---|---|
| `itp/<cohort_year>-<compound>-<dose>-<start>-<sex>.yaml` | One ITP arm per file. |
| `other/…` | Non-ITP studies (second L1 worker, later), same schema. |
| `reference-ranges.yaml` | Control-lifespan reference ranges per strain and sex (C1). |
| `class-labels.yaml` | Closed class vocabulary and one label per compound (C4–C6). |
| `check.rb` | Validates rows and computes `test2_main` (C3). Run: `ruby literature/lifespan/check.rb`. |

An **arm** is one treated group compared with its own controls: one compound, one sex, one dose, one start age, one cohort year. Example: `itp/2005-rapamycin-14ppm-600d-f.yaml`. The pooled-sites result is the main row; per-site results go in `sites`.

## Rules

1. **No invented values.** A number not printed in the paper (or its supplement, or a cited public dataset) is `null`. A missing standard error stays `null`.
2. **Every number has a source location** (`table 1`, `fig 2`, `text`, `supp table S1`, or an MPD file id).
3. **Numbers computed by us** (e.g. medians from MPD per-mouse files) say so in `source` (`computed from MPD ITP1 file <id>`), never `printed`.
4. **Anything not checked against its source** is listed in `verification.to_verify`.
5. **Raw data is never committed** if over 10 MB. MPD survival data is referenced by file id; Platform fetches it by manifest (`data/datasets/`).
6. **Days, not months**, for every age and lifespan.

## Fields

| Field | Type | Meaning |
|---|---|---|
| `id` | string | File name without `.yaml`. |
| `program` | `ITP` \| `other` | ITP = NIA Interventions Testing Program. |
| `publication.doi`, `publication.citation` | string | DOI checked on Crossref (`verification.doi_checked`). |
| `cohort_year` | int | ITP cohort (`C2005` → 2005), from the MPD ITP1 listing. |
| `compound` | string | Lower-case name; must have an entry in `class-labels.yaml`. |
| `role` | `extends` \| `null` \| `shortens` \| `mixed` | Observed direction, pooled sites, log-rank p < 0.05. `mixed` = significant in opposite directions at different sites. Used for the test-2 proportions (T1-A2-4). |
| `strain`, `sex`, `start_age_days` | string, `F`\|`M`, int | `UM-HET3` for ITP. |
| `dose.value`, `dose.unit`, `dose.route` | number, string, string | As printed (`14`, `ppm`, `diet, encapsulated`). |
| `n.treated`, `n.control` | int \| null | Mice entering analysis. |
| `lifespan.treated` / `lifespan.control` | block | `median_days`, `mean_days`, `p90_days` (age at the 90th percentile of deaths), each `{value, source}` or `null`. |
| `effect.median_pct`, `effect.p90_pct` | number \| null | (treated − control) / control × 100, always computed from the printed medians / p90 ages (1 decimal). When the paper prints a % that differs after rounding, the printed % goes in `notes`. |
| `effect.se`, `effect.se_scale` | number \| null, scale | See "Standard error" below. |
| `effect.se_source` | `printed` \| `from_curve` \| `from_p_value` \| null | T1-A2-5. |
| `effect.digitisations` | int | Independent digitisations behind a `from_curve` SE (0 otherwise). |
| `effect.curve_agreement` | `pass` \| `fail` \| null | C7: result of the two-digitisation agreement check (T5-A50-3, D93). `null` if not run. |
| `effect.curve_check_ref` | string \| null | C7: path or PR of that check's record. |
| `p_values` | list | `{test, value, source}` as printed; inequalities (`< 0.0001`) kept as strings. |
| `curve_source` | block | `figure`, `mpd_id` (MPD ITP1 file id, or null). |
| `flags.control_in_ref_range` | bool | T1-A2-3 (a), C1: pooled-site control median inside the strain-and-sex range of `reference-ranges.yaml`, own cohort excluded. `false` while the range or the median is unknown. |
| `flags.reference` | string | `reference-ranges.yaml@<version>` used for the flag. |
| `flags.itp_or_replicated` | bool | T1-A2-3 (b). `true` for ITP rows. Non-ITP (C2): `true` only if a second independent lab found the same compound, same sex, same direction at p < 0.05; that row is named in `flags.replicated_by`. |
| `flags.replicated_by` | string \| null | Row id of the replication (non-ITP only). |
| `sites` | list | Per site (`TJL`, `UM`, `UT`): `median_days`, `p90_days`, effects, `p_values`. |
| `notes` | string | Caveats the authors state. |
| `extraction` | block | `by`, `date`; `second_by`, `second_date` for the double-extraction sample (T5-A2-2, D51/D63, sample drawn by Verification). |
| `verification` | block | `doi_checked` (date), `numbers_checked` (`paper` \| `secondary`), `to_verify` (field paths). |

Class labels are **not** in the rows: they are per compound, in `class-labels.yaml`.

## Standard error (T1-A2-5, C8)

`se_scale` is one of:

| Scale | Effect | Conversion when not printed |
|---|---|---|
| `log_hr` | ln(hazard ratio), treated vs control | `from_p_value`: SE = \|ln HR\| / z, z = Φ⁻¹(1 − p/2), two-sided p. Needs a printed HR and an **exact** p. |
| `log_median_ratio` | ln(treated median / control median) | From a printed 95% CI of the ratio: SE = (ln U − ln L) / (2 × 1.96). `from_curve`: by bootstrap of the reconstructed data (Guyot). |

`from_p_value` requires an exact p. A printed inequality (`< 0.0001`) gives no SE: `se` stays `null`.

## `test2_main` (C3)

Not stored in the rows. `check.rb` computes it:

```
test2_main =
     flags.control_in_ref_range == true
 and flags.itp_or_replicated == true
 and verification.to_verify is empty
 and class-labels.yaml has an approved label for compound   (approved_by set)
 and effect.se is not null
 and ( effect.se_source == printed
    or ( effect.se_source == from_curve
         and effect.digitisations >= 2
         and effect.curve_agreement == pass ) )
```

`from_p_value` and single-digitisation `from_curve` rows go to the sensitivity runs only. Rows failing the formula stay in the table.

## Control lists

`controls.md` (later PR) lists, from these rows:

- **negative controls** (T1-A2-2): ITP arms with `role: null`, per strain, sex and dose, with their observed interval;
- **positive controls** (T5-A2-3): arms with `role: extends` (rapamycin, acarbose, 17α-estradiol, NDGA, canagliflozin…), with their interval. Lifespan-shortening controls are mostly non-ITP and come with the second L1 worker.
