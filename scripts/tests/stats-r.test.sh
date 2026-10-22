#!/usr/bin/env bash
# Known-bad / known-good cases for .github/stats-r/run-case.R.
# Needs Rscript with the packages of .github/stats-r/renv.lock.
#
# Usage: scripts/tests/stats-r.test.sh
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
RUN="$ROOT/.github/stats-r/run-case.R"
FX="$ROOT/scripts/tests/fixtures/stats-r"
cd "$ROOT"

pass=0
failed=0
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

# expect <name> <want exit> <pattern> <spec>
expect() {
  local name="$1" want="$2" pat="$3" spec="$4" got=0
  # An empty spec runs the script with no arguments at all (usage case).
  Rscript "$RUN" ${spec:+"$spec" "$tmp/out"} >"$tmp/log" 2>&1 || got=$?
  if [ "$got" = "$want" ] && grep -qE "$pat" "$tmp/log"; then
    pass=$((pass + 1)); echo "ok   $name"
  else
    failed=$((failed + 1)); echo "FAIL $name (want $want, got $got, pattern \"$pat\")"
    sed 's/^/     /' "$tmp/log"
  fi
}

expect "missing spec field fails and names it" 1 "missing spec field\(s\): expected" "$FX/missing-field.yaml"
expect "case id must equal the file name" 1 "must equal the file name" "$FX/case-mismatch.yaml"
expect "value out of tolerance fails" 1 "out of tolerance" "$FX/out-of-tolerance.yaml"
expect "blocked case is skipped, never run" 0 "skipped blocked \(status: blocked\)" "$FX/blocked.yaml"
if grep -q "status: blocked" "$tmp/out/blocked.md"; then
  pass=$((pass + 1)); echo "ok   blocked case is listed in its summary"
else
  failed=$((failed + 1)); echo "FAIL blocked case is listed in its summary"
fi
expect "usage error without arguments" 2 "usage" ""
expect "real coloc-a passes" 0 "PP.H4 .*\| pass \|" "verification/second-impl/coloc-a.yaml"

echo ""
echo "stats-r tests: $pass passed, $failed failed"
[ "$failed" = 0 ]
