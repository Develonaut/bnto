/** Transform a raw Convex execution doc into the public Execution shape. */
import type { RawExecutionDoc } from "../types/raw";
import type { Execution } from "../types";

export function toExecution(doc: RawExecutionDoc): Execution {
  return {
    id: String(doc._id),
    userId: String(doc.userId),
    recipeId: doc.recipeId ? String(doc.recipeId) : undefined,
    status: doc.status,
    progress: doc.progress,
    result: doc.result ?? undefined,
    error: doc.error ?? undefined,
    outputFiles: doc.outputFiles ?? undefined,
    sessionId: doc.sessionId ?? undefined,
    startedAt: doc.startedAt,
    completedAt: doc.completedAt ?? undefined,
  };
}
