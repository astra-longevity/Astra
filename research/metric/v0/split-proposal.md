# R-Metric v0: train/test split proposal (test 2)

Status: **proposal by Research (A67 for A28)**. Verification (A50/A65) seals
it, never Research. **No score has been computed on any item.** No outcome
is written in this file for held-out items.
Cites: D114 (T1-A28-3 test 2, T1-A2-3, T1-A2-4, T1-A2-5, T1-A50-2, T1-A50-3);
D115 (T2-A28-6 strata, T2-A2-4 class labels); D118; A50 ruling in G154
(manual download log, class labels approved at seal).

## Rule 1: what A67 has already seen is training, never test

A65 (G154): a page or abstract that reports a lifespan result counts as
reading outcome data. Every such page I read is in `manual-downloads.csv`
(sha256 `n/a`). Interventions reported there go to **train** only.

Second leak, stated plainly (T1-A50-3): I am a language model, and my
training text includes most published ITP results. No published item is
truly blind to me. The held-out set below is therefore scored as
**"retrospective, not blind"**. It still protects against fitting on the
test items. Blind evidence comes only from sealed forecasts of unreleased
cohorts and held-out mice (test 4).

## Rule 2: class labels come from mechanism, not from outcomes

Each label names the primary molecular target or pathway. Sources for the
mechanism are `[to verify]` unless a DOI is given; L1 (A71) supplies them
and A65 approves them at seal (T2-A2-4). Labels must be fixed before any
held-out lifespan result is read by Research.

## Train set (outcomes already seen by A67)

| Id | Intervention | Class label (mechanism) | Seen in | Note |
|---|---|---|---|---|
| T01 | Rapamycin | mTOR inhibitor | doi:10.1038/nature08221; doi:10.1093/gerona/glq178 | also test-1 right-way control |
| T02 | Acarbose | α-glucosidase inhibitor (post-meal glucose) | doi:10.1111/acel.12170 | test-1 control |
| T03 | Canagliflozin | SGLT2 inhibitor (glucose lowering) | doi:10.1172/jci.insight.140019 | |
| T04 | Metformin alone | biguanide | doi:10.1111/acel.12496 | ITP null arm (test 3) |
| T05 | Metformin + rapamycin | combination (biguanide + mTOR) | doi:10.1111/acel.12496 | |
| T06 | 17α-estradiol | non-feminising oestrogen | doi:10.1111/acel.12170; doi:10.1111/acel.12496 | |
| T07 | NDGA | antioxidant / lipoxygenase inhibitor `[to verify]` | doi:10.1111/acel.12170; doi:10.1111/acel.12496 | |
| T08 | Protandim | Nrf2 activator (botanical mix) `[to verify]` | doi:10.1111/acel.12496 | |
| T09 | Methylene blue | redox cycler / mitochondrial `[to verify]` | doi:10.1111/acel.12170 | result not read in full |
| T10 | Glycine | amino-acid supplement (methionine clearance `[to verify]`) | doi:10.1111/acel.12953 | |
| T11 | Resveratrol | polyphenol, sirtuin activator claim `[to verify]` | doi:10.1093/gerona/glq178; doi:10.1093/gerona/gls070 | null arm |
| T12 | Simvastatin | HMG-CoA reductase inhibitor | doi:10.1093/gerona/glq178 | null arm |
| T13 | Green tea extract | polyphenol (catechins) | doi:10.1093/gerona/gls070 | null arm `[to verify]` |
| T14 | Curcumin | polyphenol | doi:10.1093/gerona/gls070 | null arm `[to verify]` |
| T15 | Oxaloacetic acid | Krebs-cycle metabolite | doi:10.1093/gerona/gls070 | |
| T16 | Medium-chain triglyceride oil | dietary lipid | doi:10.1093/gerona/gls070 | |

Classes in train: mTOR, glucose-lowering (2 drugs), biguanide, oestrogen,
antioxidant/Nrf2, mitochondrial, amino acid, polyphenol, statin,
metabolite, lipid, one combination. Life-shortening arms: none identified
yet; L1 must add them (T1-A2-4).

## Held-out set (outcomes NOT read by A67)

Candidates are named from ITP arm lists only. Whether each was an ITP arm,
its dose, sexes and sites are `[to verify]` by A71, who must **not** pass
the result to Research before the seal.

| Id | Intervention | Class label (mechanism) | Stratum | Why this stratum |
|---|---|---|---|---|
| H01 | 16α-hydroxyestriol | oestrogen | (a) known class | oestrogen class in train (T06) |
| H02 | Astaxanthin | carotenoid antioxidant | (a) known class | antioxidant class in train (T07, T08) |
| H03 | Captopril | ACE inhibitor | (b) new class | no RAS drug in train |
| H04 | Aspirin | COX inhibitor (NSAID) | (b) new class | no NSAID in train |
| H05 | Meclizine | H1 antihistamine | (b) new class | |
| H06 | Fisetin | senolytic flavonoid | (b) new class | no senolytic in train |
| H07 | 1,3-butanediol | ketone precursor | (b) new class | |
| H08 | Nicotinamide riboside | NAD+ precursor | (b) new class | |
| H09 | Rapamycin + acarbose | combination (mTOR + glucose) | (c) combination | both parts in train, pair not |
| H10 | H01–H08, female arms | as above | (c) sex transfer | model fitted on male arms of held-out items only if A50 allows; otherwise drop this stratum |

Count: 9 interventions (H01–H09), 2 in stratum (a), 6 in (b), 1 in (c),
plus sex transfer. **Below the ≥ 10 target**: L1 must add at least one
more ITP arm (ideally a life-shortening or null arm) not read by Research.

## Requirements this proposal does not yet meet

| Requirement | Status |
|---|---|
| ≥ 10 held-out interventions, ≥ 2 classes | 9; classes OK |
| Both sexes | depends on A71 `[to verify]` |
| Null and shortening arms in published proportion (T1-A2-4) | not met: none identified in held-out |
| Control median within ITP/MPD reference range, ITP or replicated (T1-A2-3) | A71 checks |
| `se_source` per row (T1-A2-5) | A71 extracts |
| Baselines sealed with split: chronological age, body weight, frailty index, "everything extends" (T1-A50-2) | listed; A65 seals |

## Hand-off

- A71 (L1): confirm H01–H09 are ITP arms, add ≥ 1 null/shortening arm,
  extract per T5-A2-2. Results go to A65 and the `scoring` environment, not
  to Research.
- A65: approve class labels and strata, seal with the fetch-job log
  (≥ 2026-11-03).
- Open issue S-1: my pretraining knowledge makes all published items
  non-blind (Rule 1). Scorecards must say so.
