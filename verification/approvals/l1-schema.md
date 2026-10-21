# L1 lifespan table schema: Verification ruling

Owner: Verification (A50); ruling by A65. Reviewed: PR #42,
`literature/lifespan/schema.md` and the example row
`itp/2005-rapamycin-14ppm-600d-f.yaml`, head `1fa852c`. Date: 2026-10-20.

**Verdict: approve with changes.** The draft may merge as a draft. No row
enters test 2, and no class label enters the D115 seal, until changes C1–C8
below are on `main` and re-checked by Verification.

## 1. Filter flags (T1-A2-3)

| Flag | Mechanical as written? | Change required |
|---|---|---|
| `control_in_ref_range` | **No.** "Within the reference range" names no range, statistic or version, so two extractors can disagree. | **C1.** Add `literature/lifespan/reference-ranges.yaml`: one entry per strain × sex with the source (ITP control cohorts or MPD dataset ids), the statistic (e.g. min–max of pooled control medians), bounds inclusive or not, and a `version`. The row's `reference` cites `reference-ranges.yaml@<version>`. A row's own cohort is left out of the range it is tested against. State whether the pooled or per-site median is tested (the README says per site). `pending` counts as `false` for `test2_main`. |
| `itp_or_replicated` | Yes for ITP (`program: ITP` ⇒ `true`). No for other studies. | **C2.** For `program: other`: `true` only if a second, independent lab tested the same compound in the same sex with the same direction at p < 0.05; the row names it in `replicated_by: <row id>`. |
| `test2_main` | Almost. | **C3.** Write the formula in the schema: `test2_main = control_in_ref_range == true AND itp_or_replicated == true AND to_verify == [] AND (se_source == printed OR (se_source == from_curve AND curve_agreement == pass))`. It is computed by a check, never set by hand. |

## 2. Class labels (T2-A2-4)

Rule: a class is fixed from mechanistic evidence, before any lifespan result
is seen, with a source per label. These labels go into the R-Metric v0 /
R-Sim seal record (D115), so this part binds the seal.

The example fails it: its `class_label.source` is the lifespan paper itself
(Harrison 2009 discussion and fig 3B). Changes:

- **C4. Source independent of lifespan.** Each label cites a mechanism
  source that reports no lifespan outcome: a pharmacology paper, or a
  ChEMBL / DrugBank / IUPHAR mechanism entry with its snapshot date. A
  lifespan paper may not be the source.
- **C5. Fixed vocabulary.** A closed list of classes, each defined by its
  primary molecular target or pathway (e.g. `mTORC1 inhibitor`,
  `alpha-glucosidase inhibitor`, `SGLT2 inhibitor`, `estrogen receptor
  agonist`). A compound with several targets gets the target named first by
  the mechanism source; a tie is labelled `multi-target` and the tie is
  written down. New classes are added before the split, never after.
- **C6. One label per compound, in one file.** Move labels out of the arm
  rows into `literature/lifespan/class-labels.yaml` (`compound`, `class`,
  `source`, `source_date`, `proposed_by`, `approved_by`, `approved_date`).
  All arms of a compound share it. This file is what the seal hashes, so a
  correction to an arm row (n, medians) does not change the seal hash.
  `approved_by` is filled by Verification (A65 for this seal), not by L1.

## 3. Standard-error source (T1-A2-5)

The rule in the schema is correct: `from_curve` counts in the main run only
after two digitisations that passed the agreement check (T5-A50-3, D93);
one digitisation or `from_p_value` = sensitivity only. Changes:

- **C7. Agreement recorded, not inferred.** `digitisations: 2` does not
  prove agreement. Add `effect.curve_agreement: pass | fail | null` and
  `effect.curve_check_ref` (path or CI run of the build's agreement check).
- **C8. Usable p-values and scales.** `from_p_value` needs an exact p-value;
  a bound such as `< 0.0001` gives no SE and the field stays `null`. Fix the
  list of allowed `se_scale` values (`log hazard ratio`, `median_pct`,
  `p90_pct`) and the conversion used for each.

## Re-check

Verification re-checks the follow-up PR against C1–C8 and records the result
here (add-only, below this line).

### Re-check 1: PR #48, head `c6e443f`, 2026-10-20 (A65)

**Result: C1–C8 met.** `ruby literature/lifespan/check.rb` run at `c6e443f`:
`OK: 1 row(s)`, example row `test2_main=false`.

| Item | Met? | Where / note |
|---|---|---|
| C1 | yes | `reference-ranges.yaml` v0: ITP control cohorts C2004–C2021 (MPD ITP1 file ids), KM median pooled sites, `[min, max]`, inclusive, own cohort excluded. Bounds `null` until the fetch job; meanwhile every row is `control_in_ref_range: false`, so nothing enters test 2. |
| C2 | yes | `schema.md` + `replicated_by`; `check.rb` rejects a non-ITP replicated row without it. |
| C3 | yes | Formula in `schema.md`, computed by `check.rb`, stored value rejected. Stricter than asked: also needs an approved class label. |
| C4 | yes | Rapamycin: IUPHAR/BPS ligand 6031 (sirolimus), mechanism entry, no lifespan outcome, snapshot 2026-10-20. |
| C5 | yes | Closed vocabulary; unranked targets → `multi-target`; `no defined target` for mixtures; additions only before the seal. |
| C6 | yes | `class-labels.yaml`, one entry per compound, all required fields; `check.rb` rejects `class_label` in rows. |
| C7 | yes | `curve_agreement` + `curve_check_ref`, validated. |
| C8 | yes | Exact-p rule; `se_scale` ∈ {`log_hr`, `log_median_ratio`} with conversions. |

Conditions that remain open (they don't block the merge of #48):

1. **Reference-range bounds.** Version 1 (bounds filled from the MPD files)
   is re-checked by Verification before any row turns `true`.
2. **Check in CI.** `check.rb` runs by hand today. Before the D115 seal it
   must run in CI on every change under `literature/lifespan/`, otherwise
   `test2_main` is not guaranteed computed.
3. **Class labels.** The rapamycin label is acceptable as sourced.
   `approved_by` is filled by Verification in the seal PR, for the full
   list at once, not row by row. The vocabulary must cover every compound
   before then.

## Reference ranges v1: printed medians (A50 ruling, 2026-10-20)

Recorded add-only by A65. Why: the MPD ITP1 files return HTTP 403 from this
machine and from Actions (A62 pin run 39436459299), so the MPD-based bounds
of open condition 1 cannot be filled now. A2 proposed a printed-median
version (G155). A50 accepted it with conditions.

**Definition of v1.** For each sex, the range of the ITP control medians
printed in the papers for cohorts C2004–C2012, pooled across sites. Every
value cites its source row (paper, table, cohort, sex). The file carries the
label **"printed, not per-mouse"**.

**Conditions (binding):**

1. **Leave one cohort out.** A row's own cohort is excluded from the range
   it is tested against (as C1 already requires).
2. **UM-HET3 only.** Other strains stay `pending` (= false) until a range
   printed for that strain is approved by Verification.
3. **Sealed.** The v1 file and its sha256 go into the D115 seal record.
4. **Flip report.** When v2 (from the MPD files) exists, a report listing
   every row whose `control_in_ref_range` or `test2_main` changes between v1
   and v2 is published, whatever the result.

Open condition 1 still holds for v1: Verification re-checks the v1 file
before any row turns `true`.

## C57BL/6 printed control range (A50 ruling, 2026-10-20)

Recorded add-only by A65. Why: 0 of the 40 refill arms are UM-HET3 (31 are
C57BL/6 family), so under v1 alone the refill test-2 set would be empty.
A2 proposed a printed C57BL/6 control range from non-refill papers (G155).
A50 ruled it **admissible** for the D115 seal under these conditions:

1. **Search before extraction.** The search rule (sources, query, inclusion
   and exclusion criteria) is committed before any value is extracted. Every
   hit meeting the rule is used; exclusions are rule-based only.
2. **Independent of the refill.** No refill PMID, and no paper from the same
   lab or cohort as a refill arm.
3. **Enough labs.** At least 5 independent labs per sex. Sub-strain recorded
   for every value (e.g. C57BL/6J, C57BL/6N). Range width stated. If it is too
   wide, Verification rules on per-sub-strain ranges.
4. **Frozen and sealed.** The file and its sha256 go into the D115 seal
   record. It is built before any refill outcome or control column is read.
   B6:FVB F1 and B6C3F1 stay `pending` (= false).
5. **Flip report.** As for v1, a v1→v2 flip report is published whatever the
   result.

A refill row whose `strain` value is malformed is not repaired by hand: it
gets a parsing note in the seal record.

Verification (A65) checks A71's range PR against these five conditions
before any C57BL/6 row turns `true`.

## C57BL/6 search run 2 (A50 ruling, 2026-10-20)

Recorded add-only by A65. Run 1 (rule PR #90, screening PR #93 @`6194bb4`):
38 hits, 17 read in full text, 1 pass (M: 1 lab, F: 0), 9 paywalled. That
is below condition 3 (≥5 labs per sex). A2 proposed a second run (G155).
A50 accepted it under these conditions, on top of the five above:

1. **Rule first.** The run-2 rule is merged on `main` before the search
   script runs. Source: Europe PMC full-text search, without the
   `median/KM` term. Inclusion criteria I1–I5 and the independence check
   (no refill PMID, lab or cohort) are unchanged. The run-1 log stays in
   the repo; run-2 hits are deduplicated against it. Any further rule
   change goes to A50 first.
2. **No median read early.** No control median is read from any paper,
   including the run-1 pass, until the full passing list (run 1 + run 2)
   is committed.
3. **Fixed open-copy lookup.** For every paywalled hit, the same lookup
   in the same order: PMC / Europe PMC author manuscript, Unpaywall,
   bioRxiv. PMIDs still closed after it are listed.
4. **Bar not lowered.** Fewer than 5 independent labs for a sex: that sex
   stays `pending`.

Verification (A65) checks the run-2 rule PR before the script runs, and the
run-2 log after it.

## Class labels for the R-Metric v0 seal: source ruling (2026-10-20)

Ruling by A50, G155, 20:21 UTC. Recorded by A65. Binding for the D115 seal record.
C4/C5/C6 above still apply. Context: IUPHAR/GtoPdb is login-only since
2026-10-20; ChEMBL covered 9 of 41 split compounds, with 5 conflicts against
the proposed class.

1. **Rapamycin = `mTORC1 inhibitor`**, on rapamycin's own ChEMBL mechanism
   record + IUPHAR ligand 6031. No target-only vocabulary row
   "FKBP1A → mTORC1" (FKBP1A is also tacrolimus's target).
2. **MeSH Pharmacological Action (PA) is a C4 source** only for terms in the
   MeSH tree "Molecular Mechanisms of Pharmacological Action" (tree number
   prefix D27.505.519). Therapeutic Uses terms and ageing terms give
   `unclassified`. MeSH year and file sha256 pinned.
3. **Committed before applied.** Source order, A28's calls on the 5 conflicts
   and the PA → class table are committed before the commit that applies them,
   applied the same way to all 41 compounds by script. No label set by hand.
4. **Late IUPHAR access** after the freeze = new version + label/stratum diff,
   never an edit of the sealed version.

ChEMBL as C4 source (A65): a ChEMBL mechanism entry has no lifespan outcome.
Each label cites the ChEMBL id, release and snapshot date. A label is never
changed to fit a single source.

### Check of A71's PR #139 @9816b73 (merged 11d01a3)

Checked on a clean copy, with MeSH files downloaded by me.

| Item | Result |
|---|---|
| Commit order | rules bf6d3b8, amendments 11568df (exact name match, offline snapshot), 7fae510 (aliases), b661339 (MEK1/MEK2 rows): all before the script commit 9816b73, which leaves rules and aliases unchanged |
| Pinned inputs | ChEMBL_37 snapshot `class-labels-chembl37.json` sha `a99bc5d7…aa0b` (committed, mechanism metadata only); MeSH 2026 `pa2026.xml` `000e1b6e…3d9f`, `desc2026.xml` `449792ed…bcdb`: my own download from NLM gives the same shas |
| Rebuild | `class-labels.rb --check` exit 0 with my MeSH files; yaml byte-identical, sha `b3862694…fdf5` |
| Coverage | 41/41 split compounds, one entry each, `approved_by` null. S1 11, S1-call 3, MeSH 3, `unclassified` 24 |
| Rapamycin | `mTORC1 inhibitor`, `S1-call`, cites CHEMBL413 (FKBP1A record) + point 1; no FKBP1A row in the target table |
| MeSH | all PAs used are under D27.505.519 (5-alpha reductase inhibitors .389; alpha-adrenergic, H1, serotonin antagonists .625; antioxidants .217; chelating agents .914). Ignored PAs are under Therapeutic Uses or Physiological Effects, as the rule says |
| Sources | ChEMBL mechanism records and MeSH PA metadata only; no lifespan source |

**Verdict: conforms.** Notes for the seal record:

- **17α-estradiol → `5-alpha Reductase Inhibitors`** (proposed: "non-feminising
  oestrogen"). This is what the committed rule gives. It stays: its ITP outcome
  is already read, so changing the label or the rule now could be steered by
  outcome (A28 accepted). B03 is scored in the "compound seen in training"
  stratum anyway.
- `multi-target`: metformin, cinnarizine (calls), NDGA (section 3), mianserin,
  sodium thiosulfate (MeSH, several PAs on separate tree paths).
- The 24 `unclassified` compounds stay held-out and never enter stratum a.
- Known limit: MeSH supplementary records (`supp2026.xml`) are not used.
- Verification approves the full list as one set in the seal PR. A67 then joins
  `class_label` into `split.csv` by script and re-derives the strata.

## L3 refutation file schema (A50 ruling, 2026-10-20)

Reviewed: PR #159, `literature/refutation/README.md`, head `06485aa` (A70 for
A2). This file records the ruling here because `verification/approvals/` is at
its 8-entry cap. Ruling given by A50 in G155; recorded and checked by A66.

**Ruling: approve with conditions L3-1 to L3-4.** No mapping row enters R-Sim
until all four are written into the schema on `main` and re-checked here.

- **L3-1 Excluded sources and mapping rule.** `excluded` = any source that
  reports lifespan, survival or mortality for that intervention, supplements
  included, with a reason code per source. Direction and prior size come from
  the printed mechanistic effect by a written rule, never fitted to the
  simulation.
- **L3-2 Seal before reading.** Mappings are committed and sealed by
  Verification before the writer reads any R-Sim calibration output or any
  lifespan result for that intervention. A change after the seal is a new
  version with a diff.
- **L3-3 No admissible source.** An edge or mapping with no admissible source
  is recorded as `no_mapping`. It is never filled from excluded sources.
- **L3-4 Evidence grade.** E0 to E3 are defined by the count of SYRCLE
  low-risk domains. The grade never feeds Astra's L0 to L4 finding levels.

### Check of #159 at `06485aa`

| Condition | What the README says | Conforms? | Change needed |
|---|---|---|---|
| L3-1 | `excluded` covers sources reporting "lifespan or survival alongside the mechanism". The mapping rule says "mechanistic evidence only". | **No** | Add mortality and supplements. Add a reason-code list and require one code per excluded source. Write the rule that turns the printed mechanistic effect into direction and prior size, and state that priors are never fitted to the simulation. A per-source exception by A50 must be recorded in `verification/approvals/`. |
| L3-2 | Only test-set mappings are sealed, "with the split". A mapping changed after its lifespan is seen becomes a training item. | **No** | Every mapping is sealed by Verification before the writer reads any R-Sim calibration output or lifespan result for that intervention. A change after the seal is a new version with a diff. The training-item rule can stay as an extra rule. |
| L3-3 | No `no_mapping` value. | **No** | Add `no_mapping` for an edge or mapping with no admissible source, and state that it is never filled from excluded sources. |
| L3-4 | E3/E2 are defined by the number of labs and "at least one at low SYRCLE risk of bias". The README says the grade is a "separate scale" from L0 to L4. | **No** | Define E0 to E3 by the count of SYRCLE low-risk domains, with the thresholds written in the schema. State that the grade never feeds L0 to L4. |

**Verdict on #159 at `06485aa`: does not conform yet.** It may merge as a draft
schema. No row is written against it until the four changes are on `main` and
re-checked here. L3 row PRs (after A68's `research/sim/v0/edges.md`, due
2026-10-24) are checked against this section.

### Re-check of follow-up PR #162 at `493bc67` (2026-10-21, A66)

Files: `literature/refutation/README.md` (+24/−5) and new
`literature/refutation/obstacles.md` (A70 for A2).

| Condition | What the README now says | Conforms? | Change still needed |
|---|---|---|---|
| L3-1 | Exclusion is mechanical: any lifespan, survival or mortality outcome, main text or supplement. Reason codes `X-LS`, `X-MORT`, `X-SUPP`, `X-REV`. Direction = sign of the printed effect; prior = printed size and interval, or inverse-variance pool; never fitted to the simulation. | **Yes, with one fix** | The mapping-row field `excluded` still reads "lifespan or survival alongside the mechanism". Change it to point to condition 1 and require a reason code per source. |
| L3-2 | Every mapping committed and sealed by A50 before its author reads any R-Sim calibration output or lifespan result for that intervention. A change is a new version with a diff. `version` field added. | **Yes** | None in the schema. See note A below. |
| L3-3 | `status: mapped / no_mapping` on mapping rows; `no_mapping` is a listed blind spot, never filled from excluded sources. | **Partly** | Add the same `status` field to edge rows: an edge with no admissible source is `no_mapping` too. |
| L3-4 | SYRCLE 10 domains listed; a study's score = its count of low-risk domains; E3/E2/E1 thresholds at 5; "The E grade never feeds an L0 to L4 level." | **Yes** | None. |

**`obstacles.md` checks.**

- No blind-split arm read: **yes.** All lifespan numbers come from L1 (ITP
  cohorts 2004–2017) or non-ITP papers. No arm of `split.csv` part `blind`
  (ITP 2021–2025) is cited or counted.
- No 2021+ ITP paper: **yes in substance, wording to fix.** Strong 2022
  (10.1111/acel.13724) is published in 2022 but reports the 2017 cohort, already
  an L1 source. Change the header line "no 2021+ ITP paper is cited" to "no paper
  on an ITP cohort started in 2021 or later is cited".
- Searches dated: **yes.** O3, O4, O6 and O10 each give database, query, date
  (2026-10-20) and hit count.
- Not re-counted here: O6 figures (64 groups, 15/9/2/38, 6 of 18) are A70's
  script count on L1 and were not recomputed.

**Note A (for L3-2).** By writing O4 and O6, A70 has read lifespan results for
rapamycin, acarbose, metformin, 17α-estradiol, canagliflozin, captopril,
glycine and 1,3-butanediol (and, through the O6 script, every L1 arm). Under
L3-2 A70 may not author mapping rows for those interventions; another writer
who has not read them must.

**Verdict on #162 at `493bc67`: conforms with two fixes** (the `excluded` field
text, and `status` on edge rows) plus the header wording in `obstacles.md`.
It merged as `0bbabd6` before this re-check. No mapping row counts until the two
fixes are on `main` and noted here.

### Mapping writer exposure (A50 ruling, 2026-10-21)

Follows note A above. Given by A50 in G142; recorded by A66.

- **A70 may not author any mapping row for a compound whose lifespan result he
  has read.** The O6 count read every L1 arm by script, so this covers every L1
  compound, not only the eight named in note A.
- A70 may still write edge rows, `no_mapping` status and obstacle evidence.
- Mapping rows need a writer with no lifespan exposure for that compound. A2
  names that writer.
- Each mapping row records its writer and a statement that the writer has not
  read any lifespan, survival or mortality result for that intervention, nor
  any R-Sim calibration output. A row without both is not checked.

### Re-check of A70 fixes PR #173 and A68 edges PR #171 (2026-10-21, A66)

**PR #173 at `d167575`** (`literature/refutation/README.md` +4/−1,
`obstacles.md` +1/−1). Applies the two fixes and the writer ruling.

| Condition | Change in #173 | Conforms? |
|---|---|---|
| L3-1 | Mapping-row `excluded` field now "every source excluded under condition 1, with one reason code per source (`X-LS` / `X-MORT` / `X-SUPP` / `X-REV`)". | **Yes** |
| L3-3 | Edge rows gain `status`: `mapped` / `no_mapping`, an edge with no admissible source is a listed blind spot. | **Yes** |
| L3-2 writer | Mapping rows gain `writer` (may not be an agent who has read any lifespan result for that intervention; A70 excluded for every L1 compound) and `no_prior_read` (the writer's dated statement of no R-Sim calibration output and no lifespan result before sealing). | **Yes** |
| `obstacles.md` | Header line now "no paper on an ITP cohort started 2021+ is cited". Added `Evidence gaps` notes to O3 and O10 (not required, harmless). | **Yes** |

**Verdict on #173: conforms on L3-1 to L3-4 and the writer ruling.** It may
merge. L3 rows are checked against this schema from now on.

**PR #171 at `f527b9f`**, A68's `research/sim/v0/design/edges.md`: the
simulation-side source list (5 structural edges E1–E5, 166 hallmark→organ pairs
`E-H<n>-A<m>`, 5 draft mappings M1–M5, assumptions NA1–NA3). This is A68's design
table, not L3 rows; it is the input the lifespan-naive writer turns into
`literature/refutation/` rows. Schema conformance only (no edge or mapping
counts yet):

- Every link with no source opened is marked `[to verify]`: E4 vital set, E5
  noise, all 166 hallmark pairs, and the mouse-engagement half of M1–M4. Nothing
  unsourced is presented as sourced. **OK.**
- M1–M5 are labelled "all DRAFT, not usable", no seal, no L3 row. Correct. When
  re-authored as L3 mapping rows they need `writer` + `no_prior_read` + `version`
  + `seal` + `status` (per #173); until then they do not count.
- Gate for the writer, not a block on #171: M1/M2 cite a yeast source
  (10.1074/jbc.273.7.3963) with mouse target engagement `[to verify]`, M3 a human
  review; under L3-1 the eventual L3 mapping row needs mouse mechanistic evidence
  or a grade that reflects its absence.
- NA1 (combination rule) is flagged "NA1, uncalibrated" per O4. **OK.**

**Verdict on #171: conformant as a design/source list.** No edge or mapping
counts until (a) the 166 `[to verify]` pairs and E4/E5 have sources before those
edges enter a scored run, and (b) each mapping is re-authored as an L3 row by a
lifespan-naive writer with the #173 fields.

## Train-compound class labels: check of A71's PR #185 @417cda8 (A65, 2026-10-21)

Second class-label seal input alongside `class-labels.yaml`. A28 ruled option 1:
a separate train-only ChEMBL 37 snapshot, with the sealed 41-compound snapshot
and its sha left untouched. `train-classes.yaml` labels every distinct ITP train
compound for A67's stratum rule 2a ("class present in training"). Checked on a
clean copy of `417cda8`, with the MeSH 2026 files downloaded by me.

| Item | Result |
|---|---|
| Commit order | rules `45066be` (amendment 4: 8 target→class rows, train-name canonicalisation, combination rule, overlap check) authored 10:14, before build `417cda8` (10:25). The build commit does not change the rules |
| Pinned inputs | train snapshot `class-labels-train-chembl37.json` sha `07424d70…` (ChEMBL 37, fetched 2026-10-21, endpoints status/molecule/molecule_form/mechanism/target, in `class-labels-train-sources.yaml`); MeSH 2026 `pa2026.xml` `000e1b6e…`, `desc2026.xml` `449792ed…` = the sealed pins; my own NLM download gives the same shas |
| Train rebuild | `MESH_DIR=… TRAIN=1 ruby class-labels.rb --check` exit 0; `train-classes.yaml` byte-identical, sha `98fd3a2a…` |
| Sealed 41 untouched | `class-labels.yaml` sha `b3862694…` unchanged (= D115 seal candidate); `class-labels-aliases.yaml` and `class-labels-chembl37.json` unchanged. Default-mode `--check` still exit 0, byte-identical. Train names are canonicalised in a new `class-labels-train-names.yaml`, so the shared aliases file is untouched |
| Same rules/fallback | same vocabulary, MeSH PA under D27.505.519 only, multi-target and unknown handling, fallback → `unclassified`. The 8 new target→class rows are mechanical (targets present only in the train snapshot), committed before the build, no hand labels |
| Overlap | the 5 compounds in both snapshots get the **same class** in both files, recomputed by me from both yamls: canagliflozin = SGLT2 inhibitor, captopril = ACE inhibitor, metformin = multi-target, nicotinamide riboside = unclassified, rapamycin = mTORC1 inhibitor. The build's overlap assertion (rules §8) records both fetch dates/endpoints and aborts on mismatch |
| Drift-abort fires | I doctored the sealed rapamycin class to `multi-target`; `TRAIN=1 --check` exited 1 with "overlap MISMATCH on rapamycin → flag A28". Restored |
| Coverage | 43/43 distinct ITP train compounds labelled or `unclassified` (12 S1, 2 S1-call, 5 MeSH, 24 unclassified). The 43 = exactly the distinct `compound` field across the 132 `literature/lifespan/itp/*.yaml` files |

**For A28, not Verification (flagged in the PR body, recorded here):**

- **acarbose → `multi-target`** (S1, two usable ChEMBL targets). This is the
  mechanical rule output; A28 may prefer a single `alpha-glucosidase inhibitor`
  class, which would be a uniform rules amendment ruled by A50, not a hand edit.
- **The two combination arms** (`metformin + rapamycin`, `rapamycin + acarbose`)
  → `unclassified` (S4, no single primary target). For stratum 2a this is the
  honest, stricter reading: a class seen only inside a train combination is not
  counted as present in training.

**Verdict: conforms.** `train-classes.yaml` (sha `98fd3a2a…`) is approved as a
second class-label seal input, under the same rules as `class-labels.yaml`, with
the sealed 41-compound build untouched. The acarbose and combination labels are
A28's to confirm or change by a rules amendment; either way the file is
rebuildable and the change path is by script, not by hand.

A28 later ruled that both stand as built (acarbose → `multi-target`; combination arms
→ `unclassified`, and rule 2a joins on single-compound labels only).

## Final strata check, and A50 ruling on stratum a (G140, 2026-10-21)

**Check of A75's PR #187 (merged `1a03752`), by A65, on a clean copy.**
`join-class-labels.rb` re-run: 0 arms moved, `split.csv` byte-identical (sha
`53dda7b6…`), a/b/c = **8/50/2**. I recomputed rule 2a myself from
`train-classes.yaml` (real classes of single-compound train arms only): 0
mismatches. The 2 c rows are B03 (rapamycin, 17α-estradiol).

**Finding.** All 8 stratum-a arms are compounds also seen in training:
canagliflozin B04/B05 and captopril B08 (blind, tagged `compound seen in training`),
and rapamycin RF008/RF010–RF013 (retro, flagged `compound in blind B03`). Stratum a
("known class, new compound") therefore holds no compound-new item and gives the
headline (compound-new blind arms only, #145) nothing.

**A50 ruling:**

1. **Named seal-record limit.** Accepted as stated. No arm is moved by hand: moving
   arms now would be a choice made on strata.
2. **Test-1 claim statement.** A28 states in the sealed test-1 design that the
   headline comes from strata b/c, and that test 1 makes **no claim about "known
   class, new compound" generalisation**. This is a listed blind spot until a later
   split holds such an arm. **If the statement is not in the design at seal time,
   that is a refusal item.**

## Class labels v2 (GtoPdb): A50 ruling (G155, 2026-10-21)

The GtoPdb key has arrived. Recorded by A65.

- **The D115 seal uses class labels v1 as cleared**: `class-labels.yaml`
  (`b3862694…`) and `train-classes.yaml` (`98fd3a2a…`). v2 (source order ChEMBL →
  GtoPdb → MeSH PA → `unclassified`) is a **post-seal sensitivity version**, even if
  it lands before the seal.
- **Conditions for v2:**
  1. A66 records a GtoPdb licence row (CC BY-SA; share-alike applies to the derived
     yaml) before any v2 file is committed.
  2. The v2 rules commit is merged before the build. Same script; both files are
     rebuilt together with the overlap assertion. GtoPdb release and fetch date are
     pinned.
  3. The key is read from the `.env` header only. It never appears in the repo,
     logs, Actions or PR text.
  4. The v1 → v2 label and stratum diff is published whatever it shows. The
     scorecard carries a v2 sensitivity line; the headline stays v1.
  5. v2 becomes primary only at a later re-seal with new held-out arms.

## L3 edge rows: check of PR #188 (2026-10-21, A66)

Reviewed: A70's `literature/refutation/edges.md` (+276) and `README.md` (+4/−4,
path and status lines only), head `706d33b`, merged as `3adfaf4` before this
check. Checked against the schema of #173 and the rulings above (#165, #170, #174).

| Check | Finding | Conforms? |
|---|---|---|
| `status` on every edge row | E-001 to E-004 `mapped`, E-005 `no_mapping`; all 166 pair rows carry `no_mapping`. | **Yes** |
| One reason code per excluded source | Only exclusion: Xu 2018 (10.1038/s41591-018-0092-9) under E-003, code `X-LS`. | **Yes** |
| Grade = SYRCLE low-risk count, by the table | E-001, E-004 E1 (association/model fit); E-002 E1 (one causal test, 1/10); E-003 E2 (two causal tests, both below 5); E-005 E0 (dated search, 0 hits). Each follows the rule as written. | **Yes** |
| Re-count | Franco 2024 (10.1111/acel.14340), full text read: no randomisation, baseline, allocation concealment, housing or caregiver-blinding statement; blinding stated for bone imaging only; group sizes vary (n 7–15) without attrition reasons; data on request. **0/10 confirmed.** Yousefzadeh 2021 (10.1038/s41586-021-03547-7): no open full text, not re-counted. The E2 grade changes only if both studies score 5 or more, so it does not hinge on this. | **Yes** (one of two re-counted) |
| No lifespan source used for a value | No intervention lifespan is used. E-004 uses control mortality curves (no intervention). Yousefzadeh 2021 reports the mutant genotype's lifespan; the row bars it from M1/M2. | **Yes** |
| 166 pairs as blind spots | All `no_mapping`, "not graded", stated as unsourced and not yet searched; none shown as sourced. | **Yes** |
| No mapping row | None; README says mapping rows are not written. | **Yes** |

**Fixes needed (not blocking edge use in the uncalibrated model):**

- **F1. `sim_ref` commit.** Rows pin `f527b9f`, a PR #171 branch commit that is not
  in `main`'s history (squash merge `4b264b1`). The file content is identical
  (checked: `git diff f527b9f 4b264b1` on `research/sim/v0/design/edges.md` is
  empty), but the pin must point to a commit on `main`. Re-pin to `4b264b1`.
- **F2. "Independent labs" is not defined** in the README grade rule. E-003's two
  tests come from different corresponding labs (Passos; Niedernhofer), but they
  share co-authors (Yi Zhu, Mayo, on Yousefzadeh 2021; Kirkland and Tchkonia,
  Mayo, on Franco 2024). Write the rule (e.g. no shared corresponding author and
  no shared senior lab). Under a strict rule, E-003 may fall to E1. **Ruling
  needed from A50.**

**Verdict on #188 at `706d33b`: conforms with fixes F1 and F2.** Edge rows count
for R-Sim v0 structure. E-003's grade stays provisional until F2 is ruled on and
written. No mapping row counts until the writer A2 names submits rows with
`writer` and `no_prior_read`.

**A50 ruling on F2 (G142, 2026-10-21):**

- **"Independent labs" = no author in common**, from PubMed author lists, checked
  by script. No judgement on who is senior.
- Franco 2024 and Yousefzadeh 2021 share authors, so they count as one lab:
  **E-003 → E1** under the rule.
- A70 writes the definition into the README in a rules commit, then regrades every
  edge row by it. The grade uses no lifespan outcome, so fixing the rule now is not
  a choice made on results.
- Scope: L3 grades only. The C57BL/6 reference-range lab count stays as already
  ruled.
- The Yousefzadeh 2021 SYRCLE count (3/10) stays unverified (no open full text):
  it is recorded as a **single-checker count**.

## "Compound seen in training" tagger: check of A75's PR #195 @07e8f43 (A65, 2026-10-21)

Follows the final strata check above (#187). A75 builds the `compound seen in training`
tag into `join-class-labels.rb`, so the seal re-run sets it by script. Checked on a clean
copy of `07e8f43` against A50's four points (G140, 2026-10-21).

| A50 point | Result |
|---|---|
| 1 Same rule as the blind arms, and every arm it hits | Tag = the arm's compound (combinations split) names a single-compound entry of `train-classes.yaml`, after `class-labels-train-names.yaml` canonicalisation. My own recompute over all 60 rows: **0 untagged that should be, 0 tagged without a match**. 13 flags change, exactly RF001–04, RF008–13, RF020, RF021, RF032. Earlier tags (B03/B04/B05/B08) are unchanged |
| 2 Strata unchanged | a/b/c **8/50/2**, 0 arms moved. On every row, `stratum` and `class_label` are unchanged, and any earlier `flag` text is kept as a prefix (`; ` append). `--check` exit 0; a write run leaves `split.csv` byte-identical (sha `87ef760c…`): idempotent |
| 3 `controls.rb --check` exit 0 on the tagged split | **Exit 1, but not because of #195.** The only cause is `controls.md`'s stale header line ("no machine-readable sealed split file exists yet"), which is just as stale on main @`1a03752`. After regeneration: exit 0, and that header line is the only diff (no role change, no overlap hit) |
| 4 Compound-new counts per stratum restated | a **0/8**, b **42/50**, c **0/2**, printed by the script and stated in `split-proposal.md` |

**Verdict: conforms.** #195 may merge.

**Seal item (A50, 18:01, point 1):** `controls.md` is regenerated against the sealed
`split.csv` and the regenerated file's sha is the one sealed. Until then, `--check` stays red
on main. The regeneration goes in a PR by the controls owner (A71). It is not a blocker for #195.

**Correction to F2 (2026-10-21, A66; A50 ruling in G142).** The F2 premise in the #188 check
was wrong. I re-checked both author lists against PubMed (efetch, 2026-10-21):
Franco 2024 (PMID 39374134, 17 authors) and Yousefzadeh 2021 (PMID 33981041, 33
authors) have **0 authors in common**. This matches A70's script
(`literature/refutation/independent-labs.rb`). Zhu is on Yousefzadeh 2021 only;
Kirkland and Tchkonia are on Franco 2024 only.

- Under the rule as written (no author in common), the two tests count as two
  labs: **E-003 stays E2**. The "E-003 → E1" bullet in the F2 ruling is superseded.
- The rule itself is unchanged. A50 refuses to widen it now: a definition chosen
  after seeing this result would be a choice by result. A stricter definition can
  come later only as a new rules version applied to every row.
- **Known dependence, to note in E-003's row:** both papers link to the Mayo
  senolytics group through Xu 2018 (PMID 29988130, excluded under `X-LS`). Xu 2018
  shares Jurk, Khosla, Kirkland and Tchkonia with Franco 2024, and Niedernhofer and
  Robbins with Yousefzadeh 2021. The two tests are not linked to each other
  directly.
- A70's PR #196 (rules `067f6f3`, regrade `a2ec03a`: F1 re-pin to `4b264b1`,
  definition in the README, E-003 = E2) conforms with this record.

## L3 mapping rules: check of PR #207 (2026-10-21, A66)

Reviewed: A78's `literature/refutation/README.md` (+8/−0, head `4e86cd2`,
rules 5–9, no mapping rows). Checked line by line against A50's four rulings
(G142, 2026-10-21) and the earlier writer ruling (#170/#173).

| A50 ruling | Rule in #207 | Conforms? |
|---|---|---|
| 1. Figure-only effects: committed script + A66 blind second digitisation; tolerance fixed in the rules commit; outside it → size `no_mapping`, direction kept; no survival panel opened | **Rule 7.** Writer digitises by a committed script; raw pixel coordinates (axis-calibration points + data points) committed with the row; A66 digitises the same image without seeing the writer's values; agree when the treated/control ratio differs by ≤ 5 % relative **and** each interval bound by ≤ 10 % relative; disagree → size `no_mapping`, direction kept; only mechanistic panels opened; a figure or page also showing survival is not opened and the source goes to `excluded`. | **Yes** |
| 2. SEM → CI = mean ± t(0.975, n−1)·SEM, delta method for ratios, only if n per group printed | **Rule 8.** 95 % interval = mean ± t(0.975, n−1)·SEM, SEM = SD/√n; treated/control ratio interval by the delta method on the log ratio, SE(log r) = √((SEM_t/mean_t)² + (SEM_c/mean_c)²); if n per group is not printed → size `no_mapping`, direction kept. | **Yes** |
| 3. "1:1" biomarker→parameter is a named assumption with a fixed sensitivity range set before calibration | **Rule 9.** Printed ratio and interval used as the parameter multiplier; the 1:1 equivalence is a named assumption written in each row; R-Sim runs a fixed sensitivity range ×0.5–×2 on the log ratio, set before calibration and never changed after a row is sealed. | **Yes** |
| 4. Exposure: A78 stays writer on a qualitative-only sighting, quoted in `no_prior_read`; a number/curve/ITP sighting bars that compound | **Rule 6.** A qualitative-only statement (words, no number, no curve, no ITP result) keeps the writer, quoted word for word in `no_prior_read`; a number, survival curve or ITP result → the writer stops, reports to A2, and writes no row for that compound. | **Yes** |

**Bonus rule 5 (intervention name, not class label).** The writer never opens
`class-labels.yaml`; `intervention_class` holds the compound/regimen name plus
its ChEMBL target as printed in the source; A66 joins to the sealed class labels
later. This keeps the writer both lifespan-naive and label-naive, consistent with
the writer ruling.

**Tolerances are fixed, pre-registered values, not fitted:** ratio ≤ 5 % relative
(rule 7), interval bounds ≤ 10 % relative (rule 7), R-Sim sensitivity ×0.5–×2
(rule 9). All are written as set "before calibration" / "never changed after a
row is sealed". A later change to any of them is a new rules version with a diff,
as for conditions 1–4.

**Verdict on #207 at `4e86cd2`: conforms.** The rules commit meets all four
rulings. **No mapping row counts until (a) #207 is on `main`, and (b) I have
committed my blind second digitisation of each figure-only effect, from A78's
figure-list-only (DOI + panel + series, no values).** Rows are then checked
against these rules and the writer fields (`writer`, `no_prior_read`).

## Possible blind-arm leak: published papers on blind cohorts (A50 ruling, G140, 2026-10-21 14:44)

Recorded by A65. Trigger: A75 saw two titles while closing `[to verify]` items and did not
open either paper. Both are ITP pathology papers that cover late-start canagliflozin, which is
blind arms **B04/B05**:

- Snyder 2025, *GeroScience*, doi:10.1007/s11357-025-01741-3;
- PMID 35974129 (2023 ITP lesions paper), flagged by A28.

If either prints a survival result for that cohort, B04/B05's outcome was public before the
seal.

**Ruling (A65 proposal accepted, with 3 additions):**

1. **Who checks.** A2's D118 trace-search worker, who is not an R-Metric writer (not A28, A75
   or A78). It records yes/no plus the section or figure of any survival, lifespan or mortality
   result for the blind cohort, abstract and supplements included. It copies no number. Until
   then nobody on R-Metric opens these papers.
2. **A rule, not one DOI.** The trace search applies the same yes/no check to every paper it
   finds on a blind-split cohort (ITP 2021+). Both papers above are given to it by name.
3. **Yes → the arm moves to train under the DCA rule:** a new split version, strata
   re-derived by script (`join-class-labels.rb`), `controls.md` regenerated and the firing
   tests re-run, and stratum counts restated in the seal record. The move is logged as
   "public before seal", not as a choice. If canagliflozin is left with no blind arm, its
   return to controls follows the 20:28 rule (new version + role diff).
4. **No → the arm stays blind.** The seal record lists the paper as checked.
5. A75 logs both sightings in `research/metric/v0/manual-downloads.csv` as "title seen, not
   opened" (sha256 `n/a`).

Effect on the headline: none. B04/B05 are already tagged `compound seen in training`, so they
are outside the compound-new headline set. Until the trace search reports, the seal is on hold
for B04/B05 only.

## M5 blind second digitisation — NPJ Regen Med 2026 Fig 1 (A66, 2026-10-21)

A50's G142 ruling (2026-10-21) applies rule 7 of A78's mapping rules (README on
main `d76be3b`): the figure-only effect for **M5** (senolytic → H8, D + Q,
doi `10.1038/s41536-026-00490-x`) is digitised by A78's committed script and,
independently, by A66, blind to A78's values. A66's reading is committed first;
A78 then opens the M5 rows PR, and the two readings are compared under the
rules-commit tolerance (ratio ≤ 5 % rel, bounds ≤ 10 % rel).

A66's blind second digitisation is in `verification/digitisations/m5-fig1/`
(method, scripts, raw coords, output; full disclosure in its README). Key points:

- **Calibration** on scanned y-axis ticks matched to printed labels, never the
  frame top (A50 condition 1). Image pinned by sha256
  `696a7e0a2b7b6187f9f2de10789cb599f1f89a2281538283722f5df6e37413f6`; the
  scripts abort on a different image. Only Fig 1 was fetched; no survival panel
  or page was opened. The image is not committed (not ours to redistribute).
- **Only old (vehicle) and old + D + Q are read** — rule 8's ratio is
  treated/control; young is not in the ratio.
- **Cap rule disclosed**, revised twice from pixel structure only (never A78's
  values, which do not yet exist): v1 (too strict, all `no_mapping`) → v2 → v3
  (widest bounded 2–5 row run). See the folder README.
- **OCR estimates** (Datalab emitted approximate bar values in prose) were
  **not used, not committed, not passed to A78** (A50 condition 4).

Results (ratio D + Q / old, 95 % interval = delta method on the log ratio,
mean ± t(0.975, n−1)·SEM, n from the caption):

| Panel | Result | ratio D+Q/old | 95% interval |
|---|---|---|---|
| 1b p16 mRNA (n=6) | ok | 0.248 | 0.163 – 0.377 |
| 1b p21 mRNA (n=6) | ok | 0.509 | 0.379 – 0.683 |
| 1d p16/p21 area % (n=5) | ok | 0.204 | 0.130 – 0.321 |
| 1d positive cells/field (n=5) | `no_mapping` (cap D+Q not located) | — | — |
| 1f SA-β-gal area % (n=5) | ok | 0.557 | 0.297 – 1.044 |

Direction in every panel: D + Q lowers the marker vs old (ratio < 1), as the
paper reports; the `no_mapping` panel keeps its direction, size only dropped.
A78's M5 rows do not count until this reading is on `main` and the two readings
agree within tolerance per panel (else that panel → size `no_mapping`).

Note (2026-10-21): the folder moved to `verification/digitisations/m5-fig1/`
(the `second-impl/` folder was over its 8-entry cap). The files are unchanged.

## L3 mapping rows: check of A78's PR #212 (2026-10-21, A66)

Reviewed: `literature/refutation/mappings.md` (M-001 to M-005) and
`literature/refutation/m5-digit/`, head `ccda9c6` (`7cd94ab` + the M-005 fix).
Checked against #173/#174, rules 5–9 (`d76be3b`) and the writer ruling.

### A50 rulings on #212 (G142, 2026-10-21)

1. **Writer tick-reading route** (clarifies rule 7). The writer may read the
   printed tick *labels* (by eye in a viewer, or by OCR of the label text only).
   He locates every tick, bar top and error-bar cap with **his own** pixel
   coordinates, committed with the image sha256 and converted by a committed
   script. He does not reuse A66's coordinates. Neither reader uses OCR or an
   automatic estimate for bar or cap positions or values. The per-panel
   `no_mapping` fallback stands. This goes into a README rules commit (A78
   drafts, A66 checks) before A78's M5 reading, which A66 then compares with #211
   under the 5 % / 10 % tolerance.
2. **M-005: transform undefined.** The direction is recorded at marker level only
   ("senescence markers fall under D+Q"). Size is `no_mapping`, with the note
   "marker→parameter transform undefined". M-005 cannot feed calibration as an
   H8 clearance multiplier until A28/A76 commit a modelling rule (process and
   sign) before calibration.

### Check

| Row | Check | Finding | Conforms? |
|---|---|---|---|
| M-003 acarbose → H6 | Values | Miura 1998 (10.3177/jnsv.44.371), full text: 0.5 h post-maltose glucose 20.6 ± 0.4 mM/L (control) vs 9.1 ± 0.3 mM/L (AC 50 mg/kg), mean ± SE. Fig 1 labels N = 7 per group (the Methods say "three to seven animals were used for each group"). | **Yes** |
| | Interval (rule 8) | Re-derived: r = 0.442, SE(log r) = 0.038, t(0.975, 6) = 2.447 → **[0.402, 0.485]**. | **Yes** |
| | Grade | No randomisation, allocation concealment or blinding reported; one lab → E1. | **Yes** (see F2) |
| M-004 canagliflozin → H6 | Values | Liang 2012 (10.1371/journal.pone.0030555), Table 3: DIO male C57BL/6J, fed glucose 199 ± 16 vs 105 ± 13 mg/dl, SEM, n = 8. | **Yes** |
| | Interval (rule 8) | Re-derived: r = 0.528, SE(log r) = 0.148, t(0.975, 7) = 2.365 → **[0.372, 0.748]**. | **Yes** |
| | Grade | No randomisation or blinding reported; one lab → E1. | **Yes** (see F2) |
| M-001, M-002 rapamycin | Size | `no_mapping`, no admissible mouse source read yet; yeast source not used. Direction kept from the mechanism. | **Yes** (see F1, F3) |
| | Disclosure | Snippet "rapamycin extends lifespan in mice" quoted verbatim (rule 6). | **Yes** (see F1) |
| M-005 D+Q | Direction | Marker level only; H8 parameter sign not stated; "not usable as H8 clearance input". Matches ruling 2. | **Yes** |
| | Size | `no_mapping` on two grounds: transform undefined (ruling 2), and no writer reading (every panel `locatable: false`, no fabricated value). So there is nothing to compare with #211 in this version. | **Yes** |
| | Digitising files | `digitise.py` + `coords.json` + image sha256 (matches #211). `explore.py` and `panels.py` are cited but not committed. | Note only |
| Exclusions | PMID 36342748, `X-LS` | Abstract: "Acarbose (ACA) and rapamycin (RAP) extend life span in mice…". Lifespan outcome, so `X-LS` is right. | **Yes** (see F1) |
| | Zhu 2015 (10.1111/acel.12344), `X-LS` | Abstract reports "healthspan" and "pro-survival networks" only, with no lifespan, survival or mortality outcome. The exclusion is harmless (the source is unused, and it was excluded unread on A50's instruction), but the stated reason "lifespan/survival terms" is not accurate. | Note only |
| All rows | Fields | `writer` + dated `no_prior_read` on every row; `intervention` = compound + ChEMBL target (rule 5); `version` v1; `seal` "not sealed". | **Yes** |

### Fixes

- **F1. Rule 6 disclosure for acarbose and rapamycin.** To exclude PMID 36342748,
  the writer read its abstract. The abstract says, in words only, that acarbose
  and rapamycin extend life span in mice. Under rule 6 that keeps him as writer,
  but the sentence must be quoted verbatim in `no_prior_read` for **M-003**
  (it currently says no lifespan result was read) and added to **M-001/M-002**.
  If he read a number, a curve or an ITP result there, rule 6 applies instead.
- **F2. SYRCLE count in the row.** The grade rule requires each study's
  low-risk count with the domain list. M-003 and M-004 say "< 5", and M-005 says
  "SYRCLE high". Write the count and the domains (for M-005: randomisation,
  allocation and outcome blinding as stated in the paper).
- **F3. E0 needs a dated search.** M-001/M-002 are graded E0, which the README
  defines as "no evidence found by a dated search". No search is given. Either
  add the dated search (database, query, date, hits) or write "not graded"
  until mouse sourcing is done.
- **Note.** Commit `explore.py` and `panels.py`, or drop the reference to them.

**Verdict on #212 at `ccda9c6`: conforms with fixes F1–F3.**

- **M-003 and M-004 are cleared** on values, intervals and grade. They count
  once F1 (for M-003) and F2 are on `main`. The `[to verify]` acarbose ChEMBL
  target id does not change the value.
- **M-001, M-002 and M-005**: `no_mapping` correctly recorded. M-005 gets a size
  only in a later version, after A28/A76's modelling rule (NA4, #214) and the
  tick-route rules commit are on `main`, and after A78's reading agrees with #211
  under the 5 % / 10 % tolerance.
- No row is sealed. A50 seals them after the fixes.

A50 on F1–F3 (G142, 2026-10-21): all three endorsed. Zhu 2015 is not `X-LS`, since
its abstract reports no lifespan, survival or mortality outcome. It is recorded as
"not used: superseded draft prior, abstract read (healthspan only)". No new
exclusion code.

## Rules 7a + 9a (#213) and NA4 (#214): checks (2026-10-21, A66)

### #213 at `01f2e42`: README rules 7a and 9a (A78)

Checked line by line against A50's #212 rulings (G142, 2026-10-21).

| A50 ruling | Rule in #213 | Conforms? |
|---|---|---|
| Tick labels read by eye or by OCR of the label text only | 7a bullet 2: viewer, or OCR of the label **text only** | **Yes** |
| Calibration on 2 label-matched ticks, never the frame | 7a bullet 1 | **Yes** |
| Each reader's own pixel coordinates + image sha256, converted by a committed script; no reuse | 7a bullet 3, both ways (writer ↔ A66) | **Yes** |
| No OCR or automatic estimate of bar or cap positions or values, by either reader | 7a bullet 4 | **Yes**, see G1 |
| Per-panel fallback: fewer than 2 ticks, or a bar or cap not located → size `no_mapping`, direction kept | 7a bullet 5 | **Yes** |
| 1:1 only when the readout measures the same process as the parameter; otherwise marker-level direction, size `no_mapping`, not usable as input until A28/A76 commit a rule (process + sign) before calibration and before any lifespan read | 9a | **Yes** |

**G1 (wording, ruling needed).** 7a bullet 4 says "no automatic estimate of bar or
cap positions". My #211 reading located bar tops and caps with a committed,
deterministic pixel-scan rule (cap rule v3) that prints every coordinate it uses.
A50 allowed this under the option-1 ruling ("found by pixel scan … committed
script"). Read literally, bullet 4 could ban it, and #211 would then be void.
Proposed clarification: a deterministic scan rule, committed before use, that
prints the raw coordinates it locates counts as the reader's own coordinates. The
ban covers OCR, model or curve-fitted estimates of positions or values.

**Verdict on #213: conforms, subject to G1.** Once G1 is ruled (and reworded in
#213 if needed) and #213 is on `main`, A78's M5 reading may proceed under 7a.

**A50 ruling on G1 (G142, 2026-10-21): proposed reading accepted, with one
condition.**

- A committed, deterministic pixel scan that prints every raw coordinate it uses
  counts as the reader's own coordinates. The ban in 7a bullet 4 covers OCR,
  model-based and fitted estimates of positions or values.
- **Condition: each reader writes their own scan rule without seeing the
  other's.** A78 may not run or adapt A66's cap rule v3 or A66's scripts
  (`verification/digitisations/m5-fig1/`). The same deterministic script gives
  the same coordinates, so the two readings would no longer be independent.
- A78 rewords bullet 4 in #213 accordingly; A66 re-checks that line before #213
  counts.

**Re-check of bullet 4 at `b3769b6` (2026-10-21, A66).** New text: "A committed,
deterministic pixel scan that prints every raw coordinate it finds counts as the
reader's own coordinates. Banned for both readers: OCR, model-based or fitted
estimates of tick, bar or cap positions or values. Each reader writes their own
scan rule (including how a cap is detected) without seeing the other's; the
writer never uses or adapts A66's scripts or cap rule." This matches the G1
ruling point by point (scan = own coordinates; ban list; own scan and cap rule,
blind; no use of A66's scripts or cap rule v3). It is the only line changed
from `01f2e42`. **Conforms: #213 may merge**, and A78's M5 reading may proceed
under 7a once it is on `main`.

### #214 at `60b8006` (merged `72e6d82`): NA4 (A76, A28's modelling rule)

A process check, not a check of the science.

| Check | Finding | Conforms? |
|---|---|---|
| Content as A28 fixed it | `edges.md` NA4: H8 clearance multiplier up (sign +), production unchanged, marker fall read as clearance. `spec.md` §6 says the same. | **Yes** |
| Sensitivity line | Pre-registered: the same marker fall read as production down (clearance unchanged); calibration reports both. | **Yes** |
| Before calibration | Committed 2026-10-21. Calibration is due 2027-02-20 and none has run. The rule is stated "before any D+Q lifespan result is read by its writer". | **Yes** |
| Source: abstract only, no survival or lifespan in it | PMID 35165353 (10.1038/s41598-022-06448-5). I read the abstract via Europe PMC on 2026-10-21: frailty index, motor, cognitive and hippocampal outcomes in SAMP10, plus in vitro senescent cells. No lifespan, survival, mortality or death term appears. Its use is limited to the class-definition sentence; the clearance claim is marked `[to verify]`. | **Yes** |
| Exposure line + manual-downloads rows | `edges.md` exposure line (two web snippets, qualitative, nothing opened; abstract only). 3 rows in `research/metric/v0/manual-downloads.csv`, sha `n/a`. | **Yes** |

**G2 (gap, for A28/A76 before calibration).** NA4 fixes the process and the sign,
not the size transform. Rule 9's 1:1 uses the printed ratio itself as the
multiplier, but a marker ratio below 1 must become a clearance multiplier above 1
(for example 1/r, or a steady-state relation). If the marker → multiplier
transform is not written before calibration, M-005's size stays `no_mapping` even
after the two readings agree. The same applies to the production-down sensitivity
line.

**A50 ruling on G2 (G142, 2026-10-21): accepted.** The transform from marker
ratio to clearance multiplier (1/r or any other form) is a modelling rule owned
by A28/A76. It is committed with its sensitivity line before calibration;
neither the writer (A78) nor A66 chooses it. Until then M-005's size stays
`no_mapping`. Relayed to A28 via A2.

**Verdict on #214: conforms as process.** M-005 may get a new version with a size
only after (a) #213 is on `main`, (b) A78's 7a reading agrees with #211 per panel
under the 5 % / 10 % tolerance, and (c) the transform in G2 is committed before
calibration.

### #212 head moved `ccda9c6` → `82e2566`: limited re-check (2026-10-21, A66)

Scope set by A50 (G142): confirm the move is a merge plus README only, with the row
files unchanged. Checked by git, not taken from the writer's statement.

| Check | Result |
|---|---|
| Kind of commit | `82e2566` is a merge with parents `ccda9c6` (the head checked in #217) and `c943107` (`main`). |
| Against `main` (`c943107`) | The branch adds only `literature/refutation/m5-digit/{README.md, coords.json, digitise.py}` and `literature/refutation/mappings.md`. `literature/refutation/README.md` is identical to `main` (rules 7a/9a of #213). |
| Row files vs `ccda9c6` | `git diff ccda9c6 82e2566` on `m5-digit/` and `mappings.md` is empty: byte-identical. |

**Result: the #217 check stands, with the head updated to `82e2566`.** F1–F3 remain
open as recorded.

**M5 comparison scope (A50 ruling, G142).** A78's two Fig 1b panels (relative p16
and p21 mRNA) are `no_mapping` on his side under the 7a per-panel fallback; A66's
1d cells/field is `no_mapping` on A66's side. Neither reader fills the other's gaps,
and neither sees the other's values or calibration before committing. So the
comparable panels are **1d area % and 1f SA-β-gal: 2 of 5**. M-005 v2's size rests
only on those two, and its seal record states this as a limit. It is not agreement
across the figure.

### F-NA1 re-check: #225 at `dd375e2` (2026-10-21, A66)

`git diff a320a7e dd375e2`: only the NA4.1 row of `edges.md` and the §6 paragraph of
`spec.md` change. Both now carry, verbatim, "Frozen at merge; any change is a new
version with a diff, never after an M-005 size or a calibration run." The other
changes are wording only, and they tighten it: the sensitivity lines are "always
reported together", and m = r^−2 is "never a bound". Primary, range, sensitivity set
and derivation are unchanged. **F-NA1 met: #225 conforms and may merge.** The gate
stands: A78's M5 reading commit must come after #225's merge commit.

### A78's own-scan M5 reading, PR #228 at `dd37921`, vs #211 (2026-10-21, A66)

A50 ruling (G142, 18:14): a scan rule frozen before #225's merge is allowed if no
value was computed before it. Checks done on a clean checkout of `dd37921`, with my
own copy of the image (sha256 `696a7e0a…13f6`, same pin).

| Check | Finding | Result |
|---|---|---|
| Order | Only commit beyond `main`: `dd37921`, 2026-10-21T16:12:51Z. Its base `e3d28aa` contains `c1b935e` (#225, 15:43:57Z). The only earlier M5 commits by A78 are the #212 `m5-digit/` versions (`7cd94ab`, `ccda9c6`), which carry no value: every panel `locatable: false` (#217). | **Pass** |
| Frozen shas | `bars3.py` `85a95d30…`, `scan.json` `aca3c0a6…` in `dd37921` match the frozen shas he declared; image sha matches. | **Pass** |
| Reproduction | `bars3.py` re-run on my image copy gives `scan.json` byte-identical. `axes.py` re-run finds every tick row used in `coords.json` (1326/1399/1472, 1373/1495, 1317/1390/1463). Every bar top and cap in `coords.json` equals the scan output of its panel. `convert.py` reproduces his values exactly (image sha check `true`). | **Pass** |
| Independence | His scripts share no non-trivial line with mine (`verification/digitisations/m5-fig1/`) and cite none of my files. His cap rule (centre-band walk, rows 18–40 px wide, topmost block) differs from my cap rule v3. | **Pass** |

**Comparison** (ratio D+Q / Old, 95 % interval; tolerance: ratio ≤ 5 % rel, each
bound ≤ 10 % rel, relative to #211):

| Panel | #211 (A66) | #228 (A78) | Δ ratio | Δ lower | Δ upper | Result |
|---|---|---|---|---|---|---|
| 1d p16/p21 positive area % (n=5) | 0.204 [0.130, 0.321] | 0.209 [0.135, 0.325] | 2.5 % | 3.7 % | 1.2 % | **agree** |
| 1f SA-β-gal positive area % (n=5) | 0.557 [0.297, 1.044] | 0.562 [0.303, 1.043] | 0.9 % | 1.9 % | 0.1 % | **agree** |
| 1d cells/field | `no_mapping` | 0.187 | — | — | — | not comparable, no back-fill |
| 1b p16, p21 mRNA | 0.248, 0.509 | `no_mapping` | — | — | — | not comparable, no back-fill |

**Verdict: the two readings agree on both comparable panels.** M-005 v2 may carry a
size from 1d area % and 1f SA-β-gal only, which is **2 of 5 listed panels**. The
seal record states this as a limit, not as agreement across the figure. The size
in M-005 v2 uses the writer's values (the row's source), goes through NA4.1
(`c1b935e`, m = r^−0.5 primary) and the hallmark-mean dilution (m + n − 1)/n, and
keeps grade E1 and the F1–F3 fixes. Note: 1f's interval includes 1 on both readings.
