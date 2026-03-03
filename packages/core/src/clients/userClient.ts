"use client";

import type { UserService } from "../services/userService";
import type { AnalyticsService } from "../services/analyticsService";

/**
 * User client — public API for user profile and usage analytics.
 *
 * Usage stats are user-scoped data — one namespace for profile + analytics.
 */
export function createUserClient(
  user: UserService,
  analytics: AnalyticsService,
) {
  return {
    // ── Query Options ─────────────────────────────────────────────
    meQueryOptions: () => user.meQueryOptions(),

    /** Usage statistics for the current user. */
    usageQueryOptions: () => analytics.analyticsQueryOptions(),
    /** Per-recipe aggregates for the current user. */
    slugAggregatesQueryOptions: () => analytics.slugAggregatesQueryOptions(),

    // ── Cache Invalidation ────────────────────────────────────────
    invalidateCurrentUser: () => user.invalidateCurrentUser(),
    invalidateUsage: () => analytics.invalidateAnalytics(),
    invalidateSlugAggregates: () => analytics.invalidateSlugAggregates(),
  } as const;
}

export type UserClient = ReturnType<typeof createUserClient>;
