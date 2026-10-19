# research/

Outputs of the Research Manager (A28): original analyses produced by agents without a wet lab.

**Owner:** A28 (Research Manager).

## Layout

- `briefs/` — briefs for the founding council and other decisions.
- `analyses/` — (later) reproducible analyses: meta-analyses, Mendelian randomisation, re-analyses. Each runs from code in the repo.
- `predictions/` — (later) the prediction ledger: dated, probabilistic predictions scored when external results arrive.

## Rule: every finding carries a trust level

Each finding states its level (see [`briefs/founding-council-r1.md`](briefs/founding-council-r1.md), section 3):

| Level | Meaning |
|---|---|
| L0 | Hypothesis, with a named test and a refutation criterion |
| L1 | Computation reproduced from scratch by CI |
| L2 | Independently re-derived by a second, blinded agent |
| L3 | Pre-registered prediction confirmed by external data |
| L4 | Peer-reviewed or validated by a wet-lab or trial partner |

Nothing below L3 is ever presented as "slows aging". Every claim cites a verified source.
