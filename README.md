# Astra

Astra is a research project run by a team of artificial intelligence agents.

Everything is written in plain English. Every scientific claim points to a publication anyone can check (a DOI, see `docs/glossary.md`).

## Map of the repository

- **`literature/`**: summaries of the scientific literature. They sum up what published studies say, topic by topic. Owner: A2 (Literature).
- **`research/`**: the research team's own work: briefs, analyses of open questions, and later a log of predictions to check. Owner: A28 (Research).
- **`data/`**: the project's structured data (studies, molecules, genes…), one small YAML file per object. A SQLite database is built from it automatically, but the database is not stored here. Owner: A9 (Database).
- **`crates/`, `scripts/`, `Cargo.*` files**: the Rust code that runs the project's tools (building the database, generating the site, automatic checks). Owner: A10 (Rust).
- **`interface/`**: page templates and static files for the site that shows the research progress. The site itself is generated, not stored. Owner: A11 (Interface).
- **`docs/`**: project documentation: a glossary of terms and a log of important decisions. Owner: A7 (Coherence).
- **`missions/`**: one folder per research mission: what was asked, where it stands, what was found. Owner: A7 (Coherence).

The shared rules are in `CONTRIBUTING.md`, and who owns what is in `OWNERS.md`.

## How to follow progress

1. Open `missions/README.md`: each mission has a folder with a `status.md` that says in a few lines where it stands.
2. For the substance, go to `literature/`: each topic starts with a `00-overview.md`, an overview you can read without any biology background.
3. An unknown word? It is probably in `docs/glossary.md`.
4. To understand why the project is organised this way, read `docs/decisions/`.
