# Guyot curves 2 and 3: scope to unblock (proposal to A50)

Written 2026-10-22 by A66 at A50's request (G142, 12:33). It is a proposal. **Nobody digitises anything until A50 has ruled on it.** A50 sets D38. Cases: `verification/second-impl/guyot-icon7.yaml` and `guyot-goss.yaml`.

Decisions this applies:
- D38: Guyot counts as validated when it reproduces published medians, survival and HRs within a tolerance set by Verification, on at least 3 curves.
- D93: a curve is digitised twice, independently, as CSV with the figure reference. The build checks that the two agree. Numbers at risk come from the printed table.
- The M5 option-1 rules (A50, 2026-10-21, recorded in `approvals/l1-schema.md` and #211).

## 1. Sources, and why each curve is blocked today

| | guyot-icon7 | guyot-goss |
|---|---|---|
| Paper | Perren et al. 2011, *NEJM* 365:2484-2496, doi:10.1056/NEJMoa1103799, PMID 22204725 | Goss et al. 2005, *JNCI* 97:1262-1271, doi:10.1093/jnci/dji250, PMID 16145047 |
| Figure | Fig 2A, progression-free survival (primary analysis) | Fig 2 (p. 1265), disease-free survival |
| Arms (series) | standard chemotherapy; chemotherapy + bevacizumab | placebo; letrozole |
| Compared with | printed medians: 17.3 and 19.0 months. HR excluded (non-proportional hazards; A50 ruling, 2026-10-20) | printed HR 0.58 [0.45, 0.76]. No median is printed |
| At-risk row | inside the figure image only. Survey: 693/464 at 6/12 months, arm `[to verify]` | inside the figure image only (caption: "N = number at risk") |
| Licence | **no LA row** | **no LA row** |
| Image sha256 | **not pinned** | **not pinned** |
| Open access | no: Europe PMC says not OA, no PMC copy | no: Europe PMC says not OA, no PMC copy |

**Why both are blocked (checked 2026-10-22):**
- **B1, image:** nejm.org and academic.oup.com (article page and PDF) both return 403 to a script. Neither paper has an open copy, so no image can be pinned.
- **B2, licence:** neither figure has an LA row.
- **B3, inputs:** the 3 CSVs listed in each YAML don't exist, and only one reading exists for anything.
- **B4, ICON7 only:** we don't know which arm the 693/464 at-risk counts belong to. Only the image can settle this.

**Proposed routes out:**
- **B1:** the president downloads the two article PDFs through his own access and hands them over outside the repo, e.g. in `.uploads/`. Each reader takes Fig 2A / Fig 2 from the same file, and the file's sha256 is pinned.
  - Fallback if he has no access: the PMID pages, or an interlibrary request.
  - Not allowed: the digitised ICON7 data shipped with Wei & Royston's `ipdfc`. That is another team's single reading, the same situation as Bonner (A50: a pipeline run only, it can't feed D38).
- **B2:** two new LA rows, which A50 rules on.
  - **ICON7:** NEJM copyright.
  - **Goss:** OUP copyright.
  - Proposed conditions for both:
    - The image or PDF is never committed, only its sha256.
    - Only the digitised coordinates, the at-risk counts and the derived CSVs are committed. These are factual data read from the figure.
    - The DOI and the figure number are cited in every CSV header.
    - The PDF is deleted after the work, as for DrugAge and SLAM.
- **B3 and B4:** resolved by the method in section 2, once B1 and B2 are cleared.

## 2. Method (M5 option-1 rules, applied to step curves)

**Two readers, working blind.** Each reader:
- writes their own deterministic scan;
- commits it before converting any value;
- does not see the other's scripts, coordinates or values until both are committed.

Neither reader adapts the other's scan or cap rule (G1).

**Calibration:**
- For each axis, find 2 tick marks by pixel scan and match each to its printed label. The labels may be read by eye, or by OCR of the label text only.
- Never use the frame as a calibration point.
- If an axis lacks 2 locatable ticks, that curve fails for that reader.

**Curves:**
- For each arm, record the step corners (x, y in pixels) found by the reader's own scan, which separates the arms by colour or line style.
- Commit the raw pixel coordinates, the scan output and the image sha256. The scripts abort if the image is different.
- A committed script converts the pixels to `time, surv (%)` and writes the YAML's CSVs. There is one set per reader, so the paths gain a `-r1` or `-r2` suffix.
- Not allowed: OCR, model-based, fitted or automatic estimates of curve positions or values.

**Numbers at risk:**
- Read from the printed row as text, by eye or by label-text OCR, and type them into a committed CSV.
- Each reader states the arm order as printed. That settles B4 for ICON7.
- Where the two readers' counts differ, the curve is not used until the image is re-read and the difference explained.

**Pipeline:** each reader's CSVs go through the YAML's `call` unchanged (IPDfromKM 0.1.10), giving 2 reconstructions per curve.

**Fallback, decided by the rule:** a curve one reader can't locate is `not_digitised` for that reader. That curve then can't count towards D38's 3 curves. Neither reader fills the other's gaps.

**Second reader:**
- Reader 1 is A66.
- Reader 2: A65 is proposed. He has read no R-Metric outcome as a writer, but he does hold sealed-split material, so A50 should confirm this doesn't count against him.
- Alternative: a short-lived worker under A50, inside the fleet cap of 9, deleted after the CSVs are committed.

## 3. D38 tolerance proposal (fixed before anyone digitises)

These are proposed values only. A50 sets D38, and nothing is changed after the first reading is committed. Any change is a new version with a diff.

**(a) Agreement between the two readings, per curve.** A curve that fails (a) doesn't count towards D38.

| Quantity | Proposed limit |
|---|---|
| Reconstructed median | ≤ 3 % relative, or ≤ 0.5 month, whichever is larger |
| Reconstructed HR | ≤ 5 % relative |
| HR interval bounds | ≤ 10 % relative (same limits as M5) |
| At-risk counts | identical |

**(b) Each reading against the printed values.**

| Quantity | Proposed limit |
|---|---|
| Median | ≤ 5 % relative, or ≤ 1 month, whichever is larger |
| Survival at a printed time | ≤ 3 percentage points |
| HR | ≤ 0.05 absolute |
| HR interval bounds | ≤ 0.05 absolute |

The basis for (b) is the reconstruction error Guyot 2012 reports in Table 3. A50 may choose to tighten these limits once Bonner has run.

**Goss:** the printed HR comes from a stratified Cox model, and the reconstructed one cannot be stratified. If the gap exceeds the limit, it is recorded as a named cause. The limit is not widened (V-007).

**D38 passes** when 3 curves (Bonner, ICON7, Goss) each pass (b) for every quantity listed in the case, and ICON7 and Goss also pass (a).

Bonner is a package-digitised curve, so it gets (b) only. Whether that is enough for the third curve is A50's call. The alternative is to double-digitise Bonner as well, from Bonner 2006 Fig 1 under the same rules.

## 4. Exposure

- These are human cancer-trial survival curves, not mouse lifespan. They are still survival curves, so:
  - neither reader is an R-Metric writer (A28, A75, A78) or A78;
  - neither reader is a future L3 mapping writer.
- V-001 applies. Files a lifespan-naive writer may read never quote these survival values, so they stay under `verification/` only. If such a file mentions this work, it describes it and does not quote it.
- Both readers log the PDF download (date, sha256, where it was deleted) in their record.
