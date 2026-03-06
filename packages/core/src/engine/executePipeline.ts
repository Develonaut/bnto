// ---------------------------------------------------------------------------
// Pipeline Executor — runtime-agnostic recipe execution
//
// STUB: This file exists so tests can import it and fail for the right reason
// (missing logic, not missing module). Implementation comes in Sprint 4H Wave 2.
// ---------------------------------------------------------------------------

import type {
  FileInput,
  NodeRunner,
  PipelineDefinition,
  PipelineProgressCallback,
  PipelineResult,
} from "./types";

/**
 * Execute a pipeline definition against a set of input files.
 *
 * Walks nodes in definition order, skips I/O nodes, iterates all files
 * through each processing node, chains outputs between nodes.
 *
 * @param definition - Ordered list of nodes to execute
 * @param files - Input files to process
 * @param runNode - Runtime-specific single-file processor
 * @param onProgress - Optional progress callback
 * @returns All output files and timing metadata
 */
export async function executePipeline(
  _definition: PipelineDefinition,
  _files: FileInput[],
  _runNode: NodeRunner,
  _onProgress?: PipelineProgressCallback,
): Promise<PipelineResult> {
  throw new Error("Not implemented — Sprint 4H Wave 2");
}
