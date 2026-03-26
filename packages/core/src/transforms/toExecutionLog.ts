/** Transform a raw Convex execution log doc into the public ExecutionLog shape. */
import type { RawExecutionLogDoc } from "../types/raw";
import type { ExecutionLog } from "../types";

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
