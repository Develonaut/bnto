import type {
  FileInput,
  FileResult,
  NodeRunner,
  PipelineDefinition,
  PipelineProgressCallback,
  PipelineResult,
} from "./types";

/**
 * Runtime-agnostic pipeline executor.
 *
 * Walks an ordered list of nodes, skips I/O markers, and chains file
 * outputs through each processing node sequentially. The only external
 * dependency is `runNode` — a single-file processing function that
 * every runtime (browser WASM, CLI, server, test mock) implements.
 *
 * No browser APIs. No WASM. No Worker. No dynamic imports.
 * This function is pure orchestration logic — testable in plain Node.js.
 *
 * **Why `NodeRunner` is injected:** The executor doesn't know (or care)
 * whether files are processed by WASM in a Web Worker, a native Rust
 * binary, or a remote Go API. The caller provides the runtime-specific
 * implementation. This is what makes the function runtime-agnostic.
 *
 * **Future: loop node override.** When the `loop` node type arrives,
 * it will need special handling here (iterate its children N times).
 * That logic belongs in this function — not in any runtime adapter.
 */
export async function executePipeline(
  definition: PipelineDefinition,
  files: FileInput[],
  runNode: NodeRunner,
  onProgress?: PipelineProgressCallback,
): Promise<PipelineResult> {
  const start = performance.now();

  // Filter to only processing nodes — I/O nodes are structural markers
  const processingNodes = definition.nodes.filter(
    (node) => node.type !== "input" && node.type !== "output",
  );

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
    const nodeResults: FileResult[] = [];

    for (let fileIndex = 0; fileIndex < currentBatch.length; fileIndex++) {
      const file = currentBatch[fileIndex];

      // Build a per-file progress callback that adds pipeline-level context.
      // Always pass a function to runNode (not undefined) so runtimes
      // can call onProgress?.() without null-checking.
      const fileProgress = (percent: number, message: string) => {
        onProgress?.(nodeIndex, fileIndex, currentBatch.length, percent, message);
      };

      // FileResult is a superset of FileInput (same name/data/mimeType),
      // so it satisfies the NodeRunner's FileInput parameter directly.
      const result = await runNode(file, node.type, node.params, fileProgress);
      nodeResults.push(result);
    }

    // Chain: this node's outputs become the next node's inputs
    currentBatch = nodeResults;
  }

  return {
    files: currentBatch,
    durationMs: performance.now() - start,
  };
}
