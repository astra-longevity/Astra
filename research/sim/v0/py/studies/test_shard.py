"""Tests for shard.py and shard_challenge.py: a shard reproduces the direct study run, gather rebuilds derivatives."""

from __future__ import annotations

import json
import os
import sys
import unittest

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import shard  # noqa: E402
import shard_challenge  # noqa: E402
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


class ChallengeShardTest(unittest.TestCase):
    """--study challenge cost grid: shards through JSON + gather == a direct run."""

    def setUp(self):
        self.save = (shard_challenge.challenge.KAPPAS, shard_challenge.challenge.T0S,
                     shard_challenge.challenge.DOSES, shard_challenge.challenge.organs)
        shard_challenge.challenge.KAPPAS = (None,)
        shard_challenge.challenge.T0S = (0.5, 1.5)
        shard_challenge.challenge.DOSES = (0.3,)
        shard_challenge.challenge.organs = lambda: ["A9_kidney"]

    def tearDown(self):
        (shard_challenge.challenge.KAPPAS, shard_challenge.challenge.T0S,
         shard_challenge.challenge.DOSES, shard_challenge.challenge.organs) = self.save

    def test_list_size(self):
        # per seed: 1 baseline + 2 t0 x 1 dose challenge, x 1 organ
        self.assertEqual(len(shard_challenge.challenge_specs()), len(shard_challenge.SEEDS) * 3)

    def test_bad_spec(self):
        for bad in ("b,11,0,A2_bone", "x,11,0,A9_kidney"):
            with self.assertRaises(ValueError):
                shard_challenge.challenge_simulate(bad, N)

    def test_gather_matches_direct(self):
        cache = {sp: json.loads(json.dumps(shard_challenge.challenge_simulate(sp, N), sort_keys=True))
                 for sp in shard_challenge.challenge_specs()}
        got = shard_challenge.challenge_gather(cache.__getitem__, N)["runs"][0]["per_organ"]["A9_kidney"]
        # direct: same baseline + challenge Fisher, same share sweep
        ch = shard_challenge.challenge
        names = ch.names_for("A9_kidney", False)
        base = [ch.derivs("baseline", "A9_kidney", names, N, s) for s in shard_challenge.SEEDS]
        fb = ch.fisher(base[0], base[1], 1.0)
        self.assertEqual(json.dumps(got["baseline"], sort_keys=True),
                         json.dumps(shard_challenge._share_sweep(fb, names), sort_keys=True))
        for ti, t0 in enumerate(ch.T0S):
            ch.set_grid(None, t0)
            ch.set_dose(ch.DOSES[0])
            chh = [ch.derivs("challenge", "A9_kidney", names, N, s) for s in shard_challenge.SEEDS]
            fc = ch.fisher(chh[0], chh[1], 1.0)
            want = shard_challenge._share_sweep(ch._add(fb, fc), names)
            self.assertEqual(json.dumps(got["challenge"][f"t{t0}_d{ch.DOSES[0]}"], sort_keys=True),
                             json.dumps(want, sort_keys=True))


if __name__ == "__main__":
    unittest.main()
