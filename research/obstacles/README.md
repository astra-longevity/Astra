# research/obstacles/: obstacle register

One file per obstacle (charter part 9, T8-A28-1, T8-A10-1):
`research/obstacles/<unit>/<id>.yaml`, flat `key: value` lines. Sub-folders
group rows by owner unit (`research/`, `literature/`, `platform/`,
`verification/`, `vp/`, `database/`) because a folder holds at most 8
entries. The loader reads every sub-folder; ids are unique across all of them.

| Field | Meaning |
|---|---|
| `id` | the file name without `.yaml`, lowercase-dashed (e.g. `o13-run-manifests`) |
| `title` | one line |
| `owner` | agent id |
| `exit` | the measurable exit, one line |
| `check` | how the exit is checked: a shell command run from the repo root (exit 0 = met), `job:<CI job name>`, or `judge:<agent>` (needs `evidence:`) |
| `date` | `YYYY-MM-DD`, or `null` with `status: unowned`; relative dates such as "month 6" are refused (T8-A1-2) |
| `status` | `open`, `met`, `failed` or `unowned` (only Verification marks `met`, T8-A50-2) |
| `milestone`, `milestone_date` | optional, together: a dated intermediate milestone (T8-A28-4) |
| `milestone_status` | `open` (default) or `met` |
| `judge`, `evidence`, `note` | optional |

Months in the charter count from its merge on 2026-10-20 (month 1 =
2026-11-20, month 3 = 2027-01-20, month 6 = 2027-04-20).

Never move an exit date after the fact (T8-A28-1).

## The daily job

The `Registers` workflow runs every day at 06:37 UTC (and on demand):

```sh
python3 scripts/registers.py obstacles            # --today YYYY-MM-DD to simulate
```

For each `open` obstacle whose date has passed, it runs the check. A failing
check is reported `FAILED`; a `job:` or `judge:` check is reported `NEEDS
CHECK` (it cannot run by itself). Every `unowned` row is reported `UNOWNED`
every day until it gets a date, and a past `milestone_date` not marked met is
reported `MILESTONE DUE`. The report is issue-ready markdown in the
run summary, and the run goes red. The job never writes to the repo: the
owner or A50 opens the issue in `issues/`.

On every PR only the schema is checked (`--schema-only`).
