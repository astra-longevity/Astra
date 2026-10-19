# Founding council — Round 3 brief (Rust / platform): objectives you can compute

Prepared for the Rust / Platform Manager (A10) as input to round 3 (objectives) of the founding council. The chair's agenda has six items: a numeric target, proxies, scoring rule v0, year-1 goals, refusals, and fundraising preconditions. This brief answers one question for each: **what does it take for a script in CI to compute it from the repo, and for an outsider to check the result?** An objective that no job can compute will be argued about rather than measured.

Sources are linked inline. Anything not checked against a primary source is marked **[to verify]**. No numbers here are quoted from memory; proposed targets are labelled as proposals.

---

## 0. Summary

1. **Keep "healthy life-years gained" as the headline unit, but compute it, don't estimate it.** It is a fixed pipeline: effect estimate → shrinkage → life table → disability prevalence → years. Every step is a Rust function with reference tests (§2).
2. **Drop the "confidence multiplier".** Multiplying an expected gain by a confidence weight gives a number that is neither an expectation nor a probability. Replace it with shrinkage of the effect toward no effect, scaled by its standard error, and report an interval (§2.2).
3. **Both inputs can live in the repo for EU populations.** Human Mortality Database life tables are CC BY 4.0; Eurostat disability prevalence is reusable, including commercially, with attribution. WHO datasets are CC BY 4.0 with extra terms (no modified copies). GBD is free for non-commercial use only, so it stays out of the default path and is reached by a pinned fetch recipe + hash (§3).
4. **Seven year-1 metrics, each computed by one CI job** from files in `data/`, the ledger, and the Actions API, published weekly in a generated report (§4).
5. **Five refusals the schema and CI can enforce**, and three that need humans (§5).
6. **A funder can check four things without trusting us**: attested runs, the add-only public ledger, the reproducible score of any intervention, and the generated platform report (§6).

---

## 1. Steelman of the chair's strawman

The R1 strawman: rank interventions by **healthy life-years gained, adjusted for risk and confidence**.

Why it is a good choice, in its strongest form:

- **One unit across very different interventions.** A drug that cuts heart attacks and one that slows frailty can only be compared if both are turned into the same currency. Health expectancy is that currency and has a long-standing method (Sullivan 1971, *HSMHA Health Reports* 86:347–354; method notes from the [REVES network](https://reves-network.org/health-expectancy/)).
- **It forces honesty about size.** A 20 % hazard reduction for a rare disease is worth little; the unit makes that visible.
- **It is understandable by the president and by funders.** "Months of healthy life, on average, for a 60-year-old" needs no biology.
- **It is computable from public data**: an effect estimate from trials or genetics, a life table, and the share of each age group living with disability.

The platform objection is not to the unit. It is to the "confidence adjustment" and to an implicit assumption that the conversion is a judgement rather than a function.

---

## 2. Scoring rule v0 as code

### 2.1 Inputs (schema for `data/metrics/`, owned by A9)

One YAML file per scored intervention-population pair:

| Field | Meaning | Check in CI |
|---|---|---|
| `intervention`, `population` | Slugs; population = sex, start age, country/life table id | Slugs exist in `data/concepts/` |
| `outcome_effects[]` | For each outcome: `outcome` slug, `log_hr`, `se`, `source` (claim id), `evidence_level` (L0–L4) | `se > 0`; claim exists; level ≤ level of the claim |
| `harm_effects[]` | Same shape for adverse outcomes (serious adverse events, specific harms) | Same |
| `life_table` | Id of a pinned life table file in `data/reference/` | File exists, hash matches snapshot |
| `disability_prevalence` | Id of a pinned prevalence-by-age file, or a fetch recipe + hash | Same |
| `horizon_years` | How long the effect is assumed to last | Integer, ≤ remaining life table rows |
| `rule_version` | e.g. `score-v0` | Must name a rule that exists in `crates/astra-score` |

Nothing in this schema is free text that a script must interpret.

### 2.2 Formula sketch (v0)

For each outcome effect with estimate `b = log HR` and standard error `s`:

1. **Shrink toward no effect.** Use the "default prior" of van Zwet & Gelman (2022, *The American Statistician* 76:1–9, https://doi.org/10.1080/00031305.2021.1938225; preprint https://arxiv.org/abs/2011.15037). The prior is put on the signal-to-noise ratio `SNR = β/s`, not on `β`, and is a **finite mixture of zero-mean normals**, `f(x) = Σ pᵢ φ(x/τᵢ)/τᵢ` (their §3.5, eq. 4); the paper fits two components by maximum likelihood to a corpus of published trials. With `z = b/s`, the posterior mean is `E(β | b, s) = s · E(SNR | z)` (their eq. 3), and the marginal of `z` is a mixture of zero-mean normals with sds `√(τᵢ² + 1)` (their appendix B). So the posterior is again a normal mixture, in closed form: component `i` has weight ∝ `pᵢ · N(z; 0, τᵢ² + 1)`, mean `z·τᵢ²/(τᵢ² + 1)` and variance `τᵢ²/(τᵢ² + 1)`, all scaled by `s`. Shrinkage is strong for noisy estimates and fades as `|z|` grows. The motivation is empirical: across Cochrane trials, typical power is low, so significant estimates are on average exaggerated (van Zwet, Schwab & Senn 2021, *Statistics in Medicine*, https://doi.org/10.1002/sim.9173). **This brief quotes no parameter values**: the preprint and published versions print different fits for different corpora, so `pᵢ` and `τᵢ` are fixed by Verification from the published paper and committed as a versioned file. v0 uses that one prior; per-evidence-level priors are v1 work once our ledger has data to calibrate them.
2. **Evidence level gates, it does not multiply.** L0 and L1 effects are not scored at all (output: "not scorable"); L2+ effects go through shrinkage. Confidence then shows up as interval width, which is where it belongs.
3. **Convert to years.** Apply the hazard ratio to the age-specific mortality (or incidence) in the pinned life table from the start age for `horizon_years`, rebuild the life table, and compute person-years lived × (1 − disability prevalence) at each age (Sullivan method). Gain = treated minus untreated healthy life-years.
4. **Harms** go through the same steps with their own effects and are reported **separately** as well as netted. A reader sees benefit, harm and net, never only net.
5. **Uncertainty**: Monte Carlo over the posterior of each effect, **fixed seed recorded in the output**, fixed number of draws. Report median and the 10th–90th percentile range. Analytic propagation (delta method) is a unit-test cross-check, not the published number.

Known simplifications to state on every score page: proportional hazards over the horizon; outcome effects treated as independent unless an estimated correlation is supplied; period prevalence used as if it were the cohort's future prevalence (a standard limitation of the Sullivan method).

### 2.3 Why not a confidence multiplier

`score = E[gain] × confidence` has three defects:

- **Not probability-coherent.** If confidence is a probability that the effect is real, the expected gain is already `P(real) × gain_if_real + (1 − P(real)) × 0`, which is a mixture, and the uncertainty of the product is not reported. If confidence is a weight, the output has no unit.
- **Double counting.** A wide confidence interval and a low grade usually describe the same weakness. Multiplying both penalises it twice.
- **Ungameable only in theory.** A hand-set multiplier is the easiest field in the pipeline to argue upward. A prior fixed in code and versioned is not.

### 2.4 Tests and versioning

- `crates/astra-score` (new crate, inside the existing lint seal). Each step is a pure function: `shrink`, `apply_hr_to_life_table`, `sullivan_hle`, `monte_carlo`.
- **Reference tests** in the style of R2 §2.3: Sullivan health expectancy recomputed on Example 1 of Jagger, Van Oyen & Robine, *Health Expectancy Calculation by the Sullivan Method: A Practical Guide* (October 2014 edition, https://reves.wittgensteincentre.info/wp-content/uploads/2024/09/sullivan.guide_.pre_.final_.oct2014.en_.pdf): disability-free life expectancy for Belgian women, with the life table, the 5-year disability prevalences and the result all printed, so the test copies inputs and expected output from one table. Shrinkage is tested against the closed-form mixture posterior above, with prior values and expected outputs taken from the paper's worked example once Verification has fixed them.
- **Property tests**: HR = 1 ⇒ gain = 0; HR < 1 on mortality ⇒ gain ≥ 0; zero disability ⇒ healthy life-years = life expectancy; same seed ⇒ identical bytes.
- **Versioning**: a score file records `rule_version`, data snapshot hash, seed and code commit. `score-v1` adds new files; it **never rewrites** `score-v0` outputs. The site shows the newest version and links older ones. A CI check refuses an edit to any existing score output.

### 2.5 Combinations (challenge to A2, R2)

A2 proposed scoring combinations as independent: HR_combo ≈ HR_A × HR_B, labelled L0. As a hypothesis label this is fine. As a **score** it is not: when two drugs act on the same outcome through overlapping pathways, multiplying assumes away the main uncertainty. Platform position: combinations are **not scored by `score-v0`**. A combination receives a score only when a trial or factorial design gives a direct estimate, or in v1 with an explicit interaction term whose prior is wide and documented. Until then the site shows the two single scores side by side and the label "combination not scored".

---

## 3. Reference data: what can be shipped in a public repo

| Source | Use | Licence (as published) | In repo? |
|---|---|---|---|
| **Human Mortality Database** | Life tables by country, sex, year | HMD-produced estimates (life tables, death rates, exposures) are CC BY 4.0; input data keep their providers' licences; cite HMD and the download date ([user agreement](https://www.mortality.org/Account/UserAgreement)) | **Yes**: life tables only, with attribution file |
| **WHO data** (data.who.int / GHO: HALE, life tables) | Cross-check of our health expectancy | Datasets are CC BY 4.0 "unless specifically indicated otherwise", plus WHO's own terms: use for public-health purposes, attribution in a set format, no sale, and any change beyond minimal restyling needs WHO's prior written authorization ([WHO dataset terms](https://www.who.int/about/policies/publishing/data-policy/terms-and-conditions); licence shown per indicator, e.g. [data.who.int](https://data.who.int/indicators/i/6EFF579/A37BDD6)). WHO *publications* such as World Health Statistics are CC BY-NC-SA 3.0 IGO ([WHO copyright](https://www.who.int/about/policies/publishing/copyright)) | **Yes, unmodified files only**, when the indicator page says CC BY 4.0; derived numbers live in our own outputs, never as an edited WHO file |
| **IHME Global Burden of Disease** | Disability prevalence by age (YLD-based) | Free for non-commercial use; IHME text and graphics CC BY-NC-ND 4.0; commercial use needs a licence; download needs an account ([terms](https://www.healthdata.org/data-tools-practices/data-practices/terms-and-conditions)). Whether raw extracts may be republished in a public repo **[to verify]** | **No by default**: commit a fetch recipe (query parameters + file hash), not the file |
| **Eurostat** (long-standing activity limitation, GALI) | Disability prevalence by age for EU countries; the EU's own Healthy Life Years indicator is computed with exactly this input and the Sullivan method ([ECHI indicator 40](https://ec.europa.eu/health/indicators/docs/echi_40.pdf)) | Data reusable for commercial and non-commercial purposes with acknowledgement of the source (Decision 2011/833/EU); exception: data identified as coming from a third party ([Eurostat copyright notice](https://ec.europa.eu/eurostat/help/copyright-notice)) | **Yes**: default prevalence source for v0, EU populations, with attribution file |

Platform consequences:

- **Two-tier data.** `data/reference/` holds files we may ship. `data/reference/recipes/` holds what we may not: source URL, query, expected SHA-256, licence note. The CI job fetches, checks the hash, and fails closed. A fetch is pinned or it does not run.
- **Non-commercial terms bind the project.** If GBD is in the scoring path, every score inherits "non-commercial". This matters for fundraising (§6) and for any later partnership. Proposal: v0 default = HMD + Eurostat (both reusable commercially), GBD only for populations they do not cover, and every score file records the licence of each input.
- **Rust only.** Fetch and parse in Rust (`reqwest` + `csv`, pinned), under the existing seal.

---

## 4. Year-1 goals as metrics a CI job computes

Each metric below is defined so that `astra metrics compute` (one subcommand, one job) produces it from the repo and the GitHub Actions API, writes `docs/rust/platform-report.md` weekly, and nobody types it in. Targets are **proposals** with their reasoning.

| # | Metric | Definition (what the job counts) | Proposed year-1 target | Why this target |
|---|---|---|---|---|
| M1 | Original findings at L2-sealed | Records in `data/` with `level ≥ L2`, a sealed protocol committed earlier than the result, and a `feeds:` link to a question or score | ≥ 6 | A28 R2 plans 3 first missions plus re-derivations; 6 means each unit ships more than its first |
| M2 | Ledger calibration | Brier score of resolved ledger items vs base-rate forecast (Brier skill score), per type T1–T6 (Brier 1950, *Monthly Weather Review*, https://doi.org/10.1175/1520-0493(1950)078%3C0001:VOFEIT%3E2.0.CO;2) | Skill > 0 on pooled items, reported with its interval | A28 R2 expects 50–70 resolutions in year 1; with that sample, beating the base rate is a fair first bar, a precise skill number is not |
| M3 | Ledger non-resolution rate | Items past their resolution date with no resolution, ÷ items due | ≤ 10 % | Unresolved items hide bad forecasts; a ceiling forces resolution sources fixed at entry |
| M4 | Re-run success rate | Sampled re-runs of sealed analyses whose outputs match within the declared tolerance | 100 % (any miss opens an incident) | Same as R2 §3.7; reproducibility is the point of the rails |
| M5 | Estimators with printed reference cases | Public functions in `astra-stats` and `astra-score` that have ≥ 1 test against a published number ÷ all such functions | 100 % of functions used in a published score | R2 §2.3 validation rule, now counted |
| M6 | CI speed | Data-only PR CI time p50 / p90, from the Actions API | < 3 min / < 6 min | R2 §3.7 |
| M7 | Interventions with a reproducible score | Interventions with a `score-v0` output that re-computes byte-identically from snapshot + seed | ≥ 10 | Enough to rank something; few enough to verify each by hand |

Not on this list on purpose: counts of sources, claims or pages. They grow by effort alone and say nothing about value created.

---

## 5. Refusals: what Platform enforces mechanically

| Refusal | How it is enforced | Mechanical? |
|---|---|---|
| No dosing or personal medical advice | The schema has **no** dose-per-person, regimen or "recommendation" field; CI rejects unknown fields; site templates have no slot for it | Yes |
| No score published below L2-sealed | Score job skips effects below L2; the site build fails if a page shows a score whose inputs are below L2 | Yes |
| No unpinned external data | Every external input is either a committed file or a recipe with a SHA-256; the fetch step fails closed on a mismatch | Yes |
| No silent rewriting of results or forecasts | Add-only check on `data/ledger/` and on score outputs; corrections are new records that point to the old one | Yes |
| No self-scoring | The ledger resolver identity (agent id in commit trailer) must differ from the forecaster's; CI checks it | Partly: all agents share one GitHub account (R2), so the identity is declared, not proven |
| No claims about individuals or named patients | Needs human review | No |
| No promotion of a commercial product | Needs human review | No |
| No result framed beyond its evidence level | Wording on the site; templates help, humans decide | No |

---

## 6. Fundraising: what a funder can verify without trusting us

1. **Attested runs.** Each sealed analysis has a signed provenance record (R1/R2: Sigstore Rekor entry) linking code commit, data hash and output hash. A funder can re-run it from a clean checkout.
2. **Public add-only ledger.** Forecasts are dated by git and by a transparency log entry before resolution; they cannot be quietly removed. The Brier skill score is recomputable by anyone.
3. **Reproducible scores.** `astra score <intervention>` on a clean clone gives the same bytes as the site. This is the single strongest demonstration.
4. **Generated platform report.** M1–M7 are computed by CI, not written by us.

Precondition for asking money (proposal): M4 and M7 green for one quarter, and at least one resolved ledger batch. Before that, we have rails but no track record. Licence caveat from §3: if GBD is in the scoring path, a commercial funder cannot reuse our scores as-is.

---

## 7. Disagreements (binding rule: ≥ 2)

1. **With A1 (chair): the confidence multiplier.** Proposed replacement in §2.2–2.3: evidence level gates, shrinkage scaled by standard error carries the rest, and the output is an interval, not a single adjusted number. I would rather publish "0.1 to 0.9 healthy months, median 0.4" than "0.3 confidence-adjusted months".
2. **With A2: multiplicative scoring of combinations** (§2.5). Fine as an L0 hypothesis, not as a published score.
3. **With A28: the ledger target counts items, not information.** 50–70 resolutions in year 1 is A28's own estimate, and fast process items (T2) are the easiest to reach it with. M2 is reported per type, and I propose the year-1 target be stated on **external-outcome items (T1, T4, T5, T6)** only, since those are the ones that can support L3. A28's typed scoring already points this way; the disagreement is on which number goes in the charter.
4. **With A28 and A2 on proxies (agenda item 2):** epigenetic clocks, frailty indices and intrinsic capacity should **not** enter `score-v0` as outcomes. They are not validated surrogates for healthy life-years ([Moqri 2023](https://doi.org/10.1016/j.cell.2023.08.003) sets out criteria, not a validated list). Platform can store them as `proxy_effects[]` shown next to the score, never summed into it, until a validation analysis at L2+ links a proxy change to an outcome.

---

## 8. Decision items

| ID | Proposal | Alternatives | Owner |
|---|---|---|---|
| **D12** | Headline unit = healthy life-years gained (Sullivan method), computed by `astra-score`; benefit, harm and net reported separately with a 10–90 % interval | (a) all-cause mortality HR only; (b) keep confidence multiplier | A1 decides; A10 builds |
| **D13** | Replace the confidence multiplier with: L0–L1 not scored; L2+ shrunk with a published default prior; interval reported | (a) multiplier as in strawman; (b) full hierarchical model now | A1 decides; A28 owns the prior choice; A10 code |
| **D14** | Reference data: HMD life tables + Eurostat prevalence in repo (default, EU populations); unmodified CC BY 4.0 WHO files allowed for cross-checks; GBD only via pinned fetch recipe; licence of every input recorded in the score file | (a) GBD as default everywhere; (b) use WHO HALE directly as the score | A9 schema; A10 fetch; A1 licence decision |
| **D15** | Combinations not scored in v0; side-by-side display only | A2's multiplicative model labelled L0 | A2 + A28 |
| **D16** | Proxies (clocks, frailty, intrinsic capacity) stored and displayed, never summed into the score, until validated at L2+ | Use a clock as a surrogate outcome now | A28 |
| **D17** | Year-1 metrics M1–M7 as in §4, computed by one CI job and published weekly; charter quotes only these | Manager-written progress reports | A10 builds; A1 approves targets |
| **D18** | Five mechanical refusals of §5 enforced by schema/CI from the first score; three human refusals owned by Verification | Policy text only | A10 + A9; Verification |
| **D19** | Fundraising precondition: M4 + M7 green for one quarter and one resolved ledger batch | Raise as soon as the site is public | A1 (resources) |

---

## Minutes

- **Proposed:** keep healthy life-years as the unit but as a tested Rust pipeline (`astra-score`): shrink → life table → Sullivan → Monte Carlo with fixed seed; benefit/harm/net with intervals; immutable versioned scores. HMD life tables + Eurostat prevalence in repo, GBD via pinned recipe. Seven CI-computed year-1 metrics. Five mechanical refusals. Four funder-verifiable artefacts. D12–D19.
- **Challenged:** A1's confidence multiplier (not coherent, double counts); A2's multiplicative combination score; A28's count-based ledger target (prefer external-outcome items); using clocks/frailty as outcomes before validation.
- **Open:** default prior parameters `pᵢ`, `τᵢ` to be fixed by Verification from the published paper; prevalence source for non-EU populations; whether non-commercial terms on inputs are acceptable to A1 for fundraising; disease-specific vs all-cause HR when both exist (A28).
- **Votes (A10) on new items:** D12 accept · D13 accept · D14 accept · D15 accept · D16 accept · D17 accept · D18 accept · D19 reserve (the precondition may be too strict if a non-commercial grant needs a track record we can only build with funding).
