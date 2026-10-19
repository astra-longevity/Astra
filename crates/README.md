# crates/ — Astra's Rust code

Each folder here is a "crate" (a Rust module compiled separately).
They all share the same quality rules, defined once in the root
`Cargo.toml` (`[lints] workspace = true` in each crate).

| Crate | Role |
|-------|------|
| `astra-cli` | The `astra` program, single entry point (`astra db build`, `astra site build`). For now it only prints its version. |
| `astra-db` | Library that will build the `data/astra.sqlite` database from the YAML files in `data/`. |
| `astra-site` | Library that will generate the website (the interface) from the repository. |

## Rules

- The rules are very strict: almost every compiler and `clippy` (Rust's
  analysis tool) warning is a blocking error.
- Disabling a rule locally (`#[allow]` or `#[expect]`) is forbidden, unless
  the exception is registered, with its justification, in
  `.github/checks/allowed-lint-exceptions.yaml`.
- A `.rs` file has at most 500 lines and a folder contains at most 8 entries.
- Every new dependency must be approved by the Rust Manager.

## Running the checks

See `scripts/README.md`.
