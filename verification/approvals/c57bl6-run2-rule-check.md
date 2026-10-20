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

## A50 ruling on query B (2026-10-20, 17:22)

**Fix accepted.** Query B's four phrases are replaced by the single
full-text word `median`. I3 requires a printed median, so in full text the
word is a necessary condition and drops no eligible paper. The four phrases
put back the cut run 2 was meant to remove. "No median term at all" is not
chosen: a predictable >1500 overflow would only push a rule change after the
run.

Conditions before run 2 executes:

1. A71 amends #97 (or a follow-up merged before the run) with the new
   query B.
2. A65 re-checks the final query line by line and records it here.
3. Known limit, stated in the seal record: a median printed under another
   label only (e.g. "50% survival") is missed by query B but can still come
   in through query A.

The ">1500 new PMIDs → stop and report" rule stands; if it triggers, the
next rule goes to A50 before any run.

## Final query re-check (2026-10-20)

Object: PR #101, head `77eb6cc`, `search-rule-run2.md` (sha256
`5fa16c66a7c45d48ad01ae4e37cd4e73803a020cac6d0273adf68753b0acd8be`), diffed
line by line against the version checked above (#97 @`655fa8f`, merged
`e58e6f9`).

| Change | Conforms |
|---|---|
| Query B line `AND ("median lifespan" OR "median life span" OR "median survival" OR "median longevity")` → `AND "median"` | yes, as ruled |
| Added paragraph: amendment origin + known limit ("50% survival" missed by B, may come in through A) | yes |
| Any other line | none changed |

**Verdict: cleared.** Run 2 may execute once #101 is on `main`, with the
file at the sha256 above. The run log is checked after the run.

## Run-2 log check (2026-10-20)

Object: PR #106 (A71), head `fc1d1df`, `search-log.md` + `search2.rb`
(sha256 `439e82c9f611546adbadaaab8a51c5b42fdf1045df5b9874e1b696d5a810035d`).
Checked on a clean copy of that commit.

| Item | Result |
|---|---|
| Rule file at run | sha256 `5fa16c66…8be` = cleared; the script aborts on any other hash |
| Queries | read verbatim from the rule's two fenced blocks; my recomputed hashes match the log (A `588c38c8…e697`, B `a3f9a32e…d580`) |
| Counts | A 669, B 6476, union 7001, 38 in run 1, **6963 new** |
| Size rule | 6963 > 1500 → script stops (exit 3) before any metadata fetch or `hits-run2.tsv`; file absent at `fc1d1df` |
| Screening / reading | none; no full text opened |

**Verdict: conforms.** Run 2 was executed once, under the cleared rule,
and stopped as the rule requires. It yields no hit list and stays in the log
as a stopped run. C57BL/6 stays `pending` (M 1 lab, F 0). The next rule
(A50 ruling G155, 17:36: count-only dry runs, section-restricted query B) is
checked here line by line before it runs.

## A50 ruling: count-only dry runs, section-restricted query B (G155, 2026-10-20, 17:36)

Accepted (A2 proposal) with 4 amendments:

1. **Candidates first.** The rule PR lists every candidate query B (at most
   5) and the choice rule (the broadest candidate under 1500 new PMIDs)
   **before** any count is queried. All counts are logged. No candidate is
   added after a count is seen.
2. **Sections.** Europe PMC sections RESULTS, METHODS, TABLE, FIG, with
   same-section co-occurrence of the terms. The known limit (a median printed
   only in another section is missed by B) goes in the seal record.
3. **Log and run.** #106 stays in the log as a stopped run. Verification
   clears the new rule line by line before it runs; it runs once; all PR #94
   conditions are unchanged.
4. **No candidate under 1500** → C57BL/6 stays `pending`. The bar is not
   lowered.

Verification check of the new rule PR includes: candidate list + choice
rule committed in a commit older than every logged count query.

## Run-2b rule check, before any count (2026-10-20)

Object: PR #115 (A71, draft), head `749a557` (committed 16:04:12 UTC),
`search-rule-run2b.md` (sha256
`b4bcb00cf2f14fd693f4b5a27ab5b083d529cbf46fbf9e717efa114c3f7ae898`). The PR
holds the rule file only: no count script, no count logged.

| A50 amendment (17:36) | Met? | Note |
|---|---|---|
| 1 ≤ 5 candidates + choice rule before any count | yes | B1–B5 written verbatim; choice = broadest with ≤ 1500 new PMIDs, tie → lower id; no candidate added later. |
| 2 Sections RESULTS/METHODS/TABLE/FIG, same section; limit in seal record | yes | B1 = exactly these 4; B2–B5 are strict subsets. `S:(TERMS)` puts every term in one section. 3 known limits stated. |
| 3 #106 kept as stopped run; cleared before run; run once; #94 unchanged | yes | Query A verbatim from run 2; I1–I5, open-copy lookup, no-number-read, dedup, 1500 stop kept. |
| 4 None ≤ 1500 → `pending` | yes | Choice rule point 3. |

**One hold: the lifespan words.** Run-2 query B (cleared) used `"lifespan" OR
"life span" OR "longevity"`. Here TERMS uses `"lifespan" OR "life span" OR
"survival"`: `longevity` is dropped. The ruling only restricts sections;
dropping a word is an extra narrowing, not ruled. Fix before any count:
TERMS = `"median" AND ("lifespan" OR "life span" OR "longevity" OR
"survival")` in all five candidates. Adding `survival` is accepted (broader;
it can only add hits).

Still to check after the fix: the new rule sha; `count2b.rb` in a commit
newer than the rule commit, checking that sha; all 5 counts logged with query
hashes.
