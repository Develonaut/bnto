"use client";

import {
  getAnalyticsQuery,
  getSlugAggregatesQuery,
  getServerQuotaQuery,
} from "../adapters/convex/analyticsAdapter";
import {
  toUsageAnalytics,
  toSlugAggregate,
  toServerQuota,
} from "../transforms/analytics";
import { getQueryClient } from "../client";
import type { RawAnalyticsDoc, RawSlugAggregateDoc, RawServerQuotaDoc } from "../types/raw";

export function createAnalyticsService() {
  return {
    // ── Query Options ─────────────────────────────────────────────
    // Note: convexQuery returns opaque types, so select receives `unknown`.
    // The cast to raw types is a trust boundary — Convex docs match our
    // raw type definitions by construction (derived from the same schema).

    analyticsQueryOptions: () => ({
      ...getAnalyticsQuery(),
      select: (data: unknown) =>
        data ? toUsageAnalytics(data as RawAnalyticsDoc) : null,
    }),

    slugAggregatesQueryOptions: () => ({
      ...getSlugAggregatesQuery(),
      select: (data: unknown) =>
        Array.isArray(data)
          ? data.map((d) => toSlugAggregate(d as RawSlugAggregateDoc))
          : [],
    }),

    serverQuotaQueryOptions: () => ({
      ...getServerQuotaQuery(),
      select: (data: unknown) =>
        data ? toServerQuota(data as RawServerQuotaDoc) : null,
    }),

    // ── Cache Invalidation ────────────────────────────────────────
    invalidateAnalytics: () =>
      getQueryClient().invalidateQueries({
        queryKey: getAnalyticsQuery().queryKey,
      }),

    invalidateSlugAggregates: () =>
      getQueryClient().invalidateQueries({
        queryKey: getSlugAggregatesQuery().queryKey,
      }),

    invalidateServerQuota: () =>
      getQueryClient().invalidateQueries({
        queryKey: getServerQuotaQuery().queryKey,
      }),
  } as const;
}

export type AnalyticsService = ReturnType<typeof createAnalyticsService>;
