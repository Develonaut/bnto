/**
 * Typed message protocol for the WASM Web Worker.
 *
 * Main Thread → Worker: WorkerRequest
 * Worker → Main Thread: WorkerResponse
 *
 * Every request carries an `id` so the main thread can correlate
 * responses with the original request.
 */

// =============================================================================
// Main Thread → Worker (requests)
// =============================================================================

/** Initialize the WASM module. Must be sent once before any pipeline requests. */
export interface InitRequest {
  type: "init";
  /** Base URL for resolving WASM assets (e.g., "http://localhost:3100"). */
  baseUrl: string;
}

/** Execute an entire pipeline through the WASM executor. */
export interface ExecutePipelineRequest {
  type: "execute-pipeline";
  /** Unique ID for this request (correlates with responses). */
  id: string;
  /** JSON string of the pipeline definition. */
  definitionJson: string;
  /** Input files as transferable objects. */
  files: Array<{
    name: string;
    data: ArrayBuffer;
    mimeType: string;
  }>;
}

export type WorkerRequest = InitRequest | ExecutePipelineRequest;

// =============================================================================
// Worker → Main Thread (responses)
// =============================================================================

/** WASM module initialized and ready. */
export interface ReadyResponse {
  type: "ready";
  /** WASM engine version string. */
  version: string;
}

/** Unexpected worker-level error (not tied to a specific request). */
export interface WorkerErrorResponse {
  type: "worker-error";
  /** Human-readable error message. */
  message: string;
}

/** Structured pipeline event forwarded from the Rust executor. */
export interface PipelineProgressResponse {
  type: "pipeline-progress";
  /** The request ID this event belongs to. */
  id: string;
  /** JSON-serialized PipelineEvent from the Rust executor. */
  eventJson: string;
}

/** Pipeline execution completed — all result files. */
export interface PipelineResultResponse {
  type: "pipeline-result";
  /** The request ID this result belongs to. */
  id: string;
  /** Output files from the pipeline. */
  files: Array<{
    name: string;
    data: ArrayBuffer;
    mimeType: string;
    metadata?: string;
  }>;
  /** Total pipeline duration in milliseconds. */
  durationMs: number;
}

/** Pipeline execution failed. */
export interface PipelineErrorResponse {
  type: "pipeline-error";
  /** The request ID this error belongs to. */
  id: string;
  /** Human-readable error message. */
  message: string;
}

export type WorkerResponse =
  | ReadyResponse
  | WorkerErrorResponse
  | PipelineProgressResponse
  | PipelineResultResponse
  | PipelineErrorResponse;
