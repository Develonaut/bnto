#!/usr/bin/env bash
# Validate that .github/workflows/release.yml publishes exactly the set of
# publishable crates in the engine workspace, in valid dependency order.
#
# Catches drift like: adding a new crate but forgetting to add it to the
# release workflow, or referencing a crate that was removed/renamed.
set -euo pipefail

WORKSPACE="engine/Cargo.toml"
WORKFLOW=".github/workflows/release.yml"

# --- 1. Extract workspace members from Cargo.toml ---
# Parses the members array, strips quotes/commas/whitespace
workspace_members=()
in_members=false
while IFS= read -r line; do
  if [[ "$line" =~ ^members ]]; then
    in_members=true
    continue
  fi
  if $in_members; then
    [[ "$line" =~ ^\] ]] && break
    member=$(echo "$line" | sed 's/[",]//g' | xargs)
    [[ -n "$member" ]] && workspace_members+=("$member")
  fi
done < "$WORKSPACE"

# --- 2. Filter to publishable crates (no `publish = false`) ---
publishable=()
for member in "${workspace_members[@]}"; do
  cargo_toml="engine/$member/Cargo.toml"
  if [[ ! -f "$cargo_toml" ]]; then
    echo "ERROR: Workspace member $member has no Cargo.toml at $cargo_toml"
    exit 1
  fi
  # Skip crates that opt out of publishing
  if grep -q 'publish = false' "$cargo_toml"; then
    continue
  fi
  # Extract the package name (may differ from directory name)
  pkg_name=$(grep '^name' "$cargo_toml" | head -1 | sed 's/name = "//;s/"//')
  publishable+=("$pkg_name")
done

# --- 3. Extract crates listed in the release workflow ---
# Looks for lines matching `publish <crate-name>` in the cargo-publish job
workflow_crates=()
while IFS= read -r line; do
  crate=$(echo "$line" | sed 's/.*publish //;s/ .*//' | xargs)
  [[ -n "$crate" ]] && workflow_crates+=("$crate")
done < <(grep -E '^\s+publish [a-z]' "$WORKFLOW")

# --- 4. Compare: every publishable crate must be in the workflow ---
errors=0

for crate in "${publishable[@]}"; do
  found=false
  for wf_crate in "${workflow_crates[@]}"; do
    [[ "$crate" == "$wf_crate" ]] && found=true && break
  done
  if ! $found; then
    echo "MISSING from release workflow: $crate"
    ((errors++))
  fi
done

# --- 5. Compare: every workflow crate must be a publishable workspace member ---
for wf_crate in "${workflow_crates[@]}"; do
  found=false
  for crate in "${publishable[@]}"; do
    [[ "$wf_crate" == "$crate" ]] && found=true && break
  done
  if ! $found; then
    echo "STALE in release workflow: $wf_crate (not a publishable workspace member)"
    ((errors++))
  fi
done

if [[ $errors -gt 0 ]]; then
  echo ""
  echo "Publishable crates: ${publishable[*]}"
  echo "Workflow crates:    ${workflow_crates[*]}"
  echo ""
  echo "Fix .github/workflows/release.yml to match the workspace."
  exit 1
fi

echo "Crate publish list is in sync (${#publishable[@]} crates)."
