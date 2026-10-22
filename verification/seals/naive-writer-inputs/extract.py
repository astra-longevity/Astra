#!/usr/bin/env python3
"""Extract compound names for the lifespan-naive L3 mapping writer (A78).

A50 ruling (G140, 2026-10-22 12:34): from research/metric/v0/split.csv, keep only
rows with part=blind and stratum=b, read ONLY the compound column, split
combinations into single compounds, deduplicate, sort. No other column is
written. The script aborts unless split.csv has the pinned sha256.

Usage: python3 extract.py            # writes compounds.txt next to this file
       python3 extract.py --check    # exit 1 if compounds.txt differs
"""
import csv, hashlib, pathlib, re, sys

ROOT = pathlib.Path(__file__).resolve().parents[3]
SPLIT = ROOT / "research/metric/v0/split.csv"
PINNED = "87ef760cc5c36e07a920fc0e8f385cd130ac30eda5ccc56e1202380053f31a45"
OUT = pathlib.Path(__file__).with_name("compounds.txt")

data = SPLIT.read_bytes()
sha = hashlib.sha256(data).hexdigest()
if sha != PINNED:
    sys.exit(f"split.csv sha256 {sha} != pinned {PINNED}")

names = set()
for row in csv.DictReader(data.decode("utf-8").splitlines()):
    if row["part"] != "blind" or row["stratum"] != "b":
        continue
    for part in re.split(r"\s*(?:\+|/|\band\b)\s*", row["compound"]):
        if part.strip():
            names.add(part.strip())

text = "".join(f"{n}\n" for n in sorted(names, key=str.lower))
if "--check" in sys.argv:
    sys.exit(0 if OUT.exists() and OUT.read_text() == text else 1)
OUT.write_text(text)
print(f"{len(names)} compounds -> {OUT.name}, sha256 {hashlib.sha256(text.encode()).hexdigest()}")
