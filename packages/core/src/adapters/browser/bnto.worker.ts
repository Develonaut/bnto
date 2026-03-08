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
    // Build the files array in the format WASM expects:
    // [{name: string, data: Uint8Array, mimeType: string}]
    const wasmFiles = files.map((f) => ({
      name: f.name,
      data: new Uint8Array(f.data),
      mimeType: f.mimeType,
    }));

    // Progress callback — forwards each PipelineEvent JSON string
    // from the Rust executor to the main thread.
    const progressCallback = (eventJson: string) => {
      sendPipelineProgress(id, eventJson);
    };

    // Call the WASM execute_pipeline export.
    // Returns { files: [{name, data, mimeType, metadata?}], durationMs }
    const result = wasmModule.execute_pipeline(definitionJson, wasmFiles, progressCallback);

    // Convert result files: Uint8Array → ArrayBuffer for transfer.
    const resultFiles = Array.from(
      result.files as Iterable<{
        name: string;
        data: Uint8Array;
        mimeType: string;
        metadata?: string;
      }>,
    ).map((f) => ({
      name: f.name as string,
      data: (f.data as Uint8Array).buffer.slice(
        (f.data as Uint8Array).byteOffset,
        (f.data as Uint8Array).byteOffset + (f.data as Uint8Array).byteLength,
      ) as ArrayBuffer,
      mimeType: f.mimeType as string,
      metadata: f.metadata,
    }));

    sendPipelineResult(id, resultFiles, result.durationMs as number);
  } catch (err) {
    sendPipelineError(id, err instanceof Error ? err.message : String(err));
  }
}
