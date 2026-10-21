#!/usr/bin/env python3
"""N1 (A66 record #233): exact PubMed queries for the M-001 / M-002 searches.

The v2 rows gave 226 / 281 hits and 39 / 203 with lifespan terms excluded.
The exclusion string behind 39 / 203 was NOT recorded, so those two counts are
not reproducible. This file is now the record: the exact strings below, run
through NCBI E-utilities esearch (rettype=count), with the UTC run time and
PubMed's own query translation written to pubmed-counts.json.

Counts drift as PubMed grows; re-run and compare with the stored run time.

    python3 literature/refutation/v3-inputs/pubmed_counts.py
"""
import datetime
import json
import os
import time
import urllib.parse
import urllib.request

HERE = os.path.dirname(os.path.abspath(__file__))
EUTILS = "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/esearch.fcgi"

BASE = {
    "M-001_pS6": 'rapamycin AND (pS6 OR S6K1 OR "S6 phosphorylation") AND mice AND tissue',
    "M-002_autophagy": 'rapamycin AND (LC3-II OR autophagic flux) AND mice AND (liver OR muscle OR kidney)',
}
EXCLUDE = ' NOT (lifespan OR "life span" OR survival OR longevity OR mortality)'


def count(term):
    q = urllib.parse.urlencode({"db": "pubmed", "term": term, "retmode": "json", "retmax": 0})
    with urllib.request.urlopen(f"{EUTILS}?{q}", timeout=30) as r:
        res = json.load(r)["esearchresult"]
    time.sleep(0.4)  # NCBI: <= 3 requests/s without a key
    return int(res["count"]), res.get("querytranslation", "")


def main():
    run = datetime.datetime.now(datetime.timezone.utc).replace(microsecond=0).isoformat()
    rows = []
    for key, q in BASE.items():
        for label, term in (("all", q), ("lifespan_terms_excluded", q + EXCLUDE)):
            n, tr = count(term)
            rows.append({"search": key, "variant": label, "query": term, "count": n,
                         "pubmed_translation": tr})
    out = {"run_utc": run, "source": "NCBI E-utilities esearch, db=pubmed", "results": rows,
           "note": "v2 counts 39 / 203 (exclusion variant) used an unrecorded string; superseded by this file."}
    json.dump(out, open(os.path.join(HERE, "pubmed-counts.json"), "w"), indent=1)
    for r in rows:
        print(f"{r['search']:18} {r['variant']:24} {r['count']}")


if __name__ == "__main__":
    main()
