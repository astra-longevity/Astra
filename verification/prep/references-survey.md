# Reference-case survey for astra-stats estimators (D67 prep)

Read-only preparation under council decision D67. No data read beyond what papers print, no code run.

**Rule D6 (amended):** a reference case counts only if (a) its numbers are printed in a peer-reviewed paper that also prints the inputs, or (b) the numbers come from a different, independent implementation. Package vignettes are smoke tests only.

All DOIs below were checked on Crossref on 2026-10-19. Gaps are marked `[to verify]`.

## Summary

| # | Estimator | Best reference | Inputs printed in full? | Meets D6 |
|---|-----------|----------------|-------------------------|----------|
| 1 | MR IVW, MR-Egger, weighted median | Second implementation (TwoSampleMR) on per-SNP inputs from a printed table | Not in Burgess 2013, Bowden 2015/2016 or Yavorska 2017 `[to verify]` | Only via (b) |
| 2 | Random effects DL / REML | Normand 1999, stroke-unit data | Yes (9 studies, n/mean/SD per arm) | Yes (a) |
| 3 | Hartung-Knapp (HKSJ) | IntHout 2014, two worked examples | Partly `[to verify]` | (a) if the per-study table is complete, otherwise (b) |
| 4 | Mantel-Haenszel (no CC), Peto OR | Efthimiou 2018, Table 2 | `[to verify]` | (a) if the 2x2 tables are printed, otherwise (b) |
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

**Proposed reference:** take the per-SNP inputs from a printed table (Waterworth supplement `[to verify]`) and compare with an independent implementation (TwoSampleMR `mr_ivw`, `mr_egger_regression`, `mr_weighted_median`) run in GitHub Actions.

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

Source: IntHout, Ioannidis & Borm 2014, *BMC Med Res Methodol* 14:25, doi:10.1186/1471-2288-14-25. The paper gives an Excel sheet (additional file 1) and R code (appendix).

**Printed outputs:**

- Example 1 (10 trials, hazard ratios): DL HR 0.86 [0.77, 0.97], z = −2.48, p = 0.013. HKSJ HR 0.86 [0.77, 0.96], t = −3.19, df = 9, p = 0.011. The HKSJ interval is narrower here (q < 1), which makes it a good test of plain HK against the modified version with max(1, q).
- Example 2 (5 studies, SMD, I² = 75%, tau² = 0.13): HKSJ SMD −0.39 [−1.02, 0.24], |t| = 1.73, df = 4, p = 0.16.

**Inputs:** whether the per-study estimates and SEs are printed in full: `[to verify]`.

**Tolerance:** 0.005 absolute on estimates and CI limits, ±0.005 on t, ±0.0005 on p. Against metafor `rma(test = "knha")`: 1e-6 relative.

## 4. Rare events: Mantel-Haenszel (no continuity correction) and Peto OR

Bradburn DOI confirmed: Bradburn, Deeks, Berlin & Localio, "Much ado about nothing: a comparison of the performance of meta-analytical methods with rare events", *Stat Med* 26:53-77, doi:10.1002/sim.2528. Published online in 2006, in the 2007 volume. It is a simulation study and prints no applied 2x2 case `[to verify]`. Sweeting, Sutton & Lambert 2004, *Stat Med* 23:1351-1375, doi:10.1002/sim.1761, is also a simulation study.

**Candidate printed case:** Efthimiou 2018, "Practical guide to the meta-analysis of rare events", *Evid Based Ment Health* 21:72-76, doi:10.1136/eb-2018-102911 (PMC10270432). Table 2 gives two examples:

| Method | Example A OR (95% CI) | Example B OR (95% CI) |
|--------|-----------------------|-----------------------|
| IV, treatment-arm CC | 0.70 (0.26-1.90) | 0.71 (0.38-1.34) |
| Peto | 0.75 (0.24-2.36) | 0.67 (0.35-1.25) |
| MH, no CC | 0.77 (0.26-2.28) | 0.66 (0.34-1.26) |

One example includes 3:1 randomised trials, so the known Peto bias with unbalanced arms is visible.

**Inputs:** whether the per-study 2x2 tables are printed: `[to verify]`.

**Tolerance:** 0.005 absolute on OR and CI limits (2-dp printing). Against metafor `rma.mh` / `rma.peto`: 1e-6 relative.

## 5. Restricted mean survival time

Royston & Parmar 2013, *BMC Med Res Methodol* 13:152, doi:10.1186/1471-2288-13-152. Table 6 prints RMST (years, control/research) for four MRC trials: ASTEC t* = 6.1, 5.39/5.28; BA06 t* = 6.7, 3.69/4.02; ICON4 t* = 6.3, 2.46/2.79; OE02 t* = 10.8, 2.68/3.13. The individual data are not printed, so this does **not** meet D6. Plausibility check only.

**Proposed reference:** Gehan 1965, *Biometrika* 52:203-224, doi:10.1093/biomet/52.1-2.203, prints all 42 remission times of the Freireich 6-MP leukaemia trial (21 per arm). The area under the KM step function is computed by hand (exact sum of rectangles), and survRM2 `rmst2` is run in Actions as a second implementation. A printed RMST on these data: `[to verify]`.

**Tolerance:** hand vs astra-stats 1e-9 (exact arithmetic); against survRM2 1e-6 for RMST and its variance.

## 6. Guyot 2012 Kaplan-Meier reconstruction

Source: Guyot, Ades, Ouwens & Welton 2012, *BMC Med Res Methodol* 12:9, doi:10.1186/1471-2288-12-9.

**Worked example (Table 3):** Bonner et al. 2006, *NEJM* 354:567-578, doi:10.1056/NEJMoa053422 (radiotherapy ± cetuximab, head and neck cancer). The paper cites it as ref [7]; the match with Bonner is `[to verify]`. Original vs reconstructed:

| Arm | 1 y | 2 y | 3 y | Median (months) |
|-----|-----|-----|-----|-----------------|
| Radiotherapy | 55 / 56.1 | 41 / 41.1 | 34 / 34.7 | 14.9 / 14.9 |
| Radiotherapy + cetuximab | - | 50 / 50.4 | 47 / 46.7 | 24.4 / 24.3 |

Overall validation covered 4 publications (22 survival probabilities, 7 medians, 6 HRs, 4 SE(log HR)). The other 3 curves are not itemised in the text: `[to verify]`. With full information, the reported accuracy is a mean survival error of −0.103% (CI −0.260; 0.055) and a mean absolute error of 0.272%.

**Second implementation:** Wei & Royston 2017, *Stata J* 17:786-802, doi:10.1177/1536867X1801700402 (`ipdfc`), reproduces the same example.

**Gap:** D6 needs ≥3 curves and only 1 is itemised. The other 3 Guyot publications, or the curves in Wei & Royston 2017, need to be identified: `[to verify]`.

**Tolerance:** survival probability 1.5 percentage points absolute; median 1 month or 5% relative; HR 5% relative, and only when the at-risk table and total events are printed.

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

1. MR: find a printed per-SNP table (Waterworth supplement or the Bowden papers).
2. HK and rare events: check that the per-study inputs in IntHout 2014 and Efthimiou 2018 are complete.
3. RMST: check whether any paper prints an RMST for the Gehan/Freireich data.
4. Guyot: identify 2 more curves with printed medians/HRs.
