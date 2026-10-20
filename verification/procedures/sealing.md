# Sealing a protocol

How Verification seals a D2 protocol before anyone reads its data. Every
rule below cites the founding-council decision it comes from
(`docs/decisions/founding-council-log.md`). Rules with no decision are listed
at the end under "Proposed, needs a decision" and do not apply yet.

## Why sealing exists

A protocol fixes the analysis choices before the data is read, so the result
cannot be shaped by what the data shows. The protocol is co-signed by the
mission owner and the contributor before any data read (D2). Acceptance
criteria are fixed at sealing and change only by numbered decision, checked
by CI (D48, D69). Without a sealed protocol, work is exploratory (D18). Every
sealed protocol yields a published result, null or not (D18).

The sealed D2 protocol governs. D61 sign-off checks the result against it;
a departure without a sealed amendment is exploratory (D61).

## Who seals

Verification seals (D2, D3). Verification never red-teams its own findings
(D55). Only the president overrules a Verification finding, in writing (D47).

## Checklist

Verification seals only when every box holds. One failed box = no seal; the
protocol goes back to its owner with the failed item named.

1. **Co-signatures (D2).** The owner (producer of the headline number) and
   the contributor have both signed. For pooled numbers, the GRADE assessment
   is part of the same record with both signatures (D2).
2. **Exactly one primary outcome (D110).** Exactly one outcome has
   `primary: true`. Zero or two or more = no seal. This is also CI-checked
   (D110). Secondary outcomes are allowed, published, never counted toward
   G1 (D110).
3. **Negative-control role (D113).** If the protocol is an engine
   validation on a negative control (aspirin / ASPREE today), it carries
   `role: negative_control`. Such a protocol is excluded from G1, G5 and any
   ranking (D113).
4. **Novelty block present (D84).** The protocol contains the novelty
   search (PROSPERO, published reviews): search string and hits. Verification
   checks presence only, not search quality. Public label: "search
   unverified" (D84).
5. **Dependencies (D60).** Every derived object in the protocol has a
   `depends_on` field.
6. **Model id (D77).** The model id of the agent(s) writing the protocol is
   recorded. It is self-reported and signals drift, not independence (D77).
7. **Ledger items: base-rate class (D72).** A ledger prediction names a
   reference class from the fixed menu. Verification approves the class at
   sealing. No fitting class → the item is sealed for calibration only (no
   skill score); it counts toward G2, not G3 (D72).
8. **Ledger items: structured criteria (D1).** Each criterion has source,
   record id, field path, comparator, threshold, deadline and fallback; no
   free text. T1 criteria point to the registered CT.gov outcome measure,
   hashed at sealing. Verification approves every criterion (D1).
9. **Population match (D92).** Every effect the protocol will produce is
   declared `population_match: direct | extrapolated`.
10. **Estimator status (D6, D40, D106).** Every estimator the protocol uses
    is either validated against an independent printed reference (D6) or
    marked "not validated", in which case results are capped at L1 (D40).
    A first MR analysis is capped at L1 and cannot reach the ledger or a
    public claim before coloc passes (D106).
11. **Curves (D93, D99).** If the protocol uses digitised survival curves,
    it requires two independent digitisations in the D99 format; they stay
    drafts until the build checks agreement (D93, D99).
12. **No data read yet (D2).** The owner states that no outcome data for
    this protocol has been read. Verification records the statement.

## The seal

- The seal is the commit on `main` that adds the protocol with the
  Verification sign-off. Its hash is the seal id (see proposal P1).
- Until isolated timestamping exists, every public mention of the level
  reads **"L2-sealed (not isolated)"** (D105). The score reads only findings
  at L2-sealed or above (D3, D13, D18).
- Sealed criteria are add-only (D18).

## After the seal: amendments

- Acceptance criteria change only by numbered decision (D48), checked by CI
  (D69).
- After a retraction or correction of a source, Verification adds a linked
  amendment. The original stays; both versions are scored. The chair never
  amends (D62).
- Amendments are add-only: a new file that links the sealed one; the sealed
  file is never edited (D18, D62).

## Proposed, needs a decision

These rules are not in the decision log. They are written here so they can
be decided; until then they do not bind anyone.

- **P1. Seal record.** The seal id is the merge commit hash on `main`,
  written in a `seal:` block (hash, date, sealer agent id, model id) inside
  the protocol's amendment-free companion file `SEAL.md`.
- **P2. Who may seal.** Only an agent listed with `unit: verification` in
  `docs/agents.yaml` (the D47 write-path list) may sign a seal, and never one
  who wrote or co-signed the protocol.
- **P3. "No data read" evidence.** The owner's statement (item 12) is the
  only evidence; whether a stronger check (e.g. fetch logs) is required is
  open.
- **P4. Seal turnaround.** A target delay between request and seal decision
  (proposed: 5 working days), feeding the D3 "queue > 2 weeks" signal.
- **P5. Refusal record.** Every refused seal is recorded in
  `verification/` with the failed checklist item, so refusals are auditable.
