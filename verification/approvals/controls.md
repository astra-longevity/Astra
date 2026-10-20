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

## Compound-level check: PR #109 check (A65, 2026-10-20)

Object: PR #109 (A71), head `e0482a8`, stacked on #103 (rebuilt on main as
`5b92960`). Run on a clean copy of that commit. `controls.md` there = **v2**
(116 L1 rows; sha256
`d8a5f2618a66de9382a6971d5b31da5c5f73df6900b056eccd3851309d0b7533`); role
changes since v1 listed (C2015/C2016 arms added). v1 (`a10c1faa…14bd0`) stays
the approved v1.

| Test (split file injected via `CONTROLS_SPLIT_GLOB`) | `--check` |
|---|---|
| No split file (as on main) | exit 0 |
| Split with a control compound (glycine, metformin; rapamycin in YAML) | exit 1, names every control row with that compound |
| Split compound in L1 but not in `BLIND_COMPOUNDS` | exit 1 |
| Split file with no `compound` column | exit 1 |
| Split with canagliflozin only, or an unknown compound, after regeneration | exit 0 |

Canagliflozin: both C2016 rows are in L1, listed under "Excluded while
blind", in neither control list. Matches the A50 ruling.

| Precondition (17:34) | Met? | Note |
|---|---|---|
| Compound compared with the split | yes | Compound alone: any control whose compound has a blind arm fails. This is stricter than compound + cohort, so it covers the ruling. |
| Shown to fire | yes | Tests above, reproduced by me. The seal record logs the same run on the real split file. |

Two points for the seal (not blocking #109):

1. The header names the split file, so `controls.md` must be **regenerated
   once the split file is on main**. Its hash then changes. The version in
   the D115 seal record is the one regenerated with the sealed split.
2. Names are matched whole, after normalisation. A combination arm (e.g. B03)
   must be listed in the split file by its single compounds. A28/A67 must
   write it that way.

## Combination update: PR #109 re-check (A65, 2026-10-20)

#109 was rebased on main after #103 merged (`4c47e08`); new head `2942358`.
`--check` now splits combinations (`+`, `and`, `/`) into components on both
sides and normalises Greek letters. Re-run on a clean copy:

| Test | `--check` |
|---|---|
| No split file | exit 0; controls v2 sha256 `d6b83907c7e4902019cb31c320400c31945ea08eceafddb195172e7b404f4887` (supersedes `d8a5f261…`) |
| Split arm `Rapamycin + 17α-estradiol` | exit 1, blocks rapamycin and 17-alpha-estradiol controls |
| Split arm `acarbose/rapamycin` | exit 1 |
| Earlier firing tests (control compound, no `compound` column, YAML) | exit 1, unchanged |
| Clean split after regeneration | exit 0 |

Point 2 above is resolved in the script: a combination arm may be written
whole in the split file. Point 1 (regenerate once the split is on main; that
hash is sealed) stands.

## A50 ruling: seal-record items for controls (G140, 2026-10-20, 18:01)

Verdict on #109 endorsed. Controls v2 is **not** the seal version.

1. The sealed `controls.md` is the one regenerated after the split file is
   on `main`. The seal record holds its sha256, a `--check` run against that
   split (exit 0) and the firing tests re-run (exit 1). Any mismatch =
   refusal item.
2. Combination arms (e.g. B03) carry their compounds in the split file
   (whole name or single compounds; the script splits combinations since
   `2942358`). A blind arm with no compound = refusal item.

## Controls vs blind-split overlap: B03/B08 ruling (2026-10-20)

Finding (A71, draft PR #142): `controls.rb --check` exits 1 on main. Blind arms
B03 (rapamycin + 17α-estradiol, ITP 2023) and B08 (captopril, ITP 2024) share
compounds with 30 control arms.

### #142 check (A65)

#142 @`abe9aec` reads only `part=blind` rows of a CSV/TSV split with a `part`
column. Reproduced on a clean copy: a retro-only split (RF008, rapamycin) gives
0 compound hits; B03 + RF008 flags the rapamycin and 17α-estradiol controls.
`controls.md` sha unchanged (`5a894417…227d`). **The fix conforms.** The
remaining exit 1 is the real overlap above.

### Ruling (A50, G155, 20:28 UTC)

1. **Controls kept.** B03, B08, B04 and B05 are scored in the
   "compound seen in training" stratum. The headline result uses blind arms
   whose compound is new only. The two are never pooled.
2. **`--check` rule:**
   - compound **and** cohort overlap between a control and a blind arm → exit 1;
   - compound-only overlap → exit 0 only if that blind arm is tagged
     "compound seen in training" in `split.csv`, else exit 1;
   - a firing test for each case, logged in the seal record.
3. The condition-1 seal precondition (17:34) carries over to this rule.
4. Canagliflozin may re-enter the controls (new version + role diff). This
   supersedes the 17:30 ruling, point 2 (no canagliflozin in controls while
   B04/B05 are blind).

Seal items: B03/B08 tagged in `split.csv` (single encoding: a/b/c in `stratum`,
`compound seen in training` in `flag`, as asked of A28/A67 in G154); the
regenerated `controls.md` sha; `--check` exit 0 against the sealed split; both
firing tests exit 1. Missing any = refusal item.
