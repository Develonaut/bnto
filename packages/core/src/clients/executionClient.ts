"use client";

import type { ExecutionService } from "../services/executionService";
import type { HistoryService } from "../services/historyService";
import type { StartPredefinedInput } from "../types";

/**
 * Execution client — composes execution service with history service.
 * History routing (server vs local) is internal to the history service.
 */
export function createExecutionClient(
  executions: ExecutionService,
  history: HistoryService,
) {
  return {
    getQueryOptions: (id: string) => executions.getQueryOptions(id),
    listQueryOptions: (workflowId: string) => executions.listQueryOptions(workflowId),
    logsQueryOptions: (executionId: string) => executions.logsQueryOptions(executionId),
    historyRefMethod: () => history.serverRef(),
    localHistoryQueryOptions: () => history.localQueryOptions(),
    clearLocalHistory: () => history.clear(),
    startPredefined: (input: StartPredefinedInput) => executions.startPredefined(input),
    invalidateExecution: (id: string) => executions.invalidateExecution(id),
    invalidateExecutions: (workflowId: string) => executions.invalidateExecutions(workflowId),
  } as const;
}

export type ExecutionClient = ReturnType<typeof createExecutionClient>;
