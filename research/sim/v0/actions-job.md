# R-Sim v0: GitHub Actions job spec, joint survival + readout fit

**Owner of the spec:** A28 (Research), written by A68. **Runs it:** A10 (Platform).
Spec only: this file adds no workflow. Synthetic data only, trust level L0.

## Why

At `n_sim` 2000 the survival Fisher matrix has a Monte-Carlo noise floor of
about 1 200 (identifiability.md §2, I3, I9). That floor hides everything the
readouts add (about 300 per direction at 30 mice per age). The floor shrinks
as 1/`n_sim`, so a joint survival + readout analysis needs `n_sim` of 20 000
to 50 000. That is too heavy for the shared machine (about 1–2 CPU-hours per
seed at 20 000) but easy to shard on Actions.

## What the job computes

1. **Joint Fisher.** Cross-seed (seeds 11, 12) derivatives of survival bins
   (`identifiability.py`), cross-sectional readouts (`readouts.py`) and
   repeated measures (`longitudinal.py`) for all 54 log-parameters, ±0.1.
   Output: recoverable directions and identified parameters for survival
   alone, readouts alone, and survival + readouts, at `n_sim` 20 000 and
   50 000. Pass criterion for the study, not the job: the survival noise floor
   falls below 150.
2. **Recovery fit (step 2, after 1 is read).** Nelder-Mead on the joint
   likelihood for the parameter set that step 1 identifies, 3 synthetic data
   seeds (101–103), truth shifted as in identifiability.md §3. Recovered =
   error < 0.1 log units in every seed.

## Inputs (all pinned)

| Input | Value |
|---|---|
| Code | `research/sim/v0/py/` (studies in `py/studies/`) at a pinned commit sha |
| Python | 3.13, standard library only, no network |
| Seeds | derivatives 11, 12; data 101, 102, 103 |
| `n_sim` | 20 000 and 50 000 |
| Readout setup | ages 12/18/24/30 months, `NOISE_REL` 0.5, as in the scripts at the pinned sha |

## Sharding

- One shard = (seed, parameter, ±) → one `simulate` call, saved as JSON
  (bin probabilities + readout moments). 2 seeds × (54 × 2 + 1) = 218 shards
  per `n_sim`.
- Matrix job, at most 20 runners at once; one shard ≈ 1 min at 20 000 per
  2 000-mouse-minute measured locally (scale linearly) [to measure on runner].
- A final job gathers the shards, builds the matrices and runs the analysis.
- Scripts need one small change to support this: a `--shard seed,param,sign`
  mode that writes one shard, and a `--gather dir` mode. A68 adds it in a
  separate PR before the first run.

## Outputs

- `research/runs/sim-v0-joint-<date>/manifest.yaml` (P2 run-manifest format,
  `scripts/runs.py`), listing code sha, inputs, seeds and output sha256.
- `result.json` (directions, identified parameters, noise floors) in the same
  folder. Shards are an Actions artifact only (not committed: size).
- Determinism: one shard picked by the manifest is re-run by `Runs` CI and
  must match byte-exact.

## Limits

- Trigger: `workflow_dispatch` only (manual, by A10); never on push.
- Timeout 6 h per job; a failed shard is re-run once, then the job fails.
- No real data. The job must not read `data/` or any lifespan table.
