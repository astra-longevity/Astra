# Pre-registration: can a challenge design bound kappa? (no result yet)

Committed before any number is computed (A28 ruling, G173, 2026-10-21). The
result goes in `identifiability.md` §11 (obstacle I22).

**Why.** Kappa (repair half-saturation) is a prior, not identified. §11 shows
eta/beta split only if kappa < 8. Local Fisher shows no tested design pins kappa
(share ≤ 0.29). So the question is model comparison: can the data tell the
3 grid values apart?

**Hypotheses.** kappa ∈ {0.5 (prior), 2, 8}, as on the challenge grid
(run 39594585409).

**Data.** Repeatable readouts only: A6 heart, A9 kidney, A12 blood, A14 gut.
Per design (start ages ⊂ {6, 18 mo} × doses ⊂ {0.15, 0.3, 0.6} Xc, as in §11):
the baseline cohort plus each (age, dose) cohort; readout mean and variance at
each time point. Assay noise 0.5 × bio SD. Per sex.

**Test.** Likelihood ratio between the true kappa and the wrong one, with the
organ's other parameters (eta, beta, Xc, eps, c) refitted under the wrong kappa.
Computed in the linear (Gauss-Newton) approximation: take the difference in
readout moments between the two kappas, project out the directions the
nuisance derivatives at the truth can absorb, weight it like
`challenge.fisher` (mean n/v, variance n/2v²). Noncentrality λ = n × that sum.
Power ≥ 0.8 for χ²(1) at α 0.05 ⇔ λ ≥ 7.85.

**Criterion.** A design *bounds kappa* if both hold with power ≥ 0.8:
- truth 8: rejects kappa 0.5 and kappa 2;
- truth 0.5 and truth 2: each rejects kappa 8.

Evaluated per organ, and for the 4 organs pooled (λ summed; the organs are
treated as independent). Smallest n per time point = ⌈7.85 / λ per mouse⌉;
mice per sex = n × (1 + cohorts), as in §11. Search capped at 700 mice per sex.
Report the smallest design that meets the criterion, or say plainly that none
does up to 700.

**Known limits, stated now.** Linear approximation of a non-linear refit
(it can overstate or understate λ); the 6 times of one mouse are treated as
independent (I19, so counts are a lower bound); ideal readout (I20); only
3 kappa values.
