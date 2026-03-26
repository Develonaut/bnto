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

/** Bridge PipelineEvents to the BrowserFileProgressInput shape the execution store expects. */
function createEventHandler(
  onProgress?: (progress: BrowserFileProgressInput) => void,
  onEvent?: (event: PipelineEvent) => void,
) {
  return (event: PipelineEvent) => {
    onEvent?.(event);
    if (onProgress && event.type === "FileProgress") {
      onProgress({
        fileIndex: event.fileIndex,
        totalFiles: event.totalFiles,
        percent: event.percent,
        message: event.message,
      });
    }
  };
}

/** Convert raw WASM result files to BrowserFileResult[]. */
function toFileResults(
  files: Array<{ name: string; data: ArrayBuffer; mimeType: string; metadata?: string }>,
): BrowserFileResult[] {
  return files.map((f) => ({
    blob: new Blob([f.data], { type: f.mimeType }),
    filename: f.name,
    mimeType: f.mimeType,
    metadata: f.metadata ? JSON.parse(f.metadata) : {},
  }));
}

/** Download results — single file direct, multiple as ZIP. */
async function downloadAll(results: BrowserFileResult[], slug?: string): Promise<void> {
  if (results.length === 1) {
    downloadBlob(results[0].blob, results[0].filename);
    return;
  }
  const zipBlob = await createZipBlob(results);
  downloadBlob(zipBlob, slug ? `${slug}-results.zip` : "bnto-results.zip");
}

// ---------------------------------------------------------------------------
// Lazy worker singleton + pipeline runner (module-scoped for size compliance)
// ---------------------------------------------------------------------------

let workerInstance: BntoWorker | null = null;

async function ensureWorker(): Promise<BntoWorker> {
  if (!workerInstance) {
    if (typeof window === "undefined")
      throw new Error("Browser execution requires a browser environment.");
    workerInstance = new BntoWorker();
  }
  await workerInstance.init();
  return workerInstance;
}

async function runPipeline(
  definition: PipelineDefinition,
  files: File[],
  onProgress?: (progress: BrowserFileProgressInput) => void,
  onEvent?: (event: PipelineEvent) => void,
): Promise<BrowserFileResult[]> {
  if (files.length === 0) return [];
  const worker = await ensureWorker();
  const result = await worker.executePipeline(
    JSON.stringify(definition),
    files,
    createEventHandler(onProgress, onEvent),
  );
  return toFileResults(result.files);
}

export function createBrowserExecutionService() {
  return {
    createExecution: (): ExecutionInstance => createExecutionInstance(runPipeline),
    runPipeline,
    downloadResult: (result: BrowserFileResult) => downloadBlob(result.blob, result.filename),
    downloadAllResults: downloadAll,
  } as const;
}

export type BrowserExecutionService = ReturnType<typeof createBrowserExecutionService>;
