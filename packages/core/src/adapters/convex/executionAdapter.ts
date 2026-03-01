"use client";

import { convexQuery } from "@convex-dev/react-query";
import { api } from "@bnto/backend/convex/_generated/api";
import type { Id } from "@bnto/backend/convex/_generated/dataModel";
import { getConvexClient } from "../../client";
import type { StartExecutionInput, StartPredefinedInput } from "../../types";

// ---------------------------------------------------------------------------
// Queries
// ---------------------------------------------------------------------------

export function getExecutionQuery(id: string) {
  return convexQuery(api.executions.get, id ? { id: id as Id<"executions"> } : "skip");
}

export function getExecutionsQuery(workflowId: string) {
  return convexQuery(
    api.executions.listByWorkflow,
    workflowId ? { workflowId: workflowId as Id<"workflows"> } : "skip",
  );
}

export function getExecutionLogsQuery(executionId: string) {
  return convexQuery(
    api.executionLogs.list,
    executionId ? { executionId: executionId as Id<"executions"> } : "skip",
  );
}

// ---------------------------------------------------------------------------
// Paginated Queries (for usePaginatedQuery — returns func ref + args)
// ---------------------------------------------------------------------------

/** Returns the Convex function ref and empty args for paginated user history. */
export function getExecutionHistoryRef() {
  return {
    funcRef: api.executions.listByUser,
    args: {},
  } as const;
}

// ---------------------------------------------------------------------------
// Mutations (imperative — no React hooks)
// ---------------------------------------------------------------------------

export function startExecution(input: StartExecutionInput) {
  return getConvexClient().mutation(api.executions.start, {
    workflowId: input.workflowId as Id<"workflows">,
    slug: input.slug,
    sessionId: input.sessionId,
  });
}

export function startPredefinedExecution(input: StartPredefinedInput) {
  return getConvexClient().mutation(api.executions.startPredefined, {
    slug: input.slug,
    definition: input.definition,
    sessionId: input.sessionId,
  });
}
