# Arcanum session 1, Round 8: Literature evidence brief

Author: Literature (A2). References checked by worker W-R8-Refs (A64) on 2026-10-20. Every DOI below resolves on Crossref and matches its PubMed record; see `arcanum-r8.check.tsv`. The locators point to the abstract unless stated otherwise.

## (a) A2's R8 items in short

- **D120.** Each D114 axis is calibrated against its own functional outcome. Only the summary score is expressed as a mortality equivalent. Reason: mortality calibration underweights ageing that does not kill, and most lab mice die of cancer.
- **D121.** Metric v0 is trained on public mouse data (SLAM, ITP, the Schultz frailty clocks, JAX Diversity Outbred) before Astra asks for its own cohort. This is the proof of work required by commandment 10.
- **D122.** Negative controls come only from published ITP null arms, matched by strain and sex.
- **D123.** Literature runs three missions that replace M4 and M8: L1 lifespan table, L2 metric dossier, L3 refutation file.

## (b) References

| # | Claim in R8 | Reference | DOI | Locator | Verdict |
|---|---|---|---|---|---|
| 1 | Frailty-index clocks FRIGHT (age) and AFRAID (life expectancy) predict mouse lifespan | Schultz MB et al. 2020, *Nat Commun* 11:4618 | [10.1038/s41467-020-18446-0](https://doi.org/10.1038/s41467-020-18446-0) | Abstract | Confirmed. Caveat: male C57BL/6 only. |
| 2 | SLAM is the NIA longitudinal ageing study in C57BL/6 and UM-HET3 mice | Palliyaguru DL et al. 2021, *J Gerontol A* 76(4):552–560 | [10.1093/gerona/glaa285](https://doi.org/10.1093/gerona/glaa285) | Abstract; PMC8011709 | Design confirmed: both strains, both sexes, a longitudinal arm plus a cross-sectional tissue arm. **Public data route not found** (see corrections). |
| 3a | JAX lifespan and healthspan data are publicly available | Bogue MA et al., *J Gerontol A* 71(2):170–177 (published online 2014, print 2016) | [10.1093/gerona/glu223](https://doi.org/10.1093/gerona/glu223) | Abstract | Confirmed. Data are in the Mouse Phenome Database (phenome.jax.org), which is public. |
| 3b | JAX Diversity Outbred lifespan study with longitudinal phenotyping | Luciano A … Churchill GA 2024, *GeroScience* 46(5):4937–4954 | [10.1007/s11357-024-01226-9](https://doi.org/10.1007/s11357-024-01226-9) | Abstract; PMC11639350 | Confirmed: longitudinal frailty data to death in C57BL/6 and DO mice. Caveat: see corrections. |
| 4 | Metformin alone extends lifespan in C57BL/6 mice | Martin-Montalvo A et al. 2013, *Nat Commun* 4:2192 | [10.1038/ncomms3192](https://doi.org/10.1038/ncomms3192) | Abstract | Confirmed, with limits: males only, 0.1% w/w from middle age; 1% w/w was toxic. |
| 5 | "Metformin alone isn't an ITP arm" | Strong R et al. 2016, *Aging Cell* 15(5):872–884 | [10.1111/acel.12496](https://doi.org/10.1111/acel.12496) | Abstract, PMID 27312235 | **Wrong.** The ITP tested metformin alone: "Metformin alone, at a dose of 0.1% in the diet, did not significantly extend lifespan." |
| 6a | Resveratrol is an ITP null | Miller RA et al. 2011, *J Gerontol A* 66A(2):191–201 | [10.1093/gerona/glq178](https://doi.org/10.1093/gerona/glq178) | Abstract | Confirmed: resveratrol at 300 and 1200 ppm had no effect on survival in either sex. |
| 6b | Resveratrol is an ITP null (second test) | Strong R et al. 2013, *J Gerontol A* 68(1):6–16 | [10.1093/gerona/gls070](https://doi.org/10.1093/gerona/gls070) | Abstract | Confirmed: resveratrol, green tea extract, curcumin, oxaloacetic acid and MCT oil were all null in both sexes (log-rank). Green tea extract had a secondary signal for midlife deaths in females only. |
| 7a | Rapamycin started late in life extends lifespan | Harrison DE et al. 2009, *Nature* 460:392–395 | [10.1038/nature08221](https://doi.org/10.1038/nature08221) | Abstract | Confirmed: started at 600 days; age at 90% mortality +14% in females and +9% in males. |
| 7b | Mouse clinical frailty index | Whitehead JC et al., *J Gerontol A* 69(6):621–632 (published online 2013, print 2014) | [10.1093/gerona/glt136](https://doi.org/10.1093/gerona/glt136) | Abstract | Confirmed. |

## (c) Corrections

1. **#5 is wrong, and the correction helps D122.** Strong 2016 includes a concurrent metformin-only arm (1000 ppm, UM-HET3) that is null. D122 can use it as a published ITP null arm, by sex, following that paper's tables. The metformin + rapamycin benefit reported in the same paper is measured against historical rapamycin-only data, not against a concurrent arm, so it must not be cited as a clean combination effect.
2. **#4 must not be generalised.** Martin-Montalvo's metformin benefit is in male C57BL/6 only. Together with point 1, metformin is a positive result in one strain and sex and a null result in UM-HET3. That conflict is useful as a test of the metric, but metformin cannot be used as a clean positive or a clean negative.
3. **#2 SLAM data access is unconfirmed.** The design paper describes the study but does not give a public download route. D121 should list SLAM as "access to request from NIA (de Cabo lab)" until a route is confirmed. ITP and the JAX Mouse Phenome Database data are reachable today.
4. **#3b limits D121.** Luciano 2024 found that frailty-based prediction has real signal but "fell short of the level of reliability" needed to replace lifespan. Frailty clocks can feed metric v0, but they cannot stand in for a lifespan endpoint when the metric is validated.
5. **#1 has a narrow population.** FRIGHT and AFRAID were trained on male C57BL/6 mice. Applying them to UM-HET3, DO or female mice is extrapolation and must be labelled as such.
6. **Year discrepancies.** Bogue and Whitehead appeared online one year before print. The years in R8 should follow the print issue: Bogue 2016, Whitehead 2014. The `year` column in `arcanum-r8.check.tsv` is Crossref's first-publication (online) year, so it can be one year earlier than the print year cited here (also true for Palliyaguru, Miller and Strong 2013).
