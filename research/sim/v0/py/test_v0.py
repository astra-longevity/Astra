"""Tests for R-Sim v0 skeleton and baseline. Run: python3 -m unittest -v (from this folder)."""

from __future__ import annotations

import math
import os
import random
import unittest

import baseline as bl
import model


def _gompertz_sample(a: float, b: float, n: int, seed: int, cap: float) -> bl.Arm:
    """Inverse-CDF draws from Gompertz(a, b), right-censored at `cap`."""
    rng = random.Random(seed)
    times, events = [], []
    for _ in range(n):
        t = math.log1p(-b / a * math.log(1 - rng.random())) / b
        times.append(min(t, cap))
        events.append(1 if t <= cap else 0)
    return bl.Arm(tuple(times), tuple(events), role="control")


class ModelTests(unittest.TestCase):
    def test_same_seed_same_lifespans(self):
        self.assertEqual(model.simulate(50, 7), model.simulate(50, 7))

    def test_other_seed_differs(self):
        self.assertNotEqual(model.simulate(50, 7), model.simulate(50, 8))

    def test_record_keeps_lifespans_and_counts_survivors(self):
        rec = {1.0: [], 2.0: []}
        ls = model.simulate(100, 7, record=rec)
        self.assertEqual(ls, model.simulate(100, 7))
        for age, states in rec.items():
            self.assertEqual(len(states), sum(1 for x in ls if x > age))

    def test_track_follows_same_mouse(self):
        tr = {1.0: {}, 2.0: {}}
        ls = model.simulate(100, 7, track=tr)
        self.assertEqual(ls, model.simulate(100, 7))
        for age, states in tr.items():
            self.assertEqual(set(states), {i for i, x in enumerate(ls) if x > age})
        self.assertTrue(set(tr[2.0]) <= set(tr[1.0]))

    def test_challenge_draws_no_random_numbers(self):
        base = model.simulate(100, 7)
        self.assertEqual(model.simulate(100, 7, challenge={"A9_kidney": (9.0, 1.0)}), base)
        a, b = {1.6: {}}, {1.6: {}}
        model.simulate(100, 7, track=a)
        model.simulate(100, 7, track=b, challenge={"A9_kidney": (1.5, 0.5)})
        common = set(a[1.6]) & set(b[1.6])
        self.assertTrue(common)
        # One RNG stream for all mice: once deaths differ, later mice draw differently,
        # so compare the cohort mean, not mouse by mouse.
        gain = sum(b[1.6][i]["A9_kidney"] - a[1.6][i]["A9_kidney"] for i in common) / len(common)
        self.assertGreater(gain, 0.25)

    def test_untreated_median_in_mouse_range(self):
        ls = sorted(model.simulate(200, 1))
        self.assertTrue(1.5 < ls[100] < 3.5, ls[100])

    def test_lower_damage_production_extends_life(self):
        pert = model.Perturbation({"eta.*": 0.7})
        base = sorted(model.simulate(200, 3))[100]
        treated = sorted(model.simulate(200, 3, perturbation=pert))[100]
        self.assertGreater(treated, base)

    def test_unknown_parameter_rejected(self):
        with self.assertRaises(KeyError):
            model.simulate(1, 1, perturbation=model.Perturbation({"eta.liverr": 0.5}))

    def test_parameter_count(self):
        self.assertEqual(model.Params().free_parameter_count(), 54)

    def test_axes_match_metric_v0(self):
        self.assertEqual(len(model.AXES), 17)
        self.assertEqual(len(model.HALLMARKS), 12)
        self.assertTrue(model.NO_READOUT <= set(model.AXES))
        self.assertTrue(model.VITAL <= set(model.AXES))
        self.assertTrue(model.HALLMARKS_NO_READOUT <= set(model.HALLMARKS))
        # 11 unmeasured in axes.md: 6 organ + 5 hallmark.
        self.assertEqual(len(model.NO_READOUT) + len(model.HALLMARKS_NO_READOUT), 11)

    def test_every_axis_has_a_production_driver(self):
        for a in model.AXES:
            self.assertTrue(model._drivers("production", a), a)

    def test_hallmark_multiplier_moves_survival(self):
        base = sorted(model.simulate(200, 4))[100]
        slower = model.Perturbation({"hallmark.H6_nutrient_sensing": 0.5})
        faster = model.Perturbation({"hallmark.H5_autophagy": 0.5})
        self.assertGreater(sorted(model.simulate(200, 4, perturbation=slower))[100], base)
        self.assertLess(sorted(model.simulate(200, 4, perturbation=faster))[100], base)

    def test_frailty_readout_bounded(self):
        x = dict.fromkeys(model.AXES, 100.0)
        self.assertEqual(model.frailty(x), 1.0)
        self.assertEqual(model.frailty(dict.fromkeys(model.AXES, 0.0)), 0.0)

    def test_mappings_parameters_known_to_model(self):
        path = os.path.join(os.path.dirname(__file__), "..", "..", "mappings.yaml")
        with open(path, encoding="utf-8") as f:
            keys = [
                line.split("parameter:", 1)[1].split("#")[0].strip()
                for line in f
                if line.strip().startswith("- parameter:")
            ]
        self.assertTrue(keys)
        for k in keys:
            model._apply(model.Params(), model.Perturbation({k: 1.0}))  # raises KeyError if unknown

    def test_mappings_superseded_no_values(self):
        """Draft rows are pointers to L3 M-00n only: no prior, effect or source may come back."""
        path = os.path.join(os.path.dirname(__file__), "..", "..", "mappings.yaml")
        moves, cur = [], None
        with open(path, encoding="utf-8") as f:
            for line in f:
                s = line.split("#")[0].strip()
                if s.startswith("- parameter:"):
                    cur = {}
                    moves.append(cur)
                elif cur is not None and ":" in s:
                    k, _, v = s.partition(":")
                    cur[k.strip()] = v.strip()
        self.assertTrue(moves)
        for m in moves:
            self.assertIn(m["term"], ("production", "clearance", "coupling"))
            self.assertRegex(m["superseded_by"], r"^M-\d{3}$")
            for k in ("prior_multiplier", "effect", "direction", "evidence", "why"):
                self.assertNotIn(k, m)
        with open(path, encoding="utf-8") as f:
            body = [line.split("#")[0] for line in f]
        for k in ("prior_multiplier", "evidence:", "doi:", "effect:"):
            self.assertFalse(any(k in line for line in body), k)

    def test_survival_curve_monotone(self):
        s = [v for _, v in model.survival_curve(model.simulate(100, 2))]
        self.assertEqual(s[0], 1.0)
        self.assertTrue(all(x >= y for x, y in zip(s, s[1:])))


class BaselineTests(unittest.TestCase):
    def test_gompertz_recovers_parameters(self):
        g = bl.fit_gompertz([_gompertz_sample(0.02, 2.0, 4000, 11, cap=4.0)])
        self.assertAlmostEqual(g.b, 2.0, delta=0.15)
        self.assertAlmostEqual(math.log(g.a), math.log(0.02), delta=0.25)

    def test_gompertz_refuses_treated_arms(self):
        arm = bl.Arm((1.0,), (1,), role="treated", cls="x")
        with self.assertRaises(ValueError):
            bl.fit_gompertz([arm])

    def test_hr_refuses_test_split(self):
        g = bl.Gompertz(0.02, 2.0)
        arm = bl.Arm((1.0,), (1,), role="treated", split="test", cls="x")
        with self.assertRaises(ValueError):
            bl.fit_class_hr(g, [arm])

    def test_hr_recovered_and_pooled_used_for_new_class(self):
        g = bl.Gompertz(0.02, 2.0)
        # HR 0.5 on hazard = Gompertz with a halved.
        s = _gompertz_sample(0.01, 2.0, 4000, 5, cap=6.0)
        arm = bl.Arm(s.times, s.events, role="treated", cls="mtor")
        hrs = bl.fit_class_hr(g, [arm])
        self.assertAlmostEqual(hrs["mtor"], 0.5, delta=0.05)
        self.assertEqual(bl.predict_hr(hrs, "new", "b"), hrs["__pooled__"])

    def test_sprt_outcomes(self):
        self.assertEqual(bl.sprt([True] * 20), "model better")
        self.assertEqual(bl.sprt([False] * 20), "not better")
        self.assertEqual(bl.sprt([True, False, True]), "undecided")


if __name__ == "__main__":
    unittest.main()
