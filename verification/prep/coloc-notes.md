# Coloc reference: open notes

Search result (Brave, 2 queries, 2026-10-19): no peer-reviewed paper or supplement was found that prints full per-SNP inputs (beta, se, MAF, N) together with the resulting PP.H0–H4 for a region. Giambartolomei 2014 (doi:10.1371/journal.pgen.1004383) and Wallace 2020 (doi:10.1371/journal.pgen.1008720, Crossref OK) print only summaries and figures. Package vignettes and the pure-Python port (pycoloc) print numbers but are smoke tests only under D6. So `references/coloc/` holds a hand-computed case instead.

Open items:

- [to verify] Run `coloc::coloc.abf` (R, Actions only) on `references/coloc/inputs.csv` with `sdY = 1` and check it reproduces `expected.csv` within tolerance. If it matches, record the coloc version; this would count as a second, different implementation.
- [to verify] Case C: confirm coloc's case-control prior sd is 0.2 in the current release (it was 0.2 in Giambartolomei 2014).
- [to verify] A deeper search of supplements (e.g. GTEx or eQTLGen coloc papers) could still find a printed full case. Not done: it would need reading omics data files, which D67 forbids.
