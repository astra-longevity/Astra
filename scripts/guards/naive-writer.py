#!/usr/bin/env python3
"""Naive-writer exposure check (V-001, describe-not-quote).

Rule text: verification/procedures/rulings/V-001-describe-not-quote.md. In any
file a lifespan-naive writer may read, no lifespan, survival or mortality
OUTCOME (a value or a claim, for any intervention) may be quoted. Mechanistic
values are not covered.

The files come from .github/checks/naive-writer-paths.yaml (`paths:`, globs
allowed). Each line of each listed file, and each pair of adjacent non-blank
lines (hard-wrapped prose), is matched against the patterns below; a hit fails
the check unless its text is in `exempt:` (sha256 as printed with the hit,
plus the ruling and reason that keep it; each exempted line is printed).
Files not on the list are never read.

This is a pattern match: convention, not proof. It misses paraphrases and can
flag a description that uses outcome words with a number.

Usage: naive-writer.py [--root DIR] [--list FILE]
Exit: 0 no hit, 1 hit(s), 2 bad list.
"""

from __future__ import annotations

import argparse
import glob
import hashlib
import os
import re
import sys

OUTCOME = r"(?:life[\s-]?span|longevity|survival|mortality|death\s+rate|life\s+expectancy)"
NUM = r"\d+(?:[.,]\d+)?"
NEAR = r".{0,60}?"

PATTERNS: list[tuple[str, re.Pattern[str]]] = [
    # a percentage next to an outcome word, either order
    ("value: % with outcome", re.compile(
        rf"{NUM}\s*%{NEAR}\b{OUTCOME}|\b{OUTCOME}{NEAR}{NUM}\s*%", re.I)),
    # median / mean / maximum lifespan or survival with a number nearby
    ("value: median/mean/max outcome", re.compile(
        rf"\b(?:median|mean|average|maximum|max)\s+{OUTCOME}{NEAR}\b{NUM}"
        rf"|\b{NUM}\b{NEAR}\b(?:median|mean|average|maximum|max)\s+{OUTCOME}", re.I)),
    # hazard ratio next to survival / mortality / death, with a number
    ("value: hazard ratio", re.compile(
        rf"(?:\bHR\b|hazard\s+ratio){NEAR}{NUM}{NEAR}\b(?:{OUTCOME}|death)"
        rf"|\b(?:{OUTCOME}|death){NEAR}(?:\bHR\b|hazard\s+ratio){NEAR}{NUM}", re.I)),
    # an outcome word with a number of days / weeks / months / years
    ("value: outcome duration", re.compile(
        rf"\b{OUTCOME}{NEAR}\b{NUM}\s*(?:days?|d|weeks?|wk|months?|mo|years?|yr)\b", re.I)),
    # claims: X extends / increases / shortens lifespan
    ("claim: verb + outcome", re.compile(
        r"\b(?:extend|increas|prolong|lengthen|shorten|reduc|improv|decreas|lower|rais|boost)"
        rf"(?:e|es|ed|ing|s)?\b(?:\s+\w+){{0,3}}?\s+(?:the\s+)?{OUTCOME}", re.I)),
    ("claim: outcome + effect noun", re.compile(
        rf"\b{OUTCOME}[\s-]+(?:extension|extending|benefit|gain|increase|reduction|shortening)\b",
        re.I)),
]


def parse_list(path: str) -> tuple[list[dict], list[dict]]:
    """Fixed format: top-level `paths:` / `exempt:`, items `  - key: value`,
    continued by `    key: value`. Comments (#) and blank lines ignored."""
    sections: dict[str, list[dict]] = {"paths": [], "exempt": []}
    current: list[dict] | None = None
    item: dict | None = None
    with open(path, encoding="utf-8") as fh:
        for n, raw in enumerate(fh, 1):
            line = raw.split(" #", 1)[0].rstrip() if not raw.lstrip().startswith("#") else ""
            if not line.strip():
                continue
            if not line.startswith(" "):
                key = line.rstrip(":").strip()
                if key not in sections or not line.endswith(":"):
                    raise ValueError(f"{path}:{n}: unknown section {line!r}")
                current, item = sections[key], None
                continue
            m = re.fullmatch(r"\s*(-\s+)?([a-z_0-9]+):\s*(.*)", line)
            if current is None or not m:
                raise ValueError(f"{path}:{n}: cannot parse {line!r}")
            if m.group(1):
                item = {}
                current.append(item)
            if item is None:
                raise ValueError(f"{path}:{n}: key outside an item")
            item[m.group(2)] = m.group(3).strip().strip('"')
    for i, p in enumerate(sections["paths"]):
        if not p.get("path") or not p.get("source"):
            raise ValueError(f"{path}: paths[{i}] needs path and source")
    for i, e in enumerate(sections["exempt"]):
        if not (e.get("path") and re.fullmatch(r"[0-9a-f]{64}", e.get("line_sha256", ""))
                and e.get("ruling") and e.get("reason")):
            raise ValueError(
                f"{path}: exempt[{i}] needs path, line_sha256 (64 hex), ruling, reason")
    return sections["paths"], sections["exempt"]


def line_hash(text: str) -> str:
    return hashlib.sha256(text.strip().encode("utf-8")).hexdigest()


def scan_line(text: str) -> list[str]:
    return [kind for kind, pat in PATTERNS if pat.search(text)]


def scan_file(lines: list[str]) -> list[tuple[int, str, str, list[str]]]:
    """Hits as (line, location, text, kinds). A claim wrapped over two lines
    (prose hard-wrapped at ~80 columns) is caught on the pair, reported as
    `n-n+1`, only when neither line hits alone; its exempt hash is of the two
    stripped lines joined by one space."""
    out = []
    alone = [scan_line(t) for t in lines]
    for i, text in enumerate(lines):
        if alone[i]:
            out.append((i + 1, str(i + 1), text, alone[i]))
    for i in range(len(lines) - 1):
        if alone[i] or alone[i + 1] or not lines[i].strip() or not lines[i + 1].strip():
            continue
        pair = f"{lines[i].strip()} {lines[i + 1].strip()}"
        kinds = scan_line(pair)
        if kinds:
            out.append((i + 1, f"{i + 1}-{i + 2}", pair, kinds))
    return sorted(out, key=lambda h: h[0])


def main(argv: list[str]) -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--root", default=".")
    ap.add_argument("--list", default=".github/checks/naive-writer-paths.yaml")
    args = ap.parse_args(argv)
    root = os.path.abspath(args.root)
    list_path = args.list if os.path.isabs(args.list) else os.path.join(root, args.list)
    try:
        entries, exempt = parse_list(list_path)
    except (OSError, ValueError) as err:
        print(f"naive-writer: {err}", file=sys.stderr)
        return 2

    exempt_keys = {(e["path"], e["line_sha256"]): e for e in exempt}
    used: set[tuple[str, str]] = set()
    files: list[str] = []
    for e in entries:
        found = sorted(glob.glob(os.path.join(root, e["path"]), recursive=True))
        found = [f for f in found if os.path.isfile(f)]
        if not found:
            print(f"naive-writer: warning: no file matches {e['path']}")
        files += [os.path.relpath(f, root) for f in found if os.path.relpath(f, root) not in files]

    hits = 0
    for rel in files:
        with open(os.path.join(root, rel), encoding="utf-8", errors="replace") as fh:
            lines = fh.read().splitlines()
        for n, where, text, kinds in scan_file(lines):
            key = (rel, line_hash(text))
            if key in exempt_keys:
                used.add(key)
                ex = exempt_keys[key]
                print(f"{rel}:{where}: exempt, kept as record ({ex['ruling']}: {ex['reason']})")
                continue
            hits += 1
            snippet = text.strip()
            snippet = snippet if len(snippet) <= 200 else snippet[:197] + "..."
            print(f"{rel}:{where}: {'; '.join(kinds)} [sha256 {key[1]}]\n    {snippet}")

    for e in exempt:
        if (e["path"], e["line_sha256"]) not in used:
            print(f"naive-writer: warning: exempt line not found in {e['path']} "
                  f"({e['ruling']}: {e['reason']})")
    print(f"naive-writer (V-001): {len(files)} listed files, {hits} hit(s), "
          f"{len(used)} exempt line(s) kept as record.")
    print("convention, not proof (pattern match)")
    return 1 if hits else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
