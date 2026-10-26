#!/bin/sh
# Pre-commit: run Turbo with GIT_DIR unset (ISS-39). Git sets GIT_DIR for
# hooks in a worktree, and Turbo then leaves root files such as
# eslint.config.base.mjs out of its hashes and replays stale cached results.
# Usage: turbo.sh <turbo args...>

unset GIT_DIR
exec pnpm turbo "$@"
