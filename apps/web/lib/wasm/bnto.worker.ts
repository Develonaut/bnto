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

// Web Worker postMessage with Transferable[] support.
// The main tsconfig uses "lib": ["dom"] which types self.postMessage
// as the Window overload (no transfer list). This helper provides the
// correct Worker overload for zero-copy ArrayBuffer transfers.
function postMessageWithTransfer(message: unknown, transfer: Transferable[]): void {
  (self as unknown as { postMessage(msg: unknown, transfer: Transferable[]): void })
    .postMessage(message, transfer);
}

// =============================================================================
// WASM Module Reference
// =============================================================================

// These are populated after init() loads the WASM binary.
// We use `any` here because the WASM module is dynamically imported
// and its types aren't available in the worker bundle.
let wasmSetup: (() => void) | null = null;
let wasmVersion: (() => string) | null = null;
let wasmCompressImage: ((
  data: Uint8Array,
  filename: string,
  paramsJson: string,
  progressCallback: (percent: number, message: string) => void,
) => string) | null = null;
let wasmCompressImageBytes: ((
  data: Uint8Array,
  filename: string,
  paramsJson: string,
  progressCallback: (percent: number, message: string) => void,
) => Uint8Array) | null = null;

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
// Init — Load the WASM binary
// =============================================================================

async function handleInit(baseUrl: string): Promise<void> {
  if (initialized) {
    // Already initialized — just re-send ready.
    send({ type: "ready", version: wasmVersion?.() ?? "unknown" });
    return;
  }

  try {
    // Dynamically import the WASM glue code from the public directory.
    // The JS glue file handles fetching and instantiating the .wasm binary.
    //
    // We use absolute URLs because Web Workers created from bundled
    // module URLs (Turbopack, webpack) can't resolve root-relative
    // paths like "/wasm/...". The baseUrl comes from the main thread's
    // window.location.origin (e.g., "http://localhost:3100").
    const wasmUrl = `${baseUrl}/wasm/bnto_wasm_bg.wasm`;
    const jsUrl = `${baseUrl}/wasm/bnto_wasm.js`;

    // Fetch and evaluate the JS glue code.
    const response = await fetch(jsUrl);
    const jsCode = await response.text();

    // The wasm-pack --target web output uses ES module syntax.
    // In a worker context, we need to handle this carefully.
    // We create a blob URL to import it as a module.
    const blob = new Blob([jsCode], { type: "application/javascript" });
    const blobUrl = URL.createObjectURL(blob);

    // Use dynamic import to load the module.
    const wasmModule = await import(/* webpackIgnore: true */ blobUrl);
    URL.revokeObjectURL(blobUrl);

    // Initialize the WASM binary by passing the URL to the .wasm file.
    await wasmModule.default(wasmUrl);

    // Store references to the exported functions.
    wasmSetup = wasmModule.setup;
    wasmVersion = wasmModule.version;
    wasmCompressImage = wasmModule.compress_image;
    wasmCompressImageBytes = wasmModule.compress_image_bytes;

    // Initialize panic hooks.
    wasmSetup!();

    initialized = true;

    send({
      type: "ready",
      version: wasmVersion!(),
    });
  } catch (err) {
    sendWorkerError(
      `Failed to initialize WASM: ${err instanceof Error ? err.message : String(err)}`,
    );
  }
}

// =============================================================================
// Process — Run a node on a single file
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

  try {
    // Convert ArrayBuffer to Uint8Array for WASM.
    const bytes = new Uint8Array(data);
    const paramsJson = JSON.stringify(params);

    // Create a progress callback that forwards to the main thread.
    const progressCallback = (percent: number, message: string) => {
      sendProgress(id, percent, message);
    };

    // Route to the correct WASM function based on node type.
    switch (nodeType) {
      case "compress-images": {
        // Get metadata (for the result response).
        const metadataJson = wasmCompressImage!(
          bytes,
          filename,
          paramsJson,
          progressCallback,
        );
        const metadata = JSON.parse(metadataJson) as {
          file?: { filename?: string; mimeType?: string };
          metadata?: Record<string, unknown>;
        };

        // Get the actual compressed bytes.
        const compressedBytes = wasmCompressImageBytes!(
          bytes,
          filename,
          paramsJson,
          progressCallback,
        );

        const outputFilename = metadata.file?.filename ?? filename;
        const outputMimeType = metadata.file?.mimeType ?? "application/octet-stream";
        // Extract the compressed bytes into a new ArrayBuffer.
        // We use the ArrayBuffer constructor to guarantee the type
        // (Uint8Array.buffer is ArrayBufferLike which includes SharedArrayBuffer).
        const outputBuffer = new ArrayBuffer(compressedBytes.byteLength);
        new Uint8Array(outputBuffer).set(compressedBytes);

        sendResult(id, outputBuffer, outputFilename, outputMimeType, metadata.metadata ?? {});
        break;
      }

      default:
        sendError(id, `Unsupported node type: ${nodeType}`);
    }
  } catch (err) {
    sendError(id, err instanceof Error ? err.message : String(err));
  }
}

// =============================================================================
// Response Helpers
// =============================================================================

function send(response: WorkerResponse): void {
  self.postMessage(response);
}

function sendProgress(id: string, percent: number, message: string): void {
  const response: ProgressResponse = { type: "progress", id, percent, message };
  self.postMessage(response);
}

function sendResult(
  id: string,
  data: ArrayBuffer,
  filename: string,
  mimeType: string,
  metadata: Record<string, unknown>,
): void {
  const response: ResultResponse = {
    type: "result",
    id,
    data,
    filename,
    mimeType,
    metadata,
  };
  // Transfer the ArrayBuffer to avoid copying (zero-copy).
  postMessageWithTransfer(response, [data]);
}

function sendError(id: string, message: string): void {
  const response: ErrorResponse = { type: "error", id, message };
  self.postMessage(response);
}

function sendWorkerError(message: string): void {
  const response: WorkerErrorResponse = { type: "worker-error", message };
  self.postMessage(response);
}
