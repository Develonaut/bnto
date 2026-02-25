/**
 * Typed message protocol for the WASM Web Worker.
 *
 * Main Thread → Worker: WorkerRequest
 * Worker → Main Thread: WorkerResponse
 *
 * Every request carries an `id` so the main thread can correlate
 * responses with the original request (multiple files process concurrently).
 */

// =============================================================================
// Main Thread → Worker (requests)
// =============================================================================

/** Initialize the WASM module. Must be sent once before any process requests. */
export interface InitRequest {
  type: "init";
}

/** Process a single file through a WASM node. */
export interface ProcessRequest {
  type: "process";
  /** Unique ID for this request (correlates with responses). */
  id: string;
  /** The raw file bytes (transferred, not copied). */
  data: ArrayBuffer;
  /** Original filename (e.g., "photo.jpg"). */
  filename: string;
  /** MIME type (e.g., "image/jpeg"). */
  mimeType: string;
  /** Node type to execute (e.g., "compress-images"). */
  nodeType: string;
  /** Node-specific config as JSON-serializable params. */
  params: Record<string, unknown>;
}

export type WorkerRequest = InitRequest | ProcessRequest;

// =============================================================================
// Worker → Main Thread (responses)
// =============================================================================

/** WASM module initialized and ready to process files. */
export interface ReadyResponse {
  type: "ready";
  /** WASM engine version string. */
  version: string;
}

/** Progress update for a specific file being processed. */
export interface ProgressResponse {
  type: "progress";
  /** The request ID this progress belongs to. */
  id: string;
  /** Percentage complete (0-100). */
  percent: number;
  /** Human-readable status message. */
  message: string;
}

/** File processed successfully. */
export interface ResultResponse {
  type: "result";
  /** The request ID this result belongs to. */
  id: string;
  /** Compressed/processed file bytes (transferred, not copied). */
  data: ArrayBuffer;
  /** Output filename (e.g., "photo-compressed.jpg"). */
  filename: string;
  /** Output MIME type (e.g., "image/jpeg"). */
  mimeType: string;
  /** Processing metadata (compression ratio, timing, etc.). */
  metadata: Record<string, unknown>;
}

/** Processing failed for a specific file. */
export interface ErrorResponse {
  type: "error";
  /** The request ID this error belongs to. */
  id: string;
  /** Human-readable error message. */
  message: string;
}

/** Unexpected worker-level error (not tied to a specific request). */
export interface WorkerErrorResponse {
  type: "worker-error";
  /** Human-readable error message. */
  message: string;
}

export type WorkerResponse =
  | ReadyResponse
  | ProgressResponse
  | ResultResponse
  | ErrorResponse
  | WorkerErrorResponse;
