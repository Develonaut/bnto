"use client";

/**
 * Browser execution service — orchestrates file processing via WASM.
 *
 * The browser execution path is fundamentally different from cloud:
 * - No R2 upload/download (files stay in memory)
 * - No Convex execution record (state is local)
 * - No Go API call (WASM processes in the browser)
 * - Progress comes from the Web Worker, not polling
 *
 * The service owns the execution store and the full lifecycle:
 *   run() → start → progress → complete/fail
 * React hooks are thin bindings over the store.
 */

import {
  getBrowserEngine,
  registerBrowserEngine,
  hasBrowserEngine,
} from "../adapters/browser/engineRegistry";
import {
  isBrowserCapable,
  getBrowserNodeType,
  getBrowserCapableSlugs,
} from "../adapters/browser/slugCapability";
import { downloadBlob } from "../adapters/browser/downloadBlob";
import { createZipBlob } from "../adapters/browser/createZipBlob";
import { createBrowserExecutionStore } from "../stores/browserExecutionStore";
import type {
  BrowserEngine,
  BrowserFileResult,
  BrowserFileProgress,
} from "../types/browser";

export function createBrowserExecutionService() {
  // The service owns its store instance. React hooks read from it;
  // only the service writes to it (via run/reset).
  const store = createBrowserExecutionStore();

  // Abort flag — checked in the progress callback to stop
  // updating the store after reset() is called mid-execution.
  // Not a real WASM cancellation (Web Worker keeps running).
  let aborted = false;

  /**
   * Execute a bnto in the browser (low-level).
   *
   * Initializes the engine if needed, processes all files through
   * the WASM worker, and returns results as in-memory blobs.
   * Does NOT manage store state — use `run()` for the full lifecycle.
   */
  const execute = async (
    slug: string,
    files: File[],
    params: Record<string, unknown> = {},
    onProgress?: (progress: BrowserFileProgress) => void,
  ): Promise<BrowserFileResult[]> => {
    const engine = getBrowserEngine();
    if (!engine) {
      throw new Error(
        "No browser engine registered. " +
          "Call core.browser.registerEngine() at app startup.",
      );
    }

    const nodeType = getBrowserNodeType(slug);
    if (!nodeType) {
      throw new Error(
        `No browser implementation for slug "${slug}". ` +
          `Available: ${getBrowserCapableSlugs().join(", ")}`,
      );
    }

    // Initialize the engine (no-op if already initialized).
    await engine.init();

    // Process all files, forwarding progress updates.
    return engine.processFiles(
      files,
      nodeType,
      params,
      onProgress
        ? (fileIndex, percent, message) =>
            onProgress({
              fileIndex,
              totalFiles: files.length,
              percent,
              message,
            })
        : undefined,
    );
  };

  return {
    /** The Zustand store backing execution state. */
    store,

    // ── Capability Detection ───────────────────────────────────────
    /** Check if a slug has a working browser execution path. */
    isCapable: (slug: string) => isBrowserCapable(slug) && hasBrowserEngine(),

    /** Check if a slug has a browser implementation (ignores engine registration). */
    hasImplementation: (slug: string) => isBrowserCapable(slug),

    /** Get all slugs with browser implementations. */
    getCapableSlugs: () => getBrowserCapableSlugs(),

    // ── Engine Registration ────────────────────────────────────────
    /** Register the browser engine (called once at app startup). */
    registerEngine: (engine: BrowserEngine) => registerBrowserEngine(engine),

    /** Whether a browser engine has been registered. */
    hasEngine: () => hasBrowserEngine(),

    // ── Lifecycle (service owns the full orchestration) ─────────────

    /**
     * Run a browser execution with full lifecycle management.
     *
     * Manages the store transitions: idle → processing → completed/failed.
     * Progress updates flow through the store automatically.
     *
     * @param slug - The bnto slug (e.g., "compress-images").
     * @param files - Files to process.
     * @param params - Node-specific config (e.g., { quality: 80 }).
     */
    run: async (
      slug: string,
      files: File[],
      params: Record<string, unknown> = {},
    ): Promise<void> => {
      aborted = false;

      const id =
        typeof crypto !== "undefined" && crypto.randomUUID
          ? crypto.randomUUID()
          : `exec-${Date.now()}`;

      store.getState().start(id, Date.now());

      try {
        const onProgress = (progress: BrowserFileProgress) => {
          if (aborted) return;
          store.getState().progress(progress);
        };

        const results = await execute(slug, files, params, onProgress);

        if (aborted) return;
        store.getState().complete(results, Date.now());
      } catch (e) {
        if (aborted) return;
        store.getState().fail(
          e instanceof Error ? e.message : "Processing failed",
          Date.now(),
        );
      }
    },

    /** Reset execution state to idle. Aborts any in-progress run. */
    reset: () => {
      aborted = true;
      store.getState().reset();
    },

    // ── Low-level execution (no store management) ───────────────────
    /** Execute without lifecycle management. For callers managing their own state. */
    execute,

    // ── Download ───────────────────────────────────────────────────
    /** Download a browser execution result as a file. */
    downloadResult: (result: BrowserFileResult) => {
      downloadBlob(result.blob, result.filename);
    },

    /**
     * Download all browser execution results as a single ZIP archive.
     *
     * @param results - Processed files to bundle.
     * @param slug - The bnto slug, used for the archive filename.
     */
    downloadAllResults: async (
      results: BrowserFileResult[],
      slug?: string,
    ) => {
      const zipBlob = await createZipBlob(results);
      const name = slug ? `${slug}-results.zip` : "bnto-results.zip";
      downloadBlob(zipBlob, name);
    },
  } as const;
}

export type BrowserExecutionService = ReturnType<
  typeof createBrowserExecutionService
>;
