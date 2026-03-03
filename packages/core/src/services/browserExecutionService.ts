"use client";

/**
 * Browser execution service — orchestrates file processing via WASM.
 *
 * `createExecution()` returns an isolated `ExecutionInstance` per caller.
 * Each recipe page mount gets its own lifecycle — no cross-page state leaks.
 * Global concerns (engine, capabilities, downloads) are shared.
 *
 * The WASM engine initializes lazily on first use — no manual setup required.
 * Tests can inject a mock engine via the `engineOverride` parameter.
 */

import {
  isBrowserCapable,
  getBrowserNodeType,
  getBrowserCapableSlugs,
} from "../adapters/browser/slugCapability";
import { downloadBlob } from "../adapters/browser/downloadBlob";
import { createZipBlob } from "../adapters/browser/createZipBlob";
import { BntoWorker } from "../adapters/browser/BntoWorker";
import { toBrowserEngine } from "../adapters/browser/toBrowserEngine";
import { createExecutionInstance } from "./executionInstance";
import type { ExecutionInstance } from "./executionInstance";
import type {
  BrowserEngine,
  BrowserFileResult,
  BrowserFileProgressInput,
} from "../types/browser";

// ---------------------------------------------------------------------------
// Service factory
// ---------------------------------------------------------------------------

export function createBrowserExecutionService(engineOverride?: BrowserEngine) {
  /** Lazy-initialized engine. Created on first use, persists for page lifetime. */
  let engine: BrowserEngine | null = engineOverride ?? null;

  /** Ensure the browser engine is ready. Creates it lazily if needed. */
  async function ensureEngine(): Promise<BrowserEngine> {
    if (!engine) {
      if (typeof window === "undefined") {
        throw new Error("Browser execution requires a browser environment.");
      }
      const worker = new BntoWorker();
      engine = toBrowserEngine(worker);
    }
    await engine.init();
    return engine;
  }

  /** Low-level stateless execution — no store management. */
  const execute = async (
    slug: string,
    files: File[],
    params: Record<string, unknown> = {},
    onProgress?: (progress: BrowserFileProgressInput) => void,
  ): Promise<BrowserFileResult[]> => {
    const nodeType = getBrowserNodeType(slug);
    if (!nodeType) {
      throw new Error(
        `No browser implementation for slug "${slug}". ` +
          `Available: ${getBrowserCapableSlugs().join(", ")}`,
      );
    }

    const browserEngine = await ensureEngine();

    return browserEngine.processFiles(
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
    /** Check if a slug has a browser implementation. */
    isCapable: (slug: string) => isBrowserCapable(slug),
    /** Check if a slug has a browser implementation. */
    hasImplementation: (slug: string) => isBrowserCapable(slug),
    /** List all slugs with browser implementations. */
    getCapableSlugs: () => getBrowserCapableSlugs(),

    /**
     * Create an isolated execution instance with its own store.
     * Usage: `const [instance] = useState(() => core.executions.createExecution())`
     */
    createExecution: (): ExecutionInstance =>
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

export type BrowserExecutionService = ReturnType<typeof createBrowserExecutionService>;
