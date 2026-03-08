/**
 * Bnto WASM Web Worker
 *
 * Runs in a background thread. Loads the WASM binary, processes files,
 * and reports progress back to the main thread via postMessage.
 *
 * Files never leave the browser — all processing happens in WASM memory.
 *
 * Message flow:
 *   Main Thread → Worker: { type: "init" } or { type: "process", ... }
 *   Worker → Main Thread: { type: "ready" } or { type: "progress"|"result"|"error", ... }
 */

import type { WorkerRequest } from "./workerProtocol";
import type { WasmCombinedFn } from "./wasmLoader";
import { loadWasmModule, registerNodeTypes, callWasmNode } from "./wasmLoader";
import {
  send,
  sendProgress,
  sendResult,
  sendError,
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
const nodeRegistry = new Map<string, WasmCombinedFn>();
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
      case "process":
        handleProcess(request);
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
// Init — Load the WASM binary and register all node types
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
    registerNodeTypes(loadedModule, nodeRegistry);
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
// Process — Run a WASM node on a single file
// =============================================================================

/** Resolve the WASM function for a node type, or send an error. */
function resolveNodeFn(id: string, nodeType: string): WasmCombinedFn | null {
  if (!initialized) {
    sendError(id, "WASM not initialized. Send 'init' first.");
    return null;
  }
  const fn = nodeRegistry.get(nodeType);
  if (!fn) {
    sendError(id, `Unsupported node type: ${nodeType}`);
    return null;
  }
  return fn;
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

// =============================================================================
// Process — Run a WASM node on a single file
// =============================================================================

function handleProcess(request: {
  id: string;
  data: ArrayBuffer;
  filename: string;
  nodeType: string;
  params: Record<string, unknown>;
}): void {
  const { id, data, filename, nodeType, params } = request;
  const fn = resolveNodeFn(id, nodeType);
  if (!fn) return;

  try {
    const result = callWasmNode(fn, data, filename, params, (pct, msg) => {
      sendProgress(id, pct, msg);
    });
    sendResult(id, result.data, result.filename, result.mimeType, result.metadata);
  } catch (err) {
    sendError(id, err instanceof Error ? err.message : String(err));
  }
}
