/** Transform a raw slug aggregate doc into the public SlugAggregate shape. */
import type { RawSlugAggregateDoc } from "../types/raw";
import type { SlugAggregate } from "../types/analytics";

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
