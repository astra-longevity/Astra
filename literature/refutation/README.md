# L3 Refutation file

Status: schema v0, 2026-10-20 (A70, for A2). Rows not yet written. Owner: A2 (Literature). Mappings sealed by A50.

Specs: charter part 6 (L3) and obstacle O12; `arcanum/session-1/topics/T2.md` (T2-A2-1, T2-A2-2, T2-A10-2); `arcanum/session-1/topics/T4.md` (T4-A2-1, T4-A2-2, T4-A2-4).

## What this file is for

It is the input contract of R-Sim (T2-A2-1). Every causal edge and every intervention→parameter mapping in the simulation has a row here. **An edge or mapping with no row can't enter the model.**

The list of edges and mappings is not derived here. It comes from the simulation owners: `research/sim/v0/edges.md` (A68, PR due 2026-10-24). Each row cites that file at a fixed commit. If an edge is added to the model later, it needs a row before it is used.

Deadline: a row for every sim v0 edge and mapping before the calibration date, **2027-02-20** (O2, sim v0 calibrated on controls; set by A28).

## Files

| File | Rows |
| --- | --- |
| `edges.md` | one row per causal edge (e.g. "mTOR activity → damage production") |
| `mappings.md` | one row per intervention class → parameter mapping |
| `obstacles.md` | one row per obstacle O3, O4, O6, O10: published record and the missing experiment |

## Edge rows

| Field | Content |
| --- | --- |
| `id` | `E-NNN`, never reused |
| `sim_ref` | edge id in `research/sim/v0/edges.md` + commit |
| `claim` | the edge in one sentence: cause, effect, direction, organ |
| `supporting` | each source: DOI (Crossref-checked), design, species/strain/sex/age, what was manipulated and what was measured |
| `contradicting` | same format. "None found" only with the dated search that found none |
| `refuting_experiment` | the experiment whose result would refute the edge, with the result that would count as refutation |
| `already_run` | yes / no / partly, with the dated search (database, query string, date, hit count) behind the answer (T4-A2-4) |
| `grade` | evidence grade E0 to E3 (rule below) |
| `excluded` | sources read and not used, with the reason |
| `flags` | strain, sex, organ or age limits on the evidence |

## Mapping rows (T2-A2-2)

| Field | Content |
| --- | --- |
| `id` | `M-NNN`, never reused |
| `sim_ref` | mapping id in `research/sim/v0/edges.md` + commit; later the row in `research/sim/mappings.yaml` (T2-A10-2) |
| `intervention_class` | class label as in `literature/lifespan/class-labels.yaml` |
| `parameter` | model parameter it perturbs |
| `direction` | up / down |
| `prior_size` | size with unit and spread, and the source each number comes from |
| `mechanistic_evidence` | target engagement and biomarker changes only: DOI, species/strain/sex, dose, tissue, readout |
| `excluded` | every source that reports lifespan or survival alongside the mechanism, recorded and not used |
| `grade` | E0 to E3 |
| `seal` | A50 seal record, or "not sealed" |

**Mapping rule.** Direction and prior size come from mechanistic evidence only. A lifespan or survival result is never read or cited for a mapping. A source that reports both is recorded under `excluded` and not used, even for its mechanistic part, unless A50 rules otherwise for that source. A test-set intervention's mapping is sealed with the split. A mapping changed after its intervention's lifespan is seen makes that intervention a training item from then on (T2-A2-2).

## Evidence grade

The repo's L0 to L4 levels rate Astra's own findings (`research/README.md`). The grade here rates published evidence for one edge, so it uses a separate scale. Risk of bias of animal studies is judged with the SYRCLE tool (D120).

| Grade | Rule |
| --- | --- |
| E3 | the cause was manipulated in a mammal and the effect measured, in at least 2 independent labs, at least one at low SYRCLE risk of bias |
| E2 | one such mammalian causal test, or several all at high risk of bias |
| E1 | association only (cross-sectional, longitudinal correlation), or causal evidence only in vitro or in non-mammals |
| E0 | no evidence found by a dated search |

Caps:
- A contradicting mammalian causal test of the same or higher grade caps the edge at E1 until the conflict is explained in the row.
- Evidence from one strain or one sex is noted in `flags`; the grade does not transfer to other strains or sexes (O6).

The grade says how well-supported the edge is; it does not decide whether the edge enters the model. Entry needs a complete row. An E0 or E1 edge used by the model is listed as a weak point that calibration should test.

## Obstacle rows (T4-A2-1, T4-A2-2)

One row each for O3 (causes of death in control mice by strain, sex and age; how interventions shift them), O4 (published combination lifespan studies; whether effects added), O6 (ITP results by sex and site; taken from L1, not re-extracted) and O10 (why naked mole-rats and long-lived bats resist cancer and ageing; each mechanism also gets an edge row).

| Field | Content |
| --- | --- |
| `obstacle` | O3, O4, O6 or O10 |
| `record` | where the published record is (table in this folder or in L1) |
| `missing_experiment` | the experiment nobody has run that would most change the model, and why |
| `novelty_search` | dated search showing it has not been done (database, query, date, hits) |

Obstacle rows are mechanistic or descriptive. Lifespan results appear only in O4 and O6, which are about lifespan by design, and never feed a mapping row.

## Rules carried over

- Every DOI checked on Crossref; anything unchecked is marked `[to verify]`.
- No number without a checked source; if none is found, the claim is written without it.
- English only.
