# research/obstacles/: obstacle register

One file per obstacle (charter part 9, T8-A28-1, T8-A10-1):
`research/obstacles/<id>.yaml`, flat `key: value` lines.

| Field | Meaning |
|---|---|
| `id` | the file name without `.yaml`, lowercase-dashed (e.g. `o13-run-manifests`) |
| `title` | one line |
| `owner` | agent id |
| `exit` | the measurable exit, one line |
| `check` | how the exit is checked: a shell command run from the repo root (exit 0 = met), `job:<CI job name>`, or `judge:<agent>` (needs `evidence:`) |
| `date` | `YYYY-MM-DD`; relative dates such as "month 6" are refused (an obstacle without a date counts as unowned, T8-A1-2) |
| `status` | `open`, `met` or `failed` (only Verification marks `met`, T8-A50-2) |
| `judge`, `evidence`, `note` | optional |

Never move an exit date after the fact (T8-A28-1).

## The daily job

The `Registers` workflow runs every day at 06:37 UTC (and on demand):

```sh
python3 scripts/registers.py obstacles            # --today YYYY-MM-DD to simulate
```

For each `open` obstacle whose date has passed, it runs the check. A failing
check is reported `FAILED`; a `job:` or `judge:` check is reported `NEEDS
CHECK` (it cannot run by itself). The report is issue-ready markdown in the
run summary, and the run goes red. The job never writes to the repo: the
owner or A50 opens the issue in `issues/`.

On every PR only the schema is checked (`--schema-only`).
