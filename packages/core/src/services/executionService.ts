"use client";

import {
  getExecutionQuery,
  getExecutionsQuery,
  getExecutionLogsQuery,
  getExecutionHistoryRef,
  startExecution,
  startPredefinedExecution,
} from "../adapters/convex/executionAdapter";
import { toExecution, toExecutionLog } from "../transforms/execution";
import { getQueryClient } from "../client";
import type { StartExecutionInput, StartPredefinedInput } from "../types";
import type {
  RawExecutionDoc,
  RawExecutionLogDoc,
} from "../types/raw";

export function createExecutionService() {
  function invalidateExecution(id: string) {
    getQueryClient().invalidateQueries({
      queryKey: getExecutionQuery(id).queryKey,
    });
  }

  function invalidateExecutions(recipeId: string) {
    getQueryClient().invalidateQueries({
      queryKey: getExecutionsQuery(recipeId).queryKey,
    });
  }

  return {
    // ── Query Options ─────────────────────────────────────────────
    // Note: convexQuery returns opaque types, so select receives `unknown`.
    // The cast to Raw* types is a trust boundary — Convex docs match our
    // raw type definitions by construction (derived from the same schema).
    getQueryOptions: (id: string) => ({
      ...getExecutionQuery(id),
      select: (data: unknown) =>
        data ? toExecution(data as RawExecutionDoc) : null,
    }),

    listQueryOptions: (recipeId: string) => ({
      ...getExecutionsQuery(recipeId),
      select: (data: unknown) =>
        (data as RawExecutionDoc[]).map(toExecution),
    }),

    logsQueryOptions: (executionId: string) => ({
      ...getExecutionLogsQuery(executionId),
      select: (data: unknown) =>
        (data as RawExecutionLogDoc[]).map(toExecutionLog),
    }),

    // ── Paginated Query Refs ────────────────────────────────────────
    // For usePaginatedQuery: returns func ref, args, and transform.
    historyRefMethod: () => {
      const { funcRef, args } = getExecutionHistoryRef();
      return { funcRef, args, transform: toExecution };
    },

    // ── Mutations ─────────────────────────────────────────────────
    start: (input: StartExecutionInput) => startExecution(input),
    startPredefined: (input: StartPredefinedInput) =>
      startPredefinedExecution(input),

    // ── Cache Invalidation ────────────────────────────────────────
    invalidateExecution,
    invalidateExecutions,
  } as const;
}

export type ExecutionService = ReturnType<typeof createExecutionService>;
