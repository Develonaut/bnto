import type { RawAnalyticsDoc, RawSlugAggregateDoc } from "../types/raw";
import type { UsageAnalytics, SlugAggregate } from "../types/analytics";

export function toUsageAnalytics(doc: RawAnalyticsDoc): UsageAnalytics {
  return {
    plan: doc.plan === "starter" ? "free" : doc.plan,
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
