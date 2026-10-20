#!/usr/bin/env bash
# build-lock.sh — run a command only when one of N local build slots is free.
#
# All agents share one machine (10 cores, 16 GB). The charter caps local
# builds/test runs at 2 at once (part 6, T6-A10-2). Wrap every local build,
# test or check with this script:
#
#   scripts/build-lock.sh cargo test
#   scripts/build-lock.sh .github/checks/check-rust-lints.sh --callback
#
# A slot is a directory under $ASTRA_BUILD_LOCK_DIR (mkdir is atomic on every
# filesystem, unlike flock which macOS lacks). The slot holds the owner's PID;
# a slot whose PID is dead is reclaimed. The slot is released on exit, on
# Ctrl-C and on kill (trap).
#
# Environment:
#   ASTRA_BUILD_LOCK_DIR      default /tmp/astra-build-lock
#   ASTRA_BUILD_SLOTS         default 2
#   ASTRA_BUILD_LOCK_TIMEOUT  seconds to wait before giving up (default 0 =
#                             wait forever); exit 75 on timeout.
#
# Exit code: the wrapped command's, or 75 on timeout, 2 on usage error.
set -euo pipefail

if [ $# -eq 0 ]; then
  echo "usage: $0 <command> [args...]" >&2
  exit 2
fi

lock_dir="${ASTRA_BUILD_LOCK_DIR:-/tmp/astra-build-lock}"
slots="${ASTRA_BUILD_SLOTS:-2}"
timeout="${ASTRA_BUILD_LOCK_TIMEOUT:-0}"
mkdir -p "$lock_dir"

held=""
release() { if [ -n "$held" ]; then rm -rf "$held"; held=""; fi; }
trap release EXIT
trap 'release; exit 130' INT
trap 'release; exit 143' TERM

try_take() {
  local i slot pid
  for i in $(seq 1 "$slots"); do
    slot="$lock_dir/slot-$i"
    if mkdir "$slot" 2>/dev/null; then
      echo "$$" >"$slot/pid"
      held="$slot"
      return 0
    fi
    # Reclaim a slot whose owner died without cleaning up.
    pid="$(cat "$slot/pid" 2>/dev/null || true)"
    if [ -n "$pid" ] && ! kill -0 "$pid" 2>/dev/null; then
      rm -rf "$slot"
      if mkdir "$slot" 2>/dev/null; then
        echo "$$" >"$slot/pid"
        held="$slot"
        return 0
      fi
    fi
  done
  return 1
}

waited=0
until try_take; do
  if [ $((waited % 10)) -eq 0 ]; then
    owners="$(cat "$lock_dir"/slot-*/pid 2>/dev/null | tr '\n' ' ' || true)"
    echo "build-lock: all $slots slots busy (PIDs: ${owners:-?}); waiting... (${waited}s)" >&2
  fi
  if [ "$timeout" -gt 0 ] && [ "$waited" -ge "$timeout" ]; then
    echo "build-lock: gave up after ${waited}s" >&2
    exit 75
  fi
  sleep 1
  waited=$((waited + 1))
done

echo "build-lock: took ${held##*/} for: $*" >&2
status=0
"$@" || status=$?
release
exit "$status"
