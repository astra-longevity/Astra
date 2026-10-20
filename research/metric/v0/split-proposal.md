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

Consequence: the retrospective part has **0 ITP items**. It can only be
refilled with published non-ITP arms (meeting T1-A2-3: ITP or replicated,
control median in reference range) that A71 names without passing the
result to Research. Open issue S-2.

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
| ≥ 10 held-out interventions, ≥ 2 classes | blind part: 19, met; retrospective: 0 |
| Both sexes | ITP arms are both sexes unless noted (B04–B19 per page) |
| Null and shortening arms (T1-A2-4) | blind outcomes unknown by design; shortening arms in train: none yet (S-3) |
| Control median in reference range (T1-A2-3) | A71 at scoring |
| `se_source` per row (T1-A2-5) | A71 at scoring |
| Baselines sealed: chronological age, body weight, frailty index, "everything extends" (T1-A50-2) | listed; A65 seals |

## Open issues

- S-1: pretraining leak on all published arms (stated above, uncontrolled).
- S-2: retrospective part empty after reading the NIA page; refill only
  with non-ITP replicated arms named by A71, names only.
- S-3: no life-shortening arm in train yet; A71 to name.
- S-4: blind forecasts score only years from now (ITP arms run to death,
  ~3–4 years). Test 2 on the blind part cannot run in 2026.
