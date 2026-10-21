# R-Sim v0: what public data can calibrate, and what still needs mice

Companion to `identifiability.md`. Three parts:

1. **§A** — readout identifiability on A9's *real* public design (DRiDO / MPD
   ages, n, sex), i.e. what v0 can identify today without any new mouse.
2. **§B** — zero-mouse stand-ins: published aged-vs-young recovery curves A9
   could digitise, and exactly what they bound.
3. **§C** — first mouse-request *candidates* (not a request): the readouts
   public data is missing, plus the factorial arm O4 needs.

All synthetic fits use the same seeded Fisher machinery as `identifiability.md`
(`py/design/public_design.py`, seeds 11/12, ideal assay, assay noise `NOISE_REL` × SD).
Mice come only after metric v0 passes (charter part 6); nothing here is a
request.

## A. Identifiability from current public data (A9 coverage `26336bb`)

`public_design.py` builds the exact age/n/sex grid A9 reported and fits the
per-organ readout Fisher, reporting the share recovered on the balance
`eta − beta` (`ratio`), on `eta + beta` (`sum`) and on `xc`. A balance counts as
**pinned** at `ratio ≥ 0.80`. `n_sim 1000`, `noise_rel 0.5`.

### A.1 Female (DRiDO + MPD), 8 readout sources

| Source | Organ | Ages (mo) → n |
|---|---|---|
| DRiDO echo | A6 heart | 10→182, 22→123, 34→17 |
| DRiDO flow | A11 immune | 5→187, 16→166, 28→72 |
| MPD Petkova1 flow | A11 immune | 6/12/18/24 → 7 |
| DRiDO CBC | A12 blood | 11→182, 23→113, 35→14 |
| MPD Peters4 CBC | A12 blood | 6/12/18/24 → 7 |
| MPD Korstanje1 albuminuria | A9 kidney | 12/18/24 → 7 |
| DRiDO fasting glucose | A10 liver/metab | 5→185, 16→164, 28→68 |
| DRiDO DEXA fat/lean | A15 fat | 10→182, 22→123, 34→17 |

Balance pinned (`ratio`): **immune 0.80, blood 0.88, fat 0.84**. Below bar:
liver 0.74, heart 0.64. `sum` ≤ 0.01 and `xc` ≤ 0.08 everywhere — no single
parameter and no `Xc` is identified (`n_identified = 0`).

### A.2 Male (MPD only, n 7), 3 readout sources

immune 6/12/18/24→7, blood 6/12/18/24→7, kidney 12/18/24→7. Balance pinned:
**immune 0.80, blood 0.83**. Kidney 0.79 sits just under the bar; `sum`/`xc`
≈ 0.

### A.3 Ideal §6.4 reference (n 10 × 12/18/24/30 mo × 11 organs)

Recovers 12 directions: `eps` (identified) + 9 organ balances pinned (muscle
0.93, skin 0.94, gut 0.95, fat 0.95, liver 0.89, kidney 0.89, blood 0.89, lung
0.84, immune 0.86; brain 0.78 and heart 0.78 just under). `Xc` still never
pinned from readouts (max brain 0.21).

### A.4 The female-kidney artefact (read before using A.1)

Same MPD Korstanje1 albuminuria source gives kidney `ratio` **0.02 in the
female design, 0.79 in the male design, 0.89 in the ideal design**. The kidney
Fisher is identical; what changes is the recoverability threshold. The
threshold is `3 × |most-negative eigenvalue|` of the pooled Fisher (guard
against Monte-Carlo-negative modes). The big-n DRiDO sources (n ≈ 70–190) push
that threshold to ≈ 278 in the female pool vs ≈ 25 male, so the small n-7 kidney
mode falls under it and reads as "not recovered". This is a **Monte-Carlo /
threshold artefact, not a real loss of kidney information** — the same class of
artefact as I18. The Actions run at 20k (lower MC floor) settles it; until then
read female kidney as *unresolved*, not *zero*.

### A.5 Verdict

From current public data, v0 can pin the `eta − beta` balance in **2–3 organs**
(immune, blood; fat in females) and nothing else: no single parameter, no `Xc`.
Public data alone cannot calibrate v0 beyond those balances. Everything else
needs the readouts in §C.

## B. Zero-mouse stand-ins: published aged-vs-young recovery curves

For A9 to digitise. These bound **kappa** (repair half-saturation sets recovery
speed) and let us falsify the model's recovery *shape*; they **cannot** fit
`eta` vs `beta` separately (no independent perturbation of production vs
clearance) nor `Xc`. Snippets only; no page read reports a lifespan outcome (no
`manual-downloads.csv` row). Figure/panel and n are [to verify] unless stated.

| Organ | Paper (DOI/PMID) | Figure / panel | n per group | Ages | Digitising gives |
|---|---|---|---|---|---|
| A1 muscle | doi:10.3390/ijms21134575 (PMC7369722) | **Fig 2G** regen/damaged area; Fig 2F fibre CSA | **5** (fig n 4–5) | 3 vs 18 mo, d3/5/7 | recovery fraction young 92/99 % vs aged 13/54 % d5/d7 → kappa bound; curve stops at d7 |
| A1 muscle | doi:10.1111/acel.12815 (PMC6156497) | Fig 2e,f fibre morphology d8 | [to verify] | 2 vs 23 mo, d0–16 | aged recovery delayed; longer window than above |
| A10 liver | doi:10.18632/aging.103973 | [to verify] | [to verify] | >80 wk vs 8–10 wk | APAP repair delay in aged → kappa bound |
| A10 liver | PMC12284365 | [to verify] | 20 | 33–35 wk, 2 & 7 d | 75 % PHx regrowth curve |
| A6 heart | doi:10.3390/ijms23010174 | [to verify] echo FS over off-phase | [to verify] | 10 wk vs 22 mo (F) | impaired reverse remodelling in aged → kappa bound |
| A8 lung | doi:10.1152/ajplung.00253.2021 | [to verify] | [to verify] | old vs young, 3/6/9 wk | bleomycin ECM resolution; old slower |
| A14 gut | doi:10.1371/journal.pone.0268787 | [to verify] weight/DAI curve | [to verify] | 17–23 mo (M) | DSS weight + disease-index recovery |
| A12 blood | PMID 25317679 | [to verify] CBC d2–d7 | [to verify] | not aged | 5-FU CBC regeneration shape (age [to verify]) |

Only muscle (Rahman 2020, Fig 2G) has panel + n read from full text; the rest
need A9 to open the paper. Brain, skin, immune, fat, kidney aged recovery
curves: none located yet → named limit (see §C).

## C. First mouse-request candidates (not a request)

Readouts public data lacks, each a candidate arm if/when mice are granted.
Repeatable = measurable on the live mouse (no terminal cohort per time point).

| # | Readout | Organ | Ages (mo) | n/age/sex | Unlocks | Repeatable? | Note |
|---|---|---|---|---|---|---|---|
| R1 | GTT + lean mass | A10 liver/metab | 12/18/24/30 | 10 | liver balance (public only fasting glucose) | yes | pair GTT with a monotonic liver readout (§8.2) |
| R2 | transcutaneous GFR | A9 kidney | 12/18/24/30 | 10 | kidney balance + `Xc` (public: albuminuria only) | yes | resolves the A.4 artefact directly |
| R3 | vaccine, fixed reference antigen | A11 immune | 12/18/24/30 | 10 | immune balance, age-comparable | yes | new-antigen-per-age design loses the organ (§8.2) |
| R4 | echo in **males** | A6 heart | 12/18/24/30 | 10 | heart balance (DRiDO echo is F only) | yes | |
| R5 | 30-mo arm, both sexes | all | 30 | ≥10 | lifts late-age Fisher; public sparse >28 mo | — | adds the oldest point public data lacks |
| R6 | repeatable brain readout | A3 brain | 12/18/24/30 | 10 | brain balance + `Xc` | target yes | none found public or in §9.5 → open item (X1029) |
| R7 | repeatable muscle / skin / gut / lung readout | A1/A13/A14/A8 | 12/18/24/30 | 10 | those balances | mixed | §9.5 tests are mostly terminal |
| R8 | **factorial arm**: both singles + combo, same cohort | any scorable class pair | per design | per design | calibrates the combination rule NA1 (O4) | — | ITP has 0 factorial cohorts, so additivity is an assumption, not calibrated (A70 O4); NA1 flagged uncalibrated in `spec.md` / `edges.md` |

All R1–R8 are candidates only. The cheapest subset that pins a balance in ≥3
organs is set by the cost curve (`identifiability.md` §11, filled from A61's
20k challenge run).

## D. Repeatable brain and liver readouts (X1029)

§9.5's brain and liver tests were terminal or [to verify]. A repeatable (live,
non-terminal) readout exists for both, so these are **not** named limits — they
go on the candidate list (§C R6, R1) rather than being recorded as impossible.
Snippets only, none read in full; n/ages [to verify].

| Organ | Repeatable readout | Source (snippet) | Notes |
|---|---|---|---|
| A3 brain | resting-state fMRI network efficiency, longitudinal over the adult lifespan | Nat Commun Biol 2024, doi:10.1038/s42003-024-05873-8 | anaesthetised; aging effect from ~8–9 mo (also doi:10.3389/fnagi.2019.00277); de-differentiation mainly in females |
| A3 brain | longitudinal cognitive / exploratory-behaviour testing | Aging Cell 2026, doi:10.1111/acel.70287 | behaviour is a stable adult trait; learning/repeat-exposure effects must be controlled |
| A3 brain | structural + MRS (volume, metabolites) | Front Aging Neurosci 2016, doi:10.3389/fnagi.2016.00158 | review; MRI/MRS suited to longitudinal measures |
| A10 liver | serum ALT/AST (tail bleed) | PMC10585574 (review context) | repeatable but not liver-specific; pair with imaging |
| A10 liver | ultrasound + shear-wave elastography, imaged every 4 wk | PMID 39523997 | longitudinal cohort n 20 over 48 wk; tracks fibrosis/steatosis |
| A10 liver | ASGPR / multimodal functional imaging | PMC10585574 | functional, maps to liver-function serum indices |

Named-limit status: none for brain or liver. The still-open repeatable-readout
gaps are muscle, skin, gut, lung (§C R7); those stay candidates, not limits,
pending a dedicated search.

## Reproduce

```
cd research/sim/v0/py/studies
PUBLIC_CACHE=/tmp/public.pkl python3 ../design/public_design.py 1000 > ../../results/public-design.json
```
