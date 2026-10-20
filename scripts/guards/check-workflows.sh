#!/usr/bin/env bash
# check-workflows.sh — lint every GitHub Actions workflow with actionlint
# (YAML syntax, expressions, contexts, and shellcheck on `run:` blocks).
#
# Why: PR #60 merged a workflow with invalid YAML; GitHub never ran it and
# nothing went red. This check fails the PR instead.
#
# Usage:
#   scripts/guards/check-workflows.sh [file.yml | dir]...
#   (default: .github/workflows/*.yml and *.yaml)
#
# Needs `actionlint` on PATH (the Workflows job installs a pinned version).
# Exit 0 = clean, 1 = at least one finding, 2 = usage / actionlint missing.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"

if ! command -v actionlint >/dev/null 2>&1; then
  echo "check-workflows: actionlint not found on PATH" >&2
  exit 2
fi

files=()
[ $# -gt 0 ] || set -- "$ROOT/.github/workflows"
for arg in "$@"; do
  if [ -d "$arg" ]; then
    for f in "$arg"/*.yml "$arg"/*.yaml; do [ -e "$f" ] && files+=("$f"); done
  elif [ -f "$arg" ]; then
    files+=("$arg")
  else
    echo "check-workflows: no such file or folder: $arg" >&2
    exit 2
  fi
done
if [ ${#files[@]} -eq 0 ]; then
  echo "check-workflows: no workflow file found" >&2
  exit 2
fi

status=0
actionlint -no-color "${files[@]}" || status=$?
if [ "$status" = 0 ]; then
  echo "check-workflows: ${#files[@]} workflow(s) clean"
  exit 0
fi
echo "check-workflows: findings above (actionlint exit $status)"
exit 1
