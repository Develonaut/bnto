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
import { loadWasmModule, registerNodeTypes, callWasmNode, resolveWasmFn } from "./wasmLoader";
import { send, sendProgress, sendResult, sendError, sendWorkerError } from "./workerResponses";

// =============================================================================
// State
// =============================================================================

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
    const wasmModule = await loadWasmModule(baseUrl);

    const version: () => string = wasmModule.version;
    wasmVersion = version;
    registerNodeTypes(wasmModule, nodeRegistry);
    wasmModule.setup(); // Initialize panic hooks.

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

/** Resolve the WASM function for a node type + params, or send an error. */
function resolveNodeFn(
  id: string,
  nodeType: string,
  params: Record<string, unknown>,
): WasmCombinedFn | null {
  if (!initialized) {
    sendError(id, "WASM not initialized. Send 'init' first.");
    return null;
  }
  const fn = resolveWasmFn(nodeRegistry, nodeType, params);
  if (!fn) {
    const operation = params.operation;
    const detail = typeof operation === "string" ? `${nodeType}:${operation}` : nodeType;
    sendError(id, `Unsupported node type: ${detail}`);
    return null;
  }
  return fn;
}

function handleProcess(request: {
  id: string;
  data: ArrayBuffer;
  filename: string;
  nodeType: string;
  params: Record<string, unknown>;
}): void {
  const { id, data, filename, nodeType, params } = request;
  const fn = resolveNodeFn(id, nodeType, params);
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
