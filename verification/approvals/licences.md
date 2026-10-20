# Licence approvals

Owner: Verification (A50); kept by A66. Charter part 5 and T5-A50-2: Verification
approves every licence before a dataset is used.

## Rules

1. One row per source. A row is the approval of record; the fetch manifest
   (`data/datasets/<id>.yaml`, A9) cites it by id.
2. Verdicts: **approve**, **approve with conditions**, **refuse**. Unknown =
   **refuse** until clarified. A refusal names what would clear it.
3. "Derived" = tables we compute from the source (values extracted, merged,
   re-keyed). Facts are not copyrightable, but a licence or contract can still
   restrict them, so each row says whether derived tables may be committed.
4. If a source forbids redistribution, it enters only through a fetch manifest
   (download at build time, nothing committed), never as a copy in the repo.
5. A row is re-checked when the manifest's URL or version changes, or after
   12 months.
6. Verbatim quotes and the URLs read are the evidence. No row without a URL.

## Datasets (T5-A28-1 list)

| Id | Source | URL checked (2026-10-20) | Licence found | Verdict | Derived tables in repo | Conditions / what clears a refusal |
|---|---|---|---|---|---|---|
| LA-001 | Tabula Muris Senis, processed objects (figshare) | https://figshare.com/articles/dataset/Tabula_Muris_Senis_TMS_dataset/19545169/1 ; https://figshare.com/projects/Tabula_Muris_Senis/64982 | CC BY 4.0 (figshare item licence) | approve with conditions | yes | Cite Tabula Muris Consortium, Nature 2020 (doi 10.1038/s41586-020-2496-1) and the figshare DOI; pin item version. |
| LA-002 | Tabula Muris Senis, raw reads (AWS `s3://czb-tabula-muris-senis`) | https://registry.opendata.aws/tabula-muris-senis/ | Registry points to GitHub LICENSE = BSD-3-Clause (a code licence; no data licence stated) | refuse | no | Not needed for metric v0. Clears with a written data licence from CZ Biohub, or use LA-001 / LA-003 instead. |
| LA-003 | GEO series (any GSE, e.g. GSE132042, methylation clock sets) | https://www.ncbi.nlm.nih.gov/geo/info/disclaimer.html ; https://www.ncbi.nlm.nih.gov/home/about/policies/ | "NCBI places no restrictions on the use or distribution of the GEO data. However, some submitters may claim patent, copyright, or other intellectual property rights" | approve with conditions (per series) | yes, per series | Each GSE gets its own row before use: read the series page and the linked paper's data statement; any submitter restriction = refuse that series. GSE132042 cleared by LA-001 terms (same consortium, CC BY 4.0). |
| LA-004 | Schultz 2020 frailty clocks, Source Data file (Nat Commun 11:4618) | https://www.nature.com/articles/s41467-020-18446-0 | Article and its Source Data under CC BY 4.0 ("licensed under a Creative Commons Attribution 4.0 International License") | approve with conditions | yes | Cite Schultz, Kane et al. 2020; note changes made. |
| LA-005 | Schultz 2020 frailty data on GitHub (`SinclairLab/frailty`) | https://github.com/SinclairLab/frailty (GitHub API: `license` = null) | No licence file | refuse | no | Use LA-004 (same data, CC BY 4.0) where it overlaps. Clears with a licence added by the Sinclair lab or written permission. |
| LA-006 | Mouse Phenome Database, incl. ITP lifespan (ITP1) and JAX ageing Diversity Outbred studies | https://phenome.jax.org/about/termsofuse ; https://phenome.jax.org/projects/ITP1 | "MPD data contributors waive copyright and related rights, making their data freely available whereby MPD users may freely build upon, enhance and reuse those data for any purpose without restriction. Scholarly citation norms must be followed" | approve with conditions | yes | Cite MPD (RRID:SCR_003212) and each project per https://phenome.jax.org/about/citing ; automated fetches observe meta robots and space requests ≥ 2 s; no JAX® Mice images. Data can be corrected upstream: pin download date + sha256 in the manifest. |
| LA-007 | SLAM (NIA, C57BL/6 and UM-HET3) | none: no public route (T5-A2-1, obstacle O16) | unknown | refuse | no | Clears when A2's O16 exit gives a route and its terms are read here. |
| LA-008 | NHANES public linked mortality files | not yet checked | unknown | refuse | no | Row filled when a manifest is proposed. |

## Carried over from the D67 prep check (`verification/prep/licences.md`, read 2026-10-19 by A54)

| Id | Source | Verdict | Conditions |
|---|---|---|---|
| LA-101 | GBD disability weights (Salomon 2015) | refuse (public files) | Paper CC BY-NC-ND 4.0, IHME data non-commercial. Store DOI + table/row reference only; compute internally. |
| LA-102 | Retraction Watch via Crossref | approve | CC0; cite source on any page showing a flag; polite pool with `mailto`. |
| LA-103 | Human Mortality Database, HMD-built series | approve with conditions | CC BY 4.0; free registration; cite with download date. Raw input data: refuse. |
| LA-104 | Eurostat activity-limitation tables | approve with conditions | Acknowledge source; refuse any table credited to a non-Eurostat source. |
| LA-105 | WHO GHO datasets from data.who.int | approve with conditions | CC BY 4.0 + arbitration clause; download from data.who.int only; WHO citation format. |
| LA-106 | WHO ICD-10 codes and titles (subset) | approve with conditions | CC BY-ND 3.0 IGO: copy titles word for word, cite WHO. MeSH-ICD-10 crosswalk: refuse without WHO written agreement. |

## Not in scope

`data/sources/*.yaml` (259 files) are citations of papers (claims extracted
from literature), not fetched datasets. Citing a paper and recording its
claims needs no licence approval. A paper's figures, tables or supplementary
files enter as data only through a dataset manifest and a row here.
