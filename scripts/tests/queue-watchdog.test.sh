#!/usr/bin/env bash
# Known-bad / known-good cases for scripts/guards/queue-watchdog.py (offline
# snapshots, no network). Job and step names and times are those of the two
# 2026-10-21 cases: run 39620910334 (#212 Structure, zombie: all steps done,
# job in_progress) and run 39627832648 (#223 Runs, long live step).
#
# Usage: scripts/tests/queue-watchdog.test.sh
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
WD="$ROOT/scripts/guards/queue-watchdog.py"

pass=0
failed=0
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT

# steps <status of the last-but-one step> -> JSON step list
structure_steps='[{"name":"Set up job","status":"completed"},{"name":"Run actions/checkout@v4","status":"completed"},{"name":"Structure","status":"completed"},{"name":"Post Run actions/checkout@v4","status":"completed"},{"name":"Complete job","status":"completed"}]'
runs_steps_live='[{"name":"Set up job","status":"completed"},{"name":"Run actions/checkout@v4","status":"completed"},{"name":"Run actions/setup-python@v5","status":"completed"},{"name":"Self-tests (known-bad manifests, build lock)","status":"completed"},{"name":"Check manifests","status":"completed"},{"name":"Re-run manifests","status":"in_progress"},{"name":"Post Run actions/setup-python@v5","status":"pending"},{"name":"Post Run actions/checkout@v4","status":"pending"},{"name":"Complete job","status":"pending"}]'

entry_212() {
  echo '{"pr":212,"pr_id":"PR_212","head_sha":"aaa","runs":[{"id":39620910334,"jobs":[
    {"name":"Rust (lints · tests)","status":"completed","started_at":"2026-10-21T14:54:01Z","steps":[]},
    {"name":"Structure (lengths · folders · chain)","status":"in_progress","started_at":"2026-10-21T14:54:01Z","steps":'"$1"'}]}]}'
}
entry_223() {
  echo '{"pr":223,"pr_id":"PR_223","head_sha":"bbb","runs":[{"id":39627832648,"jobs":[
    {"name":"Runs (manifests · re-run)","status":"in_progress","started_at":"2026-10-21T15:40:41Z","steps":'"$1"'}]}]}'
}

# case <name> <snapshot json> <now> <expected exit> <grep pattern> [<pattern that must NOT appear>]
case_() {
  local name="$1" snap="$2" now="$3" want_rc="$4" want="$5" nope="${6:-}" rc=0 out
  printf '%s\n' "$snap" >"$work/snap.json"
  out="$(GITHUB_STEP_SUMMARY="$work/summary.md" python3 "$WD" run --snapshot "$work/snap.json" --now "$now" 2>&1)" || rc=$?
  if [ "$rc" -eq "$want_rc" ] && grep -q -- "$want" <<<"$out" && { [ -z "$nope" ] || ! grep -q -- "$nope" <<<"$out"; }; then
    echo "ok   $name"
    pass=$((pass + 1))
  else
    echo "FAIL $name (rc=$rc)"
    printf '%s\n' "$out" | sed 's/^/     /'
    failed=$((failed + 1))
  fi
}

case_ "stuck Structure job (#212) is dequeued" \
  "{\"entries\":[$(entry_212 "$structure_steps")]}" 2026-10-21T15:10:00Z 0 \
  '| #212 | 39620910334 | Structure (lengths · folders · chain) | 2026-10-21T14:54:01Z | DEQUEUE'

case_ "live long step (#223 Runs 'Re-run manifests') is left alone" \
  "{\"entries\":[$(entry_223 "$runs_steps_live")]}" 2026-10-21T15:58:00Z 0 \
  'no stuck job' 'DEQUEUE'

case_ "empty queue is a no-op" \
  '{"entries":[]}' 2026-10-21T15:10:00Z 0 \
  'merge queue empty' 'DEQUEUE'

case_ "all steps done but under 10 min is left alone" \
  "{\"entries\":[$(entry_212 "$structure_steps")]}" 2026-10-21T15:03:00Z 0 \
  'no stuck job' 'DEQUEUE'

case_ "in_progress job with no steps yet is left alone" \
  "{\"entries\":[$(entry_212 '[]')]}" 2026-10-21T15:30:00Z 0 \
  'no stuck job' 'DEQUEUE'

case_ "stuck and live entries: only the stuck one is dequeued" \
  "{\"entries\":[$(entry_212 "$structure_steps"),$(entry_223 "$runs_steps_live")]}" 2026-10-21T15:58:00Z 0 \
  '| #212 |' '| #223 |'

case_ "summary says it never re-arms" \
  "{\"entries\":[$(entry_212 "$structure_steps")]}" 2026-10-21T15:10:00Z 0 \
  'Not re-armed'

# The summary file must receive the same table.
if grep -q '| #212 | 39620910334 |' "$work/summary.md"; then
  echo "ok   job summary file written"; pass=$((pass + 1))
else
  echo "FAIL job summary file written"; failed=$((failed + 1))
fi

echo "queue-watchdog tests: $pass passed, $failed failed"
[ "$failed" -eq 0 ]
