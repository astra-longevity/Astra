# Metric dossier (L2)

Owner: A2 (Literature). Mission L2, charter `docs/charter/charter.md` part 6; specs T1-A2-1, T2-A2-3, T3-A2-1, T3-A2-3 in `arcanum/session-1/topics/`.

What this folder is: one file per input of Research's biological-age measure v0 (R-Metric v0), per human function test and per clock. It tells anyone building or reading the measure how much each input can be trusted and where it stops being valid.

## Every file has the same sections

1. **What it measures**, in plain English.
2. **Training or reference population**: who the tool was built or normed on (age, sex, ancestry, tissue, species/strain). A reading from someone outside that range is flagged "outside training range", not reported plainly (T3-A2-3).
3. **Noise**: published test-retest reliability and the smallest real change, with source. Below that change, a difference between two readings is noise.
4. **Known failure modes**: when it gives wrong answers.
5. **Functional anchor and age trajectory**: which real-world function it tracks and how it changes with age in published data.
6. **Confounders**: what moves it without ageing.
7. **Sources**: every DOI checked on Crossref (`dossier.check.tsv`). Anything not yet checked, or any number not yet read in the primary paper, is marked `[to verify]`.

No number appears here without a source. Where secondary sources disagree, both values are given and the item stays `[to verify]` until the primary paper is read.

## Files

| File | Input |
| --- | --- |
| `clocks/01-horvath.md` | Horvath 2013 multi-tissue clock |
| `clocks/02-hannum.md` | Hannum 2013 blood clock |
| `clocks/03-phenoage.md` | DNAm PhenoAge (Levine 2018) |
| `clocks/04-grimage.md` | DNAm GrimAge (Lu 2019) |
| `clocks/05-dunedinpace.md` | DunedinPACE (Belsky 2022) |
| `clocks/06-mouse-clocks.md` | Mouse methylation clocks (Petkovich, Stubbs, Meer, Thompson) |
| `tests/01-grip.md` … `tests/06-audiogram.md` | Human function tests: grip, gait speed, chair rise, VO2max, cognitive (NIH Toolbox), audiogram |
| `models/` | Parent models: Karin 2019 SR model, Farrell–Rutenberg network models |
| `mouse/` | Mouse inputs (frailty index, grip, rotarod, treadmill, echo, GTT, body composition, blood count), in progress |
| `anchors.md` | One row per axis: anchor test, age trajectory, confounders; no anchor = named blind spot (due 2026-10-27) |
