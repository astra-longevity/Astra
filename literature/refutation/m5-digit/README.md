# M5 figure digitisation — provenance

Source image: NPJ Regen Med 2026, doi:10.1038/s41536-026-00490-x, Figure 1
(mechanistic panels only; no survival/lifespan panel opened).

The raw image is NOT committed to the repo (publisher figure, outside the
repo per the download policy). Its hash pins the exact image used:

    sha256(fig1.png) = 696a7e0a2b7b6187f9f2de10789cb599f1f89a2281538283722f5df6e37413f6

`digitise.py` reads `coords.json`, verifies this hash against the local image,
and converts label-matched tick + bar-top + cap pixel coordinates to values
with a delta-method ratio interval. In this run every panel is `no_mapping`
(marker-level direction only; H8 parameter sign undefined): see `coords.json` method_note. A66's independent reading is
in verification (#211 @fbca6c4) and is not read by the writer.
