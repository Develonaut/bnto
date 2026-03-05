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

import type {
  WorkerRequest,
  WorkerResponse,
  ProgressResponse,
  ResultResponse,
  ErrorResponse,
  WorkerErrorResponse,
} from "./workerProtocol";

/**
 * Signature for a combined WASM node function.
 *
 * Combined functions process a file ONCE and return both metadata + bytes
 * in a single JS object. This replaces the old dual-function pattern
 * (metadata + bytes) that processed each file TWICE.
 *
 * The returned object has shape:
 *   { metadata: string, data: Uint8Array, filename: string, mimeType: string }
 */
type WasmCombinedFn = (
  data: Uint8Array,
  filename: string,
  paramsJson: string,
  progressCallback: (percent: number, message: string) => void,
) => { metadata: string; data: Uint8Array; filename: string; mimeType: string };

// Worker postMessage with Transferable[] for zero-copy ArrayBuffer transfers.
function postMessageWithTransfer(msg: unknown, transfer: Transferable[]): void {
  (self as unknown as { postMessage(m: unknown, t: Transferable[]): void })
    .postMessage(msg, transfer);
}

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

/** Fetch WASM glue JS, instantiate the module, return the exports object. */
async function loadWasmModule(baseUrl: string) {
  const wasmUrl = `${baseUrl}/wasm/bnto_wasm_bg.wasm`;
  const jsUrl = `${baseUrl}/wasm/bnto_wasm.js`;

  // Fetch the wasm-pack glue code and load it as a module via blob URL.
  // Absolute URLs are required because bundled workers can't resolve
  // root-relative paths. baseUrl comes from the main thread's origin.
  const response = await fetch(jsUrl);
  const blob = new Blob([await response.text()], { type: "application/javascript" });
  const blobUrl = URL.createObjectURL(blob);
  const wasmModule = await import(/* webpackIgnore: true */ blobUrl);
  URL.revokeObjectURL(blobUrl);

  await wasmModule.default(wasmUrl);
  return wasmModule;
}

/**
 * Map each node type to its combined WASM function.
 *
 * Combined functions process a file ONCE and return both metadata + bytes.
 * This replaces the old dual-function pattern that processed each file TWICE,
 * causing progress bars to jump 0→100→0→100 and doubling CPU time.
 */
// eslint-disable-next-line @typescript-eslint/no-explicit-any -- dynamic WASM module
function registerNodeTypes(m: Record<string, any>): void {
  nodeRegistry.set("compress-images", m.compress_image_combined);
  nodeRegistry.set("resize-images", m.resize_image_combined);
  nodeRegistry.set("convert-image-format", m.convert_image_format_combined);
  nodeRegistry.set("clean-csv", m.clean_csv_combined);
  nodeRegistry.set("rename-csv-columns", m.rename_csv_columns_combined);
  nodeRegistry.set("rename-files", m.rename_file_combined);
}

async function handleInit(baseUrl: string): Promise<void> {
  if (initialized) {
    send({ type: "ready", version: wasmVersion?.() ?? "unknown" });
    return;
  }

  try {
    const wasmModule = await loadWasmModule(baseUrl);

    const version: () => string = wasmModule.version;
    wasmVersion = version;
    registerNodeTypes(wasmModule);
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

/** Copy WASM output bytes into a standalone ArrayBuffer (avoids linear memory invalidation). */
function copyToBuffer(source: Uint8Array): ArrayBuffer {
  const buffer = new ArrayBuffer(source.byteLength);
  new Uint8Array(buffer).set(source);
  return buffer;
}

/**
 * Call a combined WASM function and package the output.
 * Single WASM call — processes the file once, returns metadata + bytes.
 */
function callWasmNode(
  fn: WasmCombinedFn,
  data: ArrayBuffer,
  filename: string,
  params: Record<string, unknown>,
  onProgress: (percent: number, message: string) => void,
) {
  const result = fn(new Uint8Array(data), filename, JSON.stringify(params), onProgress);
  return {
    data: copyToBuffer(result.data),
    filename: result.filename,
    mimeType: result.mimeType,
    metadata: JSON.parse(result.metadata) as Record<string, unknown>,
  };
}

// =============================================================================
// Response Helpers
// =============================================================================

function send(response: WorkerResponse): void {
  self.postMessage(response);
}

function sendProgress(id: string, percent: number, message: string): void {
  self.postMessage({ type: "progress", id, percent, message } satisfies ProgressResponse);
}

function sendResult(
  id: string, data: ArrayBuffer, filename: string,
  mimeType: string, metadata: Record<string, unknown>,
): void {
  const response: ResultResponse = { type: "result", id, data, filename, mimeType, metadata };
  postMessageWithTransfer(response, [data]);
}

function sendError(id: string, message: string): void {
  self.postMessage({ type: "error", id, message } satisfies ErrorResponse);
}

function sendWorkerError(message: string): void {
  self.postMessage({ type: "worker-error", message } satisfies WorkerErrorResponse);
}
