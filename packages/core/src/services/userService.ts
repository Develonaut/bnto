"use client";

import {
  getCurrentUserQuery,
  getRunsRemainingQuery,
} from "../adapters/convex/userAdapter";
import { toUser } from "../transforms/user";
import { getQueryClient } from "../client";

export function createUserService() {
  function invalidateCurrentUser() {
    getQueryClient().invalidateQueries({
      queryKey: getCurrentUserQuery().queryKey,
    });
  }

  function invalidateRunsRemaining() {
    getQueryClient().invalidateQueries({
      queryKey: getRunsRemainingQuery().queryKey,
    });
  }

  return {
    // ── Query Options ─────────────────────────────────────────────
    meQueryOptions: () => ({
      ...getCurrentUserQuery(),
      select: (data: unknown) =>
        data ? toUser(data as Parameters<typeof toUser>[0]) : null,
    }),

    runsRemainingQueryOptions: () => getRunsRemainingQuery(),

    // ── Cache Invalidation ────────────────────────────────────────
    invalidateCurrentUser,
    invalidateRunsRemaining,
  } as const;
}

export type UserService = ReturnType<typeof createUserService>;
