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
import {
  getExecutionHistoryRef,
  logExecutionEventStart,
  completeExecutionEvent,
  migrateLocalHistory,
} from "../adapters/convex/executionAdapter";
import { toExecutionFromEvent } from "../transforms/executionEvent";
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
      return { funcRef, args, transform: toExecutionFromEvent };
    },

    localQueryOptions: () => ({
      queryKey: LOCAL_HISTORY_QUERY_KEY,
      queryFn: getEntries,
      staleTime: Infinity,
    }),

    /** Record to local history (always called — IndexedDB). */
    record: async (entry: LocalHistoryEntry) => {
      await addEntry(entry);
      invalidateLocal();
    },

    /**
     * Record to server history (Convex executionEvents).
     * Returns the event ID for later completion, or null if unauthenticated.
     */
    recordServerStart: async (slug: string): Promise<string | null> => {
      try {
        const eventId = await logExecutionEventStart(slug);
        return eventId ? String(eventId) : null;
      } catch {
        return null;
      }
    },

    /** Complete a server execution event with status and duration. */
    recordServerComplete: async (
      eventId: string,
      durationMs: number,
      status: "completed" | "failed",
    ) => {
      try {
        await completeExecutionEvent(eventId, durationMs, status);
      } catch {
        // Fire-and-forget — don't block execution on history updates
      }
    },

    /** Migrate local history entries to Convex (on signup). */
    migrateToServer: async () => {
      const entries = await getEntries();
      if (entries.length === 0) return { migrated: 0 };

      try {
        const result = await migrateLocalHistory(entries);
        if (result && result.migrated > 0) {
          await clearEntries();
          invalidateLocal();
        }
        return result ?? { migrated: 0 };
      } catch {
        return { migrated: 0 };
      }
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
