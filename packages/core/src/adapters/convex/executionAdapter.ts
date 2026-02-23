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
  return convexQuery(api.executions.get, { id: id as Id<"executions"> });
}

export function getExecutionsQuery(workflowId: string) {
  return convexQuery(api.executions.listByWorkflow, {
    workflowId: workflowId as Id<"workflows">,
  });
}

export function getExecutionLogsQuery(executionId: string) {
  return convexQuery(api.executionLogs.list, {
    executionId: executionId as Id<"executions">,
  });
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
