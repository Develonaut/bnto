"use client";

import type { AnalyticsService } from "../services/analyticsService";

/**
 * Analytics client — public API for usage analytics and server quota.
 */
export function createAnalyticsClient(analytics: AnalyticsService) {
  return {
    // ── Query Options ─────────────────────────────────────────────
    analyticsQueryOptions: () => analytics.analyticsQueryOptions(),
    slugAggregatesQueryOptions: () => analytics.slugAggregatesQueryOptions(),
    serverQuotaQueryOptions: () => analytics.serverQuotaQueryOptions(),

    // ── Cache Invalidation ────────────────────────────────────────
    invalidateAnalytics: () => analytics.invalidateAnalytics(),
    invalidateSlugAggregates: () => analytics.invalidateSlugAggregates(),
    invalidateServerQuota: () => analytics.invalidateServerQuota(),
  } as const;
}

export type AnalyticsClient = ReturnType<typeof createAnalyticsClient>;
