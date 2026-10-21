# research/runs/: reproducible runs

One folder per run: `research/runs/<id>/` with `manifest.yaml`, the params
file and the committed outputs. The `Runs` workflow re-runs every manifest
on each PR and fails unless the outputs come back byte-identical.

## Manifest fields (`astra-run-manifest/1`)

| Field | Meaning |
|---|---|
| `schema` | `astra-run-manifest/1` |
| `id` | the folder name, lowercase-dashed |
| `description` | one line |
| `code_commit` | commit the run was recorded at (pointer; squash merges may drop it, so it is a warning only) |
| `command` | shell command, run in a temp folder holding the params file; must contain `{seed}`; `$ASTRA_ROOT` = repo root |
| `seed` | integer |
| `params_file`, `params_sha256` | params file in the run folder and its hash |
| `code` | repo path -> sha256 of every code file the run uses (binding pin) |
| `data` | repo path -> sha256 of every input data file (`{}` if none) |
| `toolchain` | tool -> version (`python3: 3.13.13`); a different minor version is a warning |
| `outputs` | output file -> sha256 |

Format: a YAML subset (top-level `key: value`, one level of nested maps, `{}`
for an empty map), parsed without dependencies.

## Commands

```sh
python3 scripts/runs.py check  research/runs/*/manifest.yaml
python3 scripts/runs.py rerun  research/runs/*/manifest.yaml
python3 scripts/runs.py record research/runs/<id>   # fills hashes from a draft
```

To add a run: commit the code first, write a draft `manifest.yaml` (list the
code, data, toolchain and output keys, any value), then `record` and commit.
Outputs must be deterministic: fixed seed, own RNG or pinned library,
rounded numbers.

Reference platform: Runs CI (ubuntu) is the reference output. macOS can
differ at ~1e-16 on libm-heavy runs (e.g. `sim-v0-joint-2026-10-20`), so a
local macOS `runs.py rerun` mismatch is not a failure; only Runs CI counts.

`toy-gompertz-v0` is a platform fixture, not science.
