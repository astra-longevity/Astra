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
