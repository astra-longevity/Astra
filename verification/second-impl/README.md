# Second implementations

One YAML file per reference case. Each file tells the R workflow (`.github/workflows/stats-r.yml`, owned by A10) which package to run, how to call it, which inputs to feed it, and how to compare its output with our reference values (rules D6, D38, D106).

The workflow reads these files and posts results as artifacts and a PR comment. It never writes to `verification/`. Verification agents commit the results.

## Fields

| Field | Meaning |
|---|---|
| `case` | Case id, same as the file name. |
| `reference` | Folder under `verification/references/` with the reference values. |
| `reference_kind` | `hand-computed`, `printed`, or `2-implementation agreement` (no printed reference exists). |
| `package` | R package `name`, exact `version`, and `source` (CRAN or GitHub with a commit). |
| `inputs` | Input file and the rows to use. Inputs are text files in the repo. |
| `call` | Exact R code to run. It must produce the objects named in `outputs`. |
| `outputs` | Package output name → reference quantity name. |
| `expected` | File and rows holding the values to match. |
| `tolerance` | Absolute difference allowed. Left empty when Verification has not set it yet (Guyot, D38). |
| `notes` | Pitfalls that would make a correct package give a wrong answer. |

## Cases

- `coloc-a.yaml`, `coloc-b.yaml`, `coloc-c.yaml`: `coloc::coloc.abf` against the hand-computed cases in `references/coloc/`.
- `mr-ldl-chd.yaml`: IVW, MR-Egger and weighted median in MendelianRandomization and TwoSampleMR on the same 28 variants. No printed reference exists, so this tests agreement between the two packages. Vignette values are a smoke test only.
- `guyot-bonner.yaml`: IPDfromKM on Bonner 2006 (curves shipped with the package), compared with the values printed in Bonner. Tolerance left empty (D38).
- `guyot-icon7.yaml`, `guyot-goss.yaml`: same method, `status: blocked`.

## Gaps

- ICON7 and Goss curves are not digitised. Each needs two curve CSVs and an at-risk CSV under `verification/references/guyot/`, as listed in its YAML.
- MR tolerances are proposed by the author of the spec; Verification confirms them.
- A `status: blocked` case is skipped by the workflow and reported as skipped.
