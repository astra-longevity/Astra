"""Tests for R-Sim v0 skeleton and baseline. Run: python3 -m unittest -v (from this folder)."""

from __future__ import annotations

import math
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
        self.assertEqual(model.Params().free_parameter_count(), 24)

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
