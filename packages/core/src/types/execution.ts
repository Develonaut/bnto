// ---------------------------------------------------------------------------
// Execution types (transport-agnostic — no Convex imports)
// ---------------------------------------------------------------------------

export interface Execution {
  id: string;
  userId: string;
  workflowId: string;
  status: "pending" | "running" | "completed" | "failed";
  progress: NodeProgress[];
  result?: RunResult;
  error?: string;
  startedAt: number;
  completedAt?: number;
}

export interface ExecutionLog {
  id: string;
  executionId: string;
  nodeId: string;
  level: "info" | "warn" | "error" | "debug";
  message: string;
  timestamp: number;
}

export interface NodeProgress {
  nodeId: string;
  status: string;
}

export interface RunResult {
  status: string;
  nodesExecuted: number;
  nodeOutputs: Record<string, unknown>;
  duration: number;
  error?: string;
}
