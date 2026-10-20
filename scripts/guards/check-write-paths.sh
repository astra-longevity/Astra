#!/usr/bin/env bash
# check-write-paths.sh — write-path rules for verification/ (D47) and
# decision citations (D69).
#
# Convention, not proof: `Agent: A<n>` trailers are self-declared and every
# agent shares one GitHub token (D68 refused). This check catches mistakes,
# not a determined bypass.
#
# Usage:
#   scripts/guards/check-write-paths.sh --base <sha> --head <sha> [--body-file <file>]
#
#   --base       base commit (main). The agent map and the decisions are read
#                from this commit, never from the PR.
#   --head       PR head commit. Commits checked = base..head (no merges).
#   --body-file  PR description. `Dnn` ids cited there must exist (D69) and
#                can authorise a non-Verification change to verification/.
#
# Rules:
#   R1  A commit touching verification/** must come from an active
#       Verification agent, unless the PR body cites a decision that exists in
#       docs/decisions/ on base and names the path (or a parent folder).
#       A commit with no `Agent:` trailer touching verification/** fails.
#   R2  A Verification agent's commit may only touch verification/** and the
#       `unit: verification` rows of docs/agents.yaml.
#   R3  Rows with `unit: verification` in docs/agents.yaml may only be added,
#       changed or removed by an agent that is Verification on base.
#   R4  An agent whose base row says `status: ended` may not author commits.
#   D69 Every `Dnn` cited in the PR body exists in docs/decisions/ on base.
#   Missing or unknown trailer outside verification/**: warning only.
#
# Exit 0 = pass (warnings allowed), 1 = at least one violation, 2 = usage.
set -euo pipefail

MAP_PATH="docs/agents.yaml"
DECISIONS_DIR="docs/decisions"
VERIF_DIR="verification/"

base=""
head=""
body_file=""
while [ $# -gt 0 ]; do
  case "$1" in
    --base) base="${2:-}"; shift 2 ;;
    --head) head="${2:-}"; shift 2 ;;
    --body-file) body_file="${2:-}"; shift 2 ;;
    *) echo "usage: $0 --base <sha> --head <sha> [--body-file <file>]" >&2; exit 2 ;;
  esac
done
if [ -z "$base" ] || [ -z "$head" ]; then
  echo "usage: $0 --base <sha> --head <sha> [--body-file <file>]" >&2
  exit 2
fi

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

errors=0
warnings=0
fail() { echo "FAIL: $*"; errors=$((errors + 1)); }
warn() { echo "WARN: $*"; warnings=$((warnings + 1)); }

# --- agents.yaml parsing -----------------------------------------------------

# Prints one line per agent: "id|unit|status|note".
parse_map() {
  awk '
    function flush() { if (id != "") print id "|" unit "|" status "|" note; id = unit = status = note = "" }
    /^[[:space:]]*-[[:space:]]*id:/ { flush(); v = $0; sub(/^[^:]*:[[:space:]]*/, "", v); id = v; next }
    /^[[:space:]]+unit:/   { v = $0; sub(/^[^:]*:[[:space:]]*/, "", v); unit = v; next }
    /^[[:space:]]+status:/ { v = $0; sub(/^[^:]*:[[:space:]]*/, "", v); status = v; next }
    /^[[:space:]]+note:/   { v = $0; sub(/^[^:]*:[[:space:]]*/, "", v); note = v; next }
    END { flush() }
  '
}

# Map file content at a commit, empty if absent.
map_at() { git show "$1:$MAP_PATH" 2>/dev/null | parse_map || true; }

base_has_map=1
git cat-file -e "$base:$MAP_PATH" 2>/dev/null || base_has_map=0
map_at "$base" >"$tmp/base.map"

field_of() { awk -F'|' -v id="$1" -v f="$2" '$1 == id { print $f; exit }' "$tmp/base.map"; }
is_verif() { [ "$(field_of "$1" 2)" = "verification" ] && [ "$(field_of "$1" 3)" = "active" ]; }
is_verif_any_status() { [ "$(field_of "$1" 2)" = "verification" ]; }

# Verification rows (by unit on either side) of a parsed map, sorted.
verif_rows() { awk -F'|' '$2 == "verification"' "$1" | sort; }
other_rows() { awk -F'|' '$2 != "verification"' "$1" | sort; }

# --- decisions ---------------------------------------------------------------

# Decision text on base: table rows "| Dnn | ..." or headings "# Dnn ...".
decision_text() {
  git grep -h -E "^(\|[[:space:]]*$1[[:space:]]*\||#+[[:space:]]+$1([^0-9]|$))" \
    "$base" -- "$DECISIONS_DIR" 2>/dev/null || true
}

cited=""
if [ -n "$body_file" ] && [ -s "$body_file" ]; then
  cited="$(grep -oE '\bD[0-9]+\b' "$body_file" | sort -u || true)"
fi
for d in $cited; do
  if [ -z "$(decision_text "$d")" ]; then
    fail "D69: PR body cites $d, which does not exist in $DECISIONS_DIR/ on base"
  fi
done

# True if a cited decision on base names $1 or one of its parent folders.
authorised_by_decision() {
  local path="$1" d text p
  for d in $cited; do
    text="$(decision_text "$d")"
    [ -n "$text" ] || continue
    # The file itself, then each parent folder written with a trailing slash.
    case "$text" in *"$path"*) echo "$d"; return 0 ;; esac
    p="$path"
    while case "$p" in */*) true ;; *) false ;; esac; do
      p="${p%/*}"
      case "$text" in *"$p/"*) echo "$d"; return 0 ;; esac
    done
  done
  return 1
}

# --- per-commit checks -------------------------------------------------------

commits="$(git rev-list --reverse --no-merges "$base..$head")"
if [ -z "$commits" ]; then
  echo "No commits in $base..$head."
fi

for c in $commits; do
  short="$(git rev-parse --short "$c")"
  agents="$(git log -1 --format=%B "$c" \
    | sed -nE 's/^[[:space:]]*(\*[[:space:]]*)?Agent:[[:space:]]*(A[0-9]+)[[:space:]]*$/\2/p' \
    | sort -u)"
  git diff-tree --no-commit-id --name-only -r "$c" >"$tmp/files"

  touches_verif=0
  grep -q "^$VERIF_DIR" "$tmp/files" && touches_verif=1

  # Missing trailer.
  if [ -z "$agents" ]; then
    if [ "$touches_verif" = 1 ]; then
      fail "$short: no 'Agent:' trailer on a commit touching $VERIF_DIR"
    else
      warn "$short: no 'Agent:' trailer"
    fi
    continue
  fi

  for a in $agents; do
    # R4: ended agents.
    if [ "$(field_of "$a" 3)" = "ended" ]; then
      fail "$short: agent $a is ended in $MAP_PATH on base"
      continue
    fi
    if [ -z "$(field_of "$a" 1)" ]; then
      warn "$short: agent $a is not in $MAP_PATH on base"
    fi

    if is_verif "$a"; then
      # R2: Verification agents stay in verification/ (+ their map rows).
      while IFS= read -r f; do
        case "$f" in
          "$VERIF_DIR"*) ;;
          "$MAP_PATH")
            git show "$c^:$MAP_PATH" 2>/dev/null | parse_map | other_rows /dev/stdin >"$tmp/o1" || true
            git show "$c:$MAP_PATH" 2>/dev/null | parse_map | other_rows /dev/stdin >"$tmp/o2" || true
            if ! cmp -s "$tmp/o1" "$tmp/o2"; then
              fail "$short: Verification agent $a changed non-verification rows of $MAP_PATH (R2)"
            fi
            ;;
          *) fail "$short: Verification agent $a touched $f, outside $VERIF_DIR (R2)" ;;
        esac
      done <"$tmp/files"
    else
      # R1: others need a cited decision naming the path.
      while IFS= read -r f; do
        case "$f" in
          "$VERIF_DIR"*)
            if d="$(authorised_by_decision "$f")"; then
              echo "OK:   $short: $a touched $f, authorised by $d (D69)"
            else
              fail "$short: $a is not an active Verification agent and touched $f; cite a decision on base naming this path (R1/D47)"
            fi
            ;;
        esac
      done <"$tmp/files"
    fi
  done

  # R3: verification rows of the map.
  if grep -qx "$MAP_PATH" "$tmp/files"; then
    git show "$c^:$MAP_PATH" 2>/dev/null | parse_map >"$tmp/m1" || true
    git show "$c:$MAP_PATH" 2>/dev/null | parse_map >"$tmp/m2" || true
    verif_rows "$tmp/m1" >"$tmp/v1"
    verif_rows "$tmp/m2" >"$tmp/v2"
    # Rows that move into or out of verification also count.
    if ! cmp -s "$tmp/v1" "$tmp/v2"; then
      if [ "$base_has_map" = 0 ]; then
        warn "$short: $MAP_PATH does not exist on base; verification rows accepted as bootstrap (R3)"
      else
        ok=1
        for a in $agents; do is_verif_any_status "$a" && [ "$(field_of "$a" 3)" = "active" ] || ok=0; done
        if [ "$ok" = 0 ]; then
          fail "$short: unit:verification rows of $MAP_PATH changed by $(echo "$agents" | tr '\n' ' ')- only agents who are Verification on base may (R3)"
        fi
      fi
    fi
  fi
done

echo ""
echo "convention, not proof (self-declared trailers; D68 refused)"
echo "write-paths: $errors violation(s), $warnings warning(s)"
[ "$errors" = 0 ]
