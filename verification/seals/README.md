# Seals

Seal records written by Verification. Text only. A seal fixes a set of
files (by hash) before anyone reads the data they will be scored on.

## Rules this folder applies

- **Only Verification seals, never an author** (D120, Sealing P2). The
  sealer is never an agent who wrote or co-signed what is sealed.
- **One record for split, strata and classes** (D115, T2-A10-5). For a
  scored model, the train/test split, the stratum of every held-out item
  and the class label of every intervention are hashed together in one
  record. Changing any one of them needs a new seal; a new seal makes every
  earlier score on the affected items "exploratory".
- **"No data read" evidence** (D120, Sealing P3): the owner's written
  statement, plus the fetch-job log, plus the dated public-trace search. A
  false statement is an S1 incident.
- **Trace search ≤ 7 days old** (D118, T5-A2-4, T5-A10-4). Literature (L1)
  runs a dated, recorded search (PubMed, bioRxiv, meeting abstract books of
  the past 2 years, the ITP site) naming the quantity forecast. Older than
  7 days on the seal date = refused.
- **Seal id** (D120, Sealing P1): the merge commit hash on `main` of the PR
  that adds the record. It is written into the record by a follow-up
  add-only commit; the record is never otherwise edited.
- **Public label** until isolated timestamping exists: "L2-sealed (not
  isolated)" (D105).
- **Refusals are recorded** (D120, Sealing P5), with every missing item named.

## Hashes and secrets

- Hash = SHA-256 of the exact file bytes (`shasum -a 256 <file>`), with the
  file path and the commit it was read from.
- Held-out labels and keys never appear in this repo in clear: only their
  hash. The clear file stays in the `scoring` environment (D117).
- A file that can't be hashed by the sealer (not shown, or only described)
  can't be sealed.

## Layout

- `TEMPLATE.md`: copy it for each seal record.
- `<record-id>.md`: one sealed record, e.g. `r-metric-v0-split.md`.
- `refusals/<record-id>-<YYYY-MM-DD>.md`: one refused request, same template
  with `decision: refused` and the missing items.

## Steps

1. The owner asks Verification for a seal and hands over: the files, the
   owner statement, the fetch-job log reference, the trace-search reference.
2. The sealer checks every field of `TEMPLATE.md`. One missing field = refusal.
3. The sealer hashes every file itself (never copies the owner's hashes),
   writes the record, opens a PR with prefix `verification:`.
4. After merge, the merge hash is added as the seal id.
