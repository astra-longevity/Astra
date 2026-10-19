# Coloc ABF reference case (hand-computed)

Reference for `coloc.abf`-style colocalisation (Giambartolomei et al. 2014, doi:10.1371/journal.pgen.1004383), using Wakefield's approximate Bayes factor (Wakefield 2009, doi:10.1002/gepi.20359). Both DOIs checked on Crossref.

This is a **hand-computed case**: three tiny 5-SNP, two-trait inputs, with every intermediate value shown so a person can redo it with a calculator. It does not come from any package vignette.

Files: `inputs.csv` (beta, se, prior sd per SNP and trait), `expected.csv` (posterior probabilities to match).

## Formulas

Per SNP and trait, with prior effect sd `w` (0.15 for a quantitative trait on the sdY = 1 scale, 0.2 for a case-control log odds ratio):

- `V = se²`, `z = beta / se`, `W = w²`, `r = W / (V + W)`
- `lABF = ½ · (ln(1 − r) + r · z²)`

Over the region, with `lse(x) = ln Σ exp(x)`:

- `S1 = lse(lABF1)`, `S2 = lse(lABF2)`, `S12 = lse(lABF1 + lABF2)`
- `lH0 = 0`
- `lH1 = ln p1 + S1`, `lH2 = ln p2 + S2`
- `lH3 = ln p1 + ln p2 + ln(exp(S1 + S2) − exp(S12))`
- `lH4 = ln p12 + S12`
- `PP.Hk = exp(lHk − lse(lH0..lH4))`
- `SNP.PP.H4_i = exp(lABF1_i + lABF2_i − S12)`

Priors: `p1 = p2 = 1e-4`, `p12 = 1e-5`. Log values use the natural log.

## Case A: shared signal at SNP3 (w = 0.15 for both traits)

| SNP | V1 | z1 | r1 | lABF1 | V2 | z2 | r2 | lABF2 | lABF1+lABF2 |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 0.0025 | 0.4 | 0.9 | −1.07929254650 | 0.0036 | 0.166666666667 | 0.862068965517 | −0.978527554357 | −2.05782010085 |
| 2 | 0.0025 | 1 | 0.9 | −0.701292546497 | 0.0036 | 0.666666666667 | 0.862068965517 | −0.798929853207 | −1.50022239970 |
| 3 | 0.0025 | 6 | 0.9 | 15.0487074535 | 0.0036 | 4.16666666667 | 0.862068965517 | 6.49273681346 | 21.5414442670 |
| 4 | 0.0025 | 2 | 0.9 | 0.648707453503 | 0.0036 | 1.33333333333 | 0.862068965517 | −0.224217209529 | 0.424490243974 |
| 5 | 0.0025 | 0.2 | 0.9 | −1.13329254650 | 0.0036 | 0.5 | 0.862068965517 | −0.882742113744 | −2.01603466024 |

S1 = 15.0487083482, S2 = 6.49581910378, S12 = 21.5414442679, ln(exp(S1+S2) − exp(S12)) = 15.7611938519.

lH0..lH4 = 0, 5.83836797624, −2.71452126820, −2.65948689205, 10.0285188029; log total = 10.0435986627.

| PP.H0 | PP.H1 | PP.H2 | PP.H3 | PP.H4 |
|---|---|---|---|---|
| 4.34630823122e-5 | 0.0149173444430 | 2.87884918884e-6 | 3.04172565255e-6 | 0.985033271900 |

SNP.PP.H4 = 5.635983431e-11, 9.843086744e-11, 0.9999999991, 6.745643614e-10, 5.876475015e-11.

## Case B: distinct signals, trait 1 at SNP2, trait 2 at SNP4 (w = 0.15)

| SNP | z1 | lABF1 | z2 | lABF2 | lABF1+lABF2 |
|---|---|---|---|---|---|
| 1 | 0.4 | −1.07929254650 | 0.166666666667 | −0.978527554357 | −2.05782010085 |
| 2 | 6 | 15.0487074535 | 0.333333333333 | −0.942608014127 | 14.1060994394 |
| 3 | 1 | −0.701292546497 | 0.833333333333 | −0.691171232518 | −1.39246377901 |
| 4 | 0.2 | −1.13329254650 | 4.16666666667 | 6.49273681346 | 5.35944426696 |
| 5 | 0 | −1.15129254650 | 0.5 | −0.882742113744 | −2.03403466024 |

V and r as in case A. S1 = 15.0487078830, S2 = 6.49527793933, S12 = 14.1062587981, ln(exp(S1+S2) − exp(S12)) = 21.5433970274.

lH0..lH4 = 0, 5.83836751098, −2.71506243264, 3.12271628343, 2.59333333309; log total = 5.94113667835.

| PP.H0 | PP.H1 | PP.H2 | PP.H3 | PP.H4 |
|---|---|---|---|---|
| 0.00262903957958 | 0.902335237862 | 0.000174044575418 | 0.0597001709611 | 0.0351615070218 |

SNP.PP.H4 = 9.550579612e-8, 0.9998406540, 1.857762997e-7, 0.0001589669036, 9.780467511e-8.

Case B is a useful trap: H3 is the true answer, but trait 2's signal is weak relative to the priors, so the result goes to H1. An implementation that returns high PP.H3 here is wrong.

## Case C: case A inputs, trait 1 case-control (w = 0.2), trait 2 quantitative (w = 0.15)

Trait 1: V1 = 0.0025, r1 = 0.941176470588; lABF1 = −1.34131255438, −0.946018436734, 15.5245697986, 0.465746269148, −1.39778314262. Trait 2 as case A.
Sums: −2.31984010874, −1.74494828994, 22.0173066120, 0.241529059619, −2.28052525636.

S1 = 15.5245702494, S2 = 6.49581910378, S12 = 22.0173066125, ln(exp(S1+S2) − exp(S12)) = 16.2369121286.

lH0..lH4 = 0, 6.31422987739, −2.71452126820, −2.18376861535, 10.5043811475; log total = 10.5194434527.

| PP.H0 | PP.H1 | PP.H2 | PP.H3 | PP.H4 |
|---|---|---|---|---|
| 2.70062171763e-5 | 0.0149175996981 | 1.78880149027e-6 | 3.04134085816e-6 | 0.985050563942 |

SNP.PP.H4 = 2.694706523e-11, 4.788327189e-11, 0.9999999995, 3.490599078e-10, 2.802758621e-11.

## How the numbers were checked

Computed twice by a throwaway script that read no data: once in 50-digit decimal arithmetic with log-sum-exp, once in plain double precision by summing the Bayes factors directly. The largest difference across all PPs was 2.4e-16. Values above are rounded to 12 significant digits.

## Tolerance

- PP.H0–H4 and SNP.PP.H4: absolute difference ≤ 1e-9.
- lABF, S1, S2, S12: absolute difference ≤ 1e-8.

These limits are far above double-precision error (~1e-16) and far below any difference that would change a conclusion. An implementation that uses `p12` as a prior on "H4 per SNP" (as coloc does) and these exact `w` values should match. Callers must pass `w` explicitly: coloc derives `w` from `sdY` for quantitative traits, so with `sdY ≠ 1` its results will differ.
