"use client";

/**
 * Unified execution history — owns both local (IndexedDB) and server (Convex)
 * paths. Transport detail is internal; consumers see a single API.
 */

import { addEntry, getEntries, clearEntries } from "../adapters/local/localHistoryAdapter";
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

function invalidateLocal() {
  getQueryClient().invalidateQueries({ queryKey: LOCAL_HISTORY_QUERY_KEY });
}

/** Migrate local history entries to Convex. Returns count of migrated entries. */
async function migrateToServer(): Promise<{ migrated: number }> {
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
}

function serverRef() {
  const { funcRef, args } = getExecutionHistoryRef();
  return { funcRef, args, transform: toExecutionFromEvent };
}

function localQueryOptions() {
  return { queryKey: LOCAL_HISTORY_QUERY_KEY, queryFn: getEntries, staleTime: Infinity } as const;
}

async function recordEntry(entry: LocalHistoryEntry) {
  await addEntry(entry);
  invalidateLocal();
}

async function recordServerStart(slug: string): Promise<string | null> {
  try {
    return String((await logExecutionEventStart(slug)) ?? "");
  } catch {
    return null;
  }
}

async function recordServerComplete(
  eventId: string,
  durationMs: number,
  status: "completed" | "failed",
) {
  try {
    await completeExecutionEvent(eventId, durationMs, status);
  } catch {
    /* fire-and-forget */
  }
}

async function clearHistory() {
  await clearEntries();
  invalidateLocal();
}

export function createHistoryService() {
  return {
    serverRef,
    localQueryOptions,
    record: recordEntry,
    recordServerStart,
    recordServerComplete,
    migrateToServer,
    clear: clearHistory,
    queryKey: LOCAL_HISTORY_QUERY_KEY,
  } as const;
}

export type HistoryService = ReturnType<typeof createHistoryService>;
