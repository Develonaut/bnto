import type { Doc } from "@bnto/backend/convex/_generated/dataModel";
import type { Execution, ExecutionLog } from "../types";

type ExecutionDoc = Doc<"executions">;
type ExecutionLogDoc = Doc<"executionLogs">;

export function toExecution(doc: ExecutionDoc): Execution {
  return {
    id: String(doc._id),
    userId: String(doc.userId),
    workflowId: doc.workflowId ? String(doc.workflowId) : undefined,
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

export function toExecutionLog(doc: ExecutionLogDoc): ExecutionLog {
  return {
    id: String(doc._id),
    executionId: String(doc.executionId),
    nodeId: doc.nodeId,
    level: doc.level,
    message: doc.message,
    timestamp: doc.timestamp,
  };
}
