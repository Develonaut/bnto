/**
 * Bnto WASM Web Worker
 *
 * Runs in a background thread. Loads the WASM binary and executes
 * pipelines via the Rust executor. Progress events are forwarded
 * to the main thread via postMessage.
 *
 * Files never leave the browser — all processing happens in WASM memory.
 *
 * Message flow:
 *   Main Thread → Worker: { type: "init" } or { type: "execute-pipeline", ... }
 *   Worker → Main Thread: { type: "ready" } or { type: "pipeline-progress"|"pipeline-result"|"pipeline-error", ... }
 */

import type { WorkerRequest } from "./workerProtocol";
import { loadWasmModule } from "./wasmLoader";
import {
  send,
  sendWorkerError,
  sendPipelineProgress,
  sendPipelineResult,
  sendPipelineError,
} from "./workerResponses";

// =============================================================================
// State
// =============================================================================

// eslint-disable-next-line @typescript-eslint/no-explicit-any -- WASM module is untyped
let wasmModule: any = null;
let wasmVersion: (() => string) | null = null;
let initialized = false;

// =============================================================================
// Message Handler
// =============================================================================

self.onmessage = async (event: MessageEvent<WorkerRequest>) => {
  const request = event.data;
  try {
    switch (request.type) {
      case "init":
        await handleInit(request.baseUrl);
        break;
      case "execute-pipeline":
        handleExecutePipeline(request);
        break;
      default:
        sendWorkerError(`Unknown request type: ${(request as { type: string }).type}`);
    }
  } catch (err) {
    sendWorkerError(err instanceof Error ? err.message : String(err));
  }
};

// =============================================================================
// Init — Load the WASM binary
// =============================================================================

async function handleInit(baseUrl: string): Promise<void> {
  if (initialized) {
    send({ type: "ready", version: wasmVersion?.() ?? "unknown" });
    return;
  }

  try {
    const loadedModule = await loadWasmModule(baseUrl);
    wasmModule = loadedModule;

    const version: () => string = loadedModule.version;
    wasmVersion = version;
    loadedModule.setup(); // Initialize panic hooks.

    initialized = true;
    send({ type: "ready", version: version() });
  } catch (err) {
    sendWorkerError(
      `Failed to initialize WASM: ${err instanceof Error ? err.message : String(err)}`,
    );
  }
}

// =============================================================================
// Execute Pipeline — Run a full pipeline through the WASM executor
// =============================================================================

/** Convert input files from ArrayBuffer to Uint8Array for WASM. */
function toWasmFiles(files: Array<{ name: string; data: ArrayBuffer; mimeType: string }>) {
  return files.map((f) => ({
    name: f.name,
    data: new Uint8Array(f.data),
    mimeType: f.mimeType,
  }));
}

interface WasmResultFile {
  name: string;
  data: Uint8Array;
  mimeType: string;
  metadata?: string;
}

/** Convert WASM result files: Uint8Array → ArrayBuffer for postMessage transfer. */
function fromWasmResults(files: Iterable<WasmResultFile>) {
  return Array.from(files).map((f) => ({
    name: f.name as string,
    data: f.data.buffer.slice(
      f.data.byteOffset,
      f.data.byteOffset + f.data.byteLength,
    ) as ArrayBuffer,
    mimeType: f.mimeType as string,
    metadata: f.metadata,
  }));
}

function handleExecutePipeline(request: {
  id: string;
  definitionJson: string;
  files: Array<{ name: string; data: ArrayBuffer; mimeType: string }>;
}): void {
  const { id, definitionJson, files } = request;

  if (!initialized || !wasmModule) {
    sendPipelineError(id, "WASM not initialized. Send 'init' first.");
    return;
  }

  try {
    const progressCallback = (eventJson: string) => sendPipelineProgress(id, eventJson);
    const result = wasmModule.execute_pipeline(
      definitionJson,
      toWasmFiles(files),
      progressCallback,
    );
    sendPipelineResult(
      id,
      fromWasmResults(result.files as Iterable<WasmResultFile>),
      result.durationMs as number,
    );
  } catch (err) {
    sendPipelineError(id, err instanceof Error ? err.message : String(err));
  }
}
