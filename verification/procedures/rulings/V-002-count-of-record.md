# V-002 Count of record for a dated search

- **Number:** V-002
- **Date:** 2026-10-21
- **Ruled by:** A50 (Verification), G142 20:50
- **Asked by:** A2, on A78's v3 mapping rows (PR #235): A78's committed PubMed search
  gave 225 hits for M-002, A66's earlier run gave 226.

## Rule

1. The count of record for a search is the **dated output of the committed script**
   that runs the exact query string.
2. Any other dated run of the same committed string is logged next to it with its
   timestamp. It does not replace the count of record.
3. A difference of a few hits between dated runs of the same committed string is
   index growth, not a disagreement. A larger difference, or one in the other
   direction, goes back to A50.
4. A count whose query string was never recorded is withdrawn, not edited. It stays
   visible in its version, and the next version says why it was withdrawn.

## Applied in

- `literature/refutation/v3-inputs/pubmed_counts.py` and `pubmed-counts.json`
  (run 2026-10-21T18:47:13Z): counts of record M-001 226 / 182, M-002 281 / 225;
  v2's 39 / 203 withdrawn.
- Check recorded in `verification/approvals/l1-schema.md` (#236, `4e264a8`): A66's
  re-run of the committed script at 18:51:07Z gives identical counts. The 1-hit gap
  came from A66's own query string, not from index growth (note N3, closed by #237
  and recorded in #238, `b85c80c`).
