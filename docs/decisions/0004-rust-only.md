# 0004 — All code is in Rust

Date: 2026-10-19. Status: accepted. Decided by the president, relayed by A1.

## Context

The project already writes its tools in Rust, under strict lints and a sealed configuration. Analyses (meta-analysis, statistics, simulations) were about to be written in other languages such as Python or R. The president decided that the whole project uses a single language for code, so that every program follows the same quality rules and checks.

## Decision

- Every program, script, analysis or tool in the project is written in Rust, inside the Cargo workspace (`crates/`).
- No Python, R, JavaScript or shell scripts for analyses or tools.
- Data and docs stay as text (YAML, Markdown).
- Two exceptions, ruled by the president on 2026-10-19:
  1. The CI guard scripts copied from context-pilot (`.github/checks/*.sh`, `chain.sh`) stay in bash, as an exact replica of context-pilot.
  2. JavaScript that runs in the browser on the generated site (`interface/static/`) is allowed. The site generator itself stays in Rust.

## Consequences

- Existing non-Rust code is listed. Each owner (see `OWNERS.md`) ports it to Rust or removes it.
- Everything outside these two exceptions must be ported to Rust or removed by its owner, for example `literature/briefs/*.check.py` (A2).
- `data/schema/schema.sql` is a database schema (data), not code.
- Any new exception requires a new decision.
- Data analyses are Rust crates, checked by the same lints and tests as the rest of the code.
