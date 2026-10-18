#!/bin/sh
# Pre-commit: reject grab-bag filenames (utils.ts, helpers.ts, hooks.ts).
# Usage: check-grab-bags.sh <staged files...>

for f in "$@"; do
  base=$(basename "$f")
  if echo "$base" | grep -qE '^(utils|helpers|hooks)\.(ts|tsx)$'; then
    echo "ERROR: Grab-bag filename detected: $f"
    exit 1
  fi
done
