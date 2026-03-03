import type { LocalHistoryEntry } from "../types/localHistory";
import type { Execution } from "../types";

/**
 * Map a local history entry to the transport-agnostic Execution type.
 * Fields absent from local history get defaults or undefined.
 */
export function localEntryToExecution(entry: LocalHistoryEntry): Execution {
  return {
    id: entry.id,
    userId: "local",
    recipeId: undefined,
    slug: entry.slug,
    status: entry.status,
    progress: [],
    result: {
      status: entry.status,
      nodesExecuted: 1,
      nodeOutputs: {},
      duration: entry.durationMs,
      error: entry.error,
    },
    error: entry.error,
    outputFiles: undefined,
    sessionId: undefined,
    startedAt: entry.timestamp,
    completedAt: entry.timestamp + entry.durationMs,
    source: "local",
  };
}
