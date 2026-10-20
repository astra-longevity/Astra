# data/ — Astra's knowledge base

This folder holds **everything the team has learned**, stored as small text
cards (YAML format, readable by a human). One card = one object.

A database (`data/astra.sqlite`) is built from these cards by the command
`astra db build`. The interface uses it for fast searches. It is **never**
committed to git: you can delete it and rebuild it at any time. **The cards
are the truth.**

> **Golden rule.** The repository is the UNIQUE source of truth: the YAML files
> are authoritative; astra.sqlite and any future data tool (graph, index…) must
> store their data in the repository and rebuild entirely from it, with no
> state of their own.

## The 8 kinds of cards

| Folder | What it is | Example |
|---|---|---|
| `missions/` | A research project of the team. | "Map of aging" |
| `sources/` | A scientific publication (article, book), identified by its DOI (a unique ID number given to each article). | Trifunovic 2004, *Nature* |
| `concepts/` | A notion: a cause of aging, a gene, a disease, a treatment, an organism… | "Mitochondrial dysfunction" |
| `claims/` | One scientific **statement**, in one sentence. | "Mice whose mitochondrial DNA breaks down age faster." |
| `evidence/` | The link between a claim and a source: the source **supports** it, **contradicts** it or **nuances** it (adds a limit). | C-0001 ← Trifunovic 2004 |
| `relations/` | A link between two concepts (A causes B, A aggravates B, A treats B…), backed by claims. | genomic instability → aggravates → mitochondria |
| `questions/` | A question that is still open (or resolved). | "Is this also true in humans?" |
| `contradictions/` | Two (or more) claims that seem to disagree, with an explanation. | |

The `schema/` folder holds the rules (what a card must contain) and
`examples/` holds one sample card of each kind.

## The evidence scale (6 levels)

Each claim states **how well it is proven**, from 1 (weak) to 6 (strong):

1. **Cells or computer** — seen in a lab dish or in a simulation.
2. **Small organisms** — yeast, worm, fly: a whole living being, but very far from us.
3. **Laboratory mammals** — mouse, rat, monkey. Closer, but many mouse results fail in humans.
4. **Observations in humans** — a link is seen in a population, without proving the cause.
5. **Randomized clinical trial** — chance decides who gets the treatment: the only way to prove a cause in humans.
6. **Approved** — cleared by a health authority (FDA, EMA).

Each claim also has a **confidence**: `established` (needs at least one
verified source), `debated` or `speculative`. A source is `verified` once we
have checked it really exists (via Crossref); otherwise it is `to_verify` and
the interface flags it.

Missions may also say whether they are `active` or `completed` and who owns
them (`owner`, an agent id such as `A2`). Contradictions may say whether they
are still `open` or `resolved`.

## Adding a card

1. Copy the card of the same kind from `examples/` into the right folder.
2. Fill in the fields. Write in plain English, short sentences.
3. Name the file following the conventions below.
4. Run `astra db build --check`: it lists every error.

A claim **must** have at least one `evidence/` file, otherwise the build refuses it.

## Names and identifiers

| Kind | Identifier | File name |
|---|---|---|
| Source | Lowercase DOI: `10.1038/nature02517` (book without a DOI: `isbn:978…`) | `/` replaced by `_`: `10.1038_nature02517.yaml` |
| Concept, mission | lowercase-with-hyphens, in English: `mitochondrial-dysfunction` | `<id>.yaml` |
| Claim | `C-0001-short-summary` | `<id>.yaml` |
| Relation | `R-0001-…` | `<id>.yaml` |
| Question | `Q-0001-…` | `<id>.yaml` |
| Contradiction | `X-0001-…` | `<id>.yaml` |
| Evidence | (no id) | `<claim id>--<source file name>.yaml` |

Dates are written in quotes: `verified_at: "2026-10-19"`.

## Avoiding conflicts when several agents work at once

- One card per file: two agents adding different cards never get in each other's way.
- Numbers `C-0001`, `R-0001`…: before picking a number, take the highest
  existing one + 1 **on the up-to-date branch**. The text after the number
  keeps the id unique even if two agents pick the same number; the build
  reports any real duplicate.
- Do not edit another project's card without telling its owner.
- Never commit `astra.sqlite` to git (already excluded by `.gitignore`).

## Dataset manifests (`datasets/`)

Besides the 8 kinds of cards, `datasets/` lists the **external datasets**
Astra downloads (mouse lifespan tables, methylation data, life tables…). One
file per dataset, `datasets/<id>.yaml`, says:

- where to download each file (`files`: url, expected `sha256`, size, format);
- who publishes it and how to get access (`access`: public, registration,
  request, unknown);
- its **licence**, and whether we may publish small tables derived from it
  (`redistribution_of_derived`: yes, no, unclear);
- `licence_status`: `pending` until Verification (A50) decides; `approved` or
  `refused` with `approved_by`, `approved_at` and `approval_ref` (the row in
  `verification/approvals/licences.md`). Only Verification changes it.

The raw data is **never** committed. A Platform job downloads approved
datasets, checks the hashes and caches them.

**Pin step.** A new file has `sha256: null` (we never guess a hash). A manual
`pin` run of the fetch job downloads it and prints the hash; a PR then writes
the hash and `fetched_at` into the manifest. From then on, a changed file
fails the job.

Rules: `schema/dataset.schema.json` and `schema/BUILD_SPEC.md` section 10.

Technical details for developers: `schema/BUILD_SPEC.md`.
