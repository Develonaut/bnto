// ---------------------------------------------------------------------------
// Pipeline Definition — the execution-oriented view of a recipe
//
// This is what gets serialized to JSON and sent to the WASM executor.
// Simpler than Definition (which has ports, positions, metadata). Just what
// the executor needs: type, params, edges, settings, and I/O markers.
// Mirrors the Rust `PipelineDefinition` in bnto-core's `pipeline.rs`.
// ---------------------------------------------------------------------------

import type { PipelineSettings } from "@bnto/registry";

/** A connection between two sibling nodes; files flow source → target. */
export interface PipelineEdge {
  id: string;
  source: string;
  target: string;
}

/** A single node in the pipeline's execution order. */
export interface PipelineNode {
  /** Unique node ID within this pipeline. */
  id: string;
  /** Node type name (e.g., "image-compress", "input", "output"). */
  type: string;
  /** Parameters for this node (quality, format, pattern, etc.). */
  params: Record<string, unknown>;
  /** Child nodes (for group and loop container nodes). */
  children?: PipelineNode[];
  /** Connections between this container's children. Absent = a linear chain. */
  edges?: PipelineEdge[];
}

/** Execution-oriented pipeline — node list for the WASM executor. */
export interface PipelineDefinition {
  /** Ordered list of nodes to execute. I/O nodes are skipped automatically. */
  nodes: PipelineNode[];
  /** Connections between root-level nodes. Absent = a linear chain. */
  edges?: PipelineEdge[];
  /** Recipe-level settings: iteration mode and the whole run's time limit. */
  settings?: PipelineSettings;
}
