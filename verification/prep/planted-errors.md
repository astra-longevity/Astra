# Planted errors: draft (D66)

Status: DRAFT for A10 + A50. Nothing here is a rule until A10 and A50 fix the cut-offs (D66).
Every threshold marked "provisional" follows A50's R5 target: detection < 80% = unreliable.

## 0. Ground rules (from D66, D71)

- At least 20 plants a quarter, spread over Verification's audit samples (D51 citation audit, D63 extraction audit).
- Error types are public (this file). Which audited items carry a plant is hidden.
- A plant is written by a Verification agent, never by a producer (Literature, Research, Database).
- A plant lives only in the audit packet handed to the auditor. It is never committed to `data/`, so it can never reach the score.
- A plant must pass every mechanical check (schema, `db build --check`, DOI resolves, CI). Plants test the reader, not the linter. A plant a linter would catch is a broken plant: fix the plant, don't count it.
- A plant counts as detected only if the auditor flags that item **and** names the right field. Flagging the item for another reason is a miss.
- Plants feed D71 (accuracy against known truth) as their own row, labelled "planted".

## 1. Error type catalogue

Each type has a mutation rule a script can apply to a copy of a real object, and the audit step expected to catch it.

| # | Type | Object / field | Mutation rule | Expected catch |
|---|------|----------------|---------------|----------------|
| P1 | Wrong estimate (digits) | extraction: point estimate | Swap two adjacent non-equal digits (0.58 → 0.85); result must stay inside the CI's plausible range, else pick another pair | D63 extraction audit, re-read at locator |
| P2 | CI bound swap / shift | extraction: CI low/high | Replace one bound by the other arm's or the adjacent row's bound in the same table | D63, re-read at locator |
| P3 | Wrong locator | evidence: `locator` | Point to a real neighbouring item of the same kind ("Fig. 2" → "Fig. 3", "Table 1" → "Table 2") that does not show the value | D51 citation audit (locator re-read) |
| P4 | Unsupported citation | evidence: `source` or `direction` | Swap the source for another source in the same claim cluster that does not report the claim, or flip `supports` ↔ `nuances` where the text does not back it | D51: result "not supported" |
| P5 | Flipped allele / effect direction | MR / genetics row: effect allele, beta sign | Swap effect and other allele, or flip beta sign without flipping the allele | D63 extraction audit; MR harmonisation review |
| P6 | Wrong arm | extraction: arm label / group id | Attach the control arm's numbers to the treatment arm (or the reverse) | D63, re-read at locator |
| P7 | Events/n swap | extraction: events, total | Swap events and non-events, or take n from the randomised total instead of the analysed total | D63; D71 CT.gov cross-check where available |
| P8 | Wrong timepoint | extraction: timepoint | Take the value at another reported follow-up time (12 → 24 months), same table or figure | D63, re-read at locator |
| P9 | Unit error | extraction: value + unit | Keep the number, change the unit (mg/dL ↔ mmol/L, months ↔ years), or convert with the wrong factor | D63 extraction audit |
| P10 | Wrong population / subgroup | extraction: population | Take the subgroup row instead of the overall row (or per-protocol instead of ITT) | D63, re-read at locator; D71 |

Spread rule (provisional): per quarter, no type gets more than 4 plants and at least 6 types are used. Types are drawn with a seeded random draw recorded in the sealed manifest (section 3).

Per-type detection rates are not estimable from one quarter (about 2 plants per type). They are reported as raw counts (detected / planted) each quarter and judged on a rolling 4-quarter pool (≈ 8 per type), labelled "indicative".

## 2. Rolling count rule (mirrors D63)

D63 uses a 3-month pool and three outcomes: pass / double the next sample / incident. Same shape here.

Pool: the last quarter's plants, n = 20 by default (n = 40 after a "double" outcome). Count m = misses (plants not detected with the right field).

The judged quantity is the true miss rate q. Target (provisional): q ≤ 20%, i.e. detection ≥ 80%.

**n = 20**

| Misses m | Outcome | Reasoning |
|----------|---------|-----------|
| 0–4 | pass | Observed detection ≥ 80%. |
| 5–7 | double: next quarter n = 40 | Observed detection 65–75%. Not strong evidence on its own: with q = 20% exactly, m ≥ 5 happens 37% of the time. |
| ≥ 8 | incident (S-level set by A50) | With q = 20%, P(m ≥ 8) = 3.2%. So m ≥ 8 is strong evidence detection is below 80%. |

**n = 40 (after a double)**

| Misses m | Outcome | Reasoning |
|----------|---------|-----------|
| 0–8 | pass | Observed detection ≥ 80%. |
| 9–13 | incident | Two quarters in a row below 80% observed. |
| ≥ 14 | incident | With q = 20%, P(m ≥ 14) = 1.9%. |

Binomial tails, P(m ≥ k) for n = 20 (computed with exact binomial sums, script in the PR description):

| k | q = 10% | q = 20% | q = 30% | q = 40% | q = 50% |
|---|---------|---------|---------|---------|---------|
| 5 | 0.043 | 0.370 | 0.762 | 0.949 | 0.994 |
| 8 | 0.000 | 0.032 | 0.228 | 0.584 | 0.868 |

What n = 20 can and cannot do (state it publicly):

- False incident when detection is truly 80%: 3.2% a quarter.
- Catching a team at 70% detection (q = 30%): incident only 23% of the time in one quarter; "double" or worse 76% of the time. The double step is what makes the rule work, not the incident line.
- Catching a team at 50% detection: incident 87% in one quarter.
- Telling 80% from 75% apart needs a few hundred plants. Out of reach; do not claim it.

Also an incident, whatever m: any plant that reaches `data/` on main, or any plant written by a producer.

Open choices for A10 + A50:
1. Should 9–13 at n = 40 be "incident" or a second double? Draft says incident (two weak signals in a row).
2. Should the first quarter of a new unit use n = 40 (mirrors D63's doubled first 3 months)? Draft says yes.
3. Does a plant flagged with the wrong cause count half? Draft says no (miss).

## 3. Key handling

The "key" is the manifest: which audit items carry a plant, which type, what mutation, and the random seed.

### Option A: commit-reveal (usable now)

1. Before the audit packet goes out, the Verification agent writes the manifest plus a random 32-byte salt.
2. It commits only `sha256(manifest || salt)` to `verification/` (one line per quarter, with date).
3. After the audits close, it commits the manifest and salt. Anyone can recompute the hash.

Protects: Verification cannot change after the fact which items were plants (no quietly dropping missed plants, no relabelling a real error as a plant). Timing is public through git history.

Does not protect: secrecy. All agents run on one computer with one shared token (D68 refused), so any agent can read the manifest file before the reveal. An auditor who looks can find the plants. This is why the public label must be **"planted (key not isolated)"** (D66). It also relies on agents keeping files out of shared paths: manifest stays outside the repo and outside any shared panel until reveal.

### Option B: isolated environment (later, needs the president)

The manifest is generated and kept inside an environment no agent can read: e.g. a GitHub Actions environment with a secret seed and required reviewer = president, or a machine/account only the president controls. Plant selection runs there; agents only receive the finished audit packet.

Protects: secrecy against agents, as long as no agent can change the workflow that holds the seed (workflow edits need review, and agents never touch protection or settings, M-tiny-12).

Does not protect: a workflow change that prints the seed if it gets merged; the president as single point of trust; and it needs a president setup step. The commit-reveal hash (option A) is still published on top, so B adds secrecy, A keeps honesty.

Draft recommendation: start with A now, label "planted (key not isolated)"; ask the president for B when D66 cut-offs are fixed.
