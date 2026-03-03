"use client";

/**
 * WASM execution service — browser-side file processing via Rust WASM.
 *
 * `createExecution()` returns an isolated `{ store, run, reset }` per caller.
 * Each recipe page mount gets its own lifecycle — no cross-page state leaks.
 * Global concerns (engine, capabilities, downloads) are shared.
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
  BrowserFileProgressInput,
  WasmRunResult,
} from "../types/wasm";
import type { StoreApi } from "zustand/vanilla";

// ---------------------------------------------------------------------------
// Execution instance
// ---------------------------------------------------------------------------

export interface WasmExecutionInstance {
  store: StoreApi<WasmExecutionState>;
  run: (
    slug: string,
    files: File[],
    params?: Record<string, unknown>,
  ) => Promise<WasmRunResult>;
  reset: () => void;
}

// ---------------------------------------------------------------------------
// Progress throttle — ~60fps, always passes first update, 100%, and new file
// ---------------------------------------------------------------------------

const THROTTLE_MS = 16;

function createThrottledProgress(
  store: ReturnType<typeof createWasmExecutionStore>,
) {
  let lastUpdate = 0;

  return (progress: BrowserFileProgressInput) => {
    const now =
      typeof performance !== "undefined" ? performance.now() : Date.now();

    const current = store.getState().fileProgress;
    const isFirst = current === null;
    const isComplete = progress.percent >= 100;
    const isNewFile = current !== null && current.fileIndex !== progress.fileIndex;

    if (isFirst || isComplete || isNewFile || now - lastUpdate >= THROTTLE_MS) {
      store.getState().progress(progress);
      lastUpdate = now;
    }
  };
}

function createExecutionInstance(
  execute: (
    slug: string,
    files: File[],
    params: Record<string, unknown>,
    onProgress?: (progress: BrowserFileProgressInput) => void,
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
    ): Promise<WasmRunResult> => {
      aborted = false;
      const id = crypto.randomUUID();
      const startedAt = Date.now();
      store.getState().start(id, startedAt);

      try {
        const throttled = createThrottledProgress(store);
        const onProgress = (progress: BrowserFileProgressInput) => {
          if (aborted) return;
          throttled(progress);
        };

        const results = await execute(slug, files, params, onProgress);
        const durationMs = Date.now() - startedAt;

        if (aborted) {
          return { status: "aborted", results: [], durationMs };
        }

        store.getState().complete(results, Date.now());
        return { status: "completed", results, durationMs };
      } catch (e) {
        const durationMs = Date.now() - startedAt;
        const error = e instanceof Error ? e.message : "Processing failed";

        if (aborted) {
          return { status: "aborted", results: [], durationMs };
        }

        store.getState().fail(error, Date.now());
        return { status: "failed", results: [], durationMs, error };
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
  /** Low-level stateless execution — no store management. */
  const execute = async (
    slug: string,
    files: File[],
    params: Record<string, unknown> = {},
    onProgress?: (progress: BrowserFileProgressInput) => void,
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

    await engine.init();

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
    isCapable: (slug: string) => isBrowserCapable(slug) && hasBrowserEngine(),
    hasImplementation: (slug: string) => isBrowserCapable(slug),
    getCapableSlugs: () => getBrowserCapableSlugs(),
    registerEngine: (engine: BrowserEngine) => registerBrowserEngine(engine),
    hasEngine: () => hasBrowserEngine(),

    /**
     * Create an isolated execution instance with its own store.
     * Usage: `const [instance] = useState(() => core.wasm.createExecution())`
     */
    createExecution: (): WasmExecutionInstance =>
      createExecutionInstance(execute),

    execute,

    downloadResult: (result: BrowserFileResult) => {
      downloadBlob(result.blob, result.filename);
    },

    downloadAllResults: async (results: BrowserFileResult[], slug?: string) => {
      const zipBlob = await createZipBlob(results);
      const name = slug ? `${slug}-results.zip` : "bnto-results.zip";
      downloadBlob(zipBlob, name);
    },
  } as const;
}

export type WasmExecutionService = ReturnType<typeof createWasmExecutionService>;
