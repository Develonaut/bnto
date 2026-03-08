// ---------------------------------------------------------------------------
// Pipeline Events — Structured progress from the Rust executor
//
// These types mirror the PipelineEvent enum in bnto-core/src/events.rs.
// The Rust executor serializes events as JSON strings; the JS adapter
// parses them into these types for UI consumption.
// ---------------------------------------------------------------------------

/** Discriminated union of all pipeline execution events. */
export type PipelineEvent =
  | PipelineStartedEvent
  | NodeStartedEvent
  | FileProgressEvent
  | NodeCompletedEvent
  | NodeFailedEvent
  | PipelineCompletedEvent
  | PipelineFailedEvent;

export interface PipelineStartedEvent {
  type: "PipelineStarted";
  totalNodes: number;
  totalFiles: number;
}

export interface NodeStartedEvent {
  type: "NodeStarted";
  nodeId: string;
  nodeIndex: number;
  totalNodes: number;
  nodeType: string;
}

export interface FileProgressEvent {
  type: "FileProgress";
  nodeId: string;
  fileIndex: number;
  totalFiles: number;
  percent: number;
  message: string;
}

export interface NodeCompletedEvent {
  type: "NodeCompleted";
  nodeId: string;
  durationMs: number;
  filesProcessed: number;
}

export interface NodeFailedEvent {
  type: "NodeFailed";
  nodeId: string;
  error: string;
}

export interface PipelineCompletedEvent {
  type: "PipelineCompleted";
  durationMs: number;
  totalFilesProcessed: number;
}

export interface PipelineFailedEvent {
  type: "PipelineFailed";
  nodeId: string;
  error: string;
}
