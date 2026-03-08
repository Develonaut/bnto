"use client";

/**
 * Browser execution service — orchestrates pipeline execution via WASM.
 *
 * `createExecution()` returns an isolated `ExecutionInstance` per caller.
 * Each recipe page mount gets its own lifecycle — no cross-page state leaks.
 *
 * The WASM engine initializes lazily on first use — no manual setup required.
 */

import { downloadBlob } from "../adapters/browser/downloadBlob";
import { createZipBlob } from "../adapters/browser/createZipBlob";
import { BntoWorker } from "../adapters/browser/BntoWorker";
import { createExecutionInstance } from "./executionInstance";
import type { ExecutionInstance } from "./executionInstance";
import type { BrowserFileResult, BrowserFileProgressInput } from "../types/browser";
import type { PipelineDefinition } from "../types/pipeline";
import type { PipelineEvent } from "../types/pipelineEvents";

// ---------------------------------------------------------------------------
// Service factory
// ---------------------------------------------------------------------------

export function createBrowserExecutionService() {
  /** Lazy-initialized worker. Created on first use, persists for page lifetime. */
  let workerInstance: BntoWorker | null = null;

  /** Ensure the WASM worker is ready. Creates it lazily if needed. */
  async function ensureWorker(): Promise<BntoWorker> {
    if (!workerInstance) {
      if (typeof window === "undefined") {
        throw new Error("Browser execution requires a browser environment.");
      }
      workerInstance = new BntoWorker();
    }
    await workerInstance.init();
    return workerInstance;
  }

  /**
   * Execute a PipelineDefinition with File[] via the WASM pipeline executor.
   *
   * The entire pipeline runs inside WASM — the Rust executor handles
   * node walking, file iteration, and output chaining. Structured
   * PipelineEvents are forwarded via the optional onEvent callback.
   */
  const runPipeline = async (
    definition: PipelineDefinition,
    files: File[],
    onProgress?: (progress: BrowserFileProgressInput) => void,
    onEvent?: (event: PipelineEvent) => void,
  ): Promise<BrowserFileResult[]> => {
    const worker = await ensureWorker();

    if (files.length === 0) return [];

    const definitionJson = JSON.stringify(definition);
    const result = await worker.executePipeline(definitionJson, files, onEvent);

    return result.files.map((f) => ({
      blob: new Blob([f.data], { type: f.mimeType }),
      filename: f.name,
      mimeType: f.mimeType,
      metadata: f.metadata ? JSON.parse(f.metadata) : {},
    }));
  };

  return {
    /**
     * Create an isolated execution instance with its own store.
     * Usage: `const [instance] = useState(() => core.executions.createExecution())`
     */
    createExecution: (): ExecutionInstance => createExecutionInstance(runPipeline),

    /** Execute a PipelineDefinition directly with File[]. */
    runPipeline,

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
