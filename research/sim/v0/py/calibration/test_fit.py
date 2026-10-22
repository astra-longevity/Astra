"""Tests for calibration/fit.py. Synthetic inputs only; no real survival data.

    python3 test_fit.py
"""

from __future__ import annotations

import math
import os
import sys
import unittest

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import fit as F  # noqa: E402
from fit import baseline, model  # noqa: E402


def arm(times, events=None, role="control", split="train"):
    return baseline.Arm(tuple(times), tuple(events or [1] * len(times)), role, split)


class Guard(unittest.TestCase):
    def test_select_controls_filters(self):
        arms = [arm([1.0]), arm([1.0], role="treated"), arm([1.0], split="test"), arm([1.0])]
        kept, rej = F.select_controls(arms, diets={3: "CR"})
        self.assertEqual(len(kept), 1)
        self.assertEqual([i for i, _ in rej], [1, 2, 3])
        self.assertIn("diet CR", rej[2][1])

    def test_fit_and_null_refuse_bad_arms(self):
        for bad in (arm([1.0], role="treated"), arm([1.0], split="test")):
            with self.assertRaises(ValueError):
                F.fit([bad], n_sim=10)
            with self.assertRaises(ValueError):
                F.gompertz_loglik([bad])
        with self.assertRaises(ValueError):
            F.fit([], n_sim=10)


class Likelihood(unittest.TestCase):
    def test_hand_computed(self):
        p = [0.0] * F.N_BINS
        p[0], p[1], p[-1] = 0.25, 0.25, 0.5
        # death in month 1 -> log .25; censored in month 0 -> log P(alive after month 0) = log .75
        ll = F.binned_loglik([1.5 / 12, 0.5 / 12], [1, 0], p)
        self.assertAlmostEqual(ll, math.log(0.25) + math.log(0.75))

    def test_sim_bins_normalised_and_smoothed(self):
        p = F.sim_bins(0.0, 0.0, n_sim=50, seed=1)
        self.assertAlmostEqual(sum(p), 1.0)
        self.assertTrue(all(x > 0 for x in p))

    def test_only_vital_axes_move(self):
        # u = 0 must equal the untreated default simulation exactly
        life = model.simulate(50, 3, t_max=F.T_MAX)
        counts = [0] * F.N_BINS
        for x in life:
            counts[F._month(x)] += 1
        tot = 50 + 0.5 * F.N_BINS
        self.assertEqual(F.sim_bins(0.0, 0.0, 50, 3), [(c + 0.5) / tot for c in counts])

    def test_gompertz_bins_sum_to_one(self):
        p = F.gompertz_bins(baseline.Gompertz(0.01, 2.0))
        self.assertAlmostEqual(sum(p), 1.0, places=9)


class GoodnessOfFit(unittest.TestCase):
    def test_chi2_sf_known_values(self):
        self.assertAlmostEqual(F.chi2_sf(3.841458820694124, 1), 0.05, places=6)
        self.assertAlmostEqual(F.chi2_sf(18.307038053275146, 10), 0.05, places=6)
        self.assertAlmostEqual(F.chi2_sf(2.0, 2), math.exp(-1), places=9)

    def test_gof_true_model_not_rejected(self):
        g = baseline.Gompertz(0.02, 1.5)
        p = F.gompertz_bins(g)
        import random
        rng = random.Random(5)
        times = []
        for _ in range(2000):  # inverse-CDF draws from the same Gompertz
            u = rng.random()
            times.append(math.log1p(-math.log(u) * g.b / g.a) / g.b)
        times = [min(t, F.T_MAX - 1e-9) for t in times]
        r = F.chi2_gof(times, [1] * len(times), p)
        self.assertGreater(r["p"], 0.01)
        self.assertGreater(r["cells"], 5)

    def test_gof_wrong_model_rejected(self):
        p_true = F.gompertz_bins(baseline.Gompertz(0.02, 1.5))
        p_wrong = F.gompertz_bins(baseline.Gompertz(0.2, 1.5))
        # expected deaths laid out exactly as the true model
        times = []
        for k, x in enumerate(p_true[:-1]):
            times += [(k + 0.5) / 12] * int(round(x * 3000))
        r = F.chi2_gof(times, [1] * len(times), p_wrong)
        self.assertLess(r["p"], 1e-6)


class Verdict(unittest.TestCase):
    def test_rules(self):
        ok = {"convergence": {"converged": True}}
        self.assertTrue(F.verdict(ok, {"p": 0.5}, -1.9)["pass"])
        self.assertFalse(F.verdict(ok, {"p": 0.5}, -2.1)["pass"])
        self.assertFalse(F.verdict(ok, {"p": 0.005}, 3.0)["pass"])
        self.assertFalse(F.verdict({"convergence": {"converged": False}}, {"p": 0.5}, 3.0)["pass"])


class RecoverySmoke(unittest.TestCase):
    """Small and loose: the pre-registered grid (n 1000/300, n_sim 4000) runs in recovery.py."""

    def test_recovers_shifted_truth(self):
        truth = (math.log(1.2), math.log(0.9))
        data = F.sim_bins  # noqa: F841 (documentation: data come from the same simulator)
        life = model.simulate(600, 101, perturbation=model.Perturbation(
            {**{f"eta.{a}": math.exp(truth[0]) for a in model.VITAL},
             **{f"xc.{a}": math.exp(truth[1]) for a in model.VITAL}}), t_max=F.T_MAX)
        r = F.fit([arm(life)], n_sim=600, stability_seeds=(), max_iter=40)
        est = r["estimate"]
        self.assertLess(abs(est["u_eta"] - truth[0]), 0.3)
        self.assertLess(abs(est["u_xc"] - truth[1]), 0.3)


class BootstrapSplit(unittest.TestCase):
    """recovery.py execution splits: synthetic JSON shapes only, no fit."""

    def test_shard_list_covers_every_rep_once(self):
        import recovery as R
        specs = R.bootstrap_shards()
        self.assertEqual(len(specs), len(R.cells()) * R.BOOT_REPS // R.BOOT_SHARD)
        self.assertEqual(len(set(specs)), len(specs))

    def test_resample_deterministic(self):
        import recovery as R
        a = arm([0.1 * i for i in range(1, 50)])
        self.assertEqual(R.resample(a, 3, 101), R.resample(a, 3, 101))
        self.assertNotEqual(R.resample(a, 3, 101), R.resample(a, 4, 101))

    def test_coverage_rule_two_of_three(self):
        import recovery as R
        t = R.TRUTHS["null"]

        def shard(seed, centre):
            reps = [{"rep": i, "u_eta": centre + (i - 100) * 1e-3, "u_xc": t[1] + (i - 100) * 1e-3}
                    for i in range(R.BOOT_REPS)]
            return {"cell": f"null,1000,none,{seed}", "first": 0, "reps": reps}

        ok = R.bootstrap_coverage([shard(101, 0.0), shard(102, 0.0), shard(103, 0.5)])
        self.assertTrue(ok["groups_ok"]["null,1000,none"])
        bad = R.bootstrap_coverage([shard(101, 0.0), shard(102, 0.5), shard(103, 0.5)])
        self.assertFalse(bad["groups_ok"]["null,1000,none"])


if __name__ == "__main__":
    unittest.main()
