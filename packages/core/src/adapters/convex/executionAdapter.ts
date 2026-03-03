"use client";

import { convexQuery } from "@convex-dev/react-query";
import { api } from "@bnto/backend/convex/_generated/api";
import type { Id } from "@bnto/backend/convex/_generated/dataModel";
import { getConvexClient } from "../../client";
import type { StartExecutionInput, StartPredefinedInput } from "../../types";
import type { LocalHistoryEntry } from "../../types/localHistory";

// ---------------------------------------------------------------------------
// Queries
// ---------------------------------------------------------------------------

export function getExecutionQuery(id: string) {
  return convexQuery(api.executions.get, id ? { id: id as Id<"executions"> } : "skip");
}

export function getExecutionsQuery(recipeId: string) {
  return convexQuery(
    api.executions.listByRecipe,
    recipeId ? { recipeId: recipeId as Id<"recipes"> } : "skip",
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

/** Returns the Convex function ref and args for paginated user execution history. */
export function getExecutionHistoryRef() {
  return {
    funcRef: api.execution_events.listByUserPaginated,
    args: {},
  } as const;
}

// ---------------------------------------------------------------------------
// Mutations (imperative — no React hooks)
// ---------------------------------------------------------------------------

export function startExecution(input: StartExecutionInput) {
  return getConvexClient().mutation(api.executions.start, {
    recipeId: input.recipeId as Id<"recipes">,
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

// ---------------------------------------------------------------------------
// Execution Event Mutations (browser execution history for authed users)
// ---------------------------------------------------------------------------

/** Log the start of a browser execution to Convex. Returns event ID or null. */
export async function logExecutionEventStart(slug: string) {
  return getConvexClient().mutation(api.execution_events.logStart, { slug });
}

/** Update an execution event with completion status and duration. */
export async function completeExecutionEvent(
  eventId: string,
  durationMs: number,
  status: "completed" | "failed",
) {
  return getConvexClient().mutation(api.execution_events.completeEvent, {
    eventId: eventId as Id<"executionEvents">,
    durationMs,
    status,
  });
}

/** Batch-migrate local history entries to Convex execution events. */
export async function migrateLocalHistory(entries: LocalHistoryEntry[]) {
  return getConvexClient().mutation(api.execution_events.migrateFromLocal, {
    entries: entries.map((e) => ({
      slug: e.slug,
      timestamp: e.timestamp,
      durationMs: e.durationMs,
      status: e.status,
    })),
  });
}
