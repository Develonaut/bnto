// ---------------------------------------------------------------------------
// Browser execution types (transport-agnostic — no WASM imports)
//
// These types define the contract between @bnto/core and whatever browser
// engine implementation is used (e.g., Rust→WASM via Web Worker).
// ---------------------------------------------------------------------------

/**
 * Abstract engine interface for browser-based execution.
 *
 * Implemented by the WASM Web Worker wrapper (BntoWorker).
 * Engine initializes lazily on first use — no manual registration needed.
 *
 * The engine processes files entirely in the browser — no backend,
 * no R2, no network. Files never leave the user's machine.
 */
export interface BrowserEngine {
  /** Initialize the engine (load WASM, warm up worker). Safe to call multiple times. */
  init(): Promise<void>;

  /** Process a single file. Returns the processed file as a blob. */
  processFile(
    file: File,
    nodeType: string,
    params?: Record<string, unknown>,
    onProgress?: (percent: number, message: string) => void,
  ): Promise<BrowserFileResult>;

  /** Process multiple files sequentially. Reports per-file progress. */
  processFiles(
    files: File[],
    nodeType: string,
    params?: Record<string, unknown>,
    onProgress?: (fileIndex: number, percent: number, message: string) => void,
  ): Promise<BrowserFileResult[]>;

  /** Terminate the engine and clean up resources. */
  terminate(): void;

  /** Whether the engine is initialized and ready to process files. */
  readonly isReady: boolean;
}

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
