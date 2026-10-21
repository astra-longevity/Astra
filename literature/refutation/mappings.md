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

---

# Version 2 (2026-10-21, writer A78)

v1 above is kept unchanged (condition 3). Each row below is **v2** and lists its diff from v1. Fixes come from A66's check of v1 (record #217: F1, F2, F3, Zhu 2015) and A50's rulings relayed by A2 (M-005 E1, M-005 v2 content). No row is sealed.

SYRCLE scoring is strict: a domain is "low risk" only if the paper reports it explicitly. Domains (README): 1 sequence generation, 2 baseline characteristics, 3 allocation concealment, 4 random housing, 5 caregiver blinding, 6 random outcome assessment, 7 assessor blinding, 8 incomplete outcome data, 9 selective reporting, 10 other bias.

Text seen by the writer and quoted under rule 6 (words only, no number, no curve, no ITP result):
- **Q1** (abstract of PMID 36342748, relayed by A66; not reopened): "Acarbose (ACA) and rapamycin (RAP) extend life span in mice".
- **Q2** (`research/sim/v0/design/edges.md` NA4 exposure line, read on `main` while checking NA4.1): "In old mice the combo improves physical function and extends remaining lifespan".

## M-001 v2 — rapamycin → H6 nutrient sensing (production)

- **grade:** **not graded**. The size is `no_mapping` because no admissible mouse source was found by two dated searches (PubMed, 2026-10-21): `rapamycin AND (pS6 OR S6K1 OR "S6 phosphorylation") AND mice AND tissue` (226 hits); `rapamycin AND (LC3-II OR autophagic flux) AND mice AND (liver OR muscle OR kidney)` (281 hits). Re-run with lifespan/survival/longevity/mortality terms excluded: 39 and 203 hits. The newest 25 of each were screened on title and abstract. Hits were irradiation-damage models (tissue confound, e.g. doi:10.3389/fphys.2019.00228, doi:10.1186/s13287-018-0963-5) or acute arginine/contraction studies, not target engagement.
- **SYRCLE:** not applicable (no study admitted).
- **no_prior_read:** v1 statement, plus Q1 verbatim.
- **version:** v2. **Diff from v1:** grade E0 → "not graded" with the two dated searches (F3); Q1 added to `no_prior_read` (F1). Direction, size (`no_mapping`) and status unchanged.

## M-002 v2 — rapamycin → H5 macroautophagy (clearance)

- **grade:** **not graded**, same two dated searches as M-001 v2.
- **SYRCLE:** not applicable.
- **no_prior_read:** v1 statement, plus Q1 verbatim.
- **version:** v2. **Diff from v1:** as M-001 v2 (F1, F3). Nothing else changed.

## M-003 v2 — acarbose → H6 nutrient sensing (production)

- **intervention:** acarbose (`CHEMBL404271`); curated ChEMBL mechanisms: maltase-glucoamylase MGAM `CHEMBL2074` (INHIBITOR) and pancreatic α-amylase `CHEMBL2045` (INHIBITOR), both human targets. ChEMBL has no mouse MGAM target record (ortholog missing in ChEMBL).
- **SYRCLE (Miura 1998):** **0/10 low risk.** All 10 domains unclear: no randomisation, allocation concealment, random housing or blinding reported; baseline characteristics not compared; attrition not stated; no protocol for selective reporting.
- **grade:** E1 (one causal test scoring below 5). Unchanged.
- **excluded:** PMID 36342748, `X-LS` (lifespan statement in the abstract, Q1; full text not opened).
- **no_prior_read:** v1 statement, plus Q1 verbatim.
- **version:** v2. **Diff from v1:** ChEMBL ids replace `[to verify]`; SYRCLE count and domain list replace "< 5" (F2); Q1 added (F1). Size **0.442 [0.402, 0.485]**, direction and status unchanged.

## M-004 v2 — canagliflozin → H6 nutrient sensing (production)

- **SYRCLE (Liang 2012):** **0/10 low risk.** Mice "housed according to standard procedures"; no randomisation, allocation concealment, random housing or blinding reported; baseline not compared; attrition not stated; no protocol. All 10 domains unclear.
- **grade:** E1. Unchanged.
- **version:** v2. **Diff from v1:** SYRCLE count and domain list replace "< 5" (F2). Size **0.528 [0.372, 0.748]**, direction and status unchanged.

## M-005 v2 — dasatinib + quercetin (senolytic) → H8 senescence clearance

- **intervention:** D+Q regimen. Dasatinib `CHEMBL1421`, curated mechanism SRC inhibitor `CHEMBL2363074` (SRC human `CHEMBL267`, mouse `CHEMBL3074`). Quercetin `CHEMBL50`: ChEMBL has no curated mechanism record, so it is recorded as multi-target.
- **parameter:** `hallmark.H8_senescence` (clearance term), edge E-H8.
- **direction:** senescence markers fall under D+Q (marker level). The sign on H8 clearance is up by NA4 (A28 rule, `research/sim/v0/design/edges.md` §4), not by the writer.
- **marker readings (per panel, never combined here):** writer's own rule-7a reading, PR #228 @ `dd37921` (committed after NA4.1 merge `c1b935e`), checked against A66's blind reading #211 in record #230 (both within 5 % / 10 %). Ratio = D+Q / Old, mean ± SEM, N = 5 per group, 95 % CI by the delta method with t(0.975, 4).

  | Panel | r | 95 % CI | Note |
  | --- | --- | --- | --- |
  | Fig 1d p16 positive area (%) | **0.209** | [0.135, 0.325] | agreed with #211 (0.204) |
  | Fig 1f SA-β-gal positive area (%) | **0.562** | [0.303, 1.043] | agreed with #211 (0.557); **the interval includes 1** |

- **NA4.1 lines per panel** (m = H8 clearance multiplier; interval endpoints mapped from r's interval; frozen rule `c1b935e`):

  | Panel | primary m = r^−0.5 | m = 1/r (first-order end) | m = 1 (saturated end) | production × r (NA4 line) | stress only: m = r^−2, outside model range |
  | --- | --- | --- | --- | --- | --- |
  | 1d p16 | 2.186 [1.755, 2.724] | 4.780 [3.080, 7.418] | 1 | × 0.209 [0.135, 0.325] | 22.85 [9.49, 55.03] |
  | 1f SA-β-gal | 1.334 [0.979, 1.818] | 1.780 [0.959, 3.304] | 1 | × 0.562 [0.303, 1.043] | 3.17 [0.92, 10.91] |

- **dilution:** m never acts on `beta` directly. It reaches organ axis i only through the hallmark mean (NA2): repair factor = (m + n_i − 1) / n_i, with n_i = 2 (H5, H8) or 3 (H5, H8, H9: muscle, bone, immune, blood, skin, gut). So an organ sees at most half of m's effect.
- **prior_size:** **no single multiplier in this row.** Both panels are listed separately, as A50 ruled; the writer does not pick, average or pool them. M-005 becomes one calibration input only through NA4.2 (A28 rule, PR #231, A76's `pool_panels.py`) once it is on `main`, computed by that script, not here.
- **named limits:**
  1. Only **2 of the 5** listed panels are comparable between the two readers (1d p16 area, 1f SA-β-gal). Fig 1b p16/p21 mRNA: `no_mapping` on the writer's side (fewer than 2 readable tick labels). Fig 1d p21 cells: A66 `no_mapping`, writer read 0.187 [0.075, 0.465]; not used, no back-filling.
  2. **High between-panel heterogeneity:** I² ≈ 0.92 (A66 re-run of the NA4.2 script, record #233); τ² weakly estimated at k = 2.
  3. NA4.1's steady-state limit: if the marker has not reached steady state, 1/r understates m.
  4. One source, one lab, one organ (kidney), C57BL/6J.
- **mechanistic_evidence:** NPJ Regen Med 2026, doi:10.1038/s41536-026-00490-x (Crossref ✓). As in v1. Only Fig 1 marker panels and the Fig 1 legend were read.
- **SYRCLE (NPJ 2026):** **4/10 low risk:** (1) sequence generation (random-number tables), (3) allocation concealment (investigators blinded to assignment), (5) caregiver blinding (blinded during administration), (7) assessor blinding (blinded outcome assessment). Unclear: (2) baseline, (4) random housing, (6) random outcome assessment (random fields only, not random animal order), (8) incomplete data, (9) selective reporting, (10) other.
- **grade:** **E1** (one causal test scoring 4 < 5). v1's "E2-equivalent" was wrong.
- **excluded / not used:** Zhu 2015, doi:10.1111/acel.12344: **not used**, superseded draft prior, abstract read (healthspan only). Not an `X-LS` exclusion. Two clean-abstract sources not opened (doi:10.1111/acel.70759, doi:10.14336/AD.2025.0976) stay `[to verify]`.
- **flags:** kidney only; C57BL/6J; D+Q from 12 to 20 months; marker→parameter via NA4/NA4.1, named assumptions.
- **status:** marker readings mapped per panel; **not a calibration input** until NA4.2 is on `main`.
- **seal:** not sealed.
- **writer:** A78.
- **no_prior_read:** A78, 2026-10-21. No R-Sim calibration output and no numeric lifespan, survival or mortality result for D+Q was read. Seen in words only, quoted under rule 6: Q2. Zhu 2015 was read at abstract level only.
- **version:** v2. **Diff from v1:** per-panel marker ratios added (from #228, check #230); NA4.1 lines and dilution added; no single multiplier; grade E2-equivalent → **E1** with the domain list (F2); ChEMBL ids replace `[to verify]`; Zhu 2015 re-recorded as "not used"; limits 1–4 added; Q2 disclosed.

## Summary, v2

| Row | Size | Grade | SYRCLE | Status |
| --- | --- | --- | --- | --- |
| M-001 v2 | `no_mapping` | not graded (2 dated searches) | n/a | no_mapping |
| M-002 v2 | `no_mapping` | not graded (2 dated searches) | n/a | no_mapping |
| M-003 v2 | 0.442 [0.402, 0.485] | E1 | 0/10 | mapped |
| M-004 v2 | 0.528 [0.372, 0.748] | E1 | 0/10 | mapped |
| M-005 v2 | per panel: 0.209 [0.135, 0.325]; 0.562 [0.303, 1.043] | E1 | 4/10 | per-panel; single input only via NA4.2 |

Open `[to verify]`: the two unopened senolytic sources (M-005).
