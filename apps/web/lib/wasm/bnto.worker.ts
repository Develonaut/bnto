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
} from "./types";

/** Signature shared by all WASM node functions (metadata or bytes variant). */
type WasmFn<T> = (
  data: Uint8Array,
  filename: string,
  paramsJson: string,
  progressCallback: (percent: number, message: string) => void,
) => T;

/** A pair of WASM functions for one node type: metadata JSON + raw bytes. */
interface WasmNodeFns {
  metadata: WasmFn<string>;
  bytes: WasmFn<Uint8Array>;
}

// Worker postMessage with Transferable[] for zero-copy ArrayBuffer transfers.
function postMessageWithTransfer(msg: unknown, transfer: Transferable[]): void {
  (self as unknown as { postMessage(m: unknown, t: Transferable[]): void })
    .postMessage(msg, transfer);
}

// =============================================================================
// State
// =============================================================================

let wasmVersion: (() => string) | null = null;
const nodeRegistry = new Map<string, WasmNodeFns>();
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

/** Map each node type to its WASM metadata + bytes function pair. */
// eslint-disable-next-line @typescript-eslint/no-explicit-any -- dynamic WASM module
function registerNodeTypes(m: Record<string, any>): void {
  nodeRegistry.set("compress-images", { metadata: m.compress_image, bytes: m.compress_image_bytes });
  nodeRegistry.set("resize-images", { metadata: m.resize_image, bytes: m.resize_image_bytes });
  nodeRegistry.set("convert-image-format", { metadata: m.convert_image_format, bytes: m.convert_image_format_bytes });
  nodeRegistry.set("clean-csv", { metadata: m.clean_csv, bytes: m.clean_csv_bytes });
  nodeRegistry.set("rename-csv-columns", { metadata: m.rename_csv_columns, bytes: m.rename_csv_columns_bytes });
  nodeRegistry.set("rename-files", { metadata: m.rename_file, bytes: m.rename_file_bytes });
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

function handleProcess(request: {
  id: string;
  data: ArrayBuffer;
  filename: string;
  mimeType: string;
  nodeType: string;
  params: Record<string, unknown>;
}): void {
  if (!initialized) {
    sendError(request.id, "WASM not initialized. Send 'init' first.");
    return;
  }

  const { id, data, filename, nodeType, params } = request;
  const fns = nodeRegistry.get(nodeType);
  if (!fns) {
    sendError(id, `Unsupported node type: ${nodeType}`);
    return;
  }

  try {
    const result = callWasmNode(fns, data, filename, params, (pct, msg) => {
      sendProgress(id, pct, msg);
    });
    sendResult(id, result.data, result.filename, result.mimeType, result.metadata);
  } catch (err) {
    sendError(id, err instanceof Error ? err.message : String(err));
  }
}

/** Call a WASM node's metadata + bytes functions and package the output. */
function callWasmNode(
  fns: WasmNodeFns,
  data: ArrayBuffer,
  filename: string,
  params: Record<string, unknown>,
  onProgress: (percent: number, message: string) => void,
) {
  const bytes = new Uint8Array(data);
  const paramsJson = JSON.stringify(params);

  const metadataJson = fns.metadata(bytes, filename, paramsJson, onProgress);
  const meta = JSON.parse(metadataJson) as {
    file?: { filename?: string; mimeType?: string };
    metadata?: Record<string, unknown>;
  };

  const processedBytes = fns.bytes(bytes, filename, paramsJson, onProgress);

  // Copy into a guaranteed ArrayBuffer (not ArrayBufferLike).
  const outputBuffer = new ArrayBuffer(processedBytes.byteLength);
  new Uint8Array(outputBuffer).set(processedBytes);

  return {
    data: outputBuffer,
    filename: meta.file?.filename ?? filename,
    mimeType: meta.file?.mimeType ?? "application/octet-stream",
    metadata: meta.metadata ?? {},
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
