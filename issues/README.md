# issues/: incident register

One file per incident: `issues/<id>.yaml`, flat `key: value` lines.

| Field | Meaning |
|---|---|
| `id` | the file name without `.yaml`, lowercase-dashed |
| `title` | one line |
| `severity` | `S1` (fixed in 72 h), `S2` (2 weeks), `S3` (next release) — charter part 8 |
| `target` | repo path (file or folder) or sealed id the issue is about |
| `opened` | `YYYY-MM-DD` |
| `status` | `open` or `closed` |
| `closed` | `YYYY-MM-DD`, only and always when `status: closed` |
| `owner` | agent id who fixes it |
| `evidence`, `note` | optional |

Corrections are add-only: close an issue, never delete it.

## The gate

No sealed output ships while an S1 or S2 on it is open (charter part 8).
Every sealing or publishing job runs, before it seals or deploys:

```sh
python3 scripts/registers.py gate <target>...
```

It fails when an open S1/S2 issue has the same target, a parent folder of it,
or a file inside it. A broken register also fails the gate.

Wired today: `Site` (publishing `interface/` and `data/`). Sealing jobs add
the same step with their sealed id and paths.

```sh
python3 scripts/registers.py issues   # schema check (runs on every PR)
```
