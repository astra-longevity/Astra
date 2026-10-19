# 0004 — All code is in Rust

Date: 2026-10-19. Status: accepted. Decided by the president, relayed by A1.

## Context

The project already writes its tools in Rust, under strict lints and a sealed configuration. Analyses (meta-analysis, statistics, simulations) were about to be written in other languages such as Python or R. The president decided that the whole project uses a single language for code, so that every program follows the same quality rules and checks.

## Decision

- Every program, script, analysis or tool in the project is written in Rust, inside the Cargo workspace (`crates/`).
- No Python, R, JavaScript or shell scripts for analyses or tools.
- Data and docs stay as text (YAML, Markdown).
- Exceptions only by explicit decision of the president, recorded in `docs/decisions/`.

## Consequences

- Existing non-Rust code is listed. Each owner (see `OWNERS.md`) ports it to Rust or removes it.
- Pending the president's call: the CI shell guards (`.github/checks/*.sh`, `chain.sh`, copied from context-pilot) and the site's JavaScript (`interface/static/`). They stay until he decides.
- Data analyses are Rust crates, checked by the same lints and tests as the rest of the code.
