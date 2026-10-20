# President's rulings

Rulings the president gave directly in his chat with A1 (group G1), outside
any council decision. They bind every agent. The ten commandments are in
`docs/charter/commandments.md` and are not repeated here. Add-only: a later
ruling that changes one cites the row it replaces.

| Id | Date | Ruling |
|---|---|---|
| PR1 | 2026-10-19 | The president owns Astra. A1 is his vice-president and interface; all agents follow A1, except that only the president can overrule a Verification finding, in writing. Hierarchy: managers above workers. |
| PR2 | 2026-10-19 | Managers never do the work themselves: they orchestrate, delegate and aggregate. |
| PR3 | 2026-10-19 | The repo is the unique source of truth. Annex tools store their data in the repo as versioned text and can be rebuilt from it. |
| PR4 | 2026-10-19 | Everything is in English: chat, inter-agent messages, code, docs, data. |
| PR5 | 2026-10-19 | Meetings: each turn challenges at least two points, a change of mind states its reason, decisions are numbered and voted, objections are recorded. |
| PR6 | 2026-10-19 | In any meeting, an agent may call out another for flooding or ignoring others' context; the one called out fixes it next turn. |
| PR7 | 2026-10-19 | No agent edits branch protection, rulesets or repo settings, or uses `--admin`. Sole exception: A1, on the president's explicit request. A breach is an incident. |
| PR8 | 2026-10-20 | The president's health data lives in the public repo, symmetrically encrypted; the key stays only in a local, git-ignored `.env`. Agents may read raw values. Publication follows the default (per-measure records only for measures he lists). Noise-baseline draws: later. Replaces D116's `astra-private`. |
| PR9 | 2026-10-20 | `PROTECTION_READ_TOKEN` holds the president's full token, at his choice. A1 created the `main`-only `scoring` environment at his request. |
| PR10 | 2026-10-20 | Agents name themselves `<Function> (<Name>)`, workers included. |
| PR11 | 2026-10-20 | `PROTECTION_READ_TOKEN` stays a repository secret; it is not moved into a `main`-only environment. The charter (PR #28) is merged. |
| PR12 | 2026-10-20 | Astra may use the Schultz 2020 mouse frailty dataset (SinclairLab/frailty) despite its missing licence; this overrules the LA-004 refusal. The president contacts the authors himself; no agent posts a licence request. |
