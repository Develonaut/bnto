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

import { isBrowserCapable, getBrowserCapableSlugs } from "../adapters/browser/slugCapability";
import { downloadBlob } from "../adapters/browser/downloadBlob";
import { createZipBlob } from "../adapters/browser/createZipBlob";
import { BntoWorker } from "../adapters/browser/BntoWorker";
import { toBrowserEngine } from "../adapters/browser/toBrowserEngine";
import { executePipeline } from "../engine/executePipeline";
import { createExecutionInstance } from "./executionInstance";
import type { ExecutionInstance } from "./executionInstance";
import type { BrowserEngine, BrowserFileResult, BrowserFileProgressInput } from "../types/browser";
import type { FileInput, NodeRunner, PipelineDefinition } from "../engine/types";
import type { PipelineEvent } from "../types/pipelineEvents";

// ---------------------------------------------------------------------------
// Conversion helpers
// ---------------------------------------------------------------------------

/** Convert a Uint8Array to an ArrayBuffer suitable for Blob construction. */
function toArrayBuffer(data: Uint8Array): ArrayBuffer {
  return data.buffer.slice(data.byteOffset, data.byteOffset + data.byteLength) as ArrayBuffer;
}

/** Convert browser File objects to platform-agnostic FileInput[]. */
async function filesToInputs(files: File[]): Promise<FileInput[]> {
  return Promise.all(
    files.map(async (f) => ({
      name: f.name,
      data: new Uint8Array(await f.arrayBuffer()),
      mimeType: f.type || "application/octet-stream",
    })),
  );
}

/** Build a NodeRunner that delegates to a BrowserEngine's processFile. */
function createRunNode(browserEngine: BrowserEngine): NodeRunner {
  return async (file, nodeType, nodeParams, fileOnProgress) => {
    const blob = new Blob([toArrayBuffer(file.data)], { type: file.mimeType });
    const browserFile = new File([blob], file.name, { type: file.mimeType });
    const result = await browserEngine.processFile(
      browserFile,
      nodeType,
      nodeParams,
      fileOnProgress,
    );
    const resultData = new Uint8Array(await result.blob.arrayBuffer());
    return {
      name: result.filename,
      data: resultData,
      mimeType: result.mimeType,
      metadata: result.metadata,
    };
  };
}

/** Convert pipeline FileResult[] to BrowserFileResult[]. */
function toBrowserResults(files: import("../engine/types").FileResult[]): BrowserFileResult[] {
  return files.map((f) => ({
    blob: new Blob([toArrayBuffer(f.data)], { type: f.mimeType }),
    filename: f.name,
    mimeType: f.mimeType,
    metadata: f.metadata ?? {},
  }));
}

// ---------------------------------------------------------------------------
// Service factory
// ---------------------------------------------------------------------------

export function createBrowserExecutionService(engineOverride?: BrowserEngine) {
  /** Lazy-initialized engine. Created on first use, persists for page lifetime. */
  let engine: BrowserEngine | null = engineOverride ?? null;
  /** Direct worker reference for WASM pipeline execution. */
  let workerInstance: BntoWorker | null = null;

  /** Ensure the browser engine is ready. Creates it lazily if needed. */
  async function ensureEngine(): Promise<BrowserEngine> {
    if (!engine) {
      if (typeof window === "undefined") {
        throw new Error("Browser execution requires a browser environment.");
      }
      workerInstance = new BntoWorker();
      engine = toBrowserEngine(workerInstance);
    }
    await engine.init();
    return engine;
  }

  /**
   * Execute a PipelineDefinition with File[] via the WASM pipeline executor.
   *
   * This is the primary execution path. The entire pipeline runs inside WASM —
   * the Rust executor handles node walking, file iteration, and output chaining.
   * Structured PipelineEvents are forwarded via the optional onEvent callback.
   *
   * Falls back to the JS-side executePipeline() when no direct worker is
   * available (e.g., when engineOverride is provided for testing).
   */
  const runPipeline = async (
    definition: PipelineDefinition,
    files: File[],
    onProgress?: (progress: BrowserFileProgressInput) => void,
    onEvent?: (event: PipelineEvent) => void,
  ): Promise<BrowserFileResult[]> => {
    await ensureEngine();

    // Primary path: WASM pipeline executor (engine owns execution).
    if (workerInstance) {
      const definitionJson = JSON.stringify(definition);
      const result = await workerInstance.executePipeline(definitionJson, files, onEvent);

      return result.files.map((f) => ({
        blob: new Blob([f.data], { type: f.mimeType }),
        filename: f.name,
        mimeType: f.mimeType,
        metadata: f.metadata ? JSON.parse(f.metadata) : {},
      }));
    }

    // Fallback: JS-side executor (for test mocks with engineOverride).
    const browserEngine = engine!;
    const fileInputs = await filesToInputs(files);
    const runNode = createRunNode(browserEngine);

    const pipelineResult = await executePipeline(
      definition,
      fileInputs,
      runNode,
      onProgress
        ? (_nodeIndex, fileIndex, totalFiles, percent, message) =>
            onProgress({ fileIndex, totalFiles, percent, message })
        : undefined,
    );

    return toBrowserResults(pipelineResult.files);
  };

  return {
    /** Check if a slug has a browser implementation. */
    isCapable: (slug: string) => isBrowserCapable(slug),
    /** @deprecated Use `isCapable` instead. Alias kept for backward compatibility. */
    hasImplementation: (slug: string) => isBrowserCapable(slug),
    /** List all slugs with browser implementations. */
    getCapableSlugs: () => getBrowserCapableSlugs(),

    /**
     * Create an isolated execution instance with its own store.
     * Usage: `const [instance] = useState(() => core.executions.createExecution())`
     */
    createExecution: (): ExecutionInstance => createExecutionInstance(runPipeline),

    /**
     * Execute a PipelineDefinition directly with File[].
     * Callers with a slug use `slugToPipeline()` first.
     */
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
