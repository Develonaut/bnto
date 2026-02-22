"use client";

import type { WorkflowService } from "../services/workflowService";
import type { ExecutionService } from "../services/executionService";

/**
 * Workflow client — public API for workflow operations.
 *
 * Composes workflow and execution services for cross-domain orchestration.
 * Running a workflow creates an execution (cross-domain side effect).
 */
export function createWorkflowClient(
  workflows: WorkflowService,
  executions: ExecutionService,
) {
  return {
    // ── Query Options ─────────────────────────────────────────────
    listQueryOptions: () => workflows.listQueryOptions(),
    getQueryOptions: (id: string) => workflows.getQueryOptions(id),

    // ── Mutations ─────────────────────────────────────────────────
    save: (args: { name: string; definition: unknown; isPublic?: boolean }) =>
      workflows.save(args),

    remove: (id: string) => workflows.remove(id),

    /** Cross-domain: starts execution and invalidates both caches. */
    run: async (workflowId: string) => {
      const executionId = await executions.start(workflowId);
      executions.invalidateExecutions(workflowId);
      return executionId;
    },

    // ── Cache Invalidation ────────────────────────────────────────
    invalidateList: () => workflows.invalidateList(),
    invalidateWorkflow: (id: string) => workflows.invalidateWorkflow(id),
  } as const;
}

export type WorkflowClient = ReturnType<typeof createWorkflowClient>;
