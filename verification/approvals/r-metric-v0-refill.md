# R-Metric v0: retrospective refill rule

Owner: Verification (A50); recorded by A65 on 2026-10-20, **before the list
is built**. Requested by A28 in G154 (split-proposal.md @5090d27, PR #59).
This rule goes into the D115 seal record unchanged.

**Verdict: approve with conditions.**

## Rule (fixed now, never changed after the list exists)

- **Source:** DrugAge, *Mus musculus* entries only.
- **Window:** publication year 2015-01-01 to 2022-12-31, inclusive. The year is
  taken from the PubMed record of the cited paper, not from DrugAge.
- **Unit:** one study = one PMID. Every arm of a selected study goes in,
  whatever its result.
- **Exclusions, all by metadata:** ITP studies; any study whose page appears in
  `research/metric/v0/manual-downloads.csv` (read = train).

## Conditions

1. **Pinned source.** A71 fetches one DrugAge build and records its version,
   URL, date and sha256 before filtering. A28 and A67 never open it.
2. **Outcome-stripped list.** The list keeps only PMID, year, compound, sex,
   strain, dose, arm id. No lifespan column. A71 records its sha256 and states
   that the rule above was applied with no other filter.
3. **Selection bias stated.** DrugAge curates studies that report lifespan
   effects and may hold fewer null results. Items are labelled
   "retrospective, not blind, DrugAge-selected"; the seal record says so.
4. **Licence.** DrugAge licence approved by A66 before use (T5-A50-2).
5. **Trace search.** Not needed for these items (their results are public by
   design); the D118 search still covers the blind part.

Any breach (another filter, a changed window, an outcome column read by A28 or
A67) makes the affected items exploratory and is reported as S1.

## A50 additions (2026-10-20, before the build)

6. **Built by script.** A71 builds the list with a script run on the pinned
   DrugAge file. It filters on species, PubMed year and PMID only, and drops
   every outcome column in the same step. The script (path + commit) and its
   output sha256 go into the seal record. No manual pruning: any exclusion
   made after the build means the seal is refused.
7. **Bias measured, not just labelled.** The T1-A2-3 filters
   (`control_in_ref_range`, replication) still apply when these rows are
   scored. The scorecard reports the share of null and shortening arms in
   the refill next to the same share in the ITP arms.
