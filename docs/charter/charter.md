# Astra founding charter (draft for D46 ratification)

Drafted by the chair (A1) from:
- the president's ten commandments (`docs/charter/commandments.md`), which
  outrank everything below;
- Arcanum session 1 decisions D114–D123 (`arcanum/session-1/topics/`,
  merged at `c24b135`);
- the president's answers of 2026-10-20 to the session-close form (D122);
- the old log D1–D113 (`docs/decisions/founding-council-log.md`), which is
  read-only history. An old decision binds only if session 1 re-issued it
  (T7, D120). Anything not re-issued is dead.

Nothing in this charter is new. Every paragraph ends with the decisions it
rests on. A paragraph with no citation, or with a citation that doesn't
cover what it says, is a defect that the D46 diff must flag.

---

## Page 1: read this first

### What Astra is

Astra is a team of AI agents, run for its president, whose purpose is to
extend mammalian lifespan by at least 2 to 3 times. The mouse target is 3×,
and it never moves. Its core is **simulation**: a computer model of how a
body ages. Real experiments, on mice and on the president's own samples,
exist only to confirm or refute what the model predicted in advance.

*Cites: commandments 1–3, D115, D117.*

### What comes first

Nothing advances without a way to **measure biological age with no blind
spots**. So the first job is a measure of biological age built as a set of
readings, one per organ system and per cause of ageing, rather than a single
number. No treatment is designed until that measure passes its tests.

*Cites: commandments 5, 6, 9, D114, D115.*

### What to expect in the first 6 months

- **Week 1:** data fetched by manifest; the train/test split sealed.
- **Months 1–6:** measure v0 on public mouse data with its scorecard; the
  simulation v0 calibrated on untreated mice and scored against a simple
  curve-fit baseline; dated forecasts on mouse studies still running.
- **By month 6:** a first request for mice, sent only with the measure's
  scorecard and a sealed prediction for the mice to test.
- **What can go wrong** is tracked as 20 named obstacles (16 + 4 for
  Verification), each with an
  owner, an exit and a date (part 9). "Impossible" is never an answer: an
  obstacle is the next target.

*Cites: commandment 4, D117, D119, D121.*

### Decisions that belong to you

| Decision | State |
|---|---|
| Separate GitHub identities for agents (D68) | **Refused** 2026-10-20. Agents share one token. |
| Three missions start before ratification (D78) | **Approved** 2026-10-20: Verification, forecast ledger, tooling. |
| Where your health data lives (D122) | **Answered** 2026-10-20: public repo, encrypted; key only in a local, git-ignored `.env`. This replaces D116's `astra-private`. |
| May agents read your raw values (D122) | **Yes.** |
| What is published about you (D122) | You left it to us: default = per-measure records only for measures you list, none until you list any. |
| First two blood draws (noise baseline) (D116, D122) | **Later**, when you say so. |
| `scoring` environment, `main` only (D122) | **Done** 2026-10-20 by A1 at your request. |
| `PROTECTION_READ_TOKEN` (D122) | **Done** with your full token, at your choice. Moving it into a `main`-only environment: **refused** 2026-10-20; it stays a repository secret. |
| Charter-level decisions D124, D125 (part 10, part 11) | Voted by the managers with D46; yours to confirm at merge. |
| Re-seal of code-protection settings (M2) | Pending: the Code Manager sends you the command. |
| Mice: when and how many | Yours. Each request comes with a sealed prediction (part 4). |
| Verification's share of workers, each quarter, ≥ 15% | Yours. |
| Overruling a Verification finding | Only you, in writing. |
| Merging this charter | You merge after the managers ratify it (D46). |

*Cites: D46, D47, D64, D68, D78, D116, D117, D120, D122, president rulings PR1, PR8, PR9 (`docs/decisions/president-rulings.md`).*

### Limits you must know

- **Rules about who writes where are conventions, not proof.** All agents
  share one token, so a careless or tricked agent can break them. We detect
  breaches after the fact (part 7).
- **Your token is stored as a GitHub secret with full rights.** Today it is
  a repository secret, so a workflow pushed on any branch could read it.
  The president chose to keep it there (PR11), so "only the daily
  protection check uses it" is a convention.
- **Your encrypted data can't be audited or withdrawn.** Agents read it
  through the local key, which leaves no trace in git, so Verification's
  monthly access audit can't run. If the key ever leaks, the encrypted
  history stays public.

*Cites: D52, D68, D80, D108, D116 (T3-A50-3), D122, PR8, PR9.*

---

## Part 1: the measure of biological age

Biological age is a **vector**: one axis per organ system and per hallmark
of ageing, each tied to a functional test (for example grip strength for
muscle). A summary number is published, but it is never an objective, so
no gain on one axis can hide a loss on another.

*Cites: commandments 5–6, D114.*

A version of the measure counts only if it passes six tests, against
baselines sealed in advance:
1. **Coverage with proof:** every organ system and hallmark has an axis,
   and each axis moves the right way under an intervention known to act on
   it, and the other way under one that shortens life.
2. **Agreement with lifespan effects** on ≥ 10 published mouse
   interventions from ≥ 2 classes, both sexes, on a sealed test set.
3. **Negative controls:** null arms of the Interventions Testing Program
   must read as null within their observed interval.
4. **Individual level:** it predicts each held-out mouse's remaining
   lifespan better than chronological age.
5. **Adversarial cases** written by Verification, never by Research.
6. **Declared validity range:** readings beyond the effect sizes it was
   validated on are labelled "extrapolated".

Every reading carries its interval (test-retest noise plus model
uncertainty); a reading without one isn't published. Each version is sealed
before its test score is computed, and its scorecard is published with its
failures. Test 2 uses only studies whose controls lived within the normal
range (ITP or replicated), includes null and life-shortening arms, and
reports an "everything extends lifespan" baseline next to the measure.
Each standard error records its source (`printed`, `from_curve` or
`from_p_value`); `from_p_value` is used only in a sensitivity run, and a
standard error read from a curve enters test 2 only if the curve was
digitised twice and agreed.

*Cites: D114 (T1-A28-3, T1-A50-1, T1-A50-2, T1-A28-5, T1-A2-3, T1-A2-4, T1-A2-5).*

Scores on published interventions are labelled "retrospective, not
blind". Only sealed forecasts and held-out mice count as blind evidence.
Test 4 is labelled "blind (key not isolated)" until its keys sit in the
`scoring` environment.

*Cites: D114 (T1-A50-3, T1-A10-4).*

A **confirmed worsening** on an axis is decided by a family-wise rule that
Verification fixes when the version is sealed, on axes that passed test 1.
Axes that didn't are published as "unmeasured" blind spots. An
intervention or combination with a confirmed worsening can't count toward
3×, can't be called progress and can't be the subject of a treatment
request, unless a sealed, tested simulation predicts that a combination
cancels it. Trade-offs are always reported in full.

*Cites: commandment 6, D114 (T1-A1-3, T1-A28-6).*

The human version uses the same axis names, built only from affordable,
repeatable samples.

*Cites: D114 (T1-A28-4), D116.*

## Part 2: the simulation

Simulation v0 is an organ-level model of damage and repair whose state is
the measure's axes. Deeper levels (cells, molecules) are added only after a
recorded failure of the level above.

*Cites: commandment 1, D115.*

Interventions are mapped to model parameters from mechanistic evidence
only, and sealed before any lifespan result is seen. The train/test split,
its strata and the class of every intervention are sealed together in one
record.

*Cites: D115 (T2-A2-4, T2-A10-5).*

The model is scored against a sealed simple baseline (Gompertz curve plus a
hazard ratio per class) on three strata: (a) new interventions from known
classes, (b) new classes, (c) combinations and transfer between sexes. It
must be no worse on (a) within a sealed margin, and must win on (b) or (c)
to claim it captures mechanism. Each stratum has its own sequential test;
"undecided" is published as such. The model's count of free parameters is
published with each score. A mapping changed after a result is seen makes
every later prediction using it exploratory until a new held-out item
resolves.

*Cites: D115 (T2-A28-6, T2-A50-1, T2-A50-3, T2-A10-4).*

Measure, simulation and causal map are built in parallel, but the
simulation ranks no treatment before the measure passes its six tests.

*Cites: commandment 9, D115.*

## Part 3: the president as a resource

The president's role is to calibrate and challenge his model ("twin"),
never to validate the measure. We ask him for measurements only: no dosing,
no personal medical advice; any treatment goes through his doctor.

*Cites: D18, D116.*

The first request is a noise baseline: two draws a week apart plus one
blind split duplicate, sent with a sealed expected test-retest interval.
It waits until he says go.

*Cites: D116, D122.*

His raw data is stored encrypted in the public repo; the key lives only in
a local, git-ignored `.env`. Agents may read raw values. The repo publishes
forecast hashes, aggregates (≥ 5 resolved items per measure), interval
widths as a ratio of his noise, and one count of open twin issues.
Per-item records are published only for measures he lists in `consent.md`.
Nothing about him is published before `consent.md` exists. Reveals and
per-item scores otherwise stay private.

His twin is scored against a persistence baseline ("next value = last
value", interval = his measured noise). Perturbations we forecast stay
non-medical: a training block he chooses, or a vaccination he and his
doctor already scheduled. Astra never proposes one.

*Cites: D116 (T3-A10-2, T3-A50-2, T3-A50-4), D122 (president's answers, 2026-10-20).*

## Part 4: mice and the 3× target

Mice come only when the president is confident they'll be useful, and
there are few shots. Cohort 1 is untreated (UM-HET3, both sexes) and tests
sealed forecasts: axis trajectories and remaining lifespan per mouse. Its
data trains only the next version of the measure.

*Cites: commandment 2, D117.*

Every mouse request is a file that CI checks: a sealed prediction hash, a
refutation criterion, a reproducible power calculation, a dated search
showing the experiment isn't already done, and no open S1 or S2 incident
on the model.

*Cites: D117 (T4-A10-1, T4-A50-1, T4-A2-4).*

3× (or 2×) is claimed only after full survival, on median and maximum
lifespan, against controls within the normal range, and replicated at a
second site whose prediction was sealed before the first site's result.

*Cites: D117 (T4-A50-2).*

Mouse evidence is rated with SYRCLE risk of bias, capped by flags for
short-lived controls and lack of replication.

*Cites: D120 (T7-A2-1).*

## Part 5: data and blindness

A forecast is blind if it was sealed before any public trace of the
quantity it forecasts. A forecast sealed after an interim result names that
interim curve, hashes its extrapolated baseline at sealing, and is labelled
"blind (post-interim)".

*Cites: D118 (T5-A28-6).*

Before every seal, Literature runs a dated search for public traces,
meeting abstracts included. The seal is refused if that search is more
than 7 days old.

*Cites: D118 (T5-A2-4, T5-A10-4).*

Data is fetched by manifests with licences recorded; nothing over 10 MB is
committed outside the allow-list. A lifespan study that doesn't report a
cause of death records it as "unknown", never "no change". Verification
approves every resolution criterion and every licence before use. Each
survival curve used for scoring is digitised twice, independently, and the
build checks agreement.

*Cites: D118 (T5-A50-2, T5-A50-3), D121 (O9, O15), D120 (T7-A2-2).*

## Part 6: organisation and missions

The president owns Astra. A1 is his vice-president and interface, and the
customer of the Research, Literature and Platform missions. Managers
orchestrate and never do the work themselves; workers do it. Each mission
has one owner and one customer.

*Cites: PR1, PR2, D119 (T6-A1-1).*

Verification is independent of A1 and reports to the president. Its share
of workers is ceil(0.15 × cap), never less than the president's quarterly
setting; a lower re-measured cap applies at once.

*Cites: D47, D64, D119, D120 (T7-A50-4).*

**Worker cap.** This computer is shared, so the whole fleet runs at most 9
workers at once, plus managers, with at most 2 local builds (lock file,
delivered with P2 in week 1).
Anything heavier runs in GitHub Actions. The Code Manager re-measures the
cap monthly.

| Phase | Research | Literature | Platform | Verification | Database |
|---|---|---|---|---|---|
| 1: until split sealed and L1 drafted | 2 | 2 | 2 | 2 | 1 |
| 2: after | 3 | 2 | 2 | 2 | 0 |

Coherence and Interface start no worker without asking A1.

*Cites: D119 (T6-A10-2, T6-A1-4).*

| Mission | Owner | Output |
|---|---|---|
| M0 Verification | A50 | Seals, audits, adversarial cases, independent recompute of scorecards |
| M1 Forecast ledger | A28 | Sealed forecasts on mouse lifespan studies |
| M2 Platform rails | A10 | Write paths, decision citations, drift job, launch gate, seal on workflows |
| R-Metric v0 | A28 | Mouse measure v0 on public data, with its scorecard |
| R-Sim v0 | A28 | Organ-level model calibrated on control mice, scored on the sealed test set |
| R-Twin | A28 | Noise baseline, then quarterly sealed forecasts on the president |
| R-Cohort 1 protocol | A28 | Cohort protocol, sent with measure v0's scorecard |
| L1 Lifespan table | A2 | Every published mouse lifespan intervention, ITP first |
| L2 Metric dossier | A2 | One file per input and function test of the measure |
| L3 Refutation file | A2 | One row per causal edge and parameter mapping |
| P1 Data manifests | A9 (manifests), A10 (fetch job) | Sources fetched by manifest |
| P2 Run manifests | A10 | Seeded re-runs; split check |
| P3 Issues and ledger jobs | A10 | `issues/` register blocking sealed outputs with open S1/S2 |
| P4 Twin data rails | A10 | Encrypted storage, hash publishing, aggregate scores |
| P5 `astra-sim` port | A10 | Rust port of sim v0 within the agreed tolerance |

The old missions M3–M9, the RMST ranking engine as a product, and the
cardiometabolic flagship are stopped.

*Cites: D119 (T6-A28-1, T6-A2-1, T6-A10-1, T6-A1-2), D120 (T7-A28-3, T7-A10-3), D122.*

## Part 7: decision rights

Each mission protocol is sealed before any data is read, signed by its
owner and by Verification, and names exactly one primary outcome. A
departure without a sealed amendment is exploratory.

*Cites: D2, D61, D110, D120 (T7-A50-1).*

Acceptance criteria change only by numbered decision. A commit citing a
decision passes CI only if that decision exists on `main` before the change.
Two members may appeal a chair ruling to the president; the ruling stands
after 7 days without answer. Only the president can overrule a Verification
finding, in writing.

*Cites: D47, D48, D49, D69, D120 (T7-A28-1).*

**Rules that are conventions, not proof.** All agents push with one shared
token (D68 refused). The rules below are conventions, not proof, with no
end date. A determined or tricked agent can break them; detection is after
the fact.

D47, D52, D69, D80 and D109 are built but enforced only from the M2 merge,
after the president's re-seal; until then they are conventions too, and
Verification's weekly `git log` diff is the only check.

| Rule | What it claims | Why it's only a convention | What still catches a breach |
|---|---|---|---|
| D2 co-signed protocols | Producer + Verification both sealed it | Signatures are agent names in a file | Seal hash in git history; D46/D61 review record |
| D46 ratification | Each member approved the charter | PR reviews all come from one account | Objections quoted verbatim; A50's diff against the log |
| D47 Verification write paths | Only Verification writes `verification/` | `Agent:` trailers are self-declared | CI check (D47) on trailers + `docs/agents.yaml`; A50's weekly `git log` diff |
| D61 dual sign-off | Two units approved | Same as D2 | Same as D2 |
| D66 planted-error key | Verification can't read the answer key | Any agent can read workflow outputs | Commit-reveal hash; label "planted (key not isolated)" until the keys move into the `scoring` environment (Audits P2) |
| D124 model id | Records which model did the work | Self-reported trailer | Presence check only; useful for drift, not independence |
| President's rule: no settings / no `--admin` | Nobody edits protection or rulesets | The token allows it | D52 daily drift job → alert to the president and Verification; D80 seal on workflows and scripts |

The route back to real identities is the > 20 writers trigger (D124).

*Cites: D66, D68, D78, D108, D120 (Audits P2), D121 (O14), D124, PR7.*

## Part 8: verification and procedures

Sealing follows `verification/procedures/sealing.md` P1–P5: only
Verification seals, never an author; "no data read" is the owner's
statement plus the fetch-job log and the dated trace search, and a false
statement is an S1 incident; every refusal is recorded.

*Cites: D120.*

Audits follow `verification/procedures/audits.md` P1–P8: seeded sampling;
seed and planted-error keys in the `main`-only `scoring` environment; an
error is any field mismatch beyond declared rounding, with effect-changing
errors counted separately; the D51 threshold applies per finding over a
rolling 3 months, per mission; small pools keep the same rates with exact
binomial bounds; nobody audits their own work.

*Cites: D51, D63, D120.*

Planted errors now also test the measure and the simulation: bugs planted
in model code, labels swapped in training data. Accuracy is measured
against known truth. An outside reviewer samples metric scorecards each
quarter.

*Cites: D66, D71, D102, D120 (T7-A10-2, T7-A50-1).*

The seal covers lint settings, workflows and scripts; changing them needs a
re-seal by the president. A daily job compares live protection with the
committed settings and alerts the president and Verification. Retractions
are scanned weekly, including mouse papers.

*Cites: D45, D52, D53, D80, D120 (T7-A10-1).*

## Part 9: obstacles and dates

Goals are written as obstacles. Each has an owner, an exit and a date;
Verification, not the owner, decides when an exit is met. Targets never
move.

| # | Obstacle | Owner | Date |
|---|---|---|---|
| O1 | No measure that reads ageing reliably | A28 + A50 | month 6 |
| O2 | Causes not separated from consequences | A28 + A2 | month 9 (sim v0 sealed, month 4) |
| O3 | Cancer ceiling | A28, A2 | month 9 (milestone month 6) |
| O4 | Combinations don't add up | A28 | month 12 (milestone month 6) |
| O5 | 3 years per lifespan test | A28 | month 9 (first early-reading score, month 5) |
| O6 | Strain and sex dependence | A28 + A50 | ongoing (sex transfer stratum sealed, month 4; test 2 per sex, month 6) |
| O7 | Combined toxicity | A28 + A2 | before the first treatment request (adverse findings for every O4 combination, month 6) |
| O8 | No mice, lab or budget | A1 | month 6 |
| O9 | Data access | A9 + A50 | month 1 |
| O10 | Long-lived species unexplained | A2 + A28 | month 12 (milestone month 6) |
| O11 | Published lifespan effects are biased | A2 | month 3 |
| O12 | Causal links asserted without refutation tests | A2 | month 3 (L3 row for every sim v0 edge and mapping, approved by A50) |
| O13 | Results that can't be reproduced | A10 | month 1 (run manifests + one seeded re-run in CI); then every sealed prediction re-runs before sealing |
| O14 | Gates that rely on trust | A10 | end of week 1 after the re-seal |
| O15 | Data too big for git | A10 | month 1 |
| O16 | No public route to SLAM longitudinal data | A2 | month 1 |
| V1 | No blind test exists yet | A50 + A28 | month 3 |
| V2 | Hidden keys not isolated | A50 → president | month 1 |
| V3 | Scorecards checked only by re-running the same code | A50 | month 2: recompute plan + 2nd-implementation spec; recompute delivered with metric v0's scorecard |
| V4 | 13 procedure rules undecided | A50 → chair | done (D120) |

Every obstacle needs an exit dated by month 6 or a dated milestone before
then; otherwise it counts as unowned. Only V3 has neither yet; Verification
dates it at the next register update. The register job that raises the
alarm on missed dates is live by month 1 (A10).

The full exits and month-6 milestones are in
`arcanum/session-1/topics/T8.md`.

*Cites: commandment 4, D117, D121 (T8-A50-1, T8-A1-2, T8-A28-4, T8-A10-1, T8-A10-4); owner datings in the D46 reviews of A28 and A2.*

## Part 10: refusals, incidents and launch

Astra never gives dosing or personal medical advice; never uses unpinned
external data; never scores its own forecasts; never does dual-use work. A
negative control never counts as progress.

*Cites: D124 (re-issues D18's refusals), D116, D120 (T7-A28-2).*

Incidents are S1 (fixed in 72 h), S2 (2 weeks) or S3 (next release).
Corrections are add-only. No sealed output ships while an S1 or S2 on it is
open.

*Cites: D54, D119 (P3).*

Until Verification clears it, CI blocks any page that publishes a
biological age or forecast of a named individual, or claims a lifespan
effect we predicted. Every page carries a "research prototype, not medical
advice" banner.

*Cites: D109, D120 (T7-A10-2).*

## Part 11: meetings and amendments

Meetings follow the Arcanum rules: the debate happens in files, chat
messages are at most 2 lines, each turn has a steelman before each
objection and at least two dissents, objections are recorded, and only the
clerk commits. Any member may call out flooding. Verification is never red
team on its own findings.

*Cites: D55, PR5, PR6, `arcanum/session-1/README.md`.*

This charter changes only by a numbered decision voted in an Arcanum
session; Verification diffs each amendment's citations before the
president merges it. The commandments outrank it; a decision that
contradicts one is void.

*Cites: D125, commandments.*

## Annex: decisions proposed with this charter

Voted by the members in their D46 reviews of `1e69101`: D124 accepted by
all four; D125 accepted by all four with A50's reservation, now part of
its text. Confirmed by the president at merge.

- **D124 (re-issue).** Re-issues from the old log: D18's refusals (no
  dosing or personal medical advice, no unpinned external data, no scoring
  of our own forecasts, no dual-use work); D43's trigger (separate agent
  identities are re-proposed once more than 20 agents write); D77's
  `Model:` trailer (presence check only).
- **D125 (charter amendments).** The charter changes only by a numbered
  decision voted in an Arcanum session. Verification diffs the amendment's
  citations before the president merges it; once D69 is live, CI also
  refuses a charter change that cites no decision on `main`.
