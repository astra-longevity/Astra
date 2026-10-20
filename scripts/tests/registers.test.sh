#!/usr/bin/env bash
# Known-bad / known-good cases for scripts/registers.py.
# Each case builds a throwaway root with issues/ and research/obstacles/.
#
# Usage: scripts/tests/registers.test.sh
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
REG="$ROOT/scripts/registers.py"

pass=0
failed=0
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT
R="$work/root"

fresh() { rm -rf "$R"; mkdir -p "$R/issues" "$R/research/obstacles"; }

# issue <id> <severity> <target> <status> [extra line]
issue() {
  {
    echo "id: $1"
    echo "title: test issue $1"
    echo "severity: $2"
    echo "target: $3"
    echo "opened: 2026-10-01"
    echo "status: $4"
    echo "owner: A50"
    [ -z "${5:-}" ] || echo "$5"
  } >"$R/issues/$1.yaml"
}

# obstacle <id> <date> <check> [status]
obstacle() {
  {
    echo "id: $1"
    echo "title: test obstacle $1"
    echo "owner: A10"
    echo "exit: the check passes"
    echo "check: $3"
    echo "date: $2"
    echo "status: ${4:-open}"
  } >"$R/research/obstacles/$1.yaml"
}

# expect <name> <want exit> [grep pattern] -- <registers.py args...>
expect() {
  local name="$1" want="$2" pat="$3" got=0
  shift 4
  python3 "$REG" --root "$R" "$@" >"$work/out" 2>&1 || got=$?
  if [ "$got" = "$want" ] && { [ -z "$pat" ] || grep -qE "$pat" "$work/out"; }; then
    pass=$((pass + 1)); echo "ok   $name"
  else
    failed=$((failed + 1)); echo "FAIL $name (want $want, got $got${pat:+, pattern: $pat})"
    sed 's/^/     /' "$work/out"
  fi
}

# --- issues + gate -------------------------------------------------------------

fresh; issue s1-split S1 research/metric-v0/split.yaml open
expect "open S1 on the target blocks" 1 "BLOCKED by issues/s1-split.yaml" -- gate research/metric-v0/split.yaml

fresh; issue s2-folder S2 research/metric-v0/ open
expect "open S2 on a parent folder blocks" 1 "BLOCKED" -- gate research/metric-v0/split.yaml

fresh; issue s1-file S1 interface/templates/base.html open
expect "open S1 inside a published folder blocks" 1 "BLOCKED" -- gate interface/

fresh; issue s1-sealed S1 SEAL-metric-v0 open
expect "open S1 on a sealed id blocks" 1 "BLOCKED" -- gate SEAL-metric-v0

fresh; issue s3-minor S3 research/metric-v0/ open
expect "open S3 does not block" 0 "no open S1/S2" -- gate research/metric-v0/split.yaml

fresh; issue s1-done S1 research/metric-v0/ closed "closed: 2026-10-03"
expect "closed S1 does not block" 0 "" -- gate research/metric-v0/split.yaml

fresh; issue s1-other S1 data/ open
expect "open S1 elsewhere does not block" 0 "" -- gate research/metric-v0/split.yaml

fresh; issue s1-prefix S1 research/metric open
expect "name prefix is not a parent folder" 0 "" -- gate research/metric-v0/split.yaml

fresh; issue bad S1 x open; sed -i.bak '/^severity:/d' "$R/issues/bad.yaml"; rm -f "$R/issues/bad.yaml.bak"
expect "issue missing field fails the schema" 1 "missing field: severity" -- issues

fresh; issue bad S1 x open; sed -i.bak '/^severity:/d' "$R/issues/bad.yaml"; rm -f "$R/issues/bad.yaml.bak"
expect "invalid register refuses the gate" 1 "register invalid" -- gate x

fresh; issue bad S4 x open
expect "unknown severity fails" 1 "severity" -- issues

fresh; issue bad S2 x closed
expect "closed without a closed date fails" 1 "closed: YYYY-MM-DD" -- issues

fresh; issue wrong-name S2 x open; mv "$R/issues/wrong-name.yaml" "$R/issues/other.yaml"
expect "id differs from file name fails" 1 "id must equal" -- issues

fresh; issue ok-1 S2 x open
expect "valid issue passes the schema" 0 "1 valid, 0 error" -- issues

# --- obstacles -----------------------------------------------------------------

fresh; obstacle o1-metric 2026-10-10 "false"
expect "past date + failing check is reported" 1 "\[FAILED\] Obstacle o1-metric" -- obstacles --today 2026-10-20

fresh; obstacle o1-metric 2026-10-10 "false"
expect "report names owner and lateness" 1 "10 day\(s\) past" -- obstacles --today 2026-10-20

fresh; obstacle o2-ok 2026-10-10 "true"
expect "past date + passing check is not reported" 0 "nothing missed" -- obstacles --today 2026-10-20

fresh; obstacle o3-later 2026-12-01 "false"
expect "future date is not run" 0 "nothing missed" -- obstacles --today 2026-10-20

fresh; obstacle o4-met 2026-10-10 "false" met
expect "met obstacle is not re-run" 0 "" -- obstacles --today 2026-10-20

fresh; obstacle o5-job 2026-10-10 "job:Runs (manifests · re-run)"
expect "due CI-job check is reported for a manual look" 1 "NEEDS CHECK" -- obstacles --today 2026-10-20

fresh; obstacle o6-bad 2026-10-10 "true"; sed -i.bak '/^owner:/d' "$R/research/obstacles/o6-bad.yaml"; rm -f "$R/research/obstacles/o6-bad.yaml.bak"
expect "obstacle missing field fails" 1 "missing field: owner" -- obstacles --schema-only

fresh; obstacle o7-nodate 2026-10-10 "true"; sed -i.bak 's/^date:.*/date: month 6/' "$R/research/obstacles/o7-nodate.yaml"; rm -f "$R/research/obstacles/o7-nodate.yaml.bak"
expect "relative date ('month 6') fails" 1 "date: YYYY-MM-DD" -- obstacles --schema-only

fresh; obstacle o8-judge 2026-10-10 "judge:A50"
expect "judge check without evidence fails" 1 "evidence" -- obstacles --schema-only

fresh; obstacle o9-ok 2026-10-10 "true"
touch "$work/before"
expect "check runs without writing to the repo" 0 "" -- obstacles --today 2026-10-20
if [ -n "$(find "$R" -newer "$work/before" -type f)" ]; then
  failed=$((failed + 1)); echo "FAIL obstacles wrote into the root"
fi

# --- unowned rows and milestones -------------------------------------------

# set_line <file> <key> <value>: replace or append a key.
set_line() {
  if grep -q "^$2:" "$1"; then sed -i.bak "s|^$2:.*|$2: $3|" "$1"; rm -f "$1.bak"
  else echo "$2: $3" >>"$1"; fi
}
F() { echo "$R/research/obstacles/$1.yaml"; }

fresh; obstacle u1 null "judge:A50" unowned; set_line "$(F u1)" evidence verification/evidence/u1.md
expect "date null + unowned is valid" 0 "1 valid, 0 error" -- obstacles --schema-only

fresh; obstacle u1 null "judge:A50" unowned; set_line "$(F u1)" evidence verification/evidence/u1.md
expect "unowned row is reported daily" 1 "\[UNOWNED\] Obstacle u1" -- obstacles --today 2026-10-20

fresh; obstacle u2 null "true" open
expect "date null with status open fails" 1 "date: null only with status: unowned" -- obstacles --schema-only

fresh; obstacle u3 2027-01-20 "true" unowned
expect "unowned with a real date fails" 1 "unowned only with date: null" -- obstacles --schema-only

fresh; obstacle m1 2027-07-20 "true"; set_line "$(F m1)" milestone "sim v0 sealed"; set_line "$(F m1)" milestone_date 2026-10-15
expect "due milestone is reported before the exit date" 1 "\[MILESTONE DUE\] Obstacle m1" -- obstacles --today 2026-10-20

fresh; obstacle m2 2027-07-20 "true"; set_line "$(F m2)" milestone "sim v0 sealed"; set_line "$(F m2)" milestone_date 2026-10-15; set_line "$(F m2)" milestone_status met
expect "met milestone is not reported" 0 "nothing missed" -- obstacles --today 2026-10-20

fresh; obstacle m3 2027-07-20 "true"; set_line "$(F m3)" milestone "sim v0 sealed"
expect "milestone without milestone_date fails" 1 "go together" -- obstacles --schema-only

fresh; obstacle m4 2027-07-20 "true"; set_line "$(F m4)" milestone "x"; set_line "$(F m4)" milestone_date "month 4"
expect "relative milestone_date fails" 1 "milestone_date: YYYY-MM-DD" -- obstacles --schema-only

echo ""
echo "registers tests: $pass passed, $failed failed"
[ "$failed" = 0 ]
