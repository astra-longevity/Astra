#!/usr/bin/env bash
# P1 dataset fetch job: downloads the files listed by `astra fetch plan`.
#
# Usage:
#   scripts/fetch-datasets.sh PLAN.tsv CACHE_DIR verify|pin
#
# PLAN.tsv lines (from `astra fetch plan`):
#   fetch\t<id>\t<n>\t<interval>\t<sha256>\t<url>    approved + pinned
#   unpinned\t<id>\t<n>\t<interval>\t-\t<url>        approved, no hash yet
#   skipped\t<id>\t<licence_status>                  licence not approved
#
# - Pinned files are stored as CACHE_DIR/<sha256>; a cache hit sends no request.
#   `astra fetch verify` checks hashes and sizes afterwards.
# - Unpinned files are downloaded only in `pin` mode, and their sha256, size and
#   date are printed to paste into the manifest by PR. Nothing is written to the repo.
# - Requests to one host are spaced by at least the manifest's
#   min_interval_seconds (a licence condition, e.g. MPD asks for 2 s).
# - HTTP 403 = "source blocks automated fetch": that file fails, the others go
#   on. No retry loop, no custom headers.
#
# Exit 0 = every needed file downloaded; 1 = at least one problem; 2 = usage.
set -euo pipefail

if [ "$#" -ne 3 ] || { [ "$3" != verify ] && [ "$3" != pin ]; }; then
  echo "usage: $0 PLAN.tsv CACHE_DIR verify|pin" >&2
  exit 2
fi
plan="$1"
cache="$2"
mode="$3"
astra="${ASTRA:-target/release/astra}"
summary="${GITHUB_STEP_SUMMARY:-/dev/null}"

mkdir -p "$cache"
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

mkdir -p "$tmp/hosts"
problems=()
pins=()
skipped=()
downloaded=0
hits=0

# Waits until `interval` seconds have passed since the last request to `host`.
wait_for_host() {
  local stamp="$tmp/hosts/$1" interval="$2" elapsed
  if [ -f "$stamp" ]; then
    elapsed=$(($(date +%s) - $(cat "$stamp")))
    if [ "$elapsed" -lt "$interval" ]; then
      sleep $((interval - elapsed))
    fi
  fi
  date +%s >"$stamp"
}

# Downloads `url` to `out`; on failure records an issue-ready problem.
download() {
  local id="$1" n="$2" interval="$3" url="$4" out="$5" host code
  host="$(printf '%s' "$url" | sed -E 's#^[a-z]+://([^/]+).*#\1#')"
  wait_for_host "$host" "$interval"
  code="$(curl -sS -L --max-time 300 -o "$out" -w '%{http_code}' "$url" 2>"$tmp/curl.err" || true)"
  case "$code" in
    200) return 0 ;;
    403) problems+=("Source blocks automated fetch: dataset \`$id\` files[$n] ($url) returned HTTP 403. Do not work around it (no header spoofing, no retries); ask the source for an access route or download it by hand under a logged procedure.") ;;
    000) problems+=("Download failed: dataset \`$id\` files[$n] ($url): $(tr '\n' ' ' <"$tmp/curl.err")") ;;
    *) problems+=("Download failed: dataset \`$id\` files[$n] ($url) returned HTTP $code.") ;;
  esac
  rm -f "$out"
  return 1
}

while IFS=$'\t' read -r kind id n interval sha url; do
  case "$kind" in
    fetch)
      if [ -f "$cache/$sha" ]; then
        hits=$((hits + 1))
      elif download "$id" "$n" "$interval" "$url" "$tmp/part"; then
        mv "$tmp/part" "$cache/$sha"
        downloaded=$((downloaded + 1))
      fi
      ;;
    unpinned)
      if [ "$mode" = pin ] && download "$id" "$n" "$interval" "$url" "$tmp/part"; then
        pins+=("$id files[$n] ($url)"$'\n'"$("$astra" fetch pin "$tmp/part" | sed 's/^/  /')")
        rm -f "$tmp/part"
      fi
      ;;
    skipped) skipped+=("$id ($n)") ;;
    "") ;;
    *) problems+=("Bad plan line: $kind $id") ;;
  esac
done <"$plan"

{
  echo "## Dataset fetch ($mode)"
  echo
  echo "Downloaded: $downloaded. Cache hits: $hits."
  if [ "${#skipped[@]}" -gt 0 ]; then
    echo
    echo "### Skipped (licence not approved)"
    printf -- '- %s\n' "${skipped[@]}"
  fi
  if [ "${#pins[@]}" -gt 0 ]; then
    echo
    echo "### Pins (paste into the manifest by PR)"
    echo '```'
    printf '%s\n' "${pins[@]}"
    echo '```'
  fi
  if [ "${#problems[@]}" -gt 0 ]; then
    echo
    echo "### Problems"
    printf -- '- %s\n' "${problems[@]}"
  fi
} | tee -a "$summary"

if [ "${#problems[@]}" -gt 0 ]; then
  exit 1
fi
