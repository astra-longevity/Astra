#!/usr/bin/env bash
# Known-bad / known-good cases for scripts/guards/naive-writer.py (V-001).
# Each case builds a tiny tree with one listed file and one unlisted file.
#
# Usage: scripts/tests/naive-writer.test.sh
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
NW="$ROOT/scripts/guards/naive-writer.py"

pass=0
failed=0
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT

# case <name> <listed file content> <expected exit> <grep pattern> [<extra list lines>]
case_() {
  local name="$1" body="$2" want_rc="$3" want="$4" extra="${5:-}" rc=0 out dir
  dir="$work/$pass-$failed"
  mkdir -p "$dir/docs"
  printf '%s\n' "$body" >"$dir/docs/listed.md"
  printf '%s\n' "Rapamycin extends lifespan in mice by 23% (median)." >"$dir/docs/unlisted.md"
  printf 'paths:\n  - path: docs/listed.md\n    source: test\n%s' "$extra" >"$dir/list.yaml"
  out="$(python3 "$NW" --root "$dir" --list list.yaml 2>&1)" || rc=$?
  if [ "$rc" -eq "$want_rc" ] && grep -q -- "$want" <<<"$out"; then
    echo "ok   $name"
    pass=$((pass + 1))
  else
    echo "FAIL $name (rc=$rc)"
    printf '%s\n' "$out" | sed 's/^/     /'
    failed=$((failed + 1))
  fi
}

case_ "quoted median-lifespan % fails" \
  'In the ITP, median lifespan rose by 23% in males.' 1 'value: % with outcome'

case_ "'rapamycin extends lifespan in mice' fails" \
  'A snippet said "rapamycin extends lifespan in mice".' 1 'claim: verb + outcome'

case_ "claim wrapped over two lines fails" \
  "$(printf 'In old mice the combo improves physical function and extends remaining\nlifespan (web snippet).')" 1 'docs/listed.md:1-2'

case_ "hazard ratio near mortality fails" \
  'Treated mice: HR 0.72 for all-cause mortality.' 1 'value: hazard ratio'

case_ "NA4.2 marker-ratio line passes" \
  'exposure (NA4.2, 2026-10-21): A28 saw both M-005 panel ratios (~0.2 Fig 1d p16 area%, ~0.56 Fig 1f SA-β-gal, 1f interval including 1) in a relay before setting' \
  0 '0 hit(s)'

case_ "unlisted file with outcome text passes" \
  'Senolytic raises H8 clearance; marker ratio r maps to m = r^-0.5.' 0 '1 listed files, 0 hit(s)'

case_ "exempt line is kept as record" \
  'A snippet said "rapamycin extends lifespan in mice".' 0 '1 exempt line(s)' \
  "exempt:
  - path: docs/listed.md
    line_sha256: $(printf '%s' 'A snippet said "rapamycin extends lifespan in mice".' | shasum -a 256 | cut -d' ' -f1)
    ruling: test
"

case_ "output says convention, not proof" \
  'Nothing here.' 0 'convention, not proof (pattern match)'

case_ "bad list (path without source) exits 2" \
  'Nothing here.' 2 'needs path and source' "  - path: docs/other.md
"

echo "naive-writer tests: $pass passed, $failed failed"
[ "$failed" -eq 0 ]
