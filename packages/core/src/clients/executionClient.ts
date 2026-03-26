"use client";

import type { ExecutionService } from "../services/executionService";
import type { BrowserExecutionService } from "../services/browserExecutionService";
import type { HistoryService } from "../services/historyService";
import type { AuthClient } from "./authClient";
import type { ExecutionInstance } from "../services/executionInstance";
import type { StartPredefinedInput } from "../types";
import type { LocalHistoryEntry } from "../types/localHistory";
import type { BrowserRunResult } from "../types/browser";
import type { PipelineDefinition } from "../types/pipeline";
import { isIoNodeType } from "@bnto/registry";

/** Build a history entry from a completed/failed execution result. */
function buildHistoryEntry(
  slug: string,
  files: File[],
  result: BrowserRunResult,
): LocalHistoryEntry {
  return {
    id: crypto.randomUUID(),
    slug,
    status: result.status as "completed" | "failed",
    timestamp: Date.now() - result.durationMs,
    durationMs: result.durationMs,
    inputFileCount: files.length,
    outputFileCount: result.results.length,
    error: result.error,
  };
}

/**
 * Execution client — unified public API for execution operations.
 *
 * Consumers use `core.executions` for everything — instance creation,
 * downloads, history, and auto-recording.
 */
/** Record execution to local + server history (fire-and-forget). */
function recordToHistory(
  history: HistoryService,
  slug: string,
  files: File[],
  result: BrowserRunResult,
  serverEventId: string | null,
): void {
  history.record(buildHistoryEntry(slug, files, result)).catch(() => {});
  if (serverEventId) {
    history
      .recordServerComplete(
        serverEventId,
        result.durationMs,
        result.status as "completed" | "failed",
      )
      .catch(() => {});
  }
}

/** Derive the history slug from a definition (first processing node's type). */
function deriveSlug(definition: PipelineDefinition): string {
  return definition.nodes.find((n) => !isIoNodeType(n.type))?.type ?? "unknown";
}

/** Wrap an instance's run() to auto-record to history. */
function wrapInstance(
  instance: ExecutionInstance,
  history: HistoryService,
  auth: AuthClient,
): ExecutionInstance {
  const originalRun = instance.run;
  return {
    ...instance,
    run: async (definition: PipelineDefinition, files: File[]): Promise<BrowserRunResult> => {
      const slug = deriveSlug(definition);
      const serverEventId = auth.isAuthenticated()
        ? await history.recordServerStart(slug).catch(() => null)
        : null;
      const result = await originalRun(definition, files);
      if (result.status !== "aborted") recordToHistory(history, slug, files, result, serverEventId);
      return result;
    },
  };
}

export function createExecutionClient(
  executions: ExecutionService,
  browser: BrowserExecutionService,
  history: HistoryService,
  auth: AuthClient,
) {
  return {
    getQueryOptions: (id: string) => executions.getQueryOptions(id),
    listQueryOptions: (recipeId: string) => executions.listQueryOptions(recipeId),
    logsQueryOptions: (executionId: string) => executions.logsQueryOptions(executionId),
    historyRefMethod: () => history.serverRef(),
    historyQueryOptions: () => history.localQueryOptions(),
    clearHistory: () => history.clear(),
    migrateHistory: () => history.migrateToServer(),
    startPredefined: (input: StartPredefinedInput) => executions.startPredefined(input),
    createExecution: (): ExecutionInstance =>
      wrapInstance(browser.createExecution(), history, auth),
    runPipeline: browser.runPipeline,
    downloadResult: browser.downloadResult,
    downloadAllResults: browser.downloadAllResults,
    invalidateExecution: (id: string) => executions.invalidateExecution(id),
    invalidateExecutions: (recipeId: string) => executions.invalidateExecutions(recipeId),
  } as const;
}
