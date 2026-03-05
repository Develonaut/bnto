"use client";

import { useQuery } from "@tanstack/react-query";
import { core } from "../core";
import { useReady } from "./useReady";
import { localEntryToExecution } from "../transforms/localHistory";

/**
 * IndexedDB-backed execution history (unauthenticated users).
 * All entries fit in one page — `loadMore` is a no-op.
 */
export function useLocalExecutionHistory(options?: { enabled?: boolean }) {
  const ready = useReady();
  const { enabled = true } = options ?? {};
  const queryOptions = core.executions.historyQueryOptions();

  const { data: localEntries, isLoading } = useQuery({
    ...queryOptions,
    select: (entries) => entries.map(localEntryToExecution),
    enabled: ready && enabled,
  });

  return {
    items: localEntries ?? [],
    isLoading: !ready || isLoading,
    status: "Exhausted" as const,
    loadMore: (_numItems: number) => {},
  };
}
