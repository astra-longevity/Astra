# Reference-case survey for astra-stats estimators (D67 prep)

Read-only preparation under council decision D67. No data read beyond what papers print, no code run.

**Rule D6 (amended):** a reference case counts only if (a) its numbers are printed in a peer-reviewed paper that also prints the inputs, or (b) the numbers come from a different, independent implementation. Package vignettes are smoke tests only.

All DOIs below were checked on Crossref on 2026-10-19. Gaps are marked `[to verify]`.

## Summary

| # | Estimator | Best reference | Inputs printed in full? | Meets D6 |
|---|-----------|----------------|-------------------------|----------|
| 1 | MR IVW, MR-Egger, weighted median | TwoSampleMR vs MendelianRandomization on the same 28 variants, in Actions | No printed per-SNP table found | Only via (b) |
| 2 | Random effects DL / REML | Normand 1999, stroke-unit data | Yes (9 studies, n/mean/SD per arm) | Yes (a) |
| 3 | Hartung-Knapp (HKSJ) | IntHout 2014, two worked examples (appendix) | Yes for the HK step (estimate + DL weight per study) | Yes (a), HK step only |
| 4 | Mantel-Haenszel (no CC), Peto OR | Efthimiou 2018, Figs 1-2 + Table 2 | Yes (events/n per arm, 18 + 24 trials, many zero-event arms) | Yes (a), hand-checked |
| 5 | RMST | Hand area under the KM curve on Gehan 1965 times + survRM2 | Yes (42 individual times) | Yes (b), hand-computed |
| 6 | Guyot KM reconstruction | Guyot 2012 Table 3 (Bonner 2006 curves) | Curve + at-risk table printed in the source trial | Partly: only 1 of the 3 required curves is itemised |
| 7 | van Zwet shrinkage prior | van Zwet, Schwab & Senn 2021, Table 1 | Parameters printed | Yes (a) |

## 1. Mendelian randomization: IVW, MR-Egger, weighted median

Candidate sources (all on Crossref):

- Burgess, Butterworth & Thompson 2013, *Genet Epidemiol* 37:658-665, doi:10.1002/gepi.21758 (IVW).
- Bowden, Davey Smith & Burgess 2015, *Int J Epidemiol* 44:512-525, doi:10.1093/ije/dyv080 (MR-Egger).
- Bowden et al. 2016, *Genet Epidemiol* 40:304-314, doi:10.1002/gepi.21965 (weighted median).
- Yavorska & Burgess 2017, *Int J Epidemiol* 46:1734-1739, doi:10.1093/ije/dyx034 (MendelianRandomization R package).

**Are the inputs printed?** Yavorska 2017 works through 28 lipid variants (data from Waterworth et al., ATVB) but shows code, not a per-SNP table. The per-SNP values ship as package data. Whether Burgess 2013, Bowden 2015 or Bowden 2016 print a complete per-SNP table with results: `[to verify]` (full text not checked).

**Printed outputs:** the LDL-C → CHD numbers in circulation (IVW 2.834, SE 0.530; weighted median 2.683, SE 0.419; simple median 1.755, SE 0.740; MR-Egger 3.253, SE 0.770, intercept −0.011, SE 0.015) come from the package vignette. Under D6 they are a **smoke test only**. Lecture slides from the Burgess book are excluded (not peer reviewed).

**Decision (A50, 2026-10-19):** no printed per-SNP table exists for these papers, so the reference is route (b): run TwoSampleMR (`mr_ivw`, `mr_egger_regression`, `mr_weighted_median`) and MendelianRandomization (`mr_ivw`, `mr_egger`, `mr_median`) on the same 28 variants in GitHub Actions, and compare both with astra-stats. The search for a printed table stops here.

**Tolerance:** 1e-3 absolute on estimates and analytic SEs against the second implementation. Weighted-median SE is bootstrapped, so 5% relative.

## 2. Random effects: DerSimonian-Laird and REML

Source: Normand 1999, "Meta-analysis: formulating, evaluating, combining, and reporting", *Stat Med* 18:321-359, doi:10.1002/(SICI)1097-0258(19990215)18:3<321::AID-SIM28>3.0.CO;2-P.

**Inputs:** Table II prints all 9 specialist stroke-unit trials in full (n, mean length of stay, SD for each arm). Complete.

**Printed outputs (Table X):**

| Model | tau² | Pooled mean difference (days) | SE | 95% CI |
|-------|------|-------------------------------|----|--------|
| Fixed effect | - | −3.5 | 0.78 | - |
| DL | 219 | −14 | 5 | (−24, −4) |
| REML | 685 (SE 380, from SAS) | −15 | 9 | (−32, 2) |

metafor's own reproduction gives DL tau² 218.72 and Q(8) = 241.06, REML tau² 685.20. Its SE of tau² (360.37) differs from the SAS value (380), so **the SE of tau² is not a target**. The metadat page shows Q 238.92 and tau² 684.65 because it uses a different within-study variance formula. The case file must state the convention: vi = SD₁²/n₁ + SD₂²/n₂.

**Tolerance:** within printed rounding (tau² ±0.5; mean, SE and CI limits ±0.5 day). Against metafor as second implementation: 1e-6 relative.

## 3. Hartung-Knapp-Sidik-Jonkman

Source: IntHout, Ioannidis & Borm 2014, *BMC Med Res Methodol* 14:25, doi:10.1186/1471-2288-14-25. The paper gives an Excel sheet (additional file 1) and R code (appendix 3).

**Inputs (printed in full in the worked examples):** each study's estimate and its DL weight (% of total). HKSJ only needs relative weights, so these inputs are complete for the HK step. They are not enough to recompute DL tau² (no SEs).

- Example 1: zinc for the common cold (Cochrane CD001364, 5 studies, 513 patients), SMD. y = −0.04, −0.07, −0.31, −1.36, −0.54; DL weights % = 24.0, 22.2, 21.3, 15.5, 17.0.
  Printed: pooled −0.39; Σw(y − ȳ)² = 20.31; variance 0.0508; SE 0.225; t₀.₉₇₅(4) = 2.78; 95% CI [−1.02, 0.24]; t = −1.73; p = 0.16.
- Example 2: donor vs no donor in ALL (Cochrane CD008818, 10 trials), HR. HR = 0.81, 0.67, 0.80, 0.91, 0.56, 0.98, 1.24, 0.75, 0.95, 0.66; DL weights % = 5.0, 2.1, 11.5, 46.7, 2.9, 9.3, 3.9, 12.7, 3.9, 2.0.
  Printed: pooled ln HR −0.15 (HR 0.86); Σ factors 1.99 → /100 = 0.0199 → /9 = 0.0022; SE 0.047; t₀.₉₇₅(9) = 2.26; HR 95% CI [0.77, 0.96]; t = −3.19; p = 0.011. The DL figures for this example (z = −2.48, p = 0.013, CI [0.77, 0.97]) come from a search snippet: `[to verify]`.

**Tolerance:** the inputs are rounded (0.01 on estimates, 0.1 on weights), so 0.01 absolute on the pooled estimate and CI limits, 0.02 on t, 0.005 on p. Second implementation: metafor `rma.uni(yi, vi = 1/w, method = "FE", test = "knha")` (the paper's own approach), 1e-6 relative.

## 4. Rare events: Mantel-Haenszel (no continuity correction) and Peto OR

Bradburn DOI confirmed: Bradburn, Deeks, Berlin & Localio, "Much ado about nothing: a comparison of the performance of meta-analytical methods with rare events", *Stat Med* 26:53-77, doi:10.1002/sim.2528. Published online in 2006, in the 2007 volume. It is a simulation study and prints no applied 2x2 case `[to verify]`. Sweeting, Sutton & Lambert 2004, *Stat Med* 23:1351-1375, doi:10.1002/sim.1761, is also a simulation study.

**Candidate printed case:** Efthimiou 2018, "Practical guide to the meta-analysis of rare events", *Evid Based Ment Health* 21:72-76, doi:10.1136/eb-2018-102911 (PMC10270432). Table 2 gives two examples:

| Method | Example A OR (95% CI) | Example B OR (95% CI) |
|--------|-----------------------|-----------------------|
| IV, treatment-arm CC | 0.70 (0.26-1.90) | 0.71 (0.38-1.34) |
| Peto | 0.75 (0.24-2.36) | 0.67 (0.35-1.25) |
| MH, no CC | 0.77 (0.26-2.28) | 0.66 (0.34-1.26) |

One example includes 3:1 randomised trials, so the known Peto bias with unbalanced arms is visible. Other printed rows: MH with treatment-arm CC 0.85 (0.36-2.00) | 0.71 (0.39-1.28); IV with 0.5 CC 0.63 (0.24-1.70) | 0.71 (0.38-1.34).

**Inputs:** printed in full in Fig 1 (A) and Fig 2 (B) as forest plots with events/total per arm (data from Kishi, Matsunaga & Iwata 2016, *Schizophr Bull* 42:1438-1445, doi:10.1093/schbul/sbw043). Transcribed by OCR from the figures; the column totals match the printed totals (A: 7/3774 vs 6/2145; B: 15/4059 vs 24/3820). Trial names were spelling-corrected from the OCR output (e.g. "Pandinda" → Pandina, "Ghue" → Chue); names and years are `[to verify]` against the figures. The counts are what matter.

Example A, death, LAI-AP vs placebo (18 trials: 1 with events in both arms, 9 with a single zero arm, 8 with zero events in both arms):

| Trial | LAI-AP events/n | Placebo events/n |
|-------|-----------------|------------------|
| Kane 2012 | 1/269 | 0/134 |
| Kane 2014 | 0/168 | 0/172 |
| Meltzer 2015 | 0/415 | 1/207 |
| Hirsch 1973 | 0/41 | 0/40 |
| Jolley 1990 | 2/27 | 0/27 |
| Odejide 1952 (as OCR'd; likely 1982 `[to verify]`) | 0/35 | 1/35 |
| Rifkin 1977 | 0/23 | 1/22 |
| Lauriello 2008 | 0/306 | 0/98 |
| Berwaerts 2015 | 0/160 | 0/145 |
| Fu 2015 | 2/164 | 0/170 |
| Gopal 2010 | 0/221 | 0/135 |
| Hough 2010 | 0/206 | 0/204 |
| Kramer 2010 | 0/163 | 0/84 |
| Nasrallah 2010 | 1/391 | 1/127 |
| Pandina 2010 | 1/488 | 0/164 |
| Takahashi 2013 | 0/160 | 1/164 |
| Kane 2003 | 0/302 | 1/98 |
| Nasser 2016 | 0/235 | 0/119 |

Example B, death, LAI-AP vs oral antipsychotic (24 trials):

| Trial | LAI-AP events/n | Oral events/n |
|-------|-----------------|---------------|
| Fleischhacker 2014 | 0/265 | 1/266 |
| Ishigooka 2015 | 1/228 | 1/227 |
| Del Giudice 1975 | 0/27 | 0/61 |
| Falloon 1978 | 0/20 | 0/24 |
| Hogarty 1973 (as OCR'd; likely 1979 `[to verify]`) | 2/55 | 0/50 |
| Kaneno 1991 | 0/127 | 1/132 |
| Rifkin 1977 | 0/23 | 1/28 |
| Ohkuma 1985 | 0/141 | 1/142 |
| Detke 2011 | 0/264 | 2/260 |
| Kane 2010 | 0/599 | 0/322 |
| Alphs 2015 | 1/230 | 0/220 |
| Schreiner 2015 | 1/352 | 1/363 |
| Bai 2007 | 0/25 | 0/25 |
| Buckley 2015 | 2/153 | 4/152 |
| Chue 2005 | 0/319 | 1/321 |
| Gaebel 2010 | 3/329 | 2/382 |
| Kamijima 2009 | 0/153 | 0/52 |
| Keks 2007 | 2/247 | 6/300 |
| Macfadden 2010 | 1/179 | 1/176 |
| Malla 2013 | 0/44 | 0/41 |
| Potapov 2008 | 0/20 | 0/21 |
| Rosenheck 2011 | 2/190 | 2/192 |
| Subotnik 2015 | 0/43 | 0/43 |
| Arango 2006 | 0/26 | 0/20 |

**Hand check (awk, standard formulas, no statistics package):** MH OR without CC = 0.7732 (A) and 0.6589 (B); Peto OR = 0.7517 [0.2390, 2.3645] (A) and 0.6658 [0.3542, 1.2514] (B). All agree with the printed 2-dp values, so the transcription is consistent with the paper's results.

**Tolerance:** 0.005 absolute on OR and CI limits (2-dp printing). Against metafor `rma.mh` / `rma.peto`: 1e-6 relative.

## 5. Restricted mean survival time

Royston & Parmar 2013, *BMC Med Res Methodol* 13:152, doi:10.1186/1471-2288-13-152. Table 6 prints RMST (years, control/research) for four MRC trials: ASTEC t* = 6.1, 5.39/5.28; BA06 t* = 6.7, 3.69/4.02; ICON4 t* = 6.3, 2.46/2.79; OE02 t* = 10.8, 2.68/3.13. The individual data are not printed, so this does **not** meet D6. Plausibility check only.

**Proposed reference:** Gehan 1965, *Biometrika* 52:203-224, doi:10.1093/biomet/52.1-2.203, prints all 42 remission times of the Freireich 6-MP leukaemia trial (21 per arm). The area under the KM step function is computed by hand (exact sum of rectangles), and survRM2 `rmst2` is run in Actions as a second implementation. A printed RMST on these data: `[to verify]`.

**Tolerance:** hand vs astra-stats 1e-9 (exact arithmetic); against survRM2 1e-6 for RMST and its variance.

## 6. Guyot 2012 Kaplan-Meier reconstruction

Source: Guyot, Ades, Ouwens & Welton 2012, *BMC Med Res Methodol* 12:9, doi:10.1186/1471-2288-12-9.

**Worked example (Table 3):** Bonner et al. 2006, *NEJM* 354:567-578, doi:10.1056/NEJMoa053422 (radiotherapy ± cetuximab, head and neck cancer). The Illustration section cites the curves as ref [12] = Bonner 2006 (checked in the PMC full text, PMC3313891); the Table 3 caption's "[7]" (Parmar 1998) looks like a typo in the paper. Original vs reconstructed:

| Arm | 1 y | 2 y | 3 y | Median (months) |
|-----|-----|-----|-----|-----------------|
| Radiotherapy | 55 / 56.1 | 41 / 41.1 | 34 / 34.7 | 14.9 / 14.9 |
| Radiotherapy + cetuximab | - | 50 / 50.4 | 47 / 46.7 | 24.4 / 24.3 |

Overall validation covered 4 publications (22 survival probabilities, 7 medians, 6 HRs, 4 SE(log HR)). The other 3 curves are not itemised in the text: `[to verify]`. With full information, the reported accuracy is a mean survival error of −0.103% (CI −0.260; 0.055) and a mean absolute error of 0.272%.

**Second implementation:** Wei & Royston 2017, *Stata J* 17:786-802, doi:10.1177/1536867X1801700402 (`ipdfc`), reproduces the same example.

**Gap:** closed. D6 needs ≥3 curves; 3 are identified (Bonner 2006; ICON7 for median/survival only; Goss 2005 for HR only), see below.

**Curve 2: ICON7.** Perren et al. 2011, *NEJM* 365:2484-2496, doi:10.1056/NEJMoa1103799 (Crossref OK), bevacizumab in ovarian cancer. The trial prints the KM figure, the at-risk table, the median and the HR. Wei & Royston 2017 reconstruct it:

| Quantity | Original | ipdfc (Wei & Royston 2017) |
|----------|----------|----------------------------|
| HR (95% CI) | 0.81 (0.70, 0.94) | 0.83 (0.72, 0.96) |
| Median, chemotherapy arm (months) | 17.3 | 17.5 (16.1, 18.7) |
| At risk at 6 / 12 months | 693 / 464 | 694 / 465 |

Endpoint and analysis, checked against the NEJM full text (2026-10-20): the values are **progression-free survival, primary analysis** (cutoff 28 Feb 2010, 759 events: 392 standard, 367 bevacizumab; median follow-up 19.4 months; Figure 2A). Printed: HR for progression or death with bevacizumab 0.81 (0.70, 0.94), log-rank P = 0.004; median PFS 17.3 months standard chemotherapy vs 19.0 months bevacizumab. So the median 17.3 is the standard-chemotherapy arm. Not to be confused with the updated analysis (cutoff 30 Nov 2010, 934 events): HR 0.87 (0.77, 0.99), medians 17.4 / 19.8.

**Ruling (A50, 2026-10-20):** the paper reports clear non-proportional hazards (test P < 0.001) and states that the conventional HR "is not meaningful" here. ICON7 therefore counts as a curve **for the median and survival at printed times only**; its HR is excluded from the D38 HR check.

At-risk counts (693 / 464 at 6 / 12 months) are printed only in the Figure 2A image, which the text scrape cannot read; which arm they belong to is `[to verify]` against the figure. Until resolved, the at-risk counts are not used for this curve.

**Not usable:** the third worked example in Wei & Royston 2017 prints no original HR or median (only log-rank p 0.0003 vs 0.0006 reconstructed, and an absolute risk reduction 1.9% vs 1.82%), so it fails the "printed median or HR" requirement.

**Candidate for curve 3:** Saluja et al. 2019, *Res Synth Methods* 10:465-475, doi:10.1002/jrsm.1362 (Crossref OK): 118 KM curves from 55 RCTs (pCODR, 2012-2016) and 77 from 46 RCTs (FDA), with reported vs reconstructed HRs. The abstract and the publicly available snippets give only pooled accuracy (Guyot mean error 0.0094, 95% CI −0.0012 to 0.020, on the log HR scale `[to verify]`); no per-trial values are visible outside the paywalled full text or supplement. Not usable until a per-trial table is found `[to verify]`.

**Guyot 2012 itself (full text checked):** only Bonner is itemised (Table 3). Table 4 gives pooled accuracy only (HR, full information: mean error 0.008 on log scale, MAE 0.017, 97.5% bound exp(0.122) = 1.13). The other three validation publications are named but their per-curve results are not printed:

- van Oers et al. 2006, *Blood* 108:3295-3301, doi:10.1182/blood-2006-05-021113 (rituximab maintenance, follicular lymphoma).
- Goss et al. 2005, *JNCI* 97:1262-1271, doi:10.1093/jnci/dji250 (letrozole, MA.17).
- Seymour et al. 2007, *Lancet* 370:143-152, doi:10.1016/S0140-6736(07)61087-3 (MRC FOCUS).

Since the D6 target is the original printed median/HR (astra-stats reconstructs, then compares), these trials can serve as curve 3 if their paper prints the KM figure, the at-risk table and a median or HR, and does not flag non-proportional hazards (DOIs checked on Crossref 2026-10-20). Checked in the order van Oers → Goss → Seymour:

- **van Oers 2006: not retained.** Full text paywalled. The abstract prints medians and HRs (PFS from first randomisation 20.2 vs 33.1 months, HR 0.65; maintenance PFS 14.9 vs 51.5 months, HR 0.40; OS HR 0.52), but the at-risk table and any PH statement cannot be checked.
- **Goss 2005: curve 3, confirmed (HR only).** Checked in the JNCI full text (OUP PDF, 2026-10-20). MA.17 updated analysis, letrozole vs placebo after 5 years of tamoxifen; 2583 / 2587 analysed (Table 1, Fig 1), median follow-up 30 months. Locators:
  - Fig 2 (p. 1265): DFS Kaplan-Meier curves; the caption states "N = number at risk; S = survival percent", so the at-risk row is printed (values in the image only, to be read when digitising).
  - Abstract and Results, "Disease-Free Survival": HR 0.58 (0.45, 0.76), P < .001; 92 vs 155 events (Table 2); 4-year DFS 94.4% vs 89.8%. No median DFS printed (too few events), so the target is the **HR only**.
  - Methods, "Statistical Analysis": PH assumption verified by the Grambsch-Therneau test; no non-PH statement anywhere in the paper.
  - Caveat: the printed HR comes from a Cox model **stratified** on receptor status, nodal status and prior chemotherapy; a reconstruction from the pooled curves gives an unstratified HR. This difference belongs in the observed-deviation data, not in the tolerance.
- **Seymour 2007:** not needed, since Goss qualifies.

**Usable-HR count:** Bonner 2006 + Goss 2005 = 2 curves with a usable HR (ICON7 excluded), which meets A50's ≥2 requirement.

**Tolerance:** not proposed here. Under D38 Verification sets it from ≥3 curves; setting it from one curve, just above the error already observed, would make the test pass by construction.

**Observed deviations on Bonner 2006 (data, `[to verify]` against the source tables):**

| Quantity | Original | Guyot 2012 | ipdfc (Wei & Royston 2017) |
|----------|----------|------------|----------------------------|
| RT survival 1 y / 2 y / 3 y (%) | 55 / 41 / 34 | 56.1 / 41.1 / 34.7 (Δ +1.1 / +0.1 / +0.7) | - |
| RT+cetuximab survival 2 y / 3 y (%) | 50 / 47 | 50.4 / 46.7 (Δ +0.4 / −0.3) | - |
| Median RT (months) | 14.9 | 14.9 (Δ 0) | 16.1 (as found in a snippet, `[to verify]`) |
| Median RT+cetuximab (months) | 24.4 | 24.3 (Δ −0.1) | - |
| HR (95% CI) | 0.68 (0.52, 0.89) | 0.73 (0.57, 0.94), +7.4% | 0.72 (0.56, 0.93), +5.9% |

## 7. van Zwet-Schwab-Senn shrinkage prior

Source: van Zwet, Schwab & Senn 2021, "The statistical properties of RCTs and a proposal for shrinkage", *Stat Med* 40:6107-6117, doi:10.1002/sim.9173 (journal version, Table 1). The fit uses 23,551 trial pairs from the Cochrane database. The arXiv preprint (2011.15004) used 23,747 pairs, so **use the journal values only**.

Mixture of 4 normal distributions, all with mean 0:

| Component | 1 | 2 | 3 | 4 |
|-----------|---|---|---|---|
| Proportion | 0.32 | 0.31 | 0.30 | 0.07 |
| SD of z | 1.17 | 1.74 | 2.38 | 5.73 |
| SD of SNR | 0.61 | 1.42 | 2.16 | 5.64 |

Relationship: SD_SNR² + 1 = SD_z². The same values are reprinted in van Zwet et al. 2023, *NEJM Evidence*, doi:10.1056/EVIDoa2300003. Printed derived outputs: median achieved power 13%; exaggeration factor ≈1.7 for results that are just significant at 5%. Posterior formulas are in the appendix (component weights; component means m_i = z·tau_i²/(tau_i² + 1)).

**Tolerance:** parameters exactly as printed (2 dp); internal check SD_SNR² + 1 = SD_z² within ±0.02; derived power and exaggeration within printed rounding.

## Open items

1. MR: closed. Route (b) TwoSampleMR vs MendelianRandomization in Actions.
2. HK: confirm the DL figures of example 2 (z, p, CI).
3. Rare events: confirm trial names in Figs 1-2 (counts already checked against totals and printed ORs).
4. RMST: check whether any paper prints an RMST for the Gehan/Freireich data.
5. Guyot: read the at-risk values from Goss Fig 2 and ICON7 Fig 2A when digitising (ICON7 arm assignment `[to verify]`).
