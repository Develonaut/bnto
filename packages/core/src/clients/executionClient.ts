"use client";

import type { ExecutionService } from "../services/executionService";
import type { StartPredefinedInput } from "../types";

/**
 * Execution client — public API for execution operations.
 */
export function createExecutionClient(executions: ExecutionService) {
  return {
    // ── Query Options ─────────────────────────────────────────────
    getQueryOptions: (id: string) => executions.getQueryOptions(id),
    listQueryOptions: (workflowId: string) => executions.listQueryOptions(workflowId),
    logsQueryOptions: (executionId: string) => executions.logsQueryOptions(executionId),

    // ── Paginated Query Refs ────────────────────────────────────────
    historyRefMethod: () => executions.historyRefMethod(),

    // ── Mutations ─────────────────────────────────────────────────
    startPredefined: (input: StartPredefinedInput) =>
      executions.startPredefined(input),

    // ── Cache Invalidation ────────────────────────────────────────
    invalidateExecution: (id: string) => executions.invalidateExecution(id),
    invalidateExecutions: (workflowId: string) => executions.invalidateExecutions(workflowId),
  } as const;
}

export type ExecutionClient = ReturnType<typeof createExecutionClient>;
