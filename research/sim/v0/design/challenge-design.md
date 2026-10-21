# R-Sim v0: candidate acute-challenge design (no mice requested)

Status: **v0 candidate design, synthetic basis only.** This is not a mouse
request. Mice come only after metric v0 passes (charter part 6). Numbers below
come from `../identifiability.md` §9 at `n_sim` 600 and must be re-checked by the
Actions run (`actions-job.md`, `n_sim` 20 000 / 50 000) before any request.

## Why a challenge

Readouts, cross-sectional or repeated, pin only each organ's balance
eta − beta (damage production minus repair) (§6, §8). An acute insult followed
by a recovery curve is the only route found that separates them; the mapped
intervention route was dropped (circular, and gains nothing, §9.4).

## Design

| Item | Value | Basis |
|---|---|---|
| Ages at challenge | 6 months (splits eta/beta) and 18 months (pins Xc, the organ's damage threshold) | §9.3: early organs are far from repair saturation, so recovery shows within 26 weeks; late challenge shifts the level and informs Xc |
| Doses | 3 doses + sham: 0.15, 0.3, 0.6 × prior Xc (model units) | 2–3 doses measure kappa (repair half-saturation, not identified, decides everything, §9.2). Only 0.3 was simulated; real dose per test = dose-finding [to verify] |
| Readout times | +1, 2, 4, 8, 16, 26 weeks after the insult | §9.1 |
| Mice | ~100 per time point per arm per sex | §9.3: n 10 never splits; n 100 splits 5 organs; n 30 = n 100 plateau may be Monte-Carlo floor (I17, I18) |
| Sexes | both (parameters are per sex) | spec.md |
| One insult per mouse | yes, one organ per cohort | two insults would couple organs through hallmarks |

## Organs

| Organ | Split eta/beta at 6 mo (share, n 100) | Xc at 18 mo (share, n 10, kappa 0.5) | Test | Readout | Repeatable |
|---|---|---|---|---|---|
| A3 brain | 0.94 | 0.98 | [to verify] | [to verify] | [to verify] |
| A6 heart | — | 0.92 | isoproterenol 30 mg/kg/d 28 d, 28 d off (doi:10.3390/ijms23010174) | echo | yes |
| A8 lung | — | 0.97 | bleomycin oropharyngeal (AJRCCM 2024;209:A2507) | histology | no |
| A9 kidney | 0.99 | 0.98 | folic acid i.p. 125–250 mg/kg (PMC11430329) | transcutaneous GFR | yes |
| A10 liver | — | 0.85 | acetaminophen (doi:10.18632/aging.103973); 75 % hepatectomy (PMC12284365) | histology / serum; imaging volume [to verify] | no ([to verify]) |
| A11 immune | — | 0.95 | fixed reference antigen [to verify] | titre | yes |
| A12 blood | 0.92 | 0.93 | 5-FU single dose (PMID 25317679) | CBC, tail bleed | yes |
| A13 skin | 0.99 | — | punch wound [to verify] | closure photo | yes |
| A14 gut | 0.86 | — | DSS 2 % 7 d + recovery (doi:10.1371/journal.pone.0268787) | weight, disease score | yes |

12 organ × age cells (5 at 6 months, 7 at 18 months). Muscle (A1), fat (A15)
gain nothing in the simulation; muscle still has the best aged-vs-young
literature (stand-ins below).

## Mouse count (per sex, at 100 per time point)

- Repeatable readout: the same 100 mice give all 6 time points → 100 per arm.
- Terminal readout: a new group per time point → 600 per arm.
- Sham: one group per age and readout type, shared across organs.

| Block | Cells | Mice per cell | Mice |
|---|---|---|---|
| Repeatable: kidney, blood, skin, gut at 6 mo; heart, kidney, blood at 18 mo | 7 | 3 doses × 100 | 2 100 |
| Terminal: brain at 6 mo; brain, lung, liver at 18 mo; immune at 18 mo counted here until its test is known | 5 | 3 doses × 600 | 9 000 |
| Sham, repeatable (2 ages) | — | 100 | 200 |
| Sham, terminal (2 ages) | — | 600 | 1 200 |
| **Full** | | | **12 500 per sex, 25 000 total** |
| **Minimal: repeatable cells only** | | | **2 300 per sex, 4 600 total** |

Terminal readouts cost 6 × more and dominate the full count. The minimal design
keeps 4 of the 5 split organs (all but brain) and 3 of the 7 Xc organs.
Before any request: (1) Actions rerun of §9 at 20 000 to confirm n 100 (n 30 may
suffice once the MC floor is gone); (2) check whether 2 doses are enough for
kappa; (3) find repeatable readouts for brain and liver.

## Public data that could stand in (no mice)

Published recovery curves in aged vs young controls can test the model's key
prediction before any mouse: if repair is saturated (prior kappa 0.5), old
organs recover much more slowly than young ones after the same insult.

| Organ | Source | Ages | Use |
|---|---|---|---|
| A1 muscle | PMC7369722; PMC6156497; PMID 26273023 | 3 vs 18 mo; 2 vs 23 mo; n 7–12/sex/age | old/young recovery-speed ratio → kappa |
| A10 liver | doi:10.18632/aging.103973 | > 80 vs 8–10 wk | same |
| A6 heart | doi:10.3390/ijms23010174 | 10 wk vs 22 mo, females | reverse remodelling speed |
| A8 lung | AJRCCM 2024;209:A2507 (abstract only) | 80 vs 10 wk | resolution at 3/6/9 wk |
| A14 gut | doi:10.1371/journal.pone.0268787 | 17–23 mo males; young arm [to verify] | weight recovery |
| A9 kidney, A12 blood | PMC11430329; PMID 25317679 | young only | dose-response shape, not age |

Limits: group means from figures (digitised, [to verify] per source), mostly
terminal readouts, different insults and strains; enough to bound kappa, not to
fit eta/beta per organ. None of these pages reports lifespan (no
`research/metric/v0/manual-downloads.csv` rows). Digitising is a separate task (data, A9).
