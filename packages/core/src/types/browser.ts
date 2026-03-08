// ---------------------------------------------------------------------------
// Browser execution types (transport-agnostic — no WASM imports)
//
// These types define the result shapes and state for browser-based execution
// via the Rust WASM pipeline executor.
// ---------------------------------------------------------------------------

/** Result of processing a single file in the browser. */
export interface BrowserFileResult {
  /** Processed file as a Blob (ready for download or display). */
  blob: Blob;
  /** Output filename (e.g., "photo-compressed.jpg"). */
  filename: string;
  /** Output MIME type (e.g., "image/jpeg"). */
  mimeType: string;
  /** Processing metadata (compression ratio, timing, etc.). */
  metadata: Record<string, unknown>;
}

/** State of an in-progress or completed browser execution. */
export interface BrowserExecution {
  /** Client-generated execution ID. */
  id: string;
  /** Current execution status. */
  status: "idle" | "processing" | "completed" | "failed";
  /** Per-file progress (updated as each file is processed). */
  fileProgress: BrowserFileProgress | null;
  /** Results for completed files (grows as files finish). */
  results: BrowserFileResult[];
  /** Error message if execution failed. */
  error?: string;
  /** Timestamp when execution started. */
  startedAt?: number;
  /** Timestamp when execution completed or failed. */
  completedAt?: number;
}

/** Progress update for the current file being processed (store output). */
export interface BrowserFileProgress {
  /** Index of the file currently being processed (0-based). */
  fileIndex: number;
  /** Total number of files to process. */
  totalFiles: number;
  /** Percentage complete for the current file (0-100). */
  percent: number;
  /** Overall batch progress across all files (0-100). Computed by the store. */
  overallPercent: number;
  /** Human-readable status message from the engine. */
  message: string;
}

/** Progress input from the engine (before the store computes overallPercent). */
export type BrowserFileProgressInput = Omit<BrowserFileProgress, "overallPercent">;

/** Result payload returned by `ExecutionInstance.run()`. */
export interface BrowserRunResult {
  /** Terminal status of the execution. */
  status: "completed" | "aborted" | "failed";
  /** Processed files (empty on abort/failure). */
  results: BrowserFileResult[];
  /** Wall-clock duration in milliseconds. */
  durationMs: number;
  /** Error message (only present when status is "failed"). */
  error?: string;
}
