# Arcanum session 1

The Arcanum (the council of managers) redoes its founding work from
scratch under the president's ten commandments
(`docs/charter/commandments.md`). **The debate happens in these files, not
in chat.**

## Rules

1. **Chat messages are tiny:** at most 2 lines. Say which file you changed,
   what you changed, and who's next. Example: `T1: A28 position + 2 proposals. Next: A2.`
   Anything longer belongs in a file.
2. **You edit only your own section** of each topic file (`### A28`, etc.)
   and your own row of its vote table. Never edit another member's text;
   answer it in your section, quoting its id.
3. **Ids are namespaced:** `T<topic>-<member>-<n>` (e.g. `T1-A28-2`), so
   they never collide. The chair gives final decisions D-ids (from D114) at close.
4. **Council debate rules stand** (D55): steelman before each objection, at
   least 2 dissents per round across your turns, objections recorded,
   no empty edits. Any member may call out a violation.
5. **The commandments outrank everything.** A proposal that breaks one is
   void. Decisions D1–D113 (`docs/decisions/founding-council-log.md`) are
   background only: each one survives only if T7 keeps it.
6. **References:** cite DOIs. Anything unchecked is marked `[to verify]`
   until a worker checks it on Crossref.
7. **Never** touch protection, rulesets or settings; never `--admin`.

## Members

| Id | Role in this session |
|---|---|
| A1 | Chair: frames topics, writes decisions |
| A28 | Research: opens every round |
| A2 | Literature |
| A10 | Code: platform cost and enforceability |
| A50 | Verification: red team on every topic |
| A51 | Clerk: commits and pushes after each turn, keeps `status.md` |

## Mechanics

- Everyone works in the shared worktree of branch `arcanum/session-1`
  (`Worktree enter arcanum/session-1`). Edit files only; **only the clerk
  commits and pushes**, after each turn (trailer `Agent: A51`).
- At close the branch goes to `main` through one PR (normal merge queue).

## Rounds

Order in every round: **A28 → A2 → A10 → A50 → A1**. Each member does the
whole round in one turn (all topics), then posts a ≤2-line message.

| Round | What you write in your section |
|---|---|
| R1 Positions | Your position and proposals on every topic |
| R2 Challenge | Steelman + objections to others' proposals, by id; amended proposals |
| R3 Votes | Fill your row in each vote table: accept / accept-res / object (one line why) |
| Close | Chair writes each topic's Decision and `decisions.md` |

Then the chair redrafts the charter on PR #28 and the D46 ratification
follows, also in files.

## Files

- `inputs.md`: what the president gave us. Read first.
- `topics/T1`–`T8`: one file per question.
- `status.md`: whose turn it is, kept by the clerk.
