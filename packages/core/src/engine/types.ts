// ---------------------------------------------------------------------------
// Pipeline Engine Types (runtime-agnostic — no browser, WASM, or Worker imports)
//
// These types define the vocabulary for the pipeline executor layer.
// Every runtime (browser WASM, CLI, server, test) implements NodeRunner.
// Everything else — node walking, file iteration, output chaining,
// progress aggregation — lives in executePipeline() and uses these types.
// ---------------------------------------------------------------------------

/**
 * A single file entering or exiting the pipeline.
 *
 * Uses `Uint8Array` (not `File` or `Blob`) so it works in Node.js,
 * WASM, CLI, and browser contexts without platform-specific APIs.
 */
export interface FileInput {
  /** Original filename (e.g., "photo.jpg"). */
  name: string;
  /** Raw file bytes. */
  data: Uint8Array;
  /** MIME type (e.g., "image/jpeg"). */
  mimeType: string;
}

/**
 * Result of processing a single file through one node.
 *
 * Same shape as FileInput so outputs chain directly as inputs
 * to the next node — no conversion needed between pipeline stages.
 */
export interface FileResult {
  /** Output filename (e.g., "photo-compressed.webp"). */
  name: string;
  /** Processed file bytes. */
  data: Uint8Array;
  /** Output MIME type. */
  mimeType: string;
  /** Optional metadata from the processor (compression ratio, timing, etc.). */
  metadata?: Record<string, unknown>;
}

/**
 * The single-file processing contract every runtime implements.
 *
 * Browser: wraps `BntoWorker.processFile()`
 * CLI: wraps Rust native `process()`
 * Server: wraps HTTP call to Go API
 * Test: `vi.fn()` mock
 *
 * One file in, one result out. Always.
 */
export type NodeRunner = (
  file: FileInput,
  nodeType: string,
  params: Record<string, unknown>,
  onProgress?: (percent: number, message: string) => void,
) => Promise<FileResult>;

// ---------------------------------------------------------------------------
// Pipeline Definition — the execution-oriented view of a recipe
// ---------------------------------------------------------------------------

/**
 * A single node in the pipeline's execution order.
 *
 * Simpler than RecipeDefinition (which has ports, edges, positions, metadata).
 * This is just what the executor needs: type, params, and I/O markers.
 *
 * Container nodes (loop, group, parallel) have `children` — a sub-pipeline
 * the executor runs recursively.
 */
export interface PipelineNode {
  /** Unique node ID within this pipeline. */
  id: string;
  /** Node type name (e.g., "image", "input", "output", "loop", "group"). */
  type: string;
  /** Parameters for this node (quality, format, pattern, etc.). */
  params: Record<string, unknown>;
  /** Child nodes for container types (loop, group, parallel). */
  children?: PipelineNode[];
}

/**
 * Execution-oriented recipe description for the pipeline executor.
 *
 * NOT the same as RecipeDefinition (which is the full Convex-backed shape
 * with ports, edges, positions, and recursive nodes). PipelineDefinition
 * is the flattened, ordered node list the executor walks.
 */
export interface PipelineDefinition {
  /** Ordered list of nodes to execute. I/O nodes are skipped automatically. */
  nodes: PipelineNode[];
}

// ---------------------------------------------------------------------------
// Progress and Results
// ---------------------------------------------------------------------------

/**
 * Progress callback signature for the pipeline executor.
 *
 * Fires once per file per node, giving consumers enough context
 * to build both per-node and overall progress indicators.
 */
export type PipelineProgressCallback = (
  /** Index of the current processing node (0-based, skips I/O). */
  nodeIndex: number,
  /** Index of the current file within the batch (0-based). */
  fileIndex: number,
  /** Total number of files in the batch. */
  totalFiles: number,
  /** Percentage complete for this file at this node (0-100). */
  percent: number,
  /** Human-readable status message from the processor. */
  message: string,
) => void;

/**
 * Final result of executing a pipeline.
 *
 * Contains all output files from the last processing node
 * plus timing metadata.
 */
export interface PipelineResult {
  /** All output files from the final processing node. */
  files: FileResult[];
  /** Wall-clock duration of the entire pipeline in milliseconds. */
  durationMs: number;
}
