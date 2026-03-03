"use client";

import type { ExecutionService } from "../services/executionService";
import type { BrowserExecutionService } from "../services/browserExecutionService";
import type { StartPredefinedInput } from "../types";

/**
 * Execution client — unified public API for execution operations.
 *
 * Consumers use `core.executions` for everything — capability detection,
 * instance creation, downloads, and history.
 */
export function createExecutionClient(
  executions: ExecutionService,
  browser: BrowserExecutionService,
) {
  return {
    // ── Query Options ─────────────────────────────────────────────
    getQueryOptions: (id: string) => executions.getQueryOptions(id),
    listQueryOptions: (recipeId: string) => executions.listQueryOptions(recipeId),
    logsQueryOptions: (executionId: string) => executions.logsQueryOptions(executionId),

    // ── Paginated Query Refs ────────────────────────────────────────
    historyRefMethod: () => executions.historyRefMethod(),

    // ── Mutations ─────────────────────────────────────────────────
    startPredefined: (input: StartPredefinedInput) =>
      executions.startPredefined(input),

    // ── Browser Execution ─────────────────────────────────────────
    /** Check if a slug can run in the browser. */
    isCapable: (slug: string) => browser.isCapable(slug),
    /** Check if a browser implementation exists for a slug. */
    hasImplementation: (slug: string) => browser.hasImplementation(slug),
    /** List all slugs capable of browser execution. */
    getCapableSlugs: () => browser.getCapableSlugs(),

    /**
     * Create an isolated execution instance with its own store.
     *
     * Each instance has independent state — no cross-page leaks.
     * Usage: `const [instance] = useState(() => core.executions.createExecution())`
     */
    createExecution: browser.createExecution,

    /** Execute without lifecycle management. For callers managing their own state. */
    execute: browser.execute,

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
