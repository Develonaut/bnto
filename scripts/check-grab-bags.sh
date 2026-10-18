#!/usr/bin/env sh
# Refuse grab-bag filenames (utils.ts, helpers.ts, hooks.ts) among the given
# files. Lives in a script, not inline in lefthook.yml, because lefthook on
# Windows mangles quotes in a multi-line `run:` and the inline loop failed to parse.
for f in "$@"; do
  base=$(basename "$f")
  if echo "$base" | grep -qE '^(utils|helpers|hooks)\.(ts|tsx)$'; then
    echo "ERROR: Grab-bag filename detected: $f"
    exit 1
  fi
done
