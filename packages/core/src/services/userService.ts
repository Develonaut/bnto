"use client";

import { getCurrentUserQuery } from "../adapters/convex/userAdapter";
import { toUser } from "../transforms/user";
import { getQueryClient } from "../client";
import type { RawUserDoc } from "../types/raw";

export function createUserService() {
  function invalidateCurrentUser() {
    getQueryClient().invalidateQueries({
      queryKey: getCurrentUserQuery().queryKey,
    });
  }

  return {
    // ── Query Options ─────────────────────────────────────────────
    // Note: convexQuery returns opaque types, so select receives `unknown`.
    // The cast to RawUserDoc is a trust boundary — Convex docs match our
    // raw type definitions by construction (derived from the same schema).
    meQueryOptions: () => ({
      ...getCurrentUserQuery(),
      select: (data: unknown) =>
        data ? toUser(data as RawUserDoc) : null,
    }),

    // ── Cache Invalidation ────────────────────────────────────────
    invalidateCurrentUser,
  } as const;
}

export type UserService = ReturnType<typeof createUserService>;
