# Seal extension (D45, D80): M2 re-seal batch

The seal (`.github/checks/protected-files.yaml` + `lint-config.chain`) covered
the lint config, the check scripts, `ci.yml`, the callbacks and `chain.sh`.
D80 extends it to every workflow and every script a workflow calls, so no
agent can weaken a gate without the president's password. Changing a sealed
file turns the `Structure` check red until the president re-seals.

This batch also carries two PRs that touch sealed files:
#32 (write-path check in `ci.yml`) and #35 (D109 site gate, D52 drift job).

## Files added to the seal

| File | What it enforces |
|---|---|
| `.github/workflows/ci.yml` (already sealed; changed) | Rust lints/tests, structure + chain, and the new write-path job (D47, D69) |
| `.github/workflows/runs.yml` | Every run manifest under `research/runs/` re-runs byte-exact (O13, G9) |
| `.github/workflows/registers.yml` | Issue + obstacle register schemas; daily obstacle due-check (part 9) |
| `.github/workflows/site.yml` | Site build: banner on every page, no score page before the D88 review (D109); S1/S2 gate before publishing (part 8) |
| `.github/workflows/lifespan.yml` | `literature/lifespan/check.rb` on every PR (L1 table) |
| `.github/workflows/protection-drift.yml` | Daily compare of live protection/ruleset with `docs/rust/` (D52); opens an issue on drift |
| `.github/workflows/workflow-lint.yml` | `actionlint` (pinned 1.7.12) on every workflow, PR + merge queue: invalid YAML, bad expressions, shellcheck in `run:` (the #60 failure) |
| `scripts/guards/check-workflows.sh` | The actionlint wrapper; fixtures in `scripts/tests/fixtures/workflows/` (one good, three known-bad) |
| `scripts/guards/check-write-paths.sh` | Only Verification writes `verification/`; cited `Dnn` exist on base (D47, D69) |
| `scripts/guards/protection-drift.sh` | Normalises and diffs live protection vs. the committed payloads (D52) |
| `scripts/runs.py` | Run manifest schema, hashes and re-run (O13) |
| `scripts/registers.py` | Issue/obstacle schemas, S1/S2 gate, obstacle due-check |
| `scripts/build-lock.sh`, `scripts/local-checks.sh` | At most 2 local builds at once (part 6); callbacks run the sealed families through the lock |
| `scripts/tests/*.test.sh` | Known-bad tests of the scripts above (a weakened test is a bypass) |
| `docs/rust/branch-protection.json`, `docs/rust/merge-queue-ruleset.json` | Drift reference; editing them would hide a drift |
| `.context-pilot/shared/callbacks.yaml` (already sealed; changed) | Blocking callbacks now call `scripts/local-checks.sh` (build lock) |

Not sealed on purpose: `literature/lifespan/check.rb` (owned by Literature,
changes with the L1 schema; its workflow is sealed), the toy run under
`scripts/toy/` (pinned by its manifest hash), `docs/agents.yaml` (rule R3 of
the write-path check governs it).

## Pending: PR #55 (A62, P1)

`.github/workflows/fetch.yml`, `.github/workflows/file-sizes.yml` and
`scripts/fetch-datasets.sh` are not on `main` yet. They join the seal either
in this batch (if #55 merges first and this branch is rebased) or in the next
re-seal.

## President: command to run

On this branch, once A10 marks it ready, from a clean checkout:

```sh
git fetch origin && git switch platform/m2-batch && git pull --ff-only
./chain.sh                       # asks for the seal password
git add .github/checks/lint-config.chain
git commit -m "seal: M2 re-seal batch (D45, D80)"
git push
```

Then `Structure` turns green and the PR goes through the merge queue.
Separately (repo settings, president only): create the `protection`
environment (deployment branches: `main`) with secret
`PROTECTION_READ_TOKEN` (fine-grained, this repo, Administration: read-only),
and mark `Write paths (D47 · D69)`, `Runs (manifests · re-run)`,
`Registers (schema · tests)`, `Lifespan (check.rb)` and
`Workflows (actionlint)` as required checks.
