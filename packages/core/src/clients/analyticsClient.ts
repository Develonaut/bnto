"use client";

import type { AnalyticsService } from "../services/analyticsService";

/**
 * Analytics client — public API for usage analytics.
 */
export function createAnalyticsClient(analytics: AnalyticsService) {
  return {
    // ── Query Options ─────────────────────────────────────────────
    analyticsQueryOptions: () => analytics.analyticsQueryOptions(),
    slugAggregatesQueryOptions: () => analytics.slugAggregatesQueryOptions(),

    // ── Cache Invalidation ────────────────────────────────────────
    invalidateAnalytics: () => analytics.invalidateAnalytics(),
    invalidateSlugAggregates: () => analytics.invalidateSlugAggregates(),
  } as const;
}

export type AnalyticsClient = ReturnType<typeof createAnalyticsClient>;
