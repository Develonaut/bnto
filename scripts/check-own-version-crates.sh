#!/usr/bin/env bash
# Fail when a crate that keeps its own version (not the workspace release
# version every bnto crate shares, e.g. tonkotsu) changed without a bump.
# crates.io already has that version with other files, so the release skips
# it ("already exists") and anything that uses the change can't publish:
# v0.17.0's bnto failed to build against tonkotsu 0.1.1 that way.
#
# For each such publishable crate whose version is on crates.io, this packages
# the crate and compares its files with the published ones. A version that
# isn't published yet passes: it's the bump.
#
# Usage: scripts/check-own-version-crates.sh   (needs network; offline it
# warns and skips, or fails when CI is set)
set -euo pipefail

cd "$(dirname "$0")/../engine"
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT

# Path of a crate in crates.io's sparse index.
index_path() {
  local name=$1
  case ${#name} in
    1) echo "1/$name" ;;
    2) echo "2/$name" ;;
    3) echo "3/${name:0:1}/$name" ;;
    *) echo "${name:0:2}/${name:2:2}/$name" ;;
  esac
}

# The first `key = "value"` in the [package] table of a manifest.
package_field() {
  awk -v key="$2" -F'"' '
    /^\[package\]/ { in_package = 1; next }
    /^\[/ { in_package = 0 }
    in_package && $0 ~ "^" key " *=" { print $2; exit }
  ' "$1"
}

# Whether the packaged files of `name` differ from the published `version`.
differs_from_published() {
  local name=$1 version=$2
  local published="$work/published" local_pkg="$work/local"
  mkdir -p "$published" "$local_pkg"
  curl -sSfL "https://static.crates.io/crates/$name/$name-$version.crate" |
    tar xz -C "$published"
  cargo package -q -p "$name" --no-verify --allow-dirty
  tar xzf "target/package/$name-$version.crate" -C "$local_pkg"
  # Cargo.toml is rewritten by cargo and .cargo_vcs_info.json names the
  # commit; Cargo.toml.orig holds the manifest as written.
  ! diff -rq -x Cargo.toml -x .cargo_vcs_info.json -x Cargo.lock \
    "$published/$name-$version" "$local_pkg/$name-$version"
}

# Fetch a crate's index entry into $work/index and print the HTTP status
# ("000" when crates.io can't be reached).
fetch_index() {
  curl -sS -o "$work/index" -w '%{http_code}' \
    "https://index.crates.io/$(index_path "$1")" 2>/dev/null || true
}

# Offline, nothing can be compared. Locally that's a warning, so `task check`
# still runs on a plane; in CI the check must run, so it fails.
unreachable() {
  echo "WARNING: couldn't reach crates.io to check $1: skipped. CI runs this check." >&2
  if [[ -n ${CI:-} ]]; then failed=1; fi
}

failed=0
for manifest in crates/*/Cargo.toml; do
  grep -q '^version.workspace *= *true' "$manifest" && continue
  grep -q '^publish *= *false' "$manifest" && continue
  name=$(package_field "$manifest" name)
  version=$(package_field "$manifest" version)
  status=$(fetch_index "$name")
  case $status in
    200) ;;
    404) echo "$name isn't on crates.io yet: it will publish."; continue ;;
    *) unreachable "$name"; continue ;;
  esac
  if ! grep -q "\"vers\":\"$version\"" "$work/index"; then
    echo "$name $version isn't on crates.io yet: it will publish."
    continue
  fi
  if differs_from_published "$name" "$version"; then
    echo "$name changed since $version was published. Bump its version in $manifest" >&2
    echo "(and in engine/Cargo.toml [workspace.dependencies]) so the release publishes it." >&2
    failed=1
  else
    echo "$name $version matches crates.io."
  fi
  rm -rf "$work/published" "$work/local"
done
exit $failed
