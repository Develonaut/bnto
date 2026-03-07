"use client";

import type { ExecutionService } from "../services/executionService";
import type { BrowserExecutionService } from "../services/browserExecutionService";
import type { HistoryService } from "../services/historyService";
import type { ExecutionInstance } from "../services/executionInstance";
import type { StartPredefinedInput } from "../types";
import type { LocalHistoryEntry } from "../types/localHistory";
import type { BrowserRunResult } from "../types/browser";
import type { PipelineDefinition } from "../engine/types";
import { slugToPipeline } from "../adapters/browser/slugToPipeline";

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

/** Auth status getter injected from the React layer. */
type AuthStatusGetter = () => boolean;

/**
 * Execution client — unified public API for execution operations.
 *
 * Consumers use `core.executions` for everything — capability detection,
 * instance creation, downloads, history, and auto-recording.
 */
export function createExecutionClient(
  executions: ExecutionService,
  browser: BrowserExecutionService,
  history: HistoryService,
) {
  let getIsAuthenticated: AuthStatusGetter = () => false;

  /** Record execution to local + server history (fire-and-forget). */
  function recordToHistory(
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

  /**
   * Wrap an instance's run() to auto-record to history.
   * Always writes to IndexedDB. Also writes to Convex for authenticated users.
   */
  function wrapInstance(instance: ExecutionInstance): ExecutionInstance {
    const originalRun = instance.run;

    return {
      ...instance,
      run: async (definition: PipelineDefinition, files: File[]): Promise<BrowserRunResult> => {
        // Derive slug from definition for history — use the first processing node's type
        const processingNode = definition.nodes.find(
          (n) => n.type !== "input" && n.type !== "output",
        );
        const historySlug = processingNode?.type ?? "unknown";

        const serverEventId = getIsAuthenticated()
          ? await history.recordServerStart(historySlug).catch(() => null)
          : null;

        const result = await originalRun(definition, files);
        if (result.status !== "aborted") {
          recordToHistory(historySlug, files, result, serverEventId);
        }
        return result;
      },
    };
  }

  return {
    // ── Query Options ─────────────────────────────────────────────
    getQueryOptions: (id: string) => executions.getQueryOptions(id),
    listQueryOptions: (recipeId: string) => executions.listQueryOptions(recipeId),
    logsQueryOptions: (executionId: string) => executions.logsQueryOptions(executionId),

    // ── History ───────────────────────────────────────────────────
    historyRefMethod: () => history.serverRef(),
    historyQueryOptions: () => history.localQueryOptions(),
    clearHistory: () => history.clear(),
    migrateHistory: () => history.migrateToServer(),

    // ── Auth Status ──────────────────────────────────────────────
    /** Inject auth status getter from the React layer. */
    setAuthStatusGetter: (getter: AuthStatusGetter) => {
      getIsAuthenticated = getter;
    },

    // ── Mutations ─────────────────────────────────────────────────
    startPredefined: (input: StartPredefinedInput) => executions.startPredefined(input),

    // ── Browser Execution ─────────────────────────────────────────
    /** Check if a slug can run in the browser. */
    isCapable: (slug: string) => browser.isCapable(slug),
    /** Check if a browser implementation exists for a slug. */
    hasImplementation: (slug: string) => browser.hasImplementation(slug),
    /** List all slugs capable of browser execution. */
    getCapableSlugs: () => browser.getCapableSlugs(),

    /**
     * Create an isolated execution instance with its own store.
     * Results are automatically recorded to history (local + server when authed).
     *
     * Each instance has independent state — no cross-page leaks.
     * Usage: `const [instance] = useState(() => core.executions.createExecution())`
     */
    createExecution: (): ExecutionInstance => wrapInstance(browser.createExecution()),

    /**
     * Execute a PipelineDefinition directly with File[].
     * Callers with a slug use `slugToPipeline()` first.
     */
    runPipeline: browser.runPipeline,

    /**
     * Resolve a predefined slug to a PipelineDefinition.
     * Returns null if the slug has no browser implementation.
     */
    slugToPipeline: (slug: string, params?: Record<string, unknown>) =>
      slugToPipeline(slug, params),

    /** Download a single browser execution result. */
    downloadResult: browser.downloadResult,
    /** Download all browser execution results as a ZIP. */
    downloadAllResults: browser.downloadAllResults,

    // ── Cache Invalidation ────────────────────────────────────────
    invalidateExecution: (id: string) => executions.invalidateExecution(id),
    invalidateExecutions: (recipeId: string) => executions.invalidateExecutions(recipeId),
  } as const;
}

export type ExecutionClient = ReturnType<typeof createExecutionClient>;
