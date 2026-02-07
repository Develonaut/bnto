"use client";

import { convexQuery, useConvexMutation } from "@convex-dev/react-query";
import { api } from "@bento/backend/convex/_generated/api";
import type { WorkflowId, ExecutionId } from "../types";

// ---------------------------------------------------------------------------
// Query option factories — each returns a convexQuery(...) result that can be
// spread into useQuery() / useSuspenseQuery().
// ---------------------------------------------------------------------------

export function workflowsQueryOptions() {
  return convexQuery(api.workflows.list, {});
}

export function workflowQueryOptions(id: WorkflowId) {
  return convexQuery(api.workflows.get, { id });
}

export function executionQueryOptions(id: ExecutionId) {
  return convexQuery(api.executions.get, { id });
}

export function executionsQueryOptions(workflowId: WorkflowId) {
  return convexQuery(api.executions.listByWorkflow, { workflowId });
}

export function executionLogsQueryOptions(executionId: ExecutionId) {
  return convexQuery(api.executionLogs.list, { executionId });
}

export function runsRemainingQueryOptions() {
  return convexQuery(api.users.getRunsRemaining, {});
}

export function currentUserQueryOptions() {
  return convexQuery(api.users.getMe, {});
}

// ---------------------------------------------------------------------------
// Mutation hook factories — each returns a mutate function from Convex.
// Called inside hooks, which wrap them in useMutation().
// ---------------------------------------------------------------------------

export function useSaveWorkflowMutation() {
  return useConvexMutation(api.workflows.save);
}

export function useRemoveWorkflowMutation() {
  return useConvexMutation(api.workflows.remove);
}

export function useStartExecutionMutation() {
  return useConvexMutation(api.executions.start);
}
