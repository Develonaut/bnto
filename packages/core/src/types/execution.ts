// ---------------------------------------------------------------------------
// Execution types (transport-agnostic — no Convex imports)
// ---------------------------------------------------------------------------

/** A file produced by execution, stored in R2 for download. */
export interface OutputFile {
  key: string;
  name: string;
  sizeBytes: number;
  contentType: string;
}

export interface Execution {
  id: string;
  userId: string;
  recipeId?: string;
  status: "pending" | "running" | "completed" | "failed";
  progress: NodeProgress[];
  result?: RunResult;
  error?: string;
  outputFiles?: OutputFile[];
  sessionId?: string;
  startedAt: number;
  completedAt?: number;
}

/** Input for starting a recipe execution. */
export interface StartExecutionInput {
  recipeId: string;
  slug?: string;
  sessionId?: string;
}

/** Input for starting a predefined bnto execution (no stored recipe). */
export interface StartPredefinedInput {
  slug: string;
  definition: unknown;
  sessionId?: string;
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
