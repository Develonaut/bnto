"use client";

/**
 * WASM execution service — orchestrates file processing via WASM.
 *
 * The WASM execution path is fundamentally different from cloud:
 * - No R2 upload/download (files stay in memory)
 * - No Convex execution record (state is local)
 * - No Go API call (WASM processes in the browser)
 * - Progress comes from the Web Worker, not polling
 *
 * Two execution modes:
 *
 * 1. **Singleton** (backward-compatible): `service.run()` / `service.store`
 *    Uses a single store created at module load. Simple, but leaks state
 *    between recipe pages.
 *
 * 2. **Per-instance** (preferred): `service.createExecution()`
 *    Factory returns `{ store, run, reset }` per caller. Each recipe page
 *    mount gets its own isolated execution lifecycle. No cross-page leaks.
 *
 * Global concerns (engine registration, capability checks, downloads) are
 * shared across all instances — they don't carry execution state.
 */

import {
  getBrowserEngine,
  registerBrowserEngine,
  hasBrowserEngine,
} from "../adapters/wasm/engineRegistry";
import {
  isBrowserCapable,
  getBrowserNodeType,
  getBrowserCapableSlugs,
} from "../adapters/wasm/slugCapability";
import { downloadBlob } from "../adapters/wasm/downloadBlob";
import { createZipBlob } from "../adapters/wasm/createZipBlob";
import {
  createWasmExecutionStore,
  type WasmExecutionState,
} from "../stores/wasmExecutionStore";
import type {
  BrowserEngine,
  BrowserFileResult,
  BrowserFileProgress,
} from "../types/wasm";
import type { StoreApi } from "zustand/vanilla";

// ---------------------------------------------------------------------------
// Execution instance — isolated store + run/reset per caller
// ---------------------------------------------------------------------------

/** An isolated execution instance with its own store and lifecycle. */
export interface WasmExecutionInstance {
  /** The Zustand store backing this execution's state. */
  store: StoreApi<WasmExecutionState>;
  /**
   * Run a WASM execution with full lifecycle management.
   * Manages store transitions: idle → processing → completed/failed.
   */
  run: (
    slug: string,
    files: File[],
    params?: Record<string, unknown>,
  ) => Promise<void>;
  /** Reset execution state to idle. Aborts any in-progress run. */
  reset: () => void;
}

/**
 * Create an isolated execution instance with its own store.
 *
 * The `execute` function is shared (stateless, uses global engine registry).
 * Each instance gets its own abort flag and store — no cross-instance leaks.
 */
function createExecutionInstance(
  execute: (
    slug: string,
    files: File[],
    params: Record<string, unknown>,
    onProgress?: (progress: BrowserFileProgress) => void,
  ) => Promise<BrowserFileResult[]>,
): WasmExecutionInstance {
  const store = createWasmExecutionStore();
  let aborted = false;

  return {
    store,

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

    reset: () => {
      aborted = true;
      store.getState().reset();
    },
  };
}

// ---------------------------------------------------------------------------
// Service factory
// ---------------------------------------------------------------------------

export function createWasmExecutionService() {
  /**
   * Execute a bnto via WASM (low-level, stateless).
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
        "No WASM engine registered. " +
          "Call core.wasm.registerEngine() at app startup.",
      );
    }

    const nodeType = getBrowserNodeType(slug);
    if (!nodeType) {
      throw new Error(
        `No WASM implementation for slug "${slug}". ` +
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

  // Singleton instance for backward compatibility.
  // New consumers should use createExecution() instead.
  const singleton = createExecutionInstance(execute);

  return {
    /** The singleton Zustand store (backward-compatible). Prefer createExecution(). */
    store: singleton.store,

    // ── Capability Detection ───────────────────────────────────────
    /** Check if a slug has a working WASM execution path. */
    isCapable: (slug: string) => isBrowserCapable(slug) && hasBrowserEngine(),

    /** Check if a slug has a WASM implementation (ignores engine registration). */
    hasImplementation: (slug: string) => isBrowserCapable(slug),

    /** Get all slugs with WASM implementations. */
    getCapableSlugs: () => getBrowserCapableSlugs(),

    // ── Engine Registration ────────────────────────────────────────
    /** Register the WASM engine (called once at app startup). */
    registerEngine: (engine: BrowserEngine) => registerBrowserEngine(engine),

    /** Whether a WASM engine has been registered. */
    hasEngine: () => hasBrowserEngine(),

    // ── Per-Instance Factory ────────────────────────────────────────

    /**
     * Create an isolated execution instance with its own store.
     *
     * Each instance has independent state (idle/processing/completed/failed),
     * its own abort flag, and its own results. Navigating between recipe pages
     * won't leak state because each page mount creates its own instance.
     *
     * Global concerns (engine, capability checks, downloads) are shared.
     *
     * Usage in React:
     *   const [instance] = useState(() => core.wasm.createExecution());
     */
    createExecution: (): WasmExecutionInstance =>
      createExecutionInstance(execute),

    // ── Singleton Lifecycle (backward-compatible) ───────────────────

    /**
     * Run via the singleton store (backward-compatible).
     * Prefer createExecution() for per-page isolation.
     */
    run: singleton.run,

    /** Reset the singleton store (backward-compatible). */
    reset: singleton.reset,

    // ── Low-level execution (no store management) ───────────────────
    /** Execute without lifecycle management. For callers managing their own state. */
    execute,

    // ── Download ───────────────────────────────────────────────────
    /** Download a WASM execution result as a file. */
    downloadResult: (result: BrowserFileResult) => {
      downloadBlob(result.blob, result.filename);
    },

    /**
     * Download all WASM execution results as a single ZIP archive.
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

export type WasmExecutionService = ReturnType<
  typeof createWasmExecutionService
>;
