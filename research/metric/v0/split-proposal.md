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
- Per-arm class label and stratum: `split.csv` (below). Labels are proposals
  from pharmacology, mechanism sources `[to verify]` (A71 adds them to
  `literature/lifespan/class-labels.yaml`; A65 approves).

Stratum counts (40 arms): (a) known class 25, (b) new class 15, (c) 0.
Proposed new classes (b): HCN channel blocker, polyamine, IGF-1R antagonist,
Cdc42 inhibitor, CD38 inhibitor, lysosomal autophagy inhibitor, NLRP3
inhibitor, multi-target (icariin, berberine). They need vocabulary entries
before the seal.

**Overlap with the blind part (flagged, not dropped; A50 condition 6):**
RF008, RF010–RF013 are rapamycin, a single compound of blind arm B03
(rapamycin + 17α-estradiol). `controls.rb --check` matches whole compound
names, so it will flag them. **A28 decision (G144):** keep them, stratum (a).
Their outcomes are held out and unread, so they cannot leak into the B03
forecast. Both B03 compounds train (finished ITP rapamycin arms; 17α-estradiol
T06), so B03 is tagged **"compound seen in training"** like B04/B05 and stays
in stratum (c). A65 confirms at the seal.

### Proposed new classes (for A65's vocabulary, before the seal)

Mechanism = primary target as named in pharmacology. Sources must report no
lifespan outcome (class-labels.yaml rules); none opened yet, so all
`[to verify]` (A71 supplies IUPHAR / ChEMBL / DrugBank entries).

| Class | Mechanism | Arms | Source |
|---|---|---|---|
| HCN channel blocker | blocks the funny current (I_f) in sinoatrial cells, lowers heart rate | RF005 ivabradine | `[to verify]` |
| polyamine | natural polycations; spermidine is the eIF5A hypusination substrate and induces autophagy | RF015–RF019 putrescine, spermidine, spermine | `[to verify]` |
| IGF-1R antagonist | blocks the IGF-1 receptor | RF024 L2-Cmu | `[to verify]` |
| Cdc42 inhibitor | inhibits the Rho GTPase Cdc42 | RF027 CASIN | `[to verify]` |
| CD38 inhibitor | inhibits the NAD+-consuming enzyme CD38 | RF036–RF037 78c | `[to verify]` |
| lysosomal autophagy inhibitor | raises lysosomal pH, blocks autophagic flux | RF038 chloroquine | `[to verify]` |
| NLRP3 inhibitor | inhibits NLRP3 inflammasome assembly | RF039 oridonin | `[to verify]` |
| multi-target | source names two or more targets without ranking (vocabulary entry exists) | RF007 icariin, RF026 berberine | `[to verify]` |

`multi-target` is already in the vocabulary, so 7 classes are new. Other
labels in `split.csv` that are not in the vocabulary yet (biguanide, NAD+
precursor, senolytic flavonoid, Krebs-cycle metabolite, lipoxygenase
inhibitor, polyphenol, and the blind-part classes) also need entries; they
are counted as known classes because the compound or class trains.

## Machine-readable split: `split.csv`

One row per arm, columns `part, arm_id, compound, cohort, pmid, sex,
class_label_proposed, stratum, flag` (A65, PR #110). Combination arm B03 is
listed as two rows, one per single compound. B04/B05 (canagliflozin) carry
stratum **"compound seen in training"** (A50 ruling, PR #105). No outcome
column.

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
