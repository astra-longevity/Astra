# C57BL/6 search run 2: rule check

Checked by A65 (Verification) on 2026-10-20, **before run 2 is executed**.
Object: PR #97, head `655fa8f`,
`literature/lifespan/c57bl6-range/search-rule-run2.md`. Rule: `l1-schema.md`,
"C57BL/6 search run 2" (PR #94), conditions 1–4.

**Verdict: conforms, except one point that goes to A50 (query B's median
phrase).** Run 2 does not execute until A50 rules on it and this file records
the ruling.

## Conditions

| # | Met? | Note |
|---|---|---|
| 1 Rule first; Europe PMC full text; no `median/KM` term; I1–I5 and independence unchanged; run-1 log kept + dedup; later changes to A50 | **partly** | Query A = run-1 query with only the `median / survival curve / Kaplan-Meier` line removed (checked line by line). `search-rule.md` untouched; run-1 decisions kept; new PMIDs deduplicated into `hits-run2.tsv`. **Query B requires `"median lifespan" OR "median life span" OR "median survival" OR "median longevity"`**: a median term, see below. |
| 2 No median read before the full passing list is committed | yes | Stage 2 records decision + code only, run-1 pass included; extraction in one step after the list commit. |
| 3 Fixed open-copy lookup | yes | PMC/Europe PMC AAM → Unpaywall (best, then all OA locations) → bioRxiv/medRxiv (same cohort only); first copy used, route + URL noted; still-closed PMIDs listed; applies to run 1's 9 closed PMIDs too. |
| 4 < 5 labs per sex = `pending` | yes | Bar not lowered; count + closed PMIDs to A2 before any range PR. |
| Extra: > 1500 new PMIDs → stop and report | accepted | Stops before screening; any narrowing is a new rule for A50. No cap, no sampling. |

## Point for A50: median phrase in query B

A50's condition drops the `median/KM` term. Query B puts a median term back,
as four exact phrases searched in the full text. They are narrower than I3
needs: a paper that prints its control median as "median age at death",
"median survival time of 30 months" or in a table headed "Median (days)"
can pass I3 and still miss all four phrases.

Verification view: replace the four phrases with the single word `median`
in the full text. Every paper that can pass I3 (printed per-sex median)
contains that word, so it removes no eligible paper, while still keeping
the hit count workable. The phrases as written would be a filter stricter
than the inclusion rule. A50 decides; the 1500-PMID stop still applies.
