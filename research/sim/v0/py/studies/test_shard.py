"""Tests for shard.py: a shard reproduces the direct study run, gather rebuilds derivatives."""

from __future__ import annotations

import json
import os
import sys
import unittest

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import shard  # noqa: E402
import readouts  # noqa: E402
import longitudinal  # noqa: E402

N = 40
NAMES = ["eta.A1_muscle", "eps"]


class ShardTest(unittest.TestCase):
    def test_list_size(self):
        self.assertEqual(len(shard.specs()), len(shard.SEEDS) * (2 * len(shard.full_names()) + 1))

    def test_bad_spec(self):
        for bad in ("11,base,+", "11,eta.A1_muscle,0", "11,nope,+", "11,eps,x"):
            with self.assertRaises(ValueError):
                shard.simulate(bad, N)

    def test_readouts_identical_to_direct_run(self):
        s = shard.simulate("11,eta.A1_muscle,+", N)
        p, m = readouts.run({"eta.A1_muscle": readouts.math.exp(readouts.H)}, N, 11)
        got = shard._ro(shard._keys_to_num(json.loads(json.dumps(s))))
        self.assertEqual(got[0], p)
        self.assertEqual(got[1], m)

    def test_gather_matches_derive(self):
        cache = {sp: shard._keys_to_num(json.loads(json.dumps(shard.simulate(sp, N))))
                 for s in shard.SEEDS for sp in [f"{s},base,0"] + [f"{s},{nm},{g}" for nm in NAMES for g in "+-"]}
        orig = shard.full_names
        shard.full_names = lambda: list(NAMES)
        captured = {}
        ra, la = readouts.analyse, longitudinal.analyse
        readouts.analyse = lambda d1, d2, n, names: captured.setdefault("ro", (d1, d2))
        longitudinal.analyse = lambda d1, d2, n, names: captured.setdefault("lo", (d1, d2))
        try:
            shard.gather(cache.__getitem__, N)
        finally:
            shard.full_names, readouts.analyse, longitudinal.analyse = orig, ra, la
        d1 = readouts.derivatives(NAMES, N, 11)
        self.assertEqual(captured["ro"][0]["dp"], d1["dp"])
        self.assertEqual(captured["ro"][0]["dm"], d1["dm"])
        self.assertEqual(len(captured["lo"][1]["dd"]), len(NAMES))


if __name__ == "__main__":
    unittest.main()
