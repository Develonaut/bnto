// ---------------------------------------------------------------------------
// Pipeline Definition — the execution-oriented view of a recipe
//
// This is what gets serialized to JSON and sent to the WASM executor.
// Simpler than Definition (which has ports, edges, positions, metadata).
// Just what the executor needs: type, params, and I/O markers.
// ---------------------------------------------------------------------------

/** A single node in the pipeline's execution order. */
export interface PipelineNode {
  /** Unique node ID within this pipeline. */
  id: string;
  /** Node type name (e.g., "compress-images", "input", "output"). */
  type: string;
  /** Parameters for this node (quality, format, pattern, etc.). */
  params: Record<string, unknown>;
  /** Child nodes (for group and loop container nodes). */
  children?: PipelineNode[];
}

/** Execution-oriented pipeline — node list for the WASM executor. */
export interface PipelineDefinition {
  /** Ordered list of nodes to execute. I/O nodes are skipped automatically. */
  nodes: PipelineNode[];
}
