# Shared rules

These rules apply to every agent working on Astra.

## Names and language

- File and folder names in English, in `kebab-case` (lowercase, words separated by hyphens).
- If order matters, use a numeric prefix: `01-...`, `02-...` (or `NNN-` for missions, `NNNN-` for decisions).
- Everything is in English: docs, data, code, commit messages, messages between agents. Use plain language, without needless jargon. A technical term that cannot be avoided is explained or added to `docs/glossary.md`. See decision `docs/decisions/0003-english-only.md`.

## All code is in Rust

- Every program, script, analysis or tool in the project is written in Rust, inside the Cargo workspace (`crates/`).
- No Python, R, JavaScript or shell scripts for analyses or tools.
- Data and docs stay as text (YAML, Markdown).
- Exception 1: the CI guard scripts copied from context-pilot (`.github/checks/*.sh`, `chain.sh`) stay in bash.
- Exception 2: browser JavaScript on the generated site (`interface/static/`) is allowed; the generator stays in Rust.

See decision `docs/decisions/0004-rust-only.md`.

## Organisation

- Every top-level folder has a `README.md` that explains its role and its owner.
- An agent writes only inside its manager's area, as defined in `OWNERS.md`.

## Scientific rigour

- Every scientific claim cites its source with a verified DOI (the link must lead to the right publication).
- The `[to verify]` marker is allowed in drafts, but forbidden in a delivered file.

## Git

- Commit messages: `<folder>: <action>`, for example `literature: add aging map overview`.
- Small commits, one topic at a time.
- `main` only changes through pull requests, for everyone. Flow: branch → PR → `gh pr merge --auto` (the merge queue squashes and merges once the required checks pass).
- Never use `--admin`, and never change branch protection or rulesets.
- Step-by-step guide: [`docs/rust/pr-workflow.md`](docs/rust/pr-workflow.md).

## The repository is the single source of truth

- Everything that matters lives in the repository, as versioned text.
- Side tools (SQLite database, graph, index, site…) are allowed only if their data is stored in the repository and the tool can rebuild everything from it.
- No tool keeps its own state. The files they produce are ignored by Git.
- No server, no external service.
- Example: `data/astra.sqlite` is rebuilt from the YAML files by `astra db build`; it is not versioned.

See decision `docs/decisions/0002-repo-single-source-of-truth.md`.
