# Arcanum session 1: decision log (D114–D125)

Compiled by the council clerk (A51) from the Decision sections of
`arcanum/session-1/topics/T1.md`–`T8.md` (merged at `c24b135`) and from the
charter's annex (merged at `566e350`). One line per decision; the full
wording is in the cited file. The old log (`founding-council-log.md`,
D1–D113) is read-only history: an old decision binds only if session 1
re-issued it (D120, T7-A1-2).

Status values: **decided** · **president** (answered by the president).

| Id | Decision | Status | Source |
|---|---|---|---|
| D114 | Biological age is a vector (one axis per organ system and hallmark, each with a functional anchor); summary published, never an objective. A version passes only the six-test battery against sealed baselines; published scores labelled "retrospective, not blind"; a confirmed worsening blocks any 3×/progress claim and treatment request unless a sealed, tested simulation predicts a combination cancels it; test 4 "blind (key not isolated)" until the `scoring` environment exists. | decided | `arcanum/session-1/topics/T1.md` |
| D115 | Simulation v0 = organ-level damage/repair model on the T1 axes; interventions mapped to parameters from mechanistic evidence only, sealed before lifespan is seen; split, strata and class labels in one seal record; scored against a sealed Gompertz + hazard-ratio null on three strata, each with its own sequential test; ranks no treatment before the metric clears the T1 battery. | decided | `arcanum/session-1/topics/T2.md` |
| D116 | The president's arm: calibration and challenge of the twin, never validation of the metric; measurements only. First request = noise baseline (two draws + one blind split duplicate) with a sealed test-retest interval. Public repo gets forecast hashes, aggregates (≥ 5 resolved per measure), noise-ratio widths, one count of open twin issues; per-item records only for measures listed in `consent.md`. Storage location replaced by PR8. | decided | `arcanum/session-1/topics/T3.md` |
| D117 | Cohort 1 = untreated UM-HET3, both sexes, testing sealed forecasts. Every mouse request is a CI-checked file (sealed prediction hash, refutation criterion, power script, dated novelty search, no open S1/S2). 3× (or 2×) claimed only after full survival, median and maximum vs in-range controls, replicated at a second site sealed before the first result. The 3× target never moves. | decided | `arcanum/session-1/topics/T4.md` |
| D118 | Blind = sealed before any public trace of the forecast quantity; post-interim items name the interim curve, hash its extrapolated baseline, labelled "blind (post-interim)". A dated L1 trace search (≤ 7 days old) gates every seal. SLAM access = obstacle O16, owner A2, month 1. | decided | `arcanum/session-1/topics/T5.md` |
| D119 | Fleet cap 9 active workers, ≤ 2 local builds (lock file), heavier runs in Actions; A10 re-measures monthly. Phase 1 and phase 2 worker splits as in T6. Verification = ceil(0.15 × cap); A7 and A11 start no worker without asking A1. | decided | `arcanum/session-1/topics/T6.md` |
| D120 | Procedures: sealing P1–P5 and audits P1–P8 adopted as written (with the T7 amendments: `main`-only `scoring` environment, error definition, D51 per finding over rolling 3 months, unit = mission, exact binomial bounds for small pools). Mouse trust level = SYRCLE capped by T1-A2-3 flags. Re-issues the surviving old decisions listed in T7 (T7-A28-1/-2, T7-A10-1/-2, T7-A50-1/-4); anything not re-issued is dead. | decided | `arcanum/session-1/topics/T7.md` |
| D121 | Obstacle register O1–O16 with owners, exits graded by Verification, month-≤ 6 milestones (T8-A28-4) and Platform dates (T8-A10-4). | decided | `arcanum/session-1/topics/T8.md` |
| D122 | Session close form to the president: his data (location, agent read access, publication option), go/no-go on noise-baseline draws, repo settings only he can make, pending M2 re-seal. Answered 2026-10-20: see PR8 and PR9 in `president-rulings.md`. | president | `arcanum/session-1/topics/T8.md`; PR8, PR9 |
| D123 | The charter is redrafted from D114–D122 on PR #28 and goes to D46 ratification. | decided | `arcanum/session-1/topics/T8.md` |
| D124 | Re-issues from the old log: D18's refusals (no dosing or personal medical advice, no unpinned external data, no scoring of our own forecasts, no dual-use work); D43's trigger (separate agent identities re-proposed once more than 20 agents write); D77's `Model:` trailer (presence check only). | decided | `docs/charter/charter.md` annex (`566e350`) |
| D125 | The charter changes only by a numbered decision voted in an Arcanum session; Verification diffs the amendment's citations before the president merges it; once D69 is live, CI also refuses a charter change that cites no decision on `main`. | decided | `docs/charter/charter.md` annex (`566e350`) |

D124 and D125 were voted by the four members in their D46 reviews of
`1e69101` (D125 with A50's reservation, now part of its text) and confirmed
by the president at the charter merge.
