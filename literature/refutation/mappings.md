# L3 mapping rows (intervention → R-Sim v0 parameter)

Schema and rules: `README.md` (mapping fields, A50 conditions 1–4, rules 5–9,
E0–E3 grading, independent-labs rule). `sim_ref` cites
`research/sim/v0/design/edges.md` §3 at commit `4b264b1` (mappings M1–M5).

Writer: **A78**, lifespan-naive by construction. No R-Sim calibration output and
no lifespan/survival/mortality result for any of these interventions was read
before sealing, except the qualitative statement disclosed in M-001/M-002
`no_prior_read`. Rows are unsealed until A50 seals them.

First PR (this file): M-001..M-005. Mapped: M-003, M-004. `no_mapping` for size:
M-001, M-002 (no admissible mouse source read yet, direction kept), M-005 (marker
level only, parameter sign undefined). Deadline for all mappings: 2027-02-20.

---

## M-001 — rapamycin → H6 nutrient sensing (production)

- **sim_ref:** M1, `research/sim/v0/design/edges.md` §3 @ `4b264b1`; later `research/sim/mappings.yaml`
- **intervention:** rapamycin; target mTOR (human `CHEMBL2842`; mouse mTORC1 `CHEMBL5465384`)
- **parameter:** `hallmark.H6_nutrient_sensing` (production term), rides on edge E-H6
- **direction:** down (mTORC1 inhibition lowers nutrient-sensing drive on damage production)
- **prior_size:** `no_mapping` — no admissible **mouse** mechanistic source (pS6 / nutrient-sensing readout) has been screened and read yet. Direction is from the mechanism of mTORC1 inhibition, not from a read effect size.
- **mechanistic_evidence:** none admitted yet. The draft (edges.md M1) cited a **yeast** source (doi:10.1074/jbc.273.7.3963), not admissible as a mouse mechanistic prior. Mouse pS6 / autophagy-flux sourcing continues for a follow-up version.
- **excluded:** none read for this row.
- **grade:** E0 (no admissible mechanistic source read).
- **seal:** not sealed.
- **version:** v1.
- **status:** `no_mapping` (size); direction kept.
- **writer:** A78.
- **no_prior_read:** A78, 2026-10-21. I have read no R-Sim calibration output and no numeric lifespan, survival or mortality result for rapamycin. While searching for a mouse pS6 source I saw a web search snippet stating, in words only, **"rapamycin extends lifespan in mice"** (no number, no survival curve, no ITP data). Per README rule 6 this qualitative statement does not bar me from writing this row.

## M-002 — rapamycin → H5 macroautophagy (clearance)

- **sim_ref:** M2, `research/sim/v0/design/edges.md` §3 @ `4b264b1`
- **intervention:** rapamycin; target mTOR (human `CHEMBL2842`; mouse mTORC1 `CHEMBL5465384`)
- **parameter:** `hallmark.H5_autophagy` (clearance term), rides on edge E-H5
- **direction:** up (mTORC1 inhibition raises autophagic clearance)
- **prior_size:** `no_mapping` — no admissible **mouse** autophagy-flux mechanistic source read yet.
- **mechanistic_evidence:** none admitted yet (draft cited the same yeast source).
- **excluded:** none read for this row.
- **grade:** E0.
- **seal:** not sealed.
- **version:** v1.
- **status:** `no_mapping` (size); direction kept.
- **writer:** A78.
- **no_prior_read:** as M-001 (same verbatim rapamycin snippet disclosed).

## M-003 — acarbose → H6 nutrient sensing (production)

- **sim_ref:** M3, `research/sim/v0/design/edges.md` §3 @ `4b264b1`
- **intervention:** acarbose; target intestinal α-glucosidase (maltase-glucoamylase MGAM / sucrase-isomaltase SI); ChEMBL target id `[to verify]`
- **parameter:** `hallmark.H6_nutrient_sensing` (production term), rides on edge E-H6
- **direction:** down (α-glucosidase inhibition lowers the post-load glucose excursion, the nutrient-sensing drive)
- **prior_size:** treated/control ratio **0.442 [0.402, 0.485]** (unitless multiplier on the nutrient-load readout). From the maltose oral tolerance test, normal ddY mice, 0.5 h post-load peak blood glucose: control 20.6 ± 0.4 mM/L (N=7) vs acarbose 50 mg/kg 9.1 ± 0.3 mM/L (N=7), mean ± SE. Interval by the delta method on the log ratio (README rule 8): SE(log r)=0.038, t(0.975, 6)=2.447.
  - **named assumption (rule 9):** the measured treated/control glucose-excursion ratio is used 1:1 as the H6 production multiplier. R-Sim applies the fixed ×0.5–×2 sensitivity range on the log effect.
- **mechanistic_evidence:** Miura 1998, doi:10.3177/jnsv.44.371 (Crossref ✓, *J Nutr Sci Vitaminol* 1998). ddY (normal) and KK-Ay mice, oral sugar tolerance tests + intestinal disaccharidase activity; acarbose 1/10/50 mg/kg oral; blood glucose and gut maltase/sucrase measured. Dose-dependent fall in post-load glucose (target engagement: gut maltase/sucrase inhibited). Values printed in text and tables with n per group.
- **excluded:** PMID 36342748 — `X-LS` (lifespan terms in abstract; deleted unread).
- **grade:** **E1** — one causal mammalian test; SYRCLE low-risk domains < 5 (no reported randomisation, allocation concealment or blinding; single lab). Independent-labs: single source, not applicable.
- **flags:** strain ddY (non-C57BL/6); readout is an acute post-load glucose excursion, a proxy for chronic nutrient-sensing drive; sex male.
- **seal:** not sealed.
- **version:** v1.
- **status:** `mapped`.
- **writer:** A78.
- **no_prior_read:** A78, 2026-10-21. No R-Sim calibration output and no lifespan/survival/mortality result for acarbose was read before sealing. (Miura 1998 full text screened: no lifespan content.) Miyamura 2010 (doi:10.1254/jphs.10014fp) was screened clean but **not used**: its food-restriction + intermittent-hypoxia design is a poor mechanistic prior (A2 direction), not a lifespan exclusion.

## M-004 — canagliflozin → H6 nutrient sensing (production)

- **sim_ref:** M4, `research/sim/v0/design/edges.md` §3 @ `4b264b1`
- **intervention:** canagliflozin (`CHEMBL2048484`); target SGLT2 (human `CHEMBL3884`; mouse `CHEMBL1075302`)
- **parameter:** `hallmark.H6_nutrient_sensing` (production term), rides on edge E-H6
- **direction:** down (SGLT2 inhibition lowers glycaemia, the nutrient-sensing drive)
- **prior_size:** treated/control ratio **0.528 [0.372, 0.748]** (unitless multiplier). Fed blood glucose, diet-induced-obese male C57BL/6J mice, canagliflozin 30 mg/kg for 4 weeks: control 199 ± 16 mg/dl (n=8) vs treated 105 ± 13 mg/dl (n=8), mean ± SEM (Table 3). Interval by the delta method on the log ratio (rule 8): SE(log r)=0.148, t(0.975, 7)=2.365.
  - **named assumption (rule 9):** the measured fed-glucose ratio is used 1:1 as the H6 production multiplier; R-Sim applies the ×0.5–×2 sensitivity range.
- **mechanistic_evidence:** Liang 2012, doi:10.1371/journal.pone.0030555 (Crossref ✓, *PLoS ONE* 2012). DIO male C57BL/6J; canagliflozin 30 mg/kg × 4 wk; n=8; fed/fasted glucose, urinary glucose excretion ×10.9 (target engagement: raised renal glucose excretion), body weight. Values printed in Table 3 with n.
- **excluded:** none read for this row. Wang 2026 (doi:10.1016/j.jphyss.2026.100079) screened clean but figure-only; not needed since Liang prints numbers.
- **grade:** **E1** — one causal mammalian test; SYRCLE low-risk < 5 (no reported randomisation/blinding; single lab).
- **flags:** C57BL/6J but diet-induced-obese (disease model, not healthy ageing); sex male; acute 4-week readout.
- **seal:** not sealed.
- **version:** v1.
- **status:** `mapped`.
- **writer:** A78.
- **no_prior_read:** A78, 2026-10-21. No R-Sim calibration output and no lifespan/survival/mortality result for canagliflozin was read before sealing. (Liang 2012 full text screened: no lifespan content.)

## M-005 — dasatinib + quercetin (senolytic) → H8 senescence clearance

- **sim_ref:** M5, `research/sim/v0/design/edges.md` §3 @ `4b264b1`
- **intervention:** dasatinib + quercetin (D+Q) regimen; dasatinib target SRC/BCR-ABL family, quercetin multi-target; recorded as the regimen, ChEMBL per-compound ids `[to verify]`
- **parameter:** `hallmark.H8_senescence` (clearance term), rides on edge E-H8; `m_H8` > 1 means more clearance of senescent damage
- **direction:** marker level only: **senescence markers (p16, p21, SA-β-gal) fall under D+Q** (from the figure legend / text, not from pixel values). The sign on the H8 clearance parameter is **not stated**: marker→parameter transform undefined; not usable as H8 clearance input (A50 ruling, 2026-10-21).
- **prior_size:** `no_mapping`. Reasons:
  1. **Marker → parameter transform undefined (A50 ruling):** a fall in senescence-marker burden can come from more clearance or less production of senescent cells, and rule 9's 1:1 does not apply across that change of quantity. Even the clearance sign is unknown. The modelling rule (process + sign) is owned by A28/A76 and must be committed before calibration; M-005 gets a size only in a later version.
  2. **Figure digitising (rule 7):** the readouts are figure-only. In this version the writer could not match ticks to printed labels (no viewer), so every panel is `no_mapping` on the writer's side (`m5-digit/`: `digitise.py`, `coords.json`, image sha256). A50 has since granted a tick-label reading route (rules commit pending); a writer reading under it goes into a later version. A66's independent reading: verification #211 @ `fbca6c4`.
- **mechanistic_evidence:** NPJ Regen Med 2026, doi:10.1038/s41536-026-00490-x (Crossref ✓, *npj Regen Med* 2026). C57BL/6J mice from 12 months; D+Q 5 + 50 mg/kg every 2 weeks for 8 months; kidney at 20 months; p16, p21 and SA-β-gal fall (Fig 1b/d/f). Randomised (random-number tables) and blinded (allocation and outcome) — SYRCLE high. **Only the mechanistic marker panels (Fig 1 b/d/f) were opened; no survival/lifespan panel was read.** The title mentions "rejuvenation"; the readouts used here are senescence markers only.
- **excluded:** Zhu 2015, doi:10.1111/acel.12344 (*Aging Cell* 2015) — `X-LS`, **excluded on abstract** (lifespan/survival terms; deleted unread). This was the draft's M5 source; the draft's "no lifespan" note on it was wrong.
- **grade:** direction supported at E2-equivalent (one high-SYRCLE causal mammalian test); **size ungraded** (`no_mapping`). Two further clean-abstract sources not opened for text: doi:10.1111/acel.70759 (heart), doi:10.14336/AD.2025.0976 (arteries) — `[to verify]`, available to strengthen direction if wanted.
- **flags:** C57BL/6J both sexes; kidney tissue; **not usable as H8 clearance input** until A28/A76 commit the marker→parameter modelling rule.
- **seal:** not sealed.
- **version:** v1.
- **status:** `no_mapping` (size and parameter sign); marker-level direction recorded.
- **writer:** A78.
- **no_prior_read:** A78, 2026-10-21. No R-Sim calibration output and no lifespan/survival/mortality result for D+Q was read before sealing (Zhu 2015 excluded on abstract, unread; NPJ 2026 only its mechanistic marker panels opened).

---

## Summary

| Row | Intervention | Parameter | Direction | Size | Grade | Status |
| --- | --- | --- | --- | --- | --- | --- |
| M-001 | rapamycin | H6 production | down | `no_mapping` | E0 | no_mapping |
| M-002 | rapamycin | H5 clearance | up | `no_mapping` | E0 | no_mapping |
| M-003 | acarbose | H6 production | down | 0.442 [0.402, 0.485] | E1 | mapped |
| M-004 | canagliflozin | H6 production | down | 0.528 [0.372, 0.748] | E1 | mapped |
| M-005 | D+Q (senolytic) | H8 clearance | marker falls (parameter sign undefined) | `no_mapping` | marker E2 / size — | no_mapping |

- **Mapped:** M-003, M-004.
- **`no_mapping` (size):** 3 — M-001, M-002 (no admissible mouse source read yet; direction kept), M-005 (marker level only; parameter sign and size undefined until A28/A76 modelling rule).
- **Exclusions:** Zhu 2015 (`X-LS`, M-005); PMID 36342748 (`X-LS`, M-003).
- **Open `[to verify]`:** acarbose ChEMBL target id (M-003); D+Q per-compound ChEMBL ids (M-005); the two extra senolytic sources (M-005).
