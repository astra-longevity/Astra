# Licence checks for external data (D67 prep)

Draft by A54 for Verification (A50). Read-only check, all pages read on 2026-10-19.
Items marked `[to verify]` were not settled by the official page and need a second look before launch.

## Summary

| Source | What Astra wants to do | Verdict |
|---|---|---|
| GBD disability weights (Salomon 2015) | Use the weights in harm weighting; show them on the public site | **Not free for public reuse as data.** Paper is CC BY-NC-ND 4.0; IHME data is non-commercial only. Cite and link; do not republish the weight table until cleared. |
| Retraction Watch via Crossref | Weekly automated retraction scan; publish retraction flags | **Allowed.** CC0. Citation requested. Follow Crossref API etiquette. |
| Human Mortality Database | Use life tables for baseline curves | **Allowed with conditions.** HMD-built series are CC BY 4.0; raw input data is not. Free registration needed. |
| Eurostat activity-limitation data | Use disability prevalence by age | **Allowed with attribution**, commercial use included, unless the table is credited to a non-Eurostat source. |
| WHO datasets (GHO) | Use health indicators | **Allowed with attribution, but two different WHO texts exist.** Download from data.who.int: CC BY 4.0 + arbitration clause; cite in its format. |
| WHO ICD-10 codes and titles | Store a subset in `data/vocab/`; show on the public site | **Allowed, commercial use included** (CC BY-ND 3.0 IGO): copy word for word, cite WHO. A word-for-word subset with codes is not an adaptation. A crosswalk to MeSH would need WHO's written agreement. |

## 1. GBD disability weights

- Source: Salomon JA, et al. *Disability weights for the Global Burden of Disease 2013 study.* Lancet Glob Health 2015;3(11):e712-e723. DOI `10.1016/S2214-109X(15)00069-8` (checked on Crossref: title, journal, volume 3, issue 11, pages e712-e723 match).
- Article licence: Crossref licence field lists `http://creativecommons.org/licenses/by-nc-nd/4.0/` (and the Elsevier TDM user licence). The article page (https://www.thelancet.com/journals/langlo/article/PIIS2214-109X(15)00069-8/fulltext) marks it open access under CC BY-NC-ND 4.0.
- IHME data terms (https://www.healthdata.org/data-tools-practices/data-practices/terms-and-conditions), covering healthdata.org, GHDx and VizHub:
  > "Data made available for download on IHME Websites can be used, shared, modified, or built upon by non-commercial users in accordance with the IHME FREE-OF-CHARGE NON-COMMERCIAL USER AGREEMENT."
  > "Commercial organizations wishing to acquire a license to access and download IHME data should contact [IHME]."
- What this means for Astra:
  - Public display: NC-ND forbids sharing modified versions ("no derivatives"). A re-keyed or merged weight table on our site is arguably a derivative. Showing the weights unchanged, with citation, for non-commercial use is probably fine, but `[to verify]` whether a table copied from the supplement counts as "the work".
  - Commercial use: not allowed without an IHME licence.
  - Safe default until cleared (in line with A10's R4b dissent): keep the weights out of public files; store only the DOI and the table/row reference; compute internally.
  - `[to verify]`: whether Astra counts as a "non-commercial user" under the IHME agreement (it is a public, unfunded project, but the agreement text itself was not read in full).

## 2. Retraction Watch database (via Crossref)

- Licence: Crossref metadata page (https://www.crossref.org/documentation/retrieve-metadata/), licensing table: "Retraction Watch database | CC0". Same page: "We release any Crossref-generated data, including aggregations, as public domain material."
- Crossref blog, 2025-01-29 (https://www.crossref.org/blog/retraction-watch-retractions-now-in-the-crossref-api/):
  > "freely available to reuse without a license"
  > "if you make use of the RW retraction metadata in a published work, we kindly request that you provide a citation to the source."
- Access options (https://www.crossref.org/documentation/retrieve-metadata/retraction-watch/):
  - REST API: `https://api.crossref.org/v1/works?filter=update-type:retraction`; the `update-to` field gives the retracted DOI and the source (`publisher` or `retraction-watch`).
  - Full CSV: `git clone https://gitlab.com/crossref/retraction-watch-data`, updated every working day. Simplest for a weekly scan: pull, diff against our cited DOIs.
- Coverage caveat from the same blog: corrections and expressions of concern are "not as comprehensive as retractions". A scan finding nothing is not proof of no correction.
- API etiquette (https://www.crossref.org/documentation/retrieve-metadata/rest-api/access-and-authentication/):
  - "we strongly recommend providing a mailto parameter in all requests" (polite pool).
  - Over the limit: HTTP 429, "wait a short while and try your request again at a lower rate". Manual block: HTTP 403.
  - Limits announced for 2025-12-01 (https://www.crossref.org/blog/announcing-changes-to-rest-api-rate-limits/): polite pool 10 req/s for single DOIs, 3 req/s for lists, 3 concurrent; public pool 5 and 1 req/s, 1 concurrent. Do not hardcode: read the `x-rate-limit-limit` and `x-rate-limit-interval` response headers.
- What this means for Astra: weekly automated scan and public retraction flags are allowed. Add a citation line ("Retraction data: Retraction Watch database, via Crossref, CC0") on any page that shows a flag. Use a project mailto that a human reads (`[to verify]`: which address; it must not be a personal one without consent).

## 3. Human Mortality Database (HMD)

- User agreement (https://www.mortality.org/Data/UserAgreement):
  > "The HMD data are licensed under a Creative Commons Attribution 4.0 International License."
  > "The data constructed by the HMD team are published under the CC BY 4.0 license. This is not the case for the input data (for all HMD data series) which remain under each provider's distribution license."
- Input data (raw births, deaths, census counts): "should not be used for commercial gain or re-published in any form without the explicit permission of the data owners".
- HMD also asks that users preferably not pass copies on: "refer them to the HMD website".
- Access needs a free registered account.
- Citation format: https://www.mortality.org/Research/CitationGuidelines (include the download date).
- What this means for Astra: claim confirmed for HMD-built series (life tables, death rates). Use and publish derived curves with attribution. Do not copy raw input files into the repo. Prefer storing the series name, country code and download date, with a fetch script, over vendoring whole HMD files (`[to verify]` with A9, since the repo is the source of truth and rebuilds need the data).

## 4. Eurostat activity-limitation data

- Copyright notice (https://ec.europa.eu/eurostat/help/copyright-notice):
  > "Reuse of statistical data, metadata, publications, and other dissemination tools published on this website for commercial or non-commercial purposes is authorised provided the source is acknowledged. The reuse policy of the European Commission is implemented by the Decision of 12 December 2011."
  - CC BY 4.0 is stated there for the site's "editorial content". The data itself is covered by the reuse decision above (attribution required), not by an explicit CC BY label. The practical effect is the same: reuse allowed, source acknowledged, changes indicated.
- Legal basis: Commission Decision 2011/833/EU (OJ L 330, 14.12.2011, p. 39), Art. 6: conditions may include acknowledging the source, not distorting the original meaning, and non-liability of the Commission.
- Exception (same notice): data "identified as belonging to sources other than Eurostat" may not be reused commercially (non-commercial reuse still allowed). Logos and trademarks are excluded.
- What this means for Astra: claim "CC BY 4.0" is close enough in effect but not literally what applies to data. Write the attribution as "Source: Eurostat (dataset code, extraction date)". `[to verify]`: exact dataset code for activity limitation (GALI, likely in the `hlth_silc_*` family) and that its metadata does not credit another source.

## 5. WHO datasets

Two WHO texts exist and they differ:

- **data.who.int datasets** (e.g. https://data.who.int/dashboards/gdhm/overview, https://data.who.int/dashboards/global-progress): "Unless specifically indicated otherwise, these Datasets are provided to you under a Creative Commons Attribution 4.0 International License (CC BY 4.0), with the additional terms below." Binding addition: disputes go to conciliation, then arbitration. Data credited to non-WHO sources is not covered.
- **who.int terms and conditions** (https://www.who.int/about/policies/publishing/data-policy/terms-and-conditions): grants use "for public health purposes"; "Copyright in the Datasets remains vested in WHO". Required attribution format: "WHO, title of dataset, year, date of access, acknowledgement of the country or countries having provided the underlying data." Only "minimal" alteration of figures and tables to match the publication's style.
- What this means for Astra:
  - Claim "CC BY 4.0 + extra terms" is confirmed for data.who.int datasets.
  - Astra's use (anti-aging research, non-commercial) fits "public health purposes" under the older text too.
  - Always use the stricter attribution format above (it satisfies both texts), including the reporting countries.
  - Which text applies to our export (checked 2026-10-19):
    - The legacy GHO page says GHO and data.who.int are "driven by a single underlying data source". The GHO OData API was deprecated at the end of 2025 (replaced by the World Health Data Hub OData API), and its page shows no licence.
    - data.who.int terms (https://data.who.int/about/data/terms-and-conditions): CC BY 4.0 plus the conciliation/arbitration clause. Citation format: "World Health Organization {YEAR} data.who.int, {ITEM_NAME} [{ITEM_TYPE}]. {URL} (Accessed on {DATE})".
    - The older who.int terms §4 allow other changes ("abbreviations, additions, or deletions") only with WHO's written authorisation; §1 allows "extract".
    - Mirrors that say "CC BY-NC-SA 3.0 IGO" mix in WHO's licence for publications, not for datasets.
  - Decision proposed: download only from data.who.int, which is explicitly CC BY 4.0. Cite in the data.who.int format and add the reporting countries. Keep extracts unchanged; record any recoding in separate columns.

## 6. WHO ICD-10 codes and titles (for `data/vocab/`) and MeSH fallback

Request (A9 via chair): store an ICD-10 subset (I21-I22, I60-I64, I50, C00-C97, F00-F03, G30, J44, N18.4-5, S72.0-2, E11) in public `data/vocab/` and show it on the public site.

**Verdict: allowed, if codes and titles are copied word for word and cited. Commercial use is allowed too.** One point is still `[to verify]` (see below). MeSH is not needed as a fallback, but it is also free to use.

- WHO copyright page (https://www.who.int/about/policies/publishing/copyright), section "Licensing of WHO Classifications":
  > "They are licensed under the Creative Commons Attribution-NoDerivs 3.0 IGO (CC BY-ND 3.0 IGO) license and may be used for commercial and non-commercial purposes, provided there is no adaptation of the codes and the work is appropriately cited."
- ICD-10 2019 edition, Volume 2 (https://icd.who.int/browse10/Content/statichtml/ICD10Volume2_en_2019.pdf):
  > "This work is available under the Creative Commons Attribution-NoDerivatives 3.0 IGO license (CC BY-ND 3.0 IGO)." "This license does not allow you to produce adaptations of the work (including translations) without permission from WHO."
- WHO classifications licence (https://icd.who.int/en/docs/icd11-license.pdf), covering ICD, ICF and ICHI: CC BY-ND 3.0 IGO; "you are NOT permitted to make 'adaptations'".
- Conflicting older text: WHO FAQ on ICD-10 licensing (https://cdn.who.int/media/docs/default-source/publishing-policies/copyright/who-faq-licensing-icd-10.pdf) says WHO "can grant licences for the use of ICD-10" and that its code files are "suitable for tabulation and display only, they cannot be used for coding a diagnosis". It predates the CC licence. Our use is tabulation and display only, so it fits even under this older text.
- What this means for Astra:
  - Store codes and titles exactly as WHO publishes them (2019 English edition). Do not shorten, reword or translate a title.
  - Our own groupings (for example "stroke = I60-I64") go in separate columns or files that point to the codes, not into the titles.
  - Put the citation and the licence in `data/vocab/README.md` and on any public page that shows the codes: "International Statistical Classification of Diseases and Related Health Problems, 10th Revision (ICD-10), 2019 edition. Geneva: World Health Organization. Licence: CC BY-ND 3.0 IGO."
  - No WHO endorsement may be implied.
  - Subset question (checked 2026-10-19): storing only our codes, titles copied word for word, is **not** an adaptation.
    - WHO classifications licence (https://icd.who.int/en/docs/icd11-license.pdf), §1.2.2: putting the classification into software is not an "adaptation" as long as it is not reproduced "in part or whole" without attribution, without the codes, or without the URIs. WHO thus expects partial reproduction and allows it under those conditions. This text is written for ICD-11; no ICD-10-specific text on subsets was found.
    - CC BY-ND 3.0 IGO legal code: including the Work in a Collection is not an Adaptation.
    - So: keep code, exact title, attribution and the ICD-10 browser URL (https://icd.who.int/browse10/2019/en#/<code>) on every row.
  - Separate limit, ICD-10 Volume 2 (2019): "Mappings or crosslinks between other classifications and terminologies and ICD-10 and/or translations are not covered by the ICD-10 license and are subject to a separate written agreement from WHO." A MeSH-to-ICD-10 crosswalk in `data/vocab/` needs WHO's written agreement. Our own event groupings (for example stroke = I60-I64) are not another terminology and are fine.
- MeSH (NLM), checked in case a fallback is needed: "NLM freely provides MeSH data" (https://www.nlm.nih.gov/databases/download/terms_and_conditions_mesh.html). NLM terms (https://www.nlm.nih.gov/databases/download/terms_and_conditions.html) require the line "Courtesy of the U.S. National Library of Medicine" and no implied NLM endorsement. MeSH translation files are not covered.

## Open items for Verification

1. GBD weights: decide public display policy (default: reference only) and whether to ask IHME.
2. Crossref mailto address for automated scans.
3. HMD: vendor or fetch-at-build (with A9).
4. Eurostat: exact GALI dataset code and source credit.
5. WHO: adopt "data.who.int only" as the download rule (A9).
6. ICD-10: no MeSH-to-ICD-10 crosswalk in the repo without WHO's written agreement (A9, A2).
