#!/usr/bin/env bash
# Set the engine release version: `[workspace.package] version` and the
# bnto-* entries in `[workspace.dependencies]` (engine/Cargo.toml), then
# refresh Cargo.lock. tonkotsu keeps its own version.
#
# Usage: scripts/bump-version.sh 0.16.1
set -euo pipefail

VERSION="${1:?usage: $0 X.Y.Z}"
WORKSPACE="engine/Cargo.toml"

if ! [[ "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z.]+)?$ ]]; then
  echo "Not a semver version: $VERSION (no leading v)" >&2
  exit 1
fi

# -i.bak works with both GNU and BSD sed.
sed -i.bak -E \
  -e '/^\[workspace\.package\]/,/^\[/ s/^version = "[^"]+"/version = "'"$VERSION"'"/' \
  -e 's/^(bnto-[a-z]+ = \{ path = "crates\/bnto-[a-z]+", version = )"[^"]+"/\1"'"$VERSION"'"/' \
  "$WORKSPACE"
rm "$WORKSPACE.bak"

cargo update --workspace --manifest-path "$WORKSPACE" --quiet
echo "Engine release version is now $VERSION."
