import type {
  FileInput,
  FileResult,
  NodeRunner,
  PipelineDefinition,
  PipelineNode,
  PipelineProgressCallback,
  PipelineResult,
} from "./types";

/** Container node types that have children sub-pipelines. */
const CONTAINER_TYPES = new Set(["loop", "group", "parallel"]);

/** I/O marker types that are skipped during execution. */
const IO_TYPES = new Set(["input", "output"]);

/**
 * Runtime-agnostic pipeline executor.
 *
 * Walks an ordered list of nodes, skips I/O markers, and chains file
 * outputs through each processing node sequentially. The only external
 * dependency is `runNode` — a single-file processing function that
 * every runtime (browser WASM, CLI, server, test mock) implements.
 *
 * Supports container nodes (loop, group) by executing their children
 * recursively. A `loop` runs its children once per file. A `group`
 * runs its children once on the full batch.
 *
 * @param definition - Ordered node list to execute. I/O nodes are skipped automatically.
 * @param files - Input files to feed through the pipeline.
 * @param runNode - Runtime-specific single-file processor (browser WASM, CLI, server, mock).
 * @param onProgress - Optional callback fired per-file per-node with pipeline-level context.
 * @returns All output files from the final processing node, plus wall-clock duration.
 */
export async function executePipeline(
  definition: PipelineDefinition,
  files: FileInput[],
  runNode: NodeRunner,
  onProgress?: PipelineProgressCallback,
): Promise<PipelineResult> {
  const start = performance.now();

  // Filter to only processing nodes — I/O nodes are structural markers
  const processingNodes = definition.nodes.filter((node) => !IO_TYPES.has(node.type));

  // No processing nodes → pass input files through unchanged
  if (processingNodes.length === 0 || files.length === 0) {
    const passthroughFiles: FileResult[] = files.map((f) => ({
      name: f.name,
      data: f.data,
      mimeType: f.mimeType,
    }));
    return { files: passthroughFiles, durationMs: performance.now() - start };
  }

  // Track the current batch as FileResult[] so metadata survives chaining.
  // Seed with input files converted to FileResult shape (no metadata yet).
  let currentBatch: FileResult[] = files.map((f) => ({
    name: f.name,
    data: f.data,
    mimeType: f.mimeType,
  }));

  for (let nodeIndex = 0; nodeIndex < processingNodes.length; nodeIndex++) {
    const node = processingNodes[nodeIndex];
    currentBatch = await executeNode(node, currentBatch, runNode, onProgress, nodeIndex);
  }

  return {
    files: currentBatch,
    durationMs: performance.now() - start,
  };
}

/** Execute a single node — dispatches to container or primitive handling. */
async function executeNode(
  node: PipelineNode,
  batch: FileResult[],
  runNode: NodeRunner,
  onProgress: PipelineProgressCallback | undefined,
  nodeIndex: number,
): Promise<FileResult[]> {
  if (CONTAINER_TYPES.has(node.type) && node.children?.length) {
    return executeContainerNode(node, batch, runNode);
  }
  return executePrimitiveNode(node, batch, runNode, onProgress, nodeIndex);
}

/** Execute a primitive node — runs each file through the node. */
async function executePrimitiveNode(
  node: PipelineNode,
  batch: FileResult[],
  runNode: NodeRunner,
  onProgress: PipelineProgressCallback | undefined,
  nodeIndex: number,
): Promise<FileResult[]> {
  const nodeResults: FileResult[] = [];

  for (let fileIndex = 0; fileIndex < batch.length; fileIndex++) {
    const file = batch[fileIndex];
    const fileProgress = (percent: number, message: string) => {
      onProgress?.(nodeIndex, fileIndex, batch.length, percent, message);
    };
    const result = await runNode(file, node.type, node.params, fileProgress);
    nodeResults.push(result);
  }

  return nodeResults;
}

/**
 * Execute a container node by running its children sub-pipeline.
 *
 * - `loop`: runs children once PER file (each iteration gets a single-file batch)
 * - `group`: runs children once on the FULL batch
 * - `parallel`: same as group for now (concurrent execution is future)
 */
async function executeContainerNode(
  node: PipelineNode,
  batch: FileResult[],
  runNode: NodeRunner,
): Promise<FileResult[]> {
  const children = node.children ?? [];
  const childPipeline: PipelineDefinition = { nodes: children };

  if (node.type === "loop") {
    const allResults: FileResult[] = [];
    for (const file of batch) {
      const iterationResult = await executePipeline(childPipeline, [file], runNode);
      allResults.push(...iterationResult.files);
    }
    return allResults;
  }

  // group / parallel: run children on the full batch
  const result = await executePipeline(childPipeline, batch, runNode);
  return result.files;
}
