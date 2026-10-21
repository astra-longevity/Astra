# R-Metric v0: train/test split proposal (test 2)

Status: **proposal by Research (A67 for A28)**. Verification (A50/A65) seals
it, never Research. **No score has been computed on any item.** No outcome
is written in this file for held-out items.
Cites: D114 (T1-A28-3 test 2, T1-A2-3, T1-A2-4, T1-A2-5, T1-A50-2, T1-A50-3);
D115 (T2-A28-6 strata, T2-A2-4 class labels); D118; A50 ruling in G154
(manual download log, class labels approved at seal); A28 decision in G144
(held-out policy (c), 2026-10-20).

## Held-out policy (A28, option (c))

The held-out set has two parts, both sealed by A65:

1. **Retrospective part** (published arms). Exclusion rule: an arm is
   excluded if its lifespan result appears on a page logged in
   `manual-downloads.csv` (A65 rule, G154). Label: **"retrospective, not
   blind"** (A50).
2. **Blind part** (arms with no public result). Arms whose lifespan result
   is not public at seal time, listed by name and start year only, as
   **sealed forecasts**. Sources are cohort-design pages, never results.

### Known, uncontrolled leak

I am a language model, and my training text includes most published ITP
results. The exclusion rule above controls what was read **in this
project**; it does not control what the model already knew. This leak is
stated, not controlled. Only the blind part is free of it.

## Rule 2: class labels come from mechanism, not from outcomes

Each label names the primary molecular target or pathway. Mechanism sources
are `[to verify]` unless a DOI is given; L1 (A71) supplies them, and A65
approves them at seal (T2-A2-4). Sources must not report a lifespan result.

## Status of the retrospective part: empty

To list the in-progress ITP arms, I opened the NIA "Supported
Interventions" page (logged in `manual-downloads.csv`). That page also has
an outcome column for every finished ITP arm from 2004 to 2022. Under the
exclusion rule, **every finished ITP arm is now "read"** and goes to train,
including former held-out items H01–H09. My mistake: I should have asked
A71 for a names-only list.

Consequence: the retrospective part has **0 ITP items**. Open issue S-2.

### Refill rule for the retrospective part (fixed before any list exists)

Selecting arms that were "replicated" would select on outcome (an effect
had to exist to be replicated). The refill therefore uses a
**metadata-only rule**, written here before any candidate list is built:

1. Source: mouse lifespan studies outside the ITP, taken from a curated
   database (e.g. DrugAge), selected by **publication-year window only**.
   The window is fixed by A28/A65 and written in the seal record before the
   list is built.
2. Unit: whole studies. **All arms** of each selected study are included,
   whatever their result (nulls and shortening arms come in naturally).
3. The list is built by A71 via A2 from compound, dose, sex, strain, year
   and PMID only, with every outcome column stripped. Research (A67) does
   not build it and does not open the papers.
4. Exclusions, applied mechanically: arms already logged in
   `manual-downloads.csv`; arms whose compound is in the blind part.
5. T1-A2-3 (control median in ITP/MPD reference range) is checked by A71
   at scoring time, after the seal; failing arms are dropped and reported,
   not replaced.
6. Label: "retrospective, not blind" (pretraining leak, above).

### Retrospective part: DrugAge refill list (filled 2026-10-20)

- List: `literature/lifespan/drugage-refill/refill-list.csv`, built by A71
  (PR #75, bb0d11f) under A65's approval (PR #62, #64), checked byte-identical
  by A65 (PR #77). 23 studies, 40 arms (RF001–RF040), PubMed years 2015–2022.
- List sha256 `792d54b653d6089c67edf9f30ea759788f8148ac63d87878b81512fd7b20786d`;
  `build.rb` sha256 `ea277d5f78dfd2844b4954dd3e2bf69f35b7e3676cbc1a40577dba7cca81f382`;
  source DrugAge Build 5, zip sha256 `d12f5971…fa80` (LA-017).
- Label: **"retrospective, not blind, DrugAge-selected"** (DrugAge
  under-reports nulls).
- Research read only the README and the outcome-stripped CSV. Neither DrugAge
  nor the 23 papers were opened.
- Per-arm class label and stratum: `split.csv` (below). The `class_label`
  column is filled by a scripted join, `join-class-labels.rb`, keyed on
  `compound` against `literature/lifespan/class-labels.yaml` (41 labels,
  built by A71's `class-labels.rb`; A65 approves). No hand edits.

### Scripted stratum re-derivation (`join-class-labels.rb`, A28 rule G144)

The same script re-derives the a/b/c stratum from the joined `class_label`,
in one pass, with the rule A28 fixed in G144 (2026-10-21):

- **c** = any combination or sex-transfer arm (read from `flag`), whatever its
  class.
- **a** = else, iff `class_label` is a real class (not `unclassified`, not
  `multi-target`) **and that class has at least one train arm**.
- **b** = else: every `unclassified` and `multi-target` arm, and any real class
  with no train example.

The "compound seen in training" tag stays in `flag` and never sets `stratum`
(A65, #144): a scorecard grouping on stratum must not mix class-novelty with
leakage.

**Train classes (rule 2a, final).** The train set is the 132 finished ITP
arms in `literature/lifespan/itp/`, not rows of this file. Each train
compound's class comes from `literature/lifespan/train-classes.yaml` (A71
#185, separate train-only ChEMBL 37 snapshot; A65 check #186). A28 rule 2a:
a class is present in train iff it labels a **single-compound** train entry.
`unclassified` / `multi-target` train entries (acarbose = multi-target)
contribute nothing; the two train combinations (metformin + rapamycin,
rapamycin + acarbose) stay `unclassified` and are not decomposed, so a class
seen only inside a combination is not present in train. Class strings match
exactly.

Stratum counts (60 arms: 20 blind + 40 retrospective):

| Stratum | Before (hand) | Provisional (#183) | Final (rule 2a) |
|---|---|---|---|
| a known class | 32 | 16 | 8 |
| b new / unclassified | 26 | 42 | 50 |
| c combination / transfer | 2 | 2 | 2 |

Provisional → final: 8 arms moved, all a → b (real class with no
single-compound train arm): B01 dichloroacetate, B09 trametinib, B12
L-deprenyl, B13 sildenafil, B14 alpelisib, B15 alogliptin, RF005 ivabradine,
RF038 chloroquine. The 8 remaining stratum-a arms are canagliflozin (B04,
B05), captopril (B08) and rapamycin (RF008, RF010–RF013): every one is also
a compound seen in training (tag in `flag`).

**Named limit (A28, G171 2026-10-21): stratum a (known class, new compound)
is empty in v0.** Stratum a scores only compounds seen in training; it is
reported apart and never enters the headline. Arms are not moved to fill it
(that would be selection).

Compound-new arms per stratum (row = held-out arm; "seen" = any component
compound names a single-compound train entry of `train-classes.yaml`, after
`class-labels-train-names.yaml` canonicalisation; exact name match):

| Stratum | Rows | Compound-new | Compound seen in training |
|---|---|---|---|
| a | 8 | 0 | 8 |
| b | 50 | 42 | 8 (NDGA RF001–RF004, nicotinamide riboside RF009, metformin RF020, RF021, RF032) |
| c | 2 | 0 | 2 (B03 rapamycin + 17α-estradiol) |

So the compound-new headline (A50 #145) rests on the 42 stratum-b arms only.

**Scripted tag (A28, G171).** `join-class-labels.rb` writes the `flag` tag
`compound seen in training` on every row matching the rule above (appended
with `; ` to any existing flag; stratum unchanged) and prints the
compound-new counts, so A65 re-runs it at the seal (`--check` writes
nothing). It aborts if a row carries the tag without a train match. First run
tagged 13 retro rows (RF001–RF004, RF008–RF013, RF020, RF021, RF032); counts
unchanged at a 0/8, b 42/50, c 0/2.

**Overlap with the blind part (flagged, not dropped; A50 condition 6):**
RF008, RF010–RF013 are rapamycin, a single compound of blind arm B03
(rapamycin + 17α-estradiol). `controls.rb --check` matches whole compound
names, so it will flag them. **A28 decision (G144):** keep them, stratum (a).
Their outcomes are held out and unread, so they cannot leak into the B03
forecast. Both B03 compounds train (finished ITP rapamycin arms; 17α-estradiol
T06), so B03 is tagged **"compound seen in training"** like B04/B05 and stays
in stratum (c). A65 confirms at the seal.

### Class vocabulary: see `class-labels.yaml`

The class of every compound is now the authoritative, scripted value in
`literature/lifespan/class-labels.yaml` (A71's `class-labels.rb`, pinned
ChEMBL 37 + MeSH 2026). This file no longer carries a hand class table; the
join reads the yaml. For A65's vocabulary review, the classes to approve are
exactly the distinct `class` values in that yaml (built: ChEMBL 11, A28 calls
3, MeSH 3, `unclassified` 24, `multi-target` 5). Any flagged label (e.g.
17α-estradiol → 5-alpha reductase inhibitor; mianserin, sodium thiosulfate →
multi-target) changes only by a rule change ruled by A50, never by hand here.

## Machine-readable split: `split.csv`

One row per arm, columns `part, arm_id, compound, cohort, pmid, sex,
class_label, stratum, flag` (A65, PR #110; `class_label` filled by the
scripted join). The `stratum` values below predate the re-derive; the
authoritative a/b/c is the script output above. Combination arm B03 is
listed as two rows, one per single compound. B03, B04, B05 and B08 use **one
encoding** (A65, G154): their a/b/c stratum in `stratum` (B03 c, B04/B05/B08 a),
and the exact text `compound seen in training` in `flag`, ruling ref after
`;` (A50 ruling PR #105 for B04/B05, PR #145 for B08, A28/A65 G144 for B03). No
outcome column. B08 (captopril) trains via the C2017 captopril arm, same case as
B04/B05. **Headline score = compound-new blind arms only, never pooled with the
"compound seen in training" arms** (A50, PR #145); `controls.rb --check` passes
a compound-only overlap only if the blind arm carries this tag.

## Train set

All finished ITP arms (cohorts 2004–2022) are train. The table lists the
ones already cited in this file set; A71's L1 table is the full list.

| Id | Intervention | Class label (mechanism) | Seen in |
|---|---|---|---|
| T01 | Rapamycin | mTOR inhibitor | doi:10.1038/nature08221; doi:10.1093/gerona/glq178 |
| T02 | Acarbose | α-glucosidase inhibitor | doi:10.1111/acel.12170 |
| T03 | Canagliflozin | SGLT2 inhibitor | doi:10.1172/jci.insight.140019 |
| T04 | Metformin alone | biguanide | doi:10.1111/acel.12496 |
| T05 | Metformin + rapamycin | combination | doi:10.1111/acel.12496 |
| T06 | 17α-estradiol | non-feminising oestrogen | doi:10.1111/acel.12170 |
| T07 | NDGA | antioxidant / lipoxygenase inhibitor `[to verify]` | doi:10.1111/acel.12170 |
| T08 | Protandim | Nrf2 activator `[to verify]` | doi:10.1111/acel.12496 |
| T09 | Methylene blue | redox cycler `[to verify]` | doi:10.1111/acel.12170 |
| T10 | Glycine | amino acid | doi:10.1111/acel.12953 |
| T11 | Resveratrol | polyphenol | doi:10.1093/gerona/glq178 |
| T12 | Simvastatin | HMG-CoA reductase inhibitor | doi:10.1093/gerona/glq178 |
| T13 | Green tea extract | polyphenol | doi:10.1093/gerona/gls070 |
| T14 | Curcumin | polyphenol | doi:10.1093/gerona/gls070 |
| T15 | Oxaloacetic acid | Krebs-cycle metabolite | doi:10.1093/gerona/gls070 |
| T16 | MCT oil | dietary lipid | doi:10.1093/gerona/gls070 |
| T17–T25 | former H01–H09: 16α-hydroxyestradiol, astaxanthin, captopril (2017), aspirin, meclizine, fisetin, 1,3-butanediol, nicotinamide riboside, rapamycin + acarbose | as in round 2 | NIA Supported Interventions page |

The NIA page also lists many null arms and arms effective in one sex only.
They give train its null proportion (T1-A2-4); A71 extracts them.
Life-shortening arms: none identified yet (S-3).

## Blind part: in-progress ITP arms (sealed forecasts)

Source: NIA "Supported Interventions" page, last updated 2026-04-13
(status "In Progress", no result listed). Dose, start age and cohort year as
printed there. Class labels are proposals; mechanism sources `[to verify]`
(A71), approved by A65.

| Id | Intervention | Cohort | Dose, start age | Class label (proposed) | Stratum |
|---|---|---|---|---|---|
| B01 | Dichloroacetate | 2021 | 30 ppm, 7 mo | PDK inhibitor (glucose oxidation) | (b) new class |
| B02 | TPPU | 2023 | 24 ppm, 7 mo | soluble epoxide hydrolase inhibitor | (b) new class |
| B03 | Rapamycin + 17α-estradiol | 2023 | 14.4 + 14 ppm, 12 mo | combination (mTOR + oestrogen) | (c) combination |
| B04 | Canagliflozin, 7 to 20 mo | 2023 | 180 ppm | SGLT2 inhibitor, stopped | (a) known class |
| B05 | Canagliflozin, low dose | 2023 | 60 ppm, 7 mo | SGLT2 inhibitor | (a) known class |
| B06 | Sodium thiosulfate | 2023 | 3,333 ppm, 7 mo | H2S donor `[to verify]` | (a) known class (SG1002, STS 2020 in train) |
| B07 | Lactoferrin | 2023 | 1.2%, 11 mo | iron-binding protein | (b) new class |
| B08 | Captopril, low dose | 2024 | 60 ppm, 7 mo | ACE inhibitor | (a) known class |
| B09 | Trametinib | 2024 | 1.44 ppm, 12 mo | MEK inhibitor | (b) new class |
| B10 | Mianserin | 2024 | 150 ppm, 7 mo | serotonin receptor antagonist | (b) new class |
| B11 | Cinnarizine | 2024 | 1600 ppm, 7 mo | calcium channel / H1 blocker `[to verify]` | (b) new class |
| B12 | L-deprenyl | 2024 | 5 ppm, 7 mo | MAO-B inhibitor | (b) new class |
| B13 | Sildenafil | 2024 | 30 ppm, 8 mo | PDE5 inhibitor | (b) new class |
| B14 | Alpelisib | 2024 | dose TBD, 8 mo | PI3Kα inhibitor (upstream of mTOR) | (b) new class |
| B15 | Alogliptin | 2025 | 300 ppm, 8 mo | DPP-4 inhibitor (glucose lowering) | (a) known class |
| B16 | BT2 | 2025 | 240 ppm, 7 mo | BCKDK inhibitor (branched-chain amino acids) `[to verify]` | (b) new class |
| B17 | Glucosamine sulfate | 2025 | 5500 ppm, 7 mo | hexosamine pathway | (b) new class |
| B18 | Taurine | 2025 | 6000 ppm, 7 mo | amino acid | (a) known class (glycine) |
| B19 | Urolithin A | 2025 | 150 ppm, 7 mo | mitophagy inducer | (b) new class |

Count: 19 arms, 6 in (a), 12 in (b), 1 in (c). Sex transfer: forecasts are
made per sex; no sex is used to fit the other.

Timing risk: B01 (2021 cohort) may be published soon. A forecast counts
only if sealed before the result is public; A65 checks publication date
against the seal date (L1 trace search, D118). Arms published before the
seal move to train.

## Requirements

| Requirement | Status |
|---|---|
| ≥ 10 held-out interventions, ≥ 2 classes | blind part: 19 arms, met; retrospective: 40 arms / 23 studies, met |
| Both sexes | ITP arms are both sexes unless noted (B04–B19 per page) |
| Null and shortening arms (T1-A2-4) | blind outcomes unknown by design; shortening arms in train: none yet (S-3) |
| Control median in reference range (T1-A2-3) | A71 at scoring |
| `se_source` per row (T1-A2-5) | A71 at scoring |
| Baselines sealed: chronological age, body weight, frailty index, "everything extends" (T1-A50-2) | listed; A65 seals |

## Open issues

- S-1: pretraining leak on all published arms (stated above, uncontrolled).
- S-2: closed: retrospective part filled by the DrugAge refill list (40 arms).
- S-6: resolved by A28 (G144, 2026-10-20): RF008, RF010–RF013 kept, stratum
  (a). B03 tagged "compound seen in training", stays stratum (c); A65 to
  confirm at the seal.
- S-7: refill class labels and strata need mechanism sources and new
  vocabulary entries before the seal.
- S-5: any blind arm whose result is published before the seal (B01 risk)
  moves to train; A65 checks at seal.
- S-3: no life-shortening arm in train yet; A71 to name.
- S-4: blind forecasts score only years from now (ITP arms run to death,
  ~3–4 years). Test 2 on the blind part cannot run in 2026.
