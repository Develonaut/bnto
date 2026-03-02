// ---------------------------------------------------------------------------
// Analytics types (transport-agnostic — no Convex imports)
// ---------------------------------------------------------------------------

/** Lifetime usage analytics for the current user. */
export interface UsageAnalytics {
  plan: "free" | "pro";
  totalRuns: number;
  lastRunAt: number | null;
}

/** Per-slug aggregate stats from execution events. */
export interface SlugAggregate {
  slug: string;
  totalRuns: number;
  completedRuns: number;
  failedRuns: number;
  avgDurationMs: number | null;
  lastRunAt: number;
}

