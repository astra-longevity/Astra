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

snippet='A snippet said "rapamycin extends lifespan in mice".'
snippet_sha="$(printf '%s' "$snippet" | shasum -a 256 | cut -d' ' -f1)"

case_ "exempt record line is printed as kept as record with its reason" \
  "$snippet" 0 'exempt, kept as record (test: disclosure)' \
  "exempt:
  - path: docs/listed.md
    line_sha256: $snippet_sha
    ruling: test
    reason: disclosure
    kind: record
"

baseline='Defaults chosen so that the untreated median lifespan lands near 2.5 years.'
baseline_sha="$(printf '%s' "$baseline" | shasum -a 256 | cut -d' ' -f1)"
notoutcome_list="exempt:
  - path: docs/listed.md
    line_sha256: $baseline_sha
    ruling: V-008
    reason: \"not an intervention outcome: untreated baseline, model target\"
    kind: not-outcome
"

case_ "not-outcome exemption is printed without 'kept as record'" \
  "$baseline" 0 'docs/listed.md:1: exempt (V-008: not an intervention outcome: untreated baseline, model target)$' \
  "$notoutcome_list"

case_ "treated-arm lifespan value fails next to an exempt baseline line" \
  "$(printf '%s\n%s' "$baseline" 'Under rapamycin the model gives a median lifespan of 3.1 years.')" 1 \
  'docs/listed.md:2: value: median/mean/max outcome' \
  "$notoutcome_list"

case_ "exemption without a reason exits 2" \
  "$snippet" 2 'needs path, line_sha256 (64 hex), ruling, reason, kind' \
  "exempt:
  - path: docs/listed.md
    line_sha256: $snippet_sha
    ruling: test
    kind: record
"

case_ "exemption without a valid kind exits 2" \
  "$snippet" 2 'kind (record | not-outcome)' \
  "exempt:
  - path: docs/listed.md
    line_sha256: $snippet_sha
    ruling: test
    reason: disclosure
"

# Failure output must never contain the offending text (public Actions logs).
quote_dir="$work/no-quote"
mkdir -p "$quote_dir/docs"
printf '%s\n' "$snippet" >"$quote_dir/docs/listed.md"
printf 'paths:\n  - path: docs/listed.md\n    source: test\n' >"$quote_dir/list.yaml"
quote_rc=0
quote_out="$(python3 "$NW" --root "$quote_dir" --list list.yaml 2>&1)" || quote_rc=$?
if [ "$quote_rc" -eq 1 ] && ! grep -qi 'rapamycin\|extends lifespan' <<<"$quote_out" \
  && grep -qF "fix: describe the sighting, don't quote the outcome (V-001); exemption only by Verification ruling" <<<"$quote_out" \
  && grep -q "docs/listed.md:1: claim: verb + outcome \[sha256 $snippet_sha\]" <<<"$quote_out"; then
  echo "ok   failure output has path, type, sha256 and fix hint, never the text"
  pass=$((pass + 1))
else
  echo "FAIL failure output has path, type, sha256 and fix hint, never the text (rc=$quote_rc)"
  printf '%s\n' "$quote_out" | sed 's/^/     /'
  failed=$((failed + 1))
fi

case_ "output says convention, not proof" \
  'Nothing here.' 0 'convention, not proof (pattern match)'

case_ "bad list (path without source) exits 2" \
  'Nothing here.' 2 'needs path and source' "  - path: docs/other.md
"

echo "naive-writer tests: $pass passed, $failed failed"
[ "$failed" -eq 0 ]
