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
