"use client";

import { convexQuery } from "@convex-dev/react-query";
import { api } from "@bnto/backend/convex/_generated/api";

// ---------------------------------------------------------------------------
// Analytics Queries
// ---------------------------------------------------------------------------

/** Lifetime usage analytics (plan, totalRuns, lastRunAt). */
export function getAnalyticsQuery() {
  return convexQuery(api.analytics.getAnalytics, {});
}

/** Per-slug aggregate stats (most-used bntos). */
export function getSlugAggregatesQuery() {
  return convexQuery(api.execution_analytics.aggregateBySlug, {});
}

// ---------------------------------------------------------------------------
// Server Quota Query
// ---------------------------------------------------------------------------

/** Server-node execution quota (monthly usage/limit/remaining). */
export function getServerQuotaQuery() {
  return convexQuery(api.users.getServerQuota, {});
}
