#!/usr/bin/env python3
"""Toy seeded run: Gompertz mouse lifespans (platform test fixture, not science).

Simulates N lifespans from a Gompertz hazard h(t) = a * exp(b t) with a
self-contained LCG (no dependence on Python's `random` across versions), and
writes summary statistics rounded to 4 decimals so outputs are byte-stable.

  python3 gompertz_toy.py --params params.yaml --seed 42 --out output.csv
"""

import argparse
import math


def read_params(path):
    out = {}
    with open(path) as f:
        for line in f:
            line = line.split("#", 1)[0].strip()
            if line:
                k, v = line.split(":", 1)
                out[k.strip()] = float(v)
    return out


def lcg(seed):
    """Numerical Recipes 64-bit LCG; yields uniforms in (0, 1)."""
    x = seed & 0xFFFFFFFFFFFFFFFF
    while True:
        x = (6364136223846793005 * x + 1442695040888963407) & 0xFFFFFFFFFFFFFFFF
        yield ((x >> 11) + 0.5) / float(1 << 53)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--params", required=True)
    ap.add_argument("--seed", type=int, required=True)
    ap.add_argument("--out", required=True)
    args = ap.parse_args()

    p = read_params(args.params)
    a, b, n = p["a"], p["b"], int(p["n"])
    u = lcg(args.seed)
    # Inverse CDF of Gompertz: t = ln(1 - b ln(U) / a) / b
    days = sorted(math.log(1.0 - b * math.log(next(u)) / a) / b for _ in range(n))
    mean = sum(days) / n
    median = (days[(n - 1) // 2] + days[n // 2]) / 2
    with open(args.out, "w") as f:
        f.write("stat,value\n")
        f.write(f"n,{n}\n")
        f.write(f"mean_days,{mean:.4f}\n")
        f.write(f"median_days,{median:.4f}\n")
        f.write(f"max_days,{days[-1]:.4f}\n")


if __name__ == "__main__":
    main()
