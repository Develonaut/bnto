#!/bin/sh
# Which surfaces a set of changed files touches, for CI and release to pick
# the jobs to run (ISS-38). Changed paths come on stdin, one per line; prints
# `engine=`, `web=` and `convex=` lines for $GITHUB_OUTPUT.
# Usage: git diff --name-only <base> HEAD | scripts/ci/changed-surfaces.sh
#
# The one list both workflows use. Each surface lists what its jobs read,
# root files included; scripts/ci/changed-surfaces.test.ts checks that every
# `$TURBO_ROOT$` input and every root path the engine's Rust code reads is here.

# A change to CI or to this list runs every job it could break.
SELF='^(\.github/workflows/ci\.yml|scripts/ci/changed-surfaces\.sh)$'

# The Rust jobs: the engine, the dogfood recipes its tests run, and the
# shared fixtures its tests read.
ENGINE='^(engine|recipes|test-fixtures)/'

# The TypeScript job: the packages, the workspace and its task graph, and the
# root files Turbo tasks read (`$TURBO_ROOT$` inputs). @bnto/core's
# integration tests read test-fixtures/ too.
WEB='^(apps/|packages/|test-fixtures/|pnpm-lock\.yaml$|package\.json$|tsconfig[^/]*\.json$|turbo\.json$|pnpm-workspace\.yaml$)'
WEB_ROOT_INPUTS='^(eslint\.config\.base\.mjs|README\.md|\.prettierrc|engine/Cargo\.toml|engine/catalog\.snapshot\.json|engine/crates/bnto-core/src/lib\.rs)$'

CONVEX='^packages/@bnto/backend/convex/'

changed=$(cat)

touches() {
  if printf '%s\n' "$changed" | grep -qE "$@"; then echo true; else echo false; fi
}

echo "engine=$(touches -e "$SELF" -e "$ENGINE")"
echo "web=$(touches -e "$SELF" -e "$WEB" -e "$WEB_ROOT_INPUTS")"
echo "convex=$(touches -e "$CONVEX")"
