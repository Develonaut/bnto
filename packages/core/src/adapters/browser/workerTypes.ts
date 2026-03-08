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
