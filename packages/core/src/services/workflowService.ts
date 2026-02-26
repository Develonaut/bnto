"use client";

import {
  getWorkflowsQuery,
  getWorkflowQuery,
  saveWorkflow,
  removeWorkflow,
} from "../adapters/convex/workflowAdapter";
import { toWorkflow, toWorkflowListItem } from "../transforms/workflow";
import { getQueryClient } from "../client";
import type {
  RawWorkflowDoc,
  RawWorkflowListProjection,
} from "../types/raw";

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
    // Note: convexQuery returns opaque types, so select receives `unknown`.
    // The cast to Raw* types is a trust boundary — Convex docs match our
    // raw type definitions by construction (derived from the same schema).
    listQueryOptions: () => ({
      ...getWorkflowsQuery(),
      select: (data: unknown) =>
        (data as RawWorkflowListProjection[]).map(toWorkflowListItem),
    }),

    getQueryOptions: (id: string) => ({
      ...getWorkflowQuery(id),
      select: (data: unknown) =>
        data ? toWorkflow(data as RawWorkflowDoc) : null,
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
