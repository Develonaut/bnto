// ---------------------------------------------------------------------------
// Raw document types — shapes that Convex docs have BEFORE transforms.
//
// These describe the data shape the adapter layer provides. Transforms accept
// these types instead of importing Doc<T> from @bnto/backend.
// Only adapters know about Convex — transforms just accept plain objects.
// ---------------------------------------------------------------------------

import type { WorkflowDefinition } from "./workflow";
import type { NodeProgress, OutputFile, RunResult } from "./execution";

// ── Workflow ────────────────────────────────────────────────────────────────

/** Raw workflow document as returned by the Convex adapter. */
export interface RawWorkflowDoc {
  _id: string;
  userId: string;
  name: string;
  definition: WorkflowDefinition;
  version: number;
  isPublic: boolean;
  createdAt: number;
  updatedAt: number;
}

/** Projected workflow fields returned by list queries. */
export interface RawWorkflowListProjection {
  _id: string;
  name: string;
  nodeCount: number;
  updatedAt: number;
}

// ── Execution ───────────────────────────────────────────────────────────────

/** Raw execution document as returned by the Convex adapter. */
export interface RawExecutionDoc {
  _id: string;
  userId: string;
  workflowId?: string | null;
  status: "pending" | "running" | "completed" | "failed";
  progress: NodeProgress[];
  result?: RunResult | null;
  error?: string | null;
  outputFiles?: OutputFile[] | null;
  sessionId?: string | null;
  startedAt: number;
  completedAt?: number | null;
}

/** Raw execution log document as returned by the Convex adapter. */
export interface RawExecutionLogDoc {
  _id: string;
  executionId: string;
  nodeId: string;
  level: "info" | "warn" | "error" | "debug";
  message: string;
  timestamp: number;
}

// ── User ────────────────────────────────────────────────────────────────────

/** Raw user document as returned by the Convex adapter. */
export interface RawUserDoc {
  _id: string;
  email?: string | null;
  name?: string | null;
  image?: string | null;
  isAnonymous?: boolean | null;
  plan?: "free" | "starter" | "pro" | null;
  runsUsed?: number | null;
  runLimit?: number | null;
  runsResetAt?: number | null;
}
