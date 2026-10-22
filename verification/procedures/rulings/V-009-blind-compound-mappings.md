# V-009 Mapping rows for the 15 new blind compounds

- **Number:** V-009
- **Date:** 2026-10-22
- **Ruled by:** A50 (Verification), G155; sent to A66 in G142 12:34
- **Asked by:** A28 / A2 (A78 to map the blind compounds before R-Sim forecasts)
- **Applies:** L3-1..L3-4, README rules 5–9 and 7a/9a, M-001..M-005 rules, V-001, V-003, V-005.

## Rule

A78 writes the L3 mapping rows for the 15 compound-new blind compounds, under these conditions:

1. **The list.** It is the output of A65's committed script: the compound column only,
   rows with `part=blind`, stratum b, combinations split into their compounds. The rows
   cite the list file's sha256. A78 never opens `research/metric/`.
2. **Same rules.** Every rule that applied to M-001..M-005 applies, plus V-001. If A78
   sees a number, a curve or an ITP result for a compound, another writer takes that
   compound. The location of the sighting goes to A50 and A65 only.
3. **No flag information.** Nothing from the flag rule or A70's trace-search table
   reaches A78. It reaches A66 only if A50 routes it.
4. **`no_mapping` stays visible.** It is never filled by another writer or source.
5. **Order.** Each mapping is committed, checked by A66 and sealed by A50 before any
   R-Sim forecast for that arm. The forecast cites the mapping's sha.

## Applied in

- A78's blind-compound mapping rows (branch `literature/l3-blind-mappings`; first PR due
  2026-10-30, all by 2026-11-06). A66 checks each row PR against this ruling.
