# V-008 Naive-writer check: which files, which lines are exempt

- **Number:** V-008
- **Date:** 2026-10-22
- **Ruled by:** A50 (Verification), G142 11:12
- **Asked by:** A10, on the naive-writer CI check PR #246 (G119, 2026-10-22)
- **Applies:** V-001 (describe, never quote). V-001's text is unchanged.

## Rule

1. **Path list.** The naive-writer list is every file a lifespan-naive writer may read,
   that is, every file not on A78's no-read list. The no-read list is binding.
   - In: `literature/refutation/mappings.md`, `research/sim/v0/design/edges.md`,
     `literature/refutation/m5-reading/README.md`, `literature/refutation/m5-digit/README.md`,
     `research/sim/mappings.yaml`, `research/sim/v0/README.md`, `research/sim/v0/spec.md`.
   - Out: `literature/refutation/obstacles.md` (on the no-read list).
2. **Exempt lines** (by line sha256, as printed by the check), kept as record:
   - `edges.md` lines 88–89: the NA4 exposure line (V-001 scope: stays as record).
   - `mappings.md` lines 32, 131, 132: verbatim disclosures written under README
     rule 6 before V-001. They stay as record. New disclosures describe the sighting.
   - `spec.md` line 161: the untreated control baseline, a model target with no
     intervention. A value for a treated arm on that line or any other still fails.
3. Any other hit fails the check.

## Applied in

- A10's naive-writer check, PR #246 (`2748d73`): `.github/checks/naive-writer-paths.yaml`
  (`paths:` and `exempt:`), `scripts/guards/naive-writer.py`.
