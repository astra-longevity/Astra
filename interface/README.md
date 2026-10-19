# Interface

Owner: A11 (Interface).

This folder holds only the page templates and static files of the site that shows research progress. The site is built by the Rust program (`astra site build`) from the repository; the output (`interface/dist/`) is not versioned.

## Viewing the site

1. `astra site build` (writes `interface/dist/`).
2. Double-click `interface/dist/index.html`. No server, no internet connection: everything is in the files (only DOI links open the publisher's site).

## Contents

| File | Role |
|---|---|
| `templates/base.html` | Shared layout: menu, footer, CSS, JS, embedded glossary |
| `templates/_macros.html` | Label maps for data values + badges (confidence, status, evidence level, direction), DOI link, filter box |
| `templates/index.html` | Dashboard: key figures, missions, latest changes |
| `templates/map.html` | Map of the 12 hallmarks of aging |
| `templates/claims.html` | Filterable list of claims |
| `templates/claim.html` | One claim and its sources |
| `templates/sources.html` | Filterable list of sources |
| `templates/source.html` | One source and the claims that use it |
| `templates/questions.html` | Open questions, contradictions, "to verify" points |
| `templates/journal.html` | Git history, by day |
| `templates/glossary.html` | Full glossary |
| `templates/doc.html` | One note from `literature/` (markdown already converted to HTML), long-form reading |
| `static/style.css` | Single stylesheet (automatic light/dark, print) |
| `static/site.js` | Filters and tooltips, no dependency |
| `glossary.yaml` | Definitions (one entry = `id`, `term`, optional `aka`, `definition`) |

## Pages to produce (for the generator)

| Output | Template | `root` |
|---|---|---|
| `index.html`, `map.html`, `claims.html`, `sources.html`, `questions.html`, `journal.html`, `glossary.html` | same name | `""` |
| `claims/<claim.id>.html` | `claim.html` | `"../"` |
| `sources/<slug>.html` | `source.html` | `"../"` |
| `literature/<folder>/<note>.html` (e.g. `literature/aging-map/08-cellular-senescence.html`) | `doc.html` | `"../../"` (one `../` per folder level) |
| `static/` | copy of `interface/static/` | — |

Source `slug` = `id` with `/` and `:` replaced by `_` (e.g. `10.1038_nature10600`). All links are relative and prefixed with `root`.

## Variables

Every page receives:

- `root`: relative prefix (`""`, `"../"`, `"../../"`).
- `page`: page id (`index`, `map`, `claims`, `claim`, `sources`, `source`, `questions`, `journal`, `glossary`, `doc`), used by the menu.
- `site`: `{ title, generated_at, commit_short }`.
- `glossary`: the entries of `glossary.yaml` (`id, term, aka?, definition`), embedded as JSON for tooltips.

### Data values

Raw data values are never shown: `_macros.html` maps them to labels. Values from `data/schema/` (A9, schema v1). The maps also accept the old French values during the switch.

- confidence: `established` | `debated` | `speculative`
- source status: `verified` | `to_verify`
- evidence direction: `supports` | `contradicts` | `nuances` (shown "Qualifies")
- evidence level (1→6): `in-vitro`, `invertebrate`, `mammal`, `human-observational`, `human-trial`, `approved` (source: `data/schema/evidence-levels.yaml`; labels copied into `_macros.html`, macro `evidence_level`)
- question / contradiction status: `open` | `resolved` (contradiction status optional)
- mission status (optional): `active` | `completed`
- relation type: `causes`, `aggravates`, `inhibits`, `activates`, `marker_of`, `treats`, `prevents`, `part_of`, `associated_with` (macro `relation`)

Fields marked `?` are optional and hidden when empty: nothing is invented. Text fields use the v1 names (`statement`, `name`, `summary`, `text`); templates read only these English fields.

### Per template

- **index.html**: `stats { missions, claims, sources, sources_verified, questions_open }`; `missions[] { id, title, summary?, status?, owner?, updated_at? }`; `recent[] { date, message }` (last 5 to 10 commits).
- **map.html**: `families[] { name, description?, hallmarks[] }` with `hallmark { number, name, summary (3 lines), confidence, claim_count, source_count, doc_href? }`. `doc_href` = relative path to the note; the "Read the note" link is hidden when empty.
- **claims.html**: `claims[] { id, statement, organism, evidence_level, confidence, mission, source_count }`; `missions[] { id, title }`.
- **claim.html**: `claim { id, statement, organism, evidence_level, confidence, mission_title? }`; `concepts[] { name }`; `evidence[] { direction, slug, title, authors_short, journal, year, locator?, status }`; `relations[] { type, id, statement }`.
- **sources.html**: `sources[] { slug, id, doi?, title, authors, authors_short, journal, year, status, claim_count }`.
- **source.html**: `source { id, doi?, title, authors, authors_short, journal?, year?, type_label?, status }`; without a DOI (books, `id` = `isbn:…`), `id` is shown instead of the DOI link. `claims[] { id, statement, confidence, direction, locator? }`.
- **questions.html**: `missions[] { id, title }`; `questions[] { text, status, mission, mission_title?, note? }`; `contradictions[] { summary, status?, mission, claim_a { id, statement }, claim_b { id, statement } }`; `to_verify[] { text, file, line?, mission }` (found in `literature/**/*.md` by the "[to verify]" marker).
- **journal.html**: `areas[] { id, label }` (e.g. `data` → "Data", `rust`, `literature`…, from the commit message prefix); `days[] { date (YYYY-MM-DD), label (e.g. "October 19, 2026"), commits[] { time, hash_short, author, message, area?, area_label? } }`.
- **doc.html**: `page = "doc"` (menu highlights "Map of aging"); `doc { title, file, html }`: `title` = first heading of the note, `file` = path in the repository (shown as "Source: …"), `html` = already-converted markdown (output as is with `|safe`, so only produce it from repository files). Styles: `.prose` class (headings, lists, scrolling tables, quotes, pulldown-cmark footnotes `.footnote-definition`).
- **glossary.html**: `glossary_sorted` = the glossary sorted by `term` (alphabetical, accents ignored).

`authors_short` = "First author et al." or "Author & Author".

## Writing rules

- All visible text in plain English for a non-biologist; a technical term is either explained on the spot or wrapped in `<abbr data-term="id">…</abbr>` with an entry in `glossary.yaml`.
- No external calls (CDN, font, script): the site must work offline.
