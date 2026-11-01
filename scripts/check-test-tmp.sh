#!/bin/sh
# The Rust tests' temp folder. engine/.cargo/config.toml points TMP, TEMP and
# TMPDIR at engine/.tmp for every process cargo runs, so a test's temp files
# never land in the user's temp folder (Windows walks %TEMP% at every sign-in,
# and leaked test folders there once slowed logon to minutes).
#
# Usage: sh scripts/check-test-tmp.sh reset|check
#   reset  empty engine/.tmp (keeping its .gitignore) and note how many bnto*
#          entries the system temp folder holds
#   check  fail naming whatever a run left in engine/.tmp, and warn when the
#          system temp folder gained bnto* entries since `reset`
set -eu

root=$(cd "$(dirname "$0")/.." && pwd)
tmp="$root/engine/.tmp"
baseline="$tmp/.system-tmp-baseline"
# This script runs outside cargo, so these still name the real temp folder.
system_tmp=${TMPDIR:-${TMP:-${TEMP:-/tmp}}}

bnto_entries() {
  find "$system_tmp" -mindepth 1 -maxdepth 1 -name 'bnto*' 2>/dev/null | wc -l | tr -d ' '
}

# Everything in the folder but the files this script and git keep there.
run_entries() {
  find "$tmp" -mindepth 1 -maxdepth 1 ! -name .gitignore ! -name .system-tmp-baseline 2>/dev/null | sort
}

reset() {
  mkdir -p "$tmp"
  run_entries | while IFS= read -r entry; do rm -rf "$entry"; done
  bnto_entries >"$baseline"
}

check() {
  left=$(run_entries)
  if [ -n "$left" ]; then
    count=$(printf '%s\n' "$left" | wc -l | tr -d ' ')
    echo "The tests left $count entries in $tmp:" >&2
    printf '%s\n' "$left" | head -n 20 >&2
    echo "Every temp file or folder a test makes must be removed when it ends (a TempDir guard, not .keep())." >&2
    exit 1
  fi
  if [ -f "$baseline" ]; then
    before=$(cat "$baseline")
    after=$(bnto_entries)
    if [ "$after" -gt "$before" ]; then
      echo "warning: $system_tmp gained $((after - before)) bnto* entries during the run (something bypassed engine/.tmp, or another bnto ran meanwhile)" >&2
    fi
  fi
}

case ${1:-} in
  reset) reset ;;
  check) check ;;
  *)
    echo "usage: sh scripts/check-test-tmp.sh reset|check" >&2
    exit 2
    ;;
esac
