#!/usr/bin/env bash
# local-checks.sh — run the sealed check families locally, through the build
# lock (at most 2 local builds/test runs at once on the shared machine).
#
#   scripts/local/local-checks.sh             # lints, tests, structure
#   scripts/local/local-checks.sh lints       # one family: lints | tests | structure
#
# The family scripts themselves (.github/checks/) are sealed and unchanged.
# The blocking callbacks in .context-pilot/shared/callbacks.yaml (also sealed)
# call this wrapper (M2 re-seal batch, docs/rust/seal-extension.md).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
LOCK="$ROOT/scripts/local/build-lock.sh"
mode="${LOCAL_CHECKS_MODE:---callback}"

families=("$@")
[ ${#families[@]} -gt 0 ] || families=(lints tests structure)

status=0
for f in "${families[@]}"; do
  case "$f" in
    lints) script="check-rust-lints.sh" ;;
    tests) script="check-rust-tests.sh" ;;
    structure) script="check-structure.sh" ;;
    *) echo "unknown family: $f (lints | tests | structure)" >&2; exit 2 ;;
  esac
  "$LOCK" bash "$ROOT/.github/checks/$script" "$mode" || status=$?
done
exit "$status"
