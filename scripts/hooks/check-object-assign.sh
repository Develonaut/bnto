#!/bin/sh
# Pre-commit: reject Object.assign compound components, which break RSC.
# Usage: check-object-assign.sh <staged files...>

if grep -n 'Object\.assign' "$@" 2>/dev/null; then
  echo "ERROR: Object.assign compound patterns break RSC. Use flat named exports."
  exit 1
fi
