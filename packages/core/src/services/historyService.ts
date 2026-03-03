"use client";

/**
 * Unified execution history — owns both local (IndexedDB) and server (Convex)
 * paths. Transport detail is internal; consumers see a single API.
 */

import {
  addEntry,
  getEntries,
  clearEntries,
} from "../adapters/local/localHistoryAdapter";
import { getExecutionHistoryRef } from "../adapters/convex/executionAdapter";
import { toExecution } from "../transforms/execution";
import { getQueryClient } from "../client";
import type { LocalHistoryEntry } from "../types/localHistory";

const LOCAL_HISTORY_QUERY_KEY = ["local-history", "executions"] as const;

export function createHistoryService() {
  function invalidateLocal() {
    getQueryClient().invalidateQueries({ queryKey: LOCAL_HISTORY_QUERY_KEY });
  }

  return {
    serverRef: () => {
      const { funcRef, args } = getExecutionHistoryRef();
      return { funcRef, args, transform: toExecution };
    },

    localQueryOptions: () => ({
      queryKey: LOCAL_HISTORY_QUERY_KEY,
      queryFn: getEntries,
      staleTime: Infinity,
    }),

    record: async (entry: LocalHistoryEntry) => {
      await addEntry(entry);
      invalidateLocal();
    },

    clear: async () => {
      await clearEntries();
      invalidateLocal();
    },

    /** For external invalidation (e.g., after sign-in migration). */
    queryKey: LOCAL_HISTORY_QUERY_KEY,
  } as const;
}

export type HistoryService = ReturnType<typeof createHistoryService>;
