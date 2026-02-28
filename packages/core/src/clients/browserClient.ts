"use client";

import type { BrowserExecutionService } from "../services/browserExecutionService";

/**
 * Browser execution client — public API for browser-side WASM execution.
 *
 * Wraps the browser execution service for architectural symmetry with
 * other domain clients (workflows, executions, user, etc.).
 */
export function createBrowserClient(browser: BrowserExecutionService) {
  return {
    /** The singleton Zustand store (backward-compatible). Prefer createExecution(). */
    store: browser.store,

    // ── Capability Detection ───────────────────────────────────────
    isCapable: (slug: string) => browser.isCapable(slug),
    hasImplementation: (slug: string) => browser.hasImplementation(slug),
    getCapableSlugs: () => browser.getCapableSlugs(),

    // ── Engine Registration ────────────────────────────────────────
    registerEngine: browser.registerEngine,
    hasEngine: () => browser.hasEngine(),

    // ── Per-Instance Factory ────────────────────────────────────────
    /**
     * Create an isolated execution instance with its own store.
     *
     * Each instance has independent state — no cross-page leaks.
     * Usage: `const [instance] = useState(() => core.browser.createExecution())`
     */
    createExecution: browser.createExecution,

    // ── Singleton Lifecycle (backward-compatible) ───────────────────
    /** Run via the singleton store (backward-compatible). Prefer createExecution(). */
    run: browser.run,
    /** Reset the singleton store (backward-compatible). */
    reset: browser.reset,

    // ── Low-level execution ────────────────────────────────────────
    /** Execute without lifecycle management. For callers managing their own state. */
    execute: browser.execute,

    // ── Download ───────────────────────────────────────────────────
    downloadResult: browser.downloadResult,
    downloadAllResults: browser.downloadAllResults,
  } as const;
}

export type BrowserClient = ReturnType<typeof createBrowserClient>;
