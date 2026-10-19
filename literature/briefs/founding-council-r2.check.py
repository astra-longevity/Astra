#!/usr/bin/env python3
"""Working note for founding-council-r2.md: verify every DOI of the brief on Crossref.

Usage: python3 founding-council-r2.check.py  (reads DOIs from founding-council-r2.md,
or from the CANDIDATES list when the brief does not exist yet). Prints DOI, year,
first author, title. Unresolved DOIs are printed with 'FAIL'.
"""
import json
import re
import sys
import time
import urllib.request
from pathlib import Path

HERE = Path(__file__).parent
BRIEF = HERE / "founding-council-r2.md"
CANDIDATES = ""
DOI_RE = re.compile(r"10\.\d{4,9}/[^\s\(\)\]>\"]+(?:\([^\s\)]*\)[^\s\(\)\]>\"]*)*")


def dois():
    text = BRIEF.read_text() if BRIEF.exists() and "--candidates" not in sys.argv else CANDIDATES
    seen = []
    for doi in DOI_RE.findall(text):
        doi = doi.rstrip(".,;")
        if doi not in seen:
            seen.append(doi)
    return seen


def lookup(doi):
    url = "https://api.crossref.org/works/" + urllib.parse.quote(doi)
    req = urllib.request.Request(url, headers={"User-Agent": "astra-brief/0.1 (mailto:astra@example.org)"})
    for attempt in range(4):
        try:
            with urllib.request.urlopen(req, timeout=20) as resp:
                msg = json.load(resp)["message"]
            year = (msg.get("issued", {}).get("date-parts") or [[None]])[0][0]
            author = (msg.get("author") or [{}])[0].get("family", "?")
            title = " ".join((msg.get("title") or ["?"])[0].split())[:90]
            journal = (msg.get("container-title") or ["?"])[0][:40]
            return f"OK\t{doi}\t{year}\t{author}\t{journal}\t{title}"
        except urllib.error.HTTPError as err:
            if err.code == 404:
                return f"FAIL\t{doi}\t404"
            time.sleep(2 + attempt * 3)
        except Exception as err:  # network hiccup
            time.sleep(2 + attempt * 3)
    return f"FAIL\t{doi}\tnetwork"


import urllib.parse  # noqa: E402

if __name__ == "__main__":
    for d in dois():
        print(lookup(d), flush=True)
