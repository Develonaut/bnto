"use client";

import {
  getExecutionQuery,
  getExecutionsQuery,
  getExecutionLogsQuery,
  startExecution,
} from "../adapters/convex/executionAdapter";
import { toExecution, toExecutionLog } from "../transforms/execution";
import { getQueryClient } from "../client";
import type { StartExecutionInput } from "../types";

export function createExecutionService() {
  function invalidateExecution(id: string) {
    getQueryClient().invalidateQueries({
      queryKey: getExecutionQuery(id).queryKey,
    });
  }

  function invalidateExecutions(workflowId: string) {
    getQueryClient().invalidateQueries({
      queryKey: getExecutionsQuery(workflowId).queryKey,
    });
  }

  return {
    // ── Query Options ─────────────────────────────────────────────
    getQueryOptions: (id: string) => ({
      ...getExecutionQuery(id),
      select: (data: unknown) =>
        data ? toExecution(data as Parameters<typeof toExecution>[0]) : null,
    }),

    listQueryOptions: (workflowId: string) => ({
      ...getExecutionsQuery(workflowId),
      select: (data: unknown[]) =>
        (data as Parameters<typeof toExecution>[0][]).map(toExecution),
    }),

    logsQueryOptions: (executionId: string) => ({
      ...getExecutionLogsQuery(executionId),
      select: (data: unknown[]) =>
        (data as Parameters<typeof toExecutionLog>[0][]).map(toExecutionLog),
    }),

    // ── Mutations ─────────────────────────────────────────────────
    start: (input: StartExecutionInput) => startExecution(input),

    // ── Cache Invalidation ────────────────────────────────────────
    invalidateExecution,
    invalidateExecutions,
  } as const;
}

export type ExecutionService = ReturnType<typeof createExecutionService>;
