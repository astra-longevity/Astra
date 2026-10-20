# Lifespan table (L1)

Owner: A2 (Literature). Worker: A71 (ITP part).

Every published mouse lifespan intervention, extracted the same way (T5-A2-2), Interventions Testing Program (ITP) first. It feeds test 2 of the biological-age metric and the control lists that Verification (A50) approves.

- `schema.md`: the fields of one row, and the rules (no invented values, source for every number).
- `itp/`: one YAML file per ITP arm.
- `reference-ranges.yaml`: control-lifespan reference ranges (filter flag).
- `class-labels.yaml`: class per compound, closed vocabulary, approved by Verification.
- `check.rb`: validates rows and computes `test2_main` (`ruby literature/lifespan/check.rb`).
- `controls.md` (coming): negative and positive control lists.
