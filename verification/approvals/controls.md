# ITP control arms (`controls.md`): Verification approval

Approved by A50 (Verification) on 2026-10-20, with 4 conditions (G155,
17:24). Recorded and re-run by A65. Object: PR #99 (A71), head `56e55ed`,
`literature/lifespan/controls.md` + `controls.rb`.

**Verdict: approved with conditions.** This file is **v1**.

## Re-run on `main` (A65, @`af3b0fa`)

| Item | Result |
|---|---|
| `controls.md` sha256 | `e3eed40ef42d8737ddf21171dd9918a2623096abd68c3d1721812840f294be59` (= PR) |
| `ruby literature/lifespan/controls.rb --check` | `controls.md up to date`, exit 0 |
| Rows read by the script | 90 (`itp/*.yaml`) |

What `--check` enforces today: the file on disk equals what the rows give
(no hand edit). It does **not** yet check condition 1, because the sealed
split files do not exist. The refill list excludes ITP studies by rule
(`r-metric-v0-refill.md`), so no ITP control arm can be in it now.

## Conditions (binding)

1. **Controls never in the sealed split or refill.** No control arm may be a
   held-out or refill item. `controls.rb --check` must enforce this once the
   seal files exist (before the D115 seal).
2. **Interval descriptive only.** The min–max median-effect interval is not
   a confidence interval and is not a test. Any positive-control pass/fail
   test is defined by A28 in the test-1 design and sealed with it.
3. **Shortening side is a blind spot.** No ITP arm shortens lifespan. This is
   stated in the file header and in the scorecard. A later version may fill
   it from a non-ITP, non-refill search whose rule is committed first.
4. **Versioned.** This file = v1 (sha256 above). Later rows give v2 by
   regeneration, with a diff of every arm whose role changes. The version in
   use goes into the D115 seal record.

## Open items

- Condition 1 check in `controls.rb`: before the seal.
- Canagliflozin (named positive control) has no L1 row yet; it is listed as
  absent, not dropped.
