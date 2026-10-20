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

## v1 for the seal: PR #103 check (A65, 2026-10-20)

Object: PR #103 (A71), head `513efbc`, `controls.md` + `controls.rb` +
`controls-roles-v1.tsv`. Run on a clean copy of that commit.

| Item | Result |
|---|---|
| `controls.md` sha256 | `a10c1faa3fa4763504f04374834463c3984f82740d80c4bf7e62e1b0be414bd0` (= PR). **This is controls v1 for the D115 seal**; it replaces `e3eed40e…be59` (same tables, header added). |
| `controls.rb --check` | `controls.md up to date`, exit 0 |
| `controls-roles-v1.tsv` | 90 arms; "Role changes since v1": none |
| Condition 1 test: a control arm id appended to `refill-list.csv` | `--check` exit 1 (`arm id is in refill-list.csv`); exit 0 after restore |

| # | Met? | Note |
|---|---|---|
| 1 Out of split and refill | partly | Refill: by arm id and PMID, tested. Split: matches L1 row ids as text in `research/metric/v0/split*`. The split names blind arms `B01`–`B19`, not L1 row ids, so this match may never fire. Before the seal the check must also compare compound (and cohort) with the blind arms. |
| 2 Interval descriptive | yes | Header: not a CI, not a pass/fail bar; tests come from A28's sealed test-1 design. |
| 3 Shortening blind spot | yes | Header: harm detection unchecked; the scorecard must say so. |
| 4 Versioned | yes | v1 roles frozen; later versions by regeneration with a role-change section. |

## A50 ruling: canagliflozin (G155, 2026-10-20, 17:30)

1. The C2016 canagliflozin arm may enter L1: it is a finished arm and
   trains under split policy (c).
2. **No canagliflozin in `controls.md` while the canagliflozin arms B04/B05
   are blind.** `controls.rb --check` adds a compound-level test: it fails
   if a control arm's compound is the compound of any blind-split arm.
3. The scorecard reports B04/B05 as a separate stratum, "compound seen in
   training", apart from the blind arms whose compound is new.

Open before the seal: the compound-level test (point 2) and the split-id
mapping (condition 1 above) in `controls.rb`. Point 3 goes into the D115
seal record (sent to A28/A67, G154).

## A50 ruling: condition 1 is a seal precondition (G140, 2026-10-20, 17:34)

Before the D115 seal, `controls.rb --check` compares compound + cohort (not
only row-id text) with the split file, and shows that it fires: a test that
injects a blind arm's compound/cohort into the controls gives exit 1, and
that run is logged in the seal record. Missing at seal time = refusal item.
