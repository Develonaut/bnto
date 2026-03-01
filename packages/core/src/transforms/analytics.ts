import type {
  RawAnalyticsDoc,
  RawSlugAggregateDoc,
  RawServerQuotaDoc,
} from "../types/raw";
import type {
  UsageAnalytics,
  SlugAggregate,
  ServerQuota,
} from "../types/analytics";

/** Map legacy "starter" plan to "free" — one paid tier now. */
function normalizePlan(plan: "free" | "starter" | "pro"): "free" | "pro" {
  return plan === "starter" ? "free" : plan;
}

export function toUsageAnalytics(doc: RawAnalyticsDoc): UsageAnalytics {
  return {
    plan: normalizePlan(doc.plan),
    totalRuns: doc.totalRuns,
    lastRunAt: doc.lastRunAt,
  };
}

export function toSlugAggregate(doc: RawSlugAggregateDoc): SlugAggregate {
  return {
    slug: doc.slug,
    totalRuns: doc.totalRuns,
    completedRuns: doc.completedRuns,
    failedRuns: doc.failedRuns,
    avgDurationMs: doc.avgDurationMs,
    lastRunAt: doc.lastRunAt,
  };
}

export function toServerQuota(doc: RawServerQuotaDoc): ServerQuota {
  return {
    serverRunsUsed: doc.serverRunsUsed,
    serverRunLimit: doc.serverRunLimit,
    serverRunsRemaining: doc.serverRunsRemaining,
  };
}
