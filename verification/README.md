# verification/

Owned by **Verification** (A50). Verification is independent: it checks the work of the other units and reports to the president (council decision D47). A1 does not overrule a Verification finding; only the president can, in writing.

## Layout

| Folder | Contents |
|---|---|
| `references/` | Reference cases for astra-stats (known inputs, printed or hand-computed outputs). |
| `prep/` | Drafts written before launch (D67). See `prep/README.md`. |
| `seals/` | Sealed protocols and ledger criteria (later). |
| `audits/YYYY/MM/` | Audit reports, one folder per month (later). |
| `incidents/` | Incident files (later). |

Each subfolder gets its own README when it is created.

## Write rule

Only Verification writes in this folder. A CI rule to enforce this is pending from A10. Until then it is a convention.

Text files only (Markdown, YAML, CSV). No generated or binary files.
