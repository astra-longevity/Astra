"""Tests for pool_panels.py (NA4.2). Made-up inputs only; no panel value of any mapping.

    python3 test_pool_panels.py
"""

from __future__ import annotations

import math
import os
import sys
import unittest

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from pool_panels import Z95, carry, pool, se_from_interval  # noqa: E402


class Pool(unittest.TestCase):
    def test_empty_is_no_mapping(self):
        self.assertEqual(pool([])["status"], "no_mapping")
        self.assertEqual(carry(pool([]))["status"], "no_mapping")

    def test_single_panel_passthrough(self):
        p = pool([(math.log(0.5), 0.2)])
        self.assertEqual(p["status"], "single_panel")
        self.assertAlmostEqual(p["lo"], math.log(0.5) - Z95 * 0.2)

    def test_identical_panels_no_heterogeneity(self):
        p = pool([(math.log(0.4), 0.1)] * 3)
        self.assertAlmostEqual(p["log_r"], math.log(0.4))
        self.assertEqual(p["tau2"], 0.0)
        self.assertEqual(p["i2"], 0.0)
        self.assertAlmostEqual(p["se"], 0.1 / math.sqrt(3))

    def test_dl_hand_computed(self):
        # y = 0, 1; se = 1, 1 -> FE mean 0.5, Q = 0.5, C = 1, tau2 = 0 (Q < k-1)
        p = pool([(0.0, 1.0), (1.0, 1.0)])
        self.assertAlmostEqual(p["log_r"], 0.5)
        self.assertEqual(p["tau2"], 0.0)
        # y = 0, 4; se = 1, 1 -> Q = 8, C = 1, tau2 = 7, I2 = 7/8
        p = pool([(0.0, 1.0), (4.0, 1.0)])
        self.assertAlmostEqual(p["tau2"], 7.0)
        self.assertAlmostEqual(p["i2"], 0.875)
        self.assertAlmostEqual(p["se"], math.sqrt(1 / (2 / 8)))

    def test_k_below_3_takes_wider_interval(self):
        p = pool([(0.0, 1.0), (4.0, 1.0)])  # RE wider here
        self.assertEqual((p["lo"], p["hi"]), p["re"])
        p = pool([(0.0, 0.1), (0.05, 0.3)])  # tau2 = 0: FE = RE, same width
        self.assertGreaterEqual(p["hi"] - p["lo"], p["fe"][1] - p["fe"][0] - 1e-12)

    def test_k3_uses_random_effects(self):
        p = pool([(0.0, 1.0), (4.0, 1.0), (2.0, 1.0)])
        self.assertEqual((p["lo"], p["hi"]), p["re"])

    def test_no_panel_priority(self):
        a = pool([(-1.0, 0.2), (-0.5, 0.3), (-0.8, 0.25)])
        b = pool([(-0.8, 0.25), (-1.0, 0.2), (-0.5, 0.3)])
        self.assertAlmostEqual(a["log_r"], b["log_r"])
        self.assertAlmostEqual(a["lo"], b["lo"])

    def test_bad_input(self):
        with self.assertRaises(ValueError):
            pool([(0.0, 0.0)])
        with self.assertRaises(ValueError):
            se_from_interval(0.5, 0.4, 2.0)

    def test_se_from_interval_roundtrip(self):
        se, t = 0.15, 2.776
        lo, hi = math.exp(math.log(0.3) - t * se), math.exp(math.log(0.3) + t * se)
        self.assertAlmostEqual(se_from_interval(lo, hi, t), se)


class Carry(unittest.TestCase):
    def test_lines_and_dilution(self):
        c = carry(pool([(math.log(0.25), 0.1)] * 2))
        self.assertAlmostEqual(c["m"]["primary_r^-0.5"][0], 2.0)
        self.assertAlmostEqual(c["m"]["first_order_1/r"][0], 4.0)
        self.assertEqual(c["m"]["saturated_1"], (1.0, 1.0, 1.0))
        self.assertAlmostEqual(c["repair_factor"]["primary_r^-0.5"][2][0], 1.5)  # (2+1)/2
        self.assertAlmostEqual(c["repair_factor"]["primary_r^-0.5"][3][0], 4 / 3)  # (2+2)/3
        m = c["m"]["primary_r^-0.5"]
        self.assertLess(m[1], m[0])
        self.assertLess(m[0], m[2])
        self.assertFalse(c["r_interval_includes_1"])

    def test_interval_including_1_gives_m_including_1(self):
        c = carry(pool([(math.log(0.6), 0.5), (math.log(0.9), 0.5)]))
        self.assertTrue(c["r_interval_includes_1"])
        for name in ("primary_r^-0.5", "first_order_1/r", "stress_r^-2_outside_range"):
            _, lo, hi = c["m"][name]
            self.assertLessEqual(lo, 1.0)
            self.assertGreaterEqual(hi, 1.0)
        _, lo, hi = c["repair_factor"]["primary_r^-0.5"][2]
        self.assertLessEqual(lo, 1.0)
        self.assertGreaterEqual(hi, 1.0)


if __name__ == "__main__":
    unittest.main()
