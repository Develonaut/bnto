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
    // ── Capability Detection ───────────────────────────────────────
    isCapable: (slug: string) => browser.isCapable(slug),
    hasImplementation: (slug: string) => browser.hasImplementation(slug),
    getCapableSlugs: () => browser.getCapableSlugs(),

    // ── Engine Registration ────────────────────────────────────────
    registerEngine: browser.registerEngine,
    hasEngine: () => browser.hasEngine(),

    // ── Execution ──────────────────────────────────────────────────
    execute: browser.execute,

    // ── Download ───────────────────────────────────────────────────
    downloadResult: browser.downloadResult,
    downloadAllResults: browser.downloadAllResults,
  } as const;
}

export type BrowserClient = ReturnType<typeof createBrowserClient>;
