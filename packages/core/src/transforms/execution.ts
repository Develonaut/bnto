import type { RawExecutionDoc, RawExecutionLogDoc } from "../types/raw";
import type { Execution, ExecutionLog } from "../types";

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

export function toExecutionLog(doc: RawExecutionLogDoc): ExecutionLog {
  return {
    id: String(doc._id),
    executionId: String(doc.executionId),
    nodeId: doc.nodeId,
    level: doc.level,
    message: doc.message,
    timestamp: doc.timestamp,
  };
}
