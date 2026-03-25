/** Transform a raw analytics doc into the public UsageAnalytics shape. */
import type { RawAnalyticsDoc } from "../types/raw";
import type { UsageAnalytics } from "../types/analytics";

export function toUsageAnalytics(doc: RawAnalyticsDoc): UsageAnalytics {
  return {
    plan: doc.plan === "starter" ? "free" : doc.plan,
    totalRuns: doc.totalRuns,
    lastRunAt: doc.lastRunAt,
  };
}
