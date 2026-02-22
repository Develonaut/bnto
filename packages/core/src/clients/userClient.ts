"use client";

import type { UserService } from "../services/userService";

/**
 * User client — public API for user operations.
 */
export function createUserClient(user: UserService) {
  return {
    // ── Query Options ─────────────────────────────────────────────
    meQueryOptions: () => user.meQueryOptions(),
    runsRemainingQueryOptions: () => user.runsRemainingQueryOptions(),

    // ── Cache Invalidation ────────────────────────────────────────
    invalidateCurrentUser: () => user.invalidateCurrentUser(),
    invalidateRunsRemaining: () => user.invalidateRunsRemaining(),
  } as const;
}

export type UserClient = ReturnType<typeof createUserClient>;
