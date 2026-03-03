import type { RawExecutionEventDoc } from "../types/raw";
import type { Execution } from "../types";

/**
 * Map a Convex executionEvent document to the transport-agnostic Execution type.
 * ExecutionEvents are lightweight (analytics/billing), so some fields get defaults.
 */
export function toExecutionFromEvent(doc: RawExecutionEventDoc): Execution {
  const status =
    doc.status === "started" ? ("running" as const) : (doc.status as "completed" | "failed");

  return {
    id: String(doc._id),
    userId: String(doc.userId),
    slug: doc.slug,
    status,
    progress: [],
    startedAt: doc.timestamp,
    completedAt: doc.durationMs ? doc.timestamp + doc.durationMs : undefined,
    error: undefined,
    source: "server",
  };
}
