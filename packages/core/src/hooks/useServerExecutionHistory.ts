"use client";

import { useMemo } from "react";
import { usePaginatedQuery } from "convex/react";
import { core } from "../core";
import { useReady } from "./useReady";
import type { Execution } from "../types";
import type { RawExecutionDoc } from "../types/raw";

/**
 * Convex-backed execution history (authenticated users).
 * Guards on `useReady()` to avoid crashes before ConvexProvider mounts.
 */
export function useServerExecutionHistory(options?: {
  pageSize?: number;
  enabled?: boolean;
}) {
  const ready = useReady();
  const { pageSize = 20, enabled = true } = options ?? {};
  const { funcRef, args, transform } = core.executions.historyRefMethod();

  const { results, status, loadMore, isLoading } = usePaginatedQuery(
    funcRef,
    ready && enabled ? args : "skip",
    { initialNumItems: pageSize },
  );

  const items: Execution[] = useMemo(
    () =>
      enabled
        ? results.map((doc) => transform(doc as unknown as RawExecutionDoc))
        : [],
    [results, transform, enabled],
  );

  return {
    items,
    isLoading: !ready || isLoading,
    status,
    loadMore,
  };
}
