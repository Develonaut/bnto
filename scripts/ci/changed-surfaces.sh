#!/bin/sh
# Which surfaces a set of changed files touches, for CI and release to pick
# the jobs to run (ISS-38). Changed paths come on stdin, one per line; prints
# `engine=`, `web=` and `convex=` lines for $GITHUB_OUTPUT.
# Usage: git diff --name-only <base> HEAD | sh scripts/ci/changed-surfaces.sh [--release]
#
# The one list both workflows use. Each surface lists what its jobs read,
# root files included; scripts/ci/changed-surfaces.test.ts checks that every
# `$TURBO_ROOT$` input and every root path the engine's Rust code reads is here.

# A change to CI or to this list runs every job it could break. --release
# leaves this out: a release doesn't run ci.yml.
SELF='^(\.github/workflows/ci\.yml|scripts/ci/changed-surfaces\.sh)$'

# The Rust jobs: the engine, the dogfood recipes its tests run, and the
# shared fixtures its tests read.
ENGINE='^(engine|recipes|test-fixtures)/'

# The TypeScript job: the packages, the workspace and its task graph, and the
# root files Turbo tasks read (`$TURBO_ROOT$` inputs). @bnto/core's
# integration tests read test-fixtures/ too, and the job runs scripts/turbo/.
WEB='^(apps/|packages/|test-fixtures/|scripts/turbo/|pnpm-lock\.yaml$|package\.json$|tsconfig[^/]*\.json$|turbo\.json$|pnpm-workspace\.yaml$)'
WEB_ROOT_INPUTS='^(eslint\.config\.base\.mjs|README\.md|\.prettierrc|engine/Cargo\.toml|engine/catalog\.snapshot\.json|engine/crates/bnto-core/src/lib\.rs)$'

CONVEX='^packages/@bnto/backend/convex/'

mode=${1:-}
changed=$(cat)

matches() { printf '%s\n' "$changed" | grep -qE "$@"; }
ci_self() { [ "$mode" != --release ] && matches -e "$SELF"; }

engine=false
web=false
convex=false
if ci_self || matches -e "$ENGINE"; then engine=true; fi
if ci_self || matches -e "$WEB" -e "$WEB_ROOT_INPUTS"; then web=true; fi
if matches -e "$CONVEX"; then convex=true; fi
printf 'engine=%s\nweb=%s\nconvex=%s\n' "$engine" "$web" "$convex"
