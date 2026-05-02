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
  | IterationStartedEvent
  | FileProgressEvent
  | NodeCompletedEvent
  | NodeFailedEvent
  | PipelineCompletedEvent
  | PipelineFailedEvent
  | CommandOutputEvent;

export interface PipelineStartedEvent {
  type: "PipelineStarted";
  totalNodes: number;
  totalFiles: number;
  /** Metadata for each processing node — lets the UI populate node types immediately. */
  nodes: Array<{ id: string; nodeType: string }>;
}

export interface NodeStartedEvent {
  type: "NodeStarted";
  nodeId: string;
  nodeIndex: number;
  totalNodes: number;
  nodeType: string;
  /** If this node is inside a container, the container's ID. Null for top-level nodes. */
  parentNodeId: string | null;
}

export interface IterationStartedEvent {
  type: "IterationStarted";
  /** The loop container node that owns this iteration. */
  nodeId: string;
  /** Zero-based iteration index. */
  iteration: number;
  /** Total number of iterations in this loop. */
  totalIterations: number;
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
  /** If this node is inside a container, the container's ID. Null for top-level nodes. */
  parentNodeId: string | null;
}

export interface NodeFailedEvent {
  type: "NodeFailed";
  nodeId: string;
  error: string;
  /** If this node is inside a container, the container's ID. Null for top-level nodes. */
  parentNodeId: string | null;
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

export interface CommandOutputEvent {
  type: "CommandOutput";
  nodeId: string;
  line: string;
}
