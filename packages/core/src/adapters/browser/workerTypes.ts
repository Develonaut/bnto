/** Callback for progress updates during file processing. */
export type ProgressCallback = (percent: number, message: string) => void;

/** The result of processing a single file. */
export interface ProcessResult {
  /** Processed file as a Blob (ready for download or display). */
  blob: Blob;
  /** Output filename (e.g., "photo-compressed.jpg"). */
  filename: string;
  /** Output MIME type. */
  mimeType: string;
  /** Processing metadata (compression ratio, timing, etc.). */
  metadata: Record<string, unknown>;
}

/** Tracks a pending request waiting for a response from the worker. */
export interface PendingRequest {
  resolve: (result: ProcessResult) => void;
  reject: (error: Error) => void;
  onProgress?: ProgressCallback;
}

/** Result of executing a full pipeline through the WASM executor. */
export interface PipelineExecutionResult {
  files: Array<{
    name: string;
    data: ArrayBuffer;
    mimeType: string;
    metadata?: string;
  }>;
  durationMs: number;
}

/** Tracks a pending pipeline execution request. */
export interface PendingPipelineRequest {
  resolve: (result: PipelineExecutionResult) => void;
  reject: (error: Error) => void;
  onEvent?: (event: import("../../types/pipelineEvents").PipelineEvent) => void;
}
