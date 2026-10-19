# Mission 001 — Report: the aging map

## What was produced

A map of what science knows about aging today, in 14 files stored in [`literature/aging-map/`](../../literature/aging-map/):

- [`00-overview.md`](../../literature/aging-map/00-overview.md): the overview, in plain English. **This is the file to read first.**
- [`framework.md`](../../literature/aging-map/framework.md): the definitions, the major theories (why do we age?) and the ways of measuring aging (biological "clocks", frailty index…).
- `01` to `12`: one file per major biological cause of aging. Researchers recognise 12 of them, called *hallmarks* since a landmark 2013 article, updated in 2023.

## How it is organised

The 12 causes are sorted into three groups:

1. **The initial damage** (`01` to `05`): DNA gets damaged, the ends of chromosomes shorten, the settings of genes drift, proteins lose their shape, the cell's waste recycling slows down.
2. **Responses that become harmful** (`06` to `08`): defence mechanisms that are useful at first (nutrient sensors, the cell's power plants, shutting down damaged cells) become harmful when they last too long.
3. **The visible consequences** (`09` to `12`): the reserves of replacement cells run out, cells communicate poorly, a background inflammation sets in, gut bacteria fall out of balance.

Each file `01` to `12` follows the same 7-part outline: definition, mechanisms, visible effects on health, links with the 11 other causes, existing or investigational treatments (with their level of evidence), open questions, references.

## How the sources were checked

- Every statement rests on a publication identified by its DOI (a unique number assigned to each scientific article).
- Each writer checked all of their references in Crossref, the official DOI database (author, year, title). Wrong references were removed.
- Independent final check: 39 randomly drawn DOIs (3 per file) were checked again. All 39 are correct.
- The verified sources are also recorded in the project database (`data/`), in a separate commit (`data: ingest aging-map`).

## Limits

- **7 points remain marked [to verify]** in the files: two old books without a DOI (`framework.md`), two points in `02`, two in `03`, and the current status of the TAME trial of metformin (`06`). They are listed in section 5 of `00-overview.md`.
- Most results come from laboratory animals; few are demonstrated in humans.
- The 12 causes are a tool for organising knowledge, not an established truth.
- The field moves fast. This is not medical advice.
