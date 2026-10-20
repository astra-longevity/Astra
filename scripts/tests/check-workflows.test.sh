#!/usr/bin/env bash
# Known-bad / known-good cases for scripts/guards/check-workflows.sh.
#
# Usage: scripts/tests/check-workflows.test.sh
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
CHECK="$ROOT/scripts/guards/check-workflows.sh"
FX="$ROOT/scripts/tests/fixtures/workflows"

pass=0
failed=0
out="$(mktemp)"
trap 'rm -f "$out"' EXIT

# expect <name> <want exit> [pattern] -- <args...>
expect() {
  local name="$1" want="$2" pat="$3" got=0
  shift 4
  "$CHECK" "$@" >"$out" 2>&1 || got=$?
  if [ "$got" = "$want" ] && { [ -z "$pat" ] || grep -qE "$pat" "$out"; }; then
    pass=$((pass + 1)); echo "ok   $name"
  else
    failed=$((failed + 1)); echo "FAIL $name (want $want, got $got${pat:+, pattern \"$pat\"})"
    sed 's/^/     /' "$out"
  fi
}

expect "good workflow passes" 0 "1 workflow\(s\) clean" -- "$FX/good.yml"
expect "invalid YAML fails (PR #60 case)" 1 "could not parse as YAML" -- "$FX/bad-yaml.yml"
expect "unknown expression context fails" 1 "evnt" -- "$FX/bad-expr.yml"
expect "shellcheck finding in run: fails" 1 "shellcheck" -- "$FX/bad-shell.yml"
expect "one bad file among good ones fails" 1 "" -- "$FX/good.yml" "$FX/bad-yaml.yml"
expect "missing path is a usage error" 2 "no such file" -- "$FX/nope.yml"
expect "repo workflows are clean" 0 "clean" --

echo ""
echo "check-workflows tests: $pass passed, $failed failed"
[ "$failed" = 0 ]
