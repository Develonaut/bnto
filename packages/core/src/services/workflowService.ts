"use client";

import {
  getWorkflowsQuery,
  getWorkflowQuery,
  saveWorkflow,
  removeWorkflow,
} from "../adapters/convex/workflowAdapter";
import { toWorkflow, toWorkflowListItem } from "../transforms/workflow";
import { getQueryClient } from "../client";

export function createWorkflowService() {
  function invalidateList() {
    getQueryClient().invalidateQueries({
      queryKey: getWorkflowsQuery().queryKey,
    });
  }

  function invalidateWorkflow(id: string) {
    getQueryClient().invalidateQueries({
      queryKey: getWorkflowQuery(id).queryKey,
    });
  }

  return {
    // ── Query Options ─────────────────────────────────────────────
    listQueryOptions: () => ({
      ...getWorkflowsQuery(),
      select: (data: unknown[]) =>
        (data as Parameters<typeof toWorkflowListItem>[0][]).map(toWorkflowListItem),
    }),

    getQueryOptions: (id: string) => ({
      ...getWorkflowQuery(id),
      select: (data: unknown) =>
        data ? toWorkflow(data as Parameters<typeof toWorkflow>[0]) : null,
    }),

    // ── Mutations ─────────────────────────────────────────────────
    save: async (args: { name: string; definition: unknown; isPublic?: boolean }) => {
      const result = await saveWorkflow(args);
      invalidateList();
      return result;
    },

    remove: async (id: string) => {
      await removeWorkflow(id);
      invalidateList();
      invalidateWorkflow(id);
    },

    // ── Cache Invalidation ────────────────────────────────────────
    invalidateList,
    invalidateWorkflow,
  } as const;
}

export type WorkflowService = ReturnType<typeof createWorkflowService>;
