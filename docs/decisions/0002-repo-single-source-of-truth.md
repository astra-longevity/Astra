# 0002 — The repository is the single source of truth

Date: 2026-10-19. Status: accepted. Decided by the president, relayed by A1.

## Context

The project will use side tools: a SQLite database, maybe a knowledge graph, a search index, a progress site. If one of them kept information that exists nowhere else, the repository would no longer be enough to understand or rebuild the project, and that information could be lost or drift apart.

## Decision

- Everything that matters lives in the Git repository, as versioned text.
- A side tool is allowed only if its data is stored in the repository and it can rebuild everything from it.
- No tool keeps its own state. The files it produces are ignored by Git.
- No server or external service.
- Example: `data/astra.sqlite` is rebuilt from the YAML files in `data/` by `astra db build`.

This decision clarifies decision 0001 without replacing it.

## Consequences

- Deleting a generated file loses nothing: just run the tool again.
- Any information must first be written in the repository before a tool uses it.
- A tool that does not meet these conditions is not accepted in the project.
