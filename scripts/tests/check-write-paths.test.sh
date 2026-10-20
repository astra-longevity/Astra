#!/usr/bin/env bash
# Known-bad / known-good cases for scripts/check-write-paths.sh.
# Each case builds a throwaway git repo: a base commit (agent map + decision
# log), then PR commits, then runs the check and compares the exit code.
#
# Usage: scripts/tests/check-write-paths.test.sh
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
CHECK="$ROOT/scripts/check-write-paths.sh"

pass=0
failed=0
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT

BASE_MAP='agents:
  - id: A50
    unit: verification
    status: active
  - id: A52
    unit: verification
    status: ended
  - id: A10
    unit: code
    status: active
'

BASE_LOG='# Log
| D47 | Verification owns its path. | decided | A50 |
| D90 | A10 may add verification/second-impl/ outputs. | decided | A10 |
| D91 | Unrelated decision about data/. | decided | A9 |
'

# new_repo [with_map=1]: fresh repo with a base commit; echoes nothing.
new_repo() {
  local with_map="${1:-1}"
  rm -rf "$work/repo"
  mkdir -p "$work/repo/docs/decisions" "$work/repo/verification"
  cd "$work/repo"
  git init -q -b main
  git config user.email t@t
  git config user.name t
  printf '%s' "$BASE_LOG" >docs/decisions/log.md
  echo seed >verification/README.md
  echo seed >README.md
  if [ "$with_map" = 1 ]; then printf '%s' "$BASE_MAP" >docs/agents.yaml; fi
  git add -A
  git commit -qm "base"
  BASE="$(git rev-parse HEAD)"
}

# commit <message> <file> [content]: write file and commit with message.
commit() {
  mkdir -p "$(dirname "$2")"
  printf '%s\n' "${3:-change}" >>"$2"
  git add -A
  git commit -qm "$1"
}

# expect <name> <exit code> [body]
expect() {
  local name="$1" want="$2" body="${3:-}" got=0
  printf '%s' "$body" >"$work/body"
  "$CHECK" --base "$BASE" --head HEAD --body-file "$work/body" >"$work/out" 2>&1 || got=$?
  if [ "$got" = "$want" ]; then
    pass=$((pass + 1)); echo "ok   $name"
  else
    failed=$((failed + 1)); echo "FAIL $name (want $want, got $got)"; sed 's/^/     /' "$work/out"
  fi
  cd "$ROOT"
}

T=$'\n\nAgent: '

new_repo; commit "v${T}A50" verification/x.md
expect "verification agent writes verification/" 0

new_repo; commit "v${T}A10" verification/x.md
expect "non-verification agent writes verification/" 1

new_repo; commit "v${T}A10" verification/second-impl/out.csv
expect "cited decision names parent folder" 0 "Per D90."

new_repo; commit "v${T}A10" verification/x.md
expect "cited decision does not name path" 1 "Per D91."

new_repo; commit "v${T}A10" README.md
expect "D69: cited decision missing on base" 1 "Per D999."

new_repo; commit "v" verification/x.md
expect "no trailer on verification/" 1

new_repo; commit "v" README.md
expect "no trailer elsewhere is a warning" 0

new_repo; commit "v${T}A77" README.md
expect "unknown agent elsewhere is a warning" 0

new_repo; commit "v${T}A50" README.md
expect "verification agent outside its path (R2)" 1

new_repo; commit "v${T}A52" verification/x.md
expect "ended agent (R4)" 1

new_repo; commit "v${T}A50" docs/agents.yaml $'  - id: A64\n    unit: verification\n    status: active'
expect "verification agent adds a verification row" 0

new_repo; commit "v${T}A10" docs/agents.yaml $'  - id: A64\n    unit: verification\n    status: active'
expect "non-verification agent adds a verification row (R3)" 1

new_repo; commit "v${T}A50" docs/agents.yaml $'  - id: A64\n    unit: code\n    status: active'
expect "verification agent adds a code row (R2)" 1

new_repo; commit "v${T}A10" docs/agents.yaml $'  - id: A64\n    unit: code\n    status: active'
expect "non-verification agent adds a code row" 0

new_repo 0; commit "v${T}A7" docs/agents.yaml "$BASE_MAP"
expect "bootstrap: map absent on base" 0

new_repo; commit $'squash (#9)\n\n* first\n\n  Agent: A50\n\n* second\n\n  Agent: A10' verification/x.md
expect "squash message: one non-verification agent fails" 1

new_repo; commit $'squash (#9)\n\n* first\n\n  Agent: A50' verification/x.md
expect "squash message: indented trailer is parsed" 0

echo ""
echo "check-write-paths tests: $pass passed, $failed failed"
[ "$failed" = 0 ]
