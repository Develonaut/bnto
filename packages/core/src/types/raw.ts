// ---------------------------------------------------------------------------
// Raw document types — shapes that Convex docs have BEFORE transforms.
//
// These describe the data shape the adapter layer provides. Transforms accept
// these types instead of importing Doc<T> from @bnto/backend.
// Only adapters know about Convex — transforms just accept plain objects.
// ---------------------------------------------------------------------------

import type { RecipeDefinition } from "./recipe";
import type { NodeProgress, OutputFile, RunResult } from "./execution";

// ── Recipe ─────────────────────────────────────────────────────────────────

/** Raw recipe document as returned by the Convex adapter. */
export interface RawRecipeDoc {
  _id: string;
  userId: string;
  name: string;
  definition: RecipeDefinition;
  version: number;
  /** Definition format version (semver) — tracks which format spec the definition uses. */
  formatVersion?: string;
  isPublic: boolean;
  createdAt: number;
  updatedAt: number;
}

/** Projected recipe fields returned by list queries. */
export interface RawRecipeListProjection {
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
  recipeId?: string | null;
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

// ── Execution Event ─────────────────────────────────────────────────────────

/** Raw execution event document as returned by the Convex adapter. */
export interface RawExecutionEventDoc {
  _id: string;
  userId: string;
  slug: string;
  timestamp: number;
  durationMs?: number | null;
  status: "started" | "completed" | "failed";
  executionId?: string | null;
}

// ── User ────────────────────────────────────────────────────────────────────

/** Raw user document as returned by the Convex adapter. */
export interface RawUserDoc {
  _id: string;
  email?: string | null;
  name?: string | null;
  image?: string | null;
  plan?: "free" | "starter" | "pro" | null;
  totalRuns: number;
  lastRunAt?: number | null;
}

/** Raw analytics response from `analytics.getAnalytics`. */
export interface RawAnalyticsDoc {
  plan: "free" | "starter" | "pro";
  totalRuns: number;
  lastRunAt: number | null;
}

/** Raw per-slug aggregate from `execution_analytics.aggregateBySlug`. */
export interface RawSlugAggregateDoc {
  slug: string;
  totalRuns: number;
  completedRuns: number;
  failedRuns: number;
  avgDurationMs: number | null;
  lastRunAt: number;
}
