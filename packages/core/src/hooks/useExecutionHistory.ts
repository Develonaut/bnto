"use client";

import { useMemo } from "react";
import { usePaginatedQuery } from "convex/react";
import { core } from "../core";
import { useReady } from "./useReady";
import type { Execution } from "../types";
import type { RawExecutionDoc } from "../types/raw";

/**
 * Paginated execution history for the current user (most recent first).
 *
 * Uses Convex native `usePaginatedQuery` for real-time per-page subscriptions.
 * Guards on `useReady()` to avoid crashes before ConvexProvider mounts.
 */
export function useExecutionHistory(options?: { pageSize?: number }) {
  const ready = useReady();
  const { pageSize = 20 } = options ?? {};
  const { funcRef, args, transform } = core.executions.historyRefMethod();

  const { results, status, loadMore, isLoading } = usePaginatedQuery(
    funcRef,
    ready ? args : "skip",
    { initialNumItems: pageSize },
  );

  const items: Execution[] = useMemo(
    () => results.map((doc) => transform(doc as unknown as RawExecutionDoc)),
    [results, transform],
  );

  return {
    items,
    isLoading: !ready || isLoading,
    status,
    loadMore,
  };
}
