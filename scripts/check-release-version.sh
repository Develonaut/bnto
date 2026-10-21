#!/usr/bin/env bash
# Fail unless a stable release tag matches the engine's release version
# (`[workspace.package] version` in engine/Cargo.toml, which every bnto crate
# inherits). Without this, a release whose crates weren't bumped skips
# crates.io silently ("already exists") while the site ships the new tag.
#
# Usage: scripts/check-release-version.sh v0.16.1
# Pre-release tags (v1.0.0-beta.1) pass: they don't publish crates.
set -euo pipefail

TAG="${1:?usage: $0 vX.Y.Z}"
WORKSPACE="engine/Cargo.toml"

if [[ "$TAG" == *-* ]]; then
  echo "$TAG is a pre-release: crates aren't published, version not checked."
  exit 0
fi

version=$(awk '
  /^\[workspace\.package\]/ { in_package = 1; next }
  /^\[/ { in_package = 0 }
  in_package && /^version *=/ { gsub(/^version *= *"|".*$/, ""); print; exit }
' "$WORKSPACE")

if [[ -z "$version" ]]; then
  echo "No [workspace.package] version in $WORKSPACE" >&2
  exit 1
fi

if [[ "v$version" != "$TAG" ]]; then
  echo "Tag $TAG doesn't match the engine release version $version." >&2
  echo "Bump it first: task version:bump -- ${TAG#v}  (then merge that PR and tag)" >&2
  exit 1
fi

echo "Tag $TAG matches the engine release version."
