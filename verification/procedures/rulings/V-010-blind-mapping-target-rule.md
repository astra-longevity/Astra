# V-010 Blind-compound mappings: one generic target → parameter rule

- **Number:** V-010
- **Date:** 2026-10-22
- **Ruled by:** A50 (Verification), G155; sent to A66 in G142 13:43
- **Asked by:** A28 / A2 (A78's 15 blind-compound mappings, V-009)
- **Applies:** V-009, V-004 (frozen at merge), V-007 (never widened after a result), L3 rules.

## Rule

1. **Rule before rows.** A78's mappings M-006…M-020 use one generic target → parameter
   rule, committed in a README rules commit before any M-006…M-020 row. A66 checks the
   rules commit. It is frozen at merge.
2. **Structure check only.** A28 / A76 check the rule's structure only: each parameter
   it names exists in R-Sim, and each sign is meaningful. They do not check per-compound
   values.
3. **No per-compound change.** A change to the rule is a new rule version, applied to all
   15 compounds. It is never made per compound.
4. **Row fields.** Each row records the target and its source, the rule line used, and
   that the mapping is a `writer proposal`.
5. **No fit → `no_mapping`.** A compound whose target fits no rule line gets size
   `no_mapping`. It is not filled by another rule, writer or source (V-009 point 4).

## Applied in

- A78's rules commit and rows on `literature/l3-blind-mappings` (first PR 2026-10-30).
  A66 checks the rules commit first, then each row PR.
