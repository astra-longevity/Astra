# research/sim/v0: simulation v0 scaffold

Organ-level stochastic model of damage and repair (charter part 2, D115).
**Uncalibrated, trust level L0.** It ranks no treatment.

| File | What |
|---|---|
| `spec.md` | model: axes, dynamics, death rule, 24 free parameters, intervention mapping rules, known issues |
| `baseline.md` | sealed null: Gompertz on controls + one HR per class, strata (a)/(b)/(c), per-stratum SPRT |
| `py/model.py` | seeded simulator (standard library only) |
| `py/baseline.py` | null fit (controls only, train split only), log score, SPRT |
| `py/run.py` | CLI: simulate N mice, write `survival.csv` + `manifest.json` |
| `py/test_v0.py` | unit tests |
| `py/studies/` | identifiability studies: `identifiability.py`, `recovery.py`, `readouts.py`, `longitudinal.py` (see `identifiability.md`) |
| `results/` | study outputs (JSON) |
| `identifiability.md` | which parameters survival, readouts and repeated measures can recover |
| `actions-job.md` | spec of the joint survival + readout job on GitHub Actions (A10 runs it) |

Mappings live in `../mappings.yaml` (empty until the seal).

## Run

```
cd research/sim/v0/py
python3 -m unittest -v
python3 run.py --n 500 --seed 1 --out /tmp/sim-v0
```

Python ≥ 3.10, no third-party packages.
