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
import { executePipeline } from "../engine/executePipeline";
import { createExecutionInstance } from "./executionInstance";
import type { ExecutionInstance } from "./executionInstance";
import type { BrowserEngine, BrowserFileResult, BrowserFileProgressInput } from "../types/browser";
import type { FileInput, NodeRunner } from "../engine/types";

// ---------------------------------------------------------------------------
// Conversion helpers: File <-> FileInput, BrowserFileResult <-> FileResult
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

    const definition = {
      nodes: [
        { id: "input", type: "input", params: {} },
        { id: "process", type: nodeType, params },
        { id: "output", type: "output", params: {} },
      ],
    };

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

    return pipelineResult.files.map((f) => ({
      blob: new Blob([toArrayBuffer(f.data)], { type: f.mimeType }),
      filename: f.name,
      mimeType: f.mimeType,
      metadata: f.metadata ?? {},
    }));
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
    createExecution: (): ExecutionInstance => createExecutionInstance(execute),

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
