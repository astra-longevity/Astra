# Specification of `astra db build`

For: Rust Manager (A10). Format owner: Database Manager (A9).
Format version: **1.0**. Any mismatch between this document and the schemas
`data/schema/*.schema.json` / `schema.sql` is a bug to report to A9.

## 1. Purpose

Read every YAML file in `data/`, validate them, and produce
`data/astra.sqlite` (gitignored). The database is **disposable**: it is deleted
and fully rebuilt on every build. No migrations, no state outside the repository.

```
astra db build            # validate + write data/astra.sqlite
astra db build --check    # validate only, write nothing (CI, pre-commit hook)
astra db build --root P   # repository root (default: current folder, must contain data/)
```

## 2. Inputs

| Folder (relative to the root) | Type | Schema | Expected file name |
|---|---|---|---|
| `data/schema/evidence-levels.yaml` | scale | (see §4, R01) | — |
| `data/missions/*.yaml` | Mission | `mission.schema.json` | `<id>.yaml` |
| `data/sources/*.yaml` | Source | `source.schema.json` | `filename(id).yaml` |
| `data/concepts/*.yaml` | Concept | `concept.schema.json` | `<id>.yaml` |
| `data/claims/*.yaml` | Claim | `claim.schema.json` | `<id>.yaml` |
| `data/evidence/*.yaml` | Evidence | `evidence.schema.json` | `<claim>--filename(source).yaml` |
| `data/relations/*.yaml` | Relation | `relation.schema.json` | `<id>.yaml` |
| `data/questions/*.yaml` | Question | `question.schema.json` | `<id>.yaml` |
| `data/contradictions/*.yaml` | Contradiction | `contradiction.schema.json` | `<id>.yaml` |

- One file = one object (a YAML mapping at the top level). `.yaml` extension only;
  any other file in these folders (except `.gitkeep` and `README.md`) is error E002.
- Missing folder = zero objects (not an error).
- `data/examples/` is **ignored** by the normal build (see §8).
- UTF-8 encoding required, NFC. No sub-folders.
- YAML: safe subset (no anchors/aliases, no `!` tags, no duplicate keys).

**`filename(source_id)`**: DOI → replace `/` with `_`; any character outside
`[a-z0-9._()+;-]` is encoded as `%xx` (lowercase hex, UTF-8 bytes).
ISBN → `isbn:9780000000000` becomes `isbn-9780000000000`.
E.g. `10.1038/nature02517` → `10.1038_nature02517.yaml`.

## 3. Loading order

1. Run `schema.sql` on a **fresh in-memory** database (`PRAGMA foreign_keys=ON`).
2. Load `evidence-levels.yaml`.
3. Read and validate (JSON Schema) **all** files, every category.
4. Insert in this order: missions → sources → concepts (+ concept_synonyms)
   → claims (+ claim_concepts) → evidence → relations (+ relation_claims)
   → questions (+ question_concepts, question_resolved_by)
   → contradictions (+ contradiction_claims).
5. Check the cross-object rules (§4, E020 onwards).
6. Fill `build_info` (§6).
7. `VACUUM INTO` a temporary file `data/astra.sqlite.tmp`, then rename it
   atomically to `data/astra.sqlite`. On error, the old file is left untouched.

The build **collects every error** before stopping (do not stop at the first
one), except for the fatal errors E001/E003.

## 4. Validation rules

Format of each message (one line, on stderr):
`<level> <code> <file>[:<field>]: <message>`
with level `error` or `warning`. Messages in quotes "…" must be reproduced
exactly; `{x}` = interpolated value.

### Reading and shape

| Code | Level | Rule | Message |
|---|---|---|---|
| R01/E001 | fatal error | `evidence-levels.yaml` exists and contains exactly levels 1 to 6, with unique codes | "invalid evidence scale: {detail}" |
| E002 | error | unexpected file in a data folder | "unexpected file (expected: *.yaml)" |
| E003 | fatal error | `schema.sql` missing or fails to run | "invalid schema.sql: {sqlite error}" |
| E010 | error | unreadable YAML / not UTF-8 / anchor, tag or duplicate key | "invalid YAML: {detail}" |
| E011 | error | does not match the JSON Schema (missing field, unknown field, type, enum, pattern) | "{field}: {reason}" (one line per violation) |
| E012 | error | file name ≠ expected name (§2) | "the file should be named {expected}" |
| E013 | error | duplicate id within one category (including DOIs that differ only by case) | "id {id} already defined in {other file}" |
| E014 | error | non-canonical DOI: uppercase, `doi:` / `https://doi.org/` prefix, spaces | "non-canonical DOI, write {canonical form}" |
| E015 | error | duplicates in a list (concepts, claims, synonyms) | "{field}: duplicate value {v}" |

### References (no orphans)

| Code | Level | Rule | Message |
|---|---|---|---|
| E020 | error | every reference points to an existing id: claim.concepts, claim.organism, claim.mission, evidence.claim, evidence.source, relation.from/to/claims, question.concepts/mission/resolved_by, contradiction.claims/mission | "{field}: {id} does not exist" |
| E021 | error | `claim.organism` refers to a concept with `kind: organism` | "organism: {id} is not an organism (kind={kind})" |
| E022 | error | relation `from` ≠ `to` | "a relation cannot link a concept to itself" |
| E023 | error | no two relations with the same (from, to, type) | "relation identical to {other id}" |
| E024 | error | one Evidence per (claim, source) pair | "pair already described in {other file}" |

### Evidence rules

| Code | Level | Rule | Message |
|---|---|---|---|
| E030 | error | every claim has ≥ 1 Evidence | "claim without a source" |
| E031 | error | for every relation, each of its claims has ≥ 1 Evidence (follows from E030; checked for a clear message) | "relation backed by {claim}, which has no source" |
| E032 | error | `confidence: established` ⇒ ≥ 1 Evidence with `direction: supports` to a source with `status: verified` | "confidence \"established\" without a verified source supporting it" |
| E033 | error | contradiction: ≥ 2 distinct claims (schema) | "a contradiction links at least 2 claims" |
| E034 | error | question `status: resolved` ⇒ `resolved_by` not empty (schema); `open` ⇒ `resolved_by` absent | "resolved_by not allowed on an open question" |
| E035 | error | `source.status: verified` ⇒ `verified_via` and `verified_at` present (schema) | "verified source without verified_via/verified_at" |
| W040 | warning | claim citing a source with `status: to_verify` | "relies on a source still to verify: {source}" |
| W041 | warning | claim with no Evidence of direction `supports` | "no source supports this claim" |
| W042 | warning | source never cited by any Evidence | "source not cited by any claim" |
| W043 | warning | concept never referenced | "concept not used anywhere" |
| W044 | warning | `verified_at` in the future, or publication year > current year + 1 | "unlikely date: {v}" |

The evidence level (`evidence_level`) is **declared** by the author and is not
recomputed (we do not know the study type of each source). Editorial rule:
level = the highest level reached by the supporting sources.

## 5. Transformations on insert

- `sources.doi` = id for a DOI, `NULL` for an ISBN.
- `sources.authors` = compact JSON (`["Trifunovic, Aleksandra",…]`, no spaces
  outside strings, Unicode not escaped); `first_author` = the part before the
  first comma of the first author.
- `claim_concepts.position` = 0-based index in the YAML list.
- Optional fields absent from the YAML (`missions.status`, `missions.owner`,
  `contradictions.status`, `note`, …) are stored as `NULL`.
- `file` column = path relative to the root, `/` separator.
- Multi-line YAML strings: keep as is (no inner trimming), only drop the
  final `\n`.

## 6. Determinism

Two builds of the same commit must produce **byte-for-byte identical** files.

- Files are read in byte-wise lexicographic order of their path; inserts follow
  that order, child lists follow YAML order.
- No timestamp, host name, absolute path or randomness in the database.
- `build_info` contains exactly: `format_version`=`1.0`,
  `schema_sql_sha256`, `n_<table>` for each data table. No date.
- End of build: `PRAGMA user_version = 1`, then `VACUUM INTO` (rewrites pages
  cleanly). No WAL mode.
- Recommended CI test: build twice, compare SHA-256.

## 7. Output and exit codes

stdout (success): one line per table `table: N`, then
`data/astra.sqlite written (N warnings)` or, with `--check`,
`data valid (N warnings)`.

| Code | Meaning |
|---|---|
| 0 | success (warnings possible) |
| 1 | at least one validation error (E0xx); nothing is written |
| 2 | usage error (unknown option, `--root` without `data/`) |
| 3 | I/O or internal SQLite error (disk, permissions) |

`--strict` (optional): warnings become errors (code 1).

## 8. Examples

`astra db build --root . --data-dir data/examples --check` must validate the
examples in `data/examples/` (same layout as `data/`). This is the minimal test
of the validator: CI must run it. If `--data-dir` is not kept by A10, add a
Rust test that loads this folder.

## 9. Changes since v0.1

Format version 0.1 → 1.0 (`build_info.format_version`). Everything is now in English.

**Field / column renames** (YAML, JSON Schema and SQL):

| Object | v0.1 | v1.0 |
|---|---|---|
| common.schema `$defs` | `text_fr` | `text` |
| evidence-levels.yaml / `evidence_levels` | `label_fr`, `definition_fr` | `label`, `definition` |
| mission / `missions` | `summary_fr` | `summary` |
| source / `sources` | `note_fr` | `note` |
| concept / `concepts` | `name_fr` + `name_en` | `name` (single, English) |
| concept / `concepts` | `description_fr` | `description` |
| claim / `claims` | `statement_fr`, `note_fr` | `statement`, `note` |
| evidence / `evidence` | `note_fr` | `note` |
| relation / `relations` | `note_fr` | `note` |
| question / `questions` | `statement_fr`, `note_fr` | `statement`, `note` |
| contradiction / `contradictions` | `explanation_fr` | `explanation` |

**Enum values:**

| Field | v0.1 | v1.0 |
|---|---|---|
| `sources.status` | `vérifié`, `à vérifier` | `verified`, `to_verify` |
| `claims.confidence` | `solide`, `débattu`, `spéculatif` | `established`, `debated`, `speculative` |
| `evidence.direction` | `soutient`, `contredit`, `nuance` | `supports`, `contradicts`, `nuances` |
| `questions.status` | `ouverte`, `résolue` | `open`, `resolved` |
| `relations.type` | `cause`, `aggrave`, `inhibe`, `active`, `marqueur_de`, `traite`, `previent`, `partie_de`, `associe_a` | `causes`, `aggravates`, `inhibits`, `activates`, `marker_of`, `treats`, `prevents`, `part_of`, `associated_with` |
| evidence-levels `code` | `in-vitro`, `invertebre`, `mammifere`, `humain-observationnel`, `humain-essai`, `approuve` | `in-vitro`, `invertebrate`, `mammal`, `human-observational`, `human-trial`, `approved` |

**New optional fields (nullable columns):**

- `missions.status` ∈ {`active`, `completed`}; `missions.owner` = agent id
  (JSON Schema pattern `^A[0-9]+$`, e.g. `A2`).
- `contradictions.status` ∈ {`open`, `resolved`}.

**Views:** `claims_by_source.statement`; `claim_summary` columns now
`statement`, `evidence_label`, `n_supports`, `n_contradicts`, `n_nuances`;
`concept_summary.name` (replaces `name_fr`, `name_en`).

**Messages:** all diagnostics are in English (§4); levels are `error` /
`warning`; stdout lines in §7 are in English.

## 10. Dataset fetch manifests (`data/datasets/`)

`data/datasets/<id>.yaml` describes one **external dataset**: where to
download it, the expected hashes and its licence. It is not one of the 8 card
kinds: `astra db build` does not load it (no table, `schema.sql` unchanged).
The fetch job (Platform, A10) reads it. Schema: `dataset.schema.json`.

Rules the fetch job and any future loader must enforce:

| Rule | Level |
|---|---|
| File validates against `dataset.schema.json`; file name = `<id>.yaml` | error |
| `id` unique across `data/datasets/` | error |
| Every `landing_url` and `files[].url` starts with `https://` | error |
| `licence` present, with `redistribution_of_derived` ∈ {yes, no, unclear} (quoted in YAML) | error |
| `files[].sha256` is `null` or 64 lowercase hex chars; never guessed | error |
| Same `url` twice in one manifest | error |
| `licence_status` ≠ `pending` ⇒ `approved_by`, `approved_at`, `approval_ref` (row `LA-nnn` in `verification/approvals/licences.md`, or `PRn` with `approved_by: president` for a president ruling in `docs/decisions/president-rulings.md`, which overrules Verification) | error |
| `access: unknown` | warning |
| `licence_status: approved` with a `files[].sha256` still `null` | warning (pin needed) |

Fetching:

1. Only `licence_status: approved` manifests are fetched; others are skipped
   and listed.
2. `fetch.min_interval_seconds` (if set) is the minimum wait between two
   requests to that host.
3. **Pin step.** While `sha256` is `null`, a manual `pin` run downloads the
   file and prints its hash; a PR then commits the hash and `fetched_at`.
   After that, a hash mismatch fails the job. The job never writes to the repo.
4. Raw files are never committed. Only small derived tables may be, and only
   when `redistribution_of_derived: yes` and the licence is approved.
