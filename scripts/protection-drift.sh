#!/usr/bin/env bash
# D52 protection drift check (founding-council-log.md, D52).
#
# Compares the LIVE branch protection of main and the merge-queue ruleset with
# the payloads committed in docs/rust/. Read-only: it only reads JSON.
#
# Usage:
#   scripts/protection-drift.sh LIVE_PROTECTION.json LIVE_RULESET.json
#     (files fetched with `gh api` GET; see .github/workflows/protection-drift.yml)
#
# Exit 0 = no drift. Exit 1 = drift (a unified diff is printed). Exit 2 = usage
# or input error. Needs jq.
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
expected_protection="${EXPECTED_PROTECTION:-$root/docs/rust/branch-protection.json}"
expected_ruleset="${EXPECTED_RULESET:-$root/docs/rust/merge-queue-ruleset.json}"

if [ "$#" -ne 2 ]; then
  echo "usage: $0 LIVE_PROTECTION.json LIVE_RULESET.json" >&2
  exit 2
fi
live_protection="$1"
live_ruleset="$2"
for f in "$live_protection" "$live_ruleset" "$expected_protection" "$expected_ruleset"; do
  if ! jq -e . "$f" >/dev/null 2>&1; then
    echo "not a readable JSON file: $f" >&2
    exit 2
  fi
done

# Settings that are not in the committed PUT payload but must keep GitHub's
# default. Any change to them is drift too.
extras_default='{
  "required_signatures": false,
  "required_linear_history": false,
  "required_conversation_resolution": false,
  "lock_branch": false,
  "block_creations": false,
  "allow_fork_syncing": false,
  "require_last_push_approval": false
}'

# Live GET response -> same shape as the committed PUT payload (+ extras).
normalise_live_protection='{
  required_status_checks: (if .required_status_checks == null then null else {
    strict: .required_status_checks.strict,
    contexts: (.required_status_checks.contexts | sort)
  } end),
  enforce_admins: (.enforce_admins.enabled // false),
  required_pull_request_reviews: (if .required_pull_request_reviews == null then null else {
    required_approving_review_count: .required_pull_request_reviews.required_approving_review_count,
    dismiss_stale_reviews: .required_pull_request_reviews.dismiss_stale_reviews,
    require_code_owner_reviews: .required_pull_request_reviews.require_code_owner_reviews
  } end),
  restrictions: (.restrictions // null),
  allow_force_pushes: (.allow_force_pushes.enabled // false),
  allow_deletions: (.allow_deletions.enabled // false),
  extras: {
    required_signatures: (.required_signatures.enabled // false),
    required_linear_history: (.required_linear_history.enabled // false),
    required_conversation_resolution: (.required_conversation_resolution.enabled // false),
    lock_branch: (.lock_branch.enabled // false),
    block_creations: (.block_creations.enabled // false),
    allow_fork_syncing: (.allow_fork_syncing.enabled // false),
    require_last_push_approval: (.required_pull_request_reviews.require_last_push_approval // false)
  }
}'

# Committed payload -> canonical shape.
normalise_expected_protection='. + {extras: $extras}
  | .required_status_checks.contexts |= (if . == null then null else sort end)'

# Ruleset: drop server-side identity and timestamps, keep everything enforced.
normalise_ruleset='del(.id, .node_id, .source, .source_type, .created_at, .updated_at,
  ._links, .current_user_can_bypass)'

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

jq -S "$normalise_live_protection" "$live_protection" >"$tmp/protection.live.json"
jq -S --argjson extras "$extras_default" "$normalise_expected_protection" \
  "$expected_protection" >"$tmp/protection.expected.json"
jq -S "$normalise_ruleset" "$live_ruleset" >"$tmp/ruleset.live.json"
jq -S "$normalise_ruleset" "$expected_ruleset" >"$tmp/ruleset.expected.json"

status=0
for name in protection ruleset; do
  if ! diff -u --label "committed $name" --label "live $name" \
    "$tmp/$name.expected.json" "$tmp/$name.live.json"; then
    status=1
  fi
done

if [ "$status" -eq 0 ]; then
  echo "No drift: live branch protection and ruleset match docs/rust/."
else
  echo "DRIFT: live settings differ from docs/rust/ (see diff above)." >&2
fi
exit "$status"
