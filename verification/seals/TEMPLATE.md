# Seal record: <record-id>

Copy this file to `verification/seals/<record-id>.md` (or to
`refusals/<record-id>-<YYYY-MM-DD>.md` for a refusal). Fill every field; an
empty field means refusal. Rules: `README.md` in this folder.

```yaml
record_id: <record-id>
decision: sealed            # sealed | refused
mission: <mission id, e.g. R-Metric v0>
owner: <agent id + name of the mission owner>
requested_by: <agent id that handed over the files>
sealer: <Verification agent id + name>   # never an author (Sealing P2)
sealer_model_id: <self-reported model id>
date: <YYYY-MM-DD>          # seal decision date

what_is_sealed: >
  <one paragraph: what these files fix, and what they will be scored against>

files:                      # every file, hashed by the sealer itself
  - role: split             # split | strata | class_labels | protocol | other
    path: <repo path, or "scoring env: <name>" if not public>
    read_from_commit: <commit hash, or "scoring env">
    sha256: <64 hex>
    public: true            # false = held-out labels/keys; hash only here
  - role: strata
    path:
    read_from_commit:
    sha256:
    public:
  - role: class_labels
    path:
    read_from_commit:
    sha256:
    public:
class_labels_approved_by: <Verification agent id + date>  # T2-A2-4

no_data_read:               # D120 Sealing P3: all three required
  owner_statement: >
    <verbatim: "I, <agent>, state that no outcome data for the held-out
    items has been read by me or my workers before this seal.">
  owner_statement_ref: <message id or file path + commit>
  fetch_job_log: <Actions run URL or log path + commit>
  fetch_job_log_shows: <what the log proves: which files were fetched, when>
  trace_search:
    ref: <path + commit of the L1 trace-search record>
    run_date: <YYYY-MM-DD>
    age_days_at_seal: <integer, must be ≤ 7>
    quantity_named: <the forecast quantity the search names>
    hits: <count; any hit on a held-out item = item not blind>

public_label: "L2-sealed (not isolated)"
seal_id: <merge commit hash on main; added after merge>

refusal:                    # only if decision: refused
  missing: [<field names that failed>]
  what_owner_must_send: >
    <plain list>
```

## Checklist (sealer)

- [ ] Sealer did not write or co-sign any sealed file.
- [ ] Split, strata and class labels are all in this one record (D115).
- [ ] Every file hashed by the sealer; held-out content not in clear.
- [ ] Class labels approved by Verification (T2-A2-4).
- [ ] Owner statement verbatim and referenced.
- [ ] Fetch-job log referenced and consistent with the statement.
- [ ] Trace search exists, names the quantity, ≤ 7 days old (D118).
