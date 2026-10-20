#!/usr/bin/env bash
# Known-bad / known-good cases for scripts/runs.py and scripts/local/build-lock.sh.
# Manifest cases copy research/runs/toy-gompertz-v0 into a temp folder of the
# same name, break one thing, and expect check/rerun to fail.
#
# Usage: scripts/tests/runs.test.sh
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
RUNS="$ROOT/scripts/runs.py"
LOCK="$ROOT/scripts/local/build-lock.sh"
SRC="$ROOT/research/runs/toy-gompertz-v0"

pass=0
failed=0
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT

ok()  { pass=$((pass + 1)); echo "ok   $1"; }
bad() { failed=$((failed + 1)); echo "FAIL $1"; sed 's/^/     /' "$work/out"; }

# fresh: copy the toy run; echoes the manifest path.
fresh() {
  rm -rf "$work/runs"
  mkdir -p "$work/runs"
  cp -R "$SRC" "$work/runs/"
  echo "$work/runs/toy-gompertz-v0/manifest.yaml"
}

# expect <name> <want exit> <check|rerun> <manifest>
expect() {
  local got=0
  (cd "$ROOT" && python3 "$RUNS" "$3" "$4") >"$work/out" 2>&1 || got=$?
  if [ "$got" = "$2" ]; then ok "$1"; else bad "$1 (want $2, got $got)"; fi
}

# set_field <manifest> <key> <value>
set_field() { sed -i.bak -E "s|^($2):.*|\\1: $3|" "$1" && rm -f "$1.bak"; }

# --- manifests ---------------------------------------------------------------

m="$(fresh)"; expect "good manifest: check" 0 check "$m"
m="$(fresh)"; expect "good manifest: rerun reproduces outputs" 0 rerun "$m"

m="$(fresh)"; set_field "$m" seed 43
expect "changed seed: rerun output mismatch" 1 rerun "$m"

m="$(fresh)"; sed -i.bak 's/^b: .*/b: 0.009/' "$(dirname "$m")/params.yaml"; rm -f "$(dirname "$m")/params.yaml.bak"
expect "changed param: params hash mismatch" 1 check "$m"

m="$(fresh)"; sed -i.bak '/^seed:/d' "$m"; rm -f "$m.bak"
expect "missing field (seed)" 1 check "$m"

m="$(fresh)"; sed -i.bak '/^toolchain:/,/^outputs:/{/^toolchain:/!{/^outputs:/!d;};}' "$m"; rm -f "$m.bak"
expect "empty toolchain map" 1 check "$m"

m="$(fresh)"; echo "tampered,1" >>"$(dirname "$m")/output.csv"
expect "committed output edited after the run" 1 check "$m" # still passes schema...
m="$(fresh)"; echo "tampered,1" >>"$(dirname "$m")/output.csv"
expect "committed output edited: rerun catches it" 1 rerun "$m"

m="$(fresh)"; sed -i.bak -E 's|(gompertz_toy.py: )[0-9a-f]{6}|\1000000|' "$m"; rm -f "$m.bak"
expect "code hash mismatch" 1 check "$m"

m="$(fresh)"; echo "extra: 1" >>"$m"
expect "unknown field" 1 check "$m"

m="$(fresh)"; set_field "$m" command 'python3 x.py --seed 42'
expect "command without {seed}" 1 check "$m"

m="$(fresh)"; set_field "$m" code_commit 1111111111111111111111111111111111111111
expect "unknown code_commit is a warning" 0 check "$m"

# --- build lock --------------------------------------------------------------

export ASTRA_BUILD_LOCK_DIR="$work/lock"
log="$work/conc.log"
: >"$log"
job() { echo "start" >>"$log"; sleep 1; echo "end" >>"$log"; }
export -f job
export log
for _ in 1 2 3 4; do "$LOCK" bash -c job 2>/dev/null & done
wait
max="$(awk '/start/{c++; if (c>m) m=c} /end/{c--} END{print m}' "$log")"
if [ "$max" -le 2 ] && [ "$(grep -c start "$log")" = 4 ]; then ok "lock: 4 jobs, at most 2 at once (max $max)"
else echo "max=$max" >"$work/out"; bad "lock: concurrency"; fi

got=0
"$LOCK" sleep 3 2>/dev/null &
"$LOCK" sleep 3 2>/dev/null &
sleep 0.5
ASTRA_BUILD_LOCK_TIMEOUT=1 "$LOCK" true >"$work/out" 2>&1 || got=$?
wait
if [ "$got" = 75 ]; then ok "lock: timeout exits 75"; else bad "lock: timeout (got $got)"; fi

mkdir -p "$ASTRA_BUILD_LOCK_DIR/slot-1" "$ASTRA_BUILD_LOCK_DIR/slot-2"
echo 999999 >"$ASTRA_BUILD_LOCK_DIR/slot-1/pid"
echo 999999 >"$ASTRA_BUILD_LOCK_DIR/slot-2/pid"
got=0
ASTRA_BUILD_LOCK_TIMEOUT=2 "$LOCK" true >"$work/out" 2>&1 || got=$?
if [ "$got" = 0 ]; then ok "lock: stale slot of a dead PID is reclaimed"; else bad "lock: stale reclaim (got $got)"; fi

rm -rf "$ASTRA_BUILD_LOCK_DIR"/slot-*
got=0
"$LOCK" bash -c 'exit 7' >"$work/out" 2>&1 || got=$?
if [ "$got" = 7 ] && [ -z "$(ls "$ASTRA_BUILD_LOCK_DIR")" ]; then ok "lock: exit code passed through, slot released"
else bad "lock: exit/release (got $got)"; fi

echo ""
echo "runs tests: $pass passed, $failed failed"
[ "$failed" = 0 ]
