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
| LA-004 | Schultz 2020 frailty clocks, Source Data file (Nat Commun 11:4618) | https://www.nature.com/articles/s41467-020-18446-0 ; Europe PMC full text https://www.ebi.ac.uk/europepmc/webservices/rest/PMC7492249/fullTextXML (re-checked 2026-10-20) | Article CC BY 4.0 (Crossref licence field). But the published supplementary list has only 3 files: `41467_2020_18446_MOESM1_ESM.pdf` (Supplementary Information), `MOESM2` (Peer Review File), `MOESM3` (Reporting Summary); `MOESM4` returns 404. The "Source Data File" named in the data statement was never published. | refuse (no file exists) | no | Superseded 2026-10-20: the approval had no file to attach to. Schultz data exists only in `SinclairLab/pub-Schultz-Kane-2020` (LA-005, refused). Clears with a licence from the Sinclair lab (ask the corresponding author), or numbers re-read from the CC BY figures/SI PDF (MOESM1) as a literature extraction, not a dataset. |
| LA-005 | Schultz 2020 frailty data on GitHub (`SinclairLab/frailty`) | https://github.com/SinclairLab/frailty (GitHub API: `license` = null) | No licence file | refuse | no | Use LA-004 (same data, CC BY 4.0) where it overlaps. Clears with a licence added by the Sinclair lab or written permission. |
| LA-006 | Mouse Phenome Database, incl. ITP lifespan (ITP1) and JAX ageing Diversity Outbred studies | https://phenome.jax.org/about/termsofuse ; https://phenome.jax.org/projects/ITP1 | "MPD data contributors waive copyright and related rights, making their data freely available whereby MPD users may freely build upon, enhance and reuse those data for any purpose without restriction. Scholarly citation norms must be followed" | approve with conditions | yes | Cite MPD (RRID:SCR_003212) and each project per https://phenome.jax.org/about/citing ; automated fetches observe meta robots and space requests ≥ 2 s; no JAX® Mice images. Data can be corrected upstream: pin download date + sha256 in the manifest. |
| LA-007 | SLAM (NIA, C57BL/6 and UM-HET3) | none: no public route (T5-A2-1, obstacle O16) | unknown | refuse | no | Clears when A2's O16 exit gives a route and its terms are read here. |
| LA-008 | NHANES public-use linked mortality files (2019 linkage) | https://www.cdc.gov/nchs/policy/data-user-agreement.html (NCHS Data User Agreement, page updated 2024-09-17; re-checked 2026-10-20) | NCHS DUA: data used "for statistical reporting and analysis only"; no attempt to learn the identity of any person; no linkage with individually identifiable data from other datasets; no research on disclosure methods. No redistribution clause. Willful disclosure of identifying information is a criminal offence. | approve with conditions | aggregate tables only | Fetch-only: no row-level file committed, cached in a public artefact or published. Derived tables must be aggregates (no cell that singles out a person). No linkage to any other person-level data. Note in every output that public-use mortality files carry perturbed follow-up/cause for some records. |

## PR #52 manifests (`data/datasets/`, checked 2026-10-20)

| Id | Manifest | URL checked | Licence found | Verdict | Derived tables in repo | Conditions / what clears a refusal |
|---|---|---|---|---|---|---|
| LA-009 | `tabula-muris-senis` (figshare item 8273102, "Processed files (to use with scanpy)", v3) | https://api.figshare.com/v2/articles/8273102 ; doi 10.6084/m9.figshare.8273102.v3 | MIT (figshare item licence field) | approve with conditions | yes | MIT grants use, copy, modify, publish, distribute without restriction; condition: keep the copyright and permission notice with any copy or substantial portion. Cite Tabula Muris Consortium, Nature 2020 (10.1038/s41586-020-2496-1) and the figshare DOI. Pin v3 + sha256. LA-001 (item 19545169, CC BY 4.0) remains valid but is not the manifest's source. |
| LA-010 | `geo-gse80672` (mouse blood RRBS, n=255) | https://www.ncbi.nlm.nih.gov/geo/query/acc.cgi?acc=GSE80672 ; NCBI eutils esummary | GEO terms (LA-003); no submitter restriction on the series record | approve with conditions | yes | Cite GSE80672 and Petkovich et al. 2017 Cell Metab (10.1016/j.cmet.2017.03.016). The paper is Elsevier, not CC: reuse the deposited data only, no paper text or figures. |
| LA-011 | `geo-gse93957` (multi-tissue methylation, n=62) | https://www.ncbi.nlm.nih.gov/geo/query/acc.cgi?acc=GSE93957 ; esummary | GEO terms; no restriction stated | approve with conditions | yes | Cite GSE93957 and Stubbs et al. 2017 Genome Biol (10.1186/s13059-017-1203-5). |
| LA-012 | `geo-gse120137` (SuperSeries, n=549) | https://www.ncbi.nlm.nih.gov/geo/query/acc.cgi?acc=GSE120137 ; esummary | GEO terms; no restriction stated | approve with conditions | yes | Cite GSE120137 and Thompson et al. 2018 Aging (PMID 30348905). SuperSeries: the manifest must list the SubSeries actually fetched; each inherits this row. |
| LA-013 | `geo-gse121141` (whole-lifespan clock, n=81) | https://www.ncbi.nlm.nih.gov/geo/query/acc.cgi?acc=GSE121141 ; esummary | GEO terms; paper eLife CC BY 4.0 | approve with conditions | yes | Cite GSE121141 and Meer et al. 2018 eLife (10.7554/eLife.40675). |
| LA-014 | `dido-diversity-outbred-dr` (DRiDO, Calico) | https://github.com/calico/DRiDO_Paper_Release (LICENSE file, GitHub API) | MIT, "Copyright (c) 2024 Calico", covering "this software and associated documentation files"; no README or separate data terms. Paper (10.1038/s41586-024-08026-3) is CC BY-NC-ND, which governs the article, not the repo. | approve with conditions | yes | The repo is the paper's data release and its only licence is MIT; keep the MIT notice with any copy, cite Di Francesco et al. 2024. Re-check if Calico publishes separate data terms. No paper figures or text reused (NC-ND). |
| LA-015 | `anage` (HAGR AnAge) | https://genomics.senescence.info/legal.html | "licensed under a Creative Commons Attribution 3.0 Unported License"; "free for all purposes, including commercial ... provided you mention the use of HAGR" | approve with conditions | yes | Cite HAGR and de Magalhães et al. 2024 NAR (10.1093/nar/gkad927). One bulk download of the dataset zip per pin, no scraping (HAGR asks to be contacted before large retrievals). |
| LA-016 | `uk-biobank` | https://www.ukbiobank.ac.uk/enable-your-research/apply-for-access | Access-controlled: approved application + Material Transfer Agreement required | refuse | no | No approved application exists. Clears only with an approved UK Biobank application (needs a sponsor institution, T5-A28-1) and a row re-checked against its MTA. Even then: no participant-level data in a public repo. |

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
