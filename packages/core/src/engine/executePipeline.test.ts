import { describe, expect, it, vi } from "vitest";

import { executePipeline } from "./executePipeline";
import type {
  FileInput,
  FileResult,
  NodeRunner,
  PipelineDefinition,
  PipelineProgressCallback,
} from "./types";

// ---------------------------------------------------------------------------
// Helpers — reusable test fixtures
// ---------------------------------------------------------------------------

/** Create a FileInput with predictable data. */
function makeFile(name: string, content = "hello"): FileInput {
  return {
    name,
    data: new TextEncoder().encode(content),
    mimeType: "text/plain",
  };
}

/** Create a FileResult that mirrors a FileInput (simulating pass-through). */
function makeResult(name: string, content = "processed"): FileResult {
  return {
    name,
    data: new TextEncoder().encode(content),
    mimeType: "text/plain",
  };
}

/**
 * Create a mock NodeRunner that transforms filenames predictably.
 * Each call appends `-{nodeType}` to the filename so chaining is verifiable.
 */
function createChainingRunner(): NodeRunner {
  return vi.fn(async (file: FileInput, nodeType: string) => ({
    name: `${file.name}-${nodeType}`,
    data: file.data,
    mimeType: file.mimeType,
  }));
}

/** A pipeline with only I/O nodes — no processing. */
const IO_ONLY_PIPELINE: PipelineDefinition = {
  nodes: [
    { id: "in", type: "input", params: {} },
    { id: "out", type: "output", params: {} },
  ],
};

/** A single-node pipeline (input → compress → output). */
const SINGLE_NODE_PIPELINE: PipelineDefinition = {
  nodes: [
    { id: "in", type: "input", params: {} },
    { id: "compress", type: "compress-images", params: { quality: 80 } },
    { id: "out", type: "output", params: {} },
  ],
};

/** A multi-node pipeline (input → compress → resize → output). */
const MULTI_NODE_PIPELINE: PipelineDefinition = {
  nodes: [
    { id: "in", type: "input", params: {} },
    { id: "compress", type: "compress-images", params: { quality: 80 } },
    { id: "resize", type: "resize-images", params: { width: 800 } },
    { id: "out", type: "output", params: {} },
  ],
};

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

describe("executePipeline", () => {
  // --- Single processing node ---

  it("processes all files through a single node", async () => {
    const runNode = vi.fn(async (file: FileInput) => makeResult(`out-${file.name}`));
    const files = [makeFile("a.jpg"), makeFile("b.jpg"), makeFile("c.jpg")];

    const result = await executePipeline(SINGLE_NODE_PIPELINE, files, runNode);

    expect(result.files).toHaveLength(3);
    expect(result.files.map((f) => f.name)).toEqual(["out-a.jpg", "out-b.jpg", "out-c.jpg"]);
  });

  // --- Multi-node pipeline ---

  it("chains outputs between nodes — output of node N becomes input of node N+1", async () => {
    const runNode = createChainingRunner();
    const files = [makeFile("photo.jpg")];

    const result = await executePipeline(MULTI_NODE_PIPELINE, files, runNode);

    // File passes through compress then resize — name accumulates both suffixes
    expect(result.files).toHaveLength(1);
    expect(result.files[0].name).toBe("photo.jpg-compress-images-resize-images");
  });

  it("chains multiple files through multiple nodes correctly", async () => {
    const runNode = createChainingRunner();
    const files = [makeFile("a.jpg"), makeFile("b.jpg")];

    const result = await executePipeline(MULTI_NODE_PIPELINE, files, runNode);

    expect(result.files).toHaveLength(2);
    expect(result.files[0].name).toBe("a.jpg-compress-images-resize-images");
    expect(result.files[1].name).toBe("b.jpg-compress-images-resize-images");
  });

  // --- I/O node skipping ---

  it("does not call runNode for input or output nodes", async () => {
    const runNode: NodeRunner = vi.fn(async (file) => makeResult(file.name));
    const files = [makeFile("test.csv")];

    await executePipeline(SINGLE_NODE_PIPELINE, files, runNode);

    // Only the compress-images node should trigger runNode, not input/output
    const mock = vi.mocked(runNode);
    expect(mock).toHaveBeenCalledTimes(1);
    for (const call of mock.mock.calls) {
      expect(call[1]).not.toBe("input");
      expect(call[1]).not.toBe("output");
    }
  });

  it("returns input files unchanged when pipeline has only I/O nodes", async () => {
    const runNode = vi.fn();
    const files = [makeFile("doc.txt", "original")];

    const result = await executePipeline(IO_ONLY_PIPELINE, files, runNode);

    expect(runNode).not.toHaveBeenCalled();
    expect(result.files).toHaveLength(1);
    expect(result.files[0].name).toBe("doc.txt");
    expect(new TextDecoder().decode(result.files[0].data)).toBe("original");
  });

  // --- Call count assertion ---

  it("calls runNode exactly processingNodes × files times", async () => {
    const runNode = vi.fn(async (file: FileInput) => makeResult(file.name));
    const files = [makeFile("a.jpg"), makeFile("b.jpg"), makeFile("c.jpg")];

    // MULTI_NODE_PIPELINE has 2 processing nodes (compress + resize)
    await executePipeline(MULTI_NODE_PIPELINE, files, runNode);

    expect(runNode).toHaveBeenCalledTimes(2 * 3); // 2 nodes × 3 files = 6
  });

  // --- Node failure propagation ---

  it("rejects with the error when runNode fails", async () => {
    const runNode = vi.fn().mockRejectedValue(new Error("WASM crashed"));
    const files = [makeFile("bad.jpg")];

    await expect(executePipeline(SINGLE_NODE_PIPELINE, files, runNode)).rejects.toThrow(
      "WASM crashed",
    );
  });

  it("stops processing remaining files when a node fails mid-batch", async () => {
    const runNode = vi.fn(async (file: FileInput) => {
      if (file.name === "b.jpg") throw new Error("corrupt file");
      return makeResult(file.name);
    });
    const files = [makeFile("a.jpg"), makeFile("b.jpg"), makeFile("c.jpg")];

    await expect(executePipeline(SINGLE_NODE_PIPELINE, files, runNode)).rejects.toThrow(
      "corrupt file",
    );
  });

  // --- Empty file array ---

  it("resolves with empty results and no runNode calls for empty file array", async () => {
    const runNode = vi.fn();

    const result = await executePipeline(SINGLE_NODE_PIPELINE, [], runNode);

    expect(runNode).not.toHaveBeenCalled();
    expect(result.files).toEqual([]);
    expect(result.durationMs).toBeGreaterThanOrEqual(0);
  });

  // --- Single file ---

  it("handles single file identically to multi-file", async () => {
    const runNode = createChainingRunner();
    const files = [makeFile("solo.png")];

    const result = await executePipeline(MULTI_NODE_PIPELINE, files, runNode);

    expect(result.files).toHaveLength(1);
    expect(result.files[0].name).toBe("solo.png-compress-images-resize-images");
    expect(runNode).toHaveBeenCalledTimes(2); // 2 processing nodes × 1 file
  });

  // --- Progress aggregation ---

  it("fires onProgress with correct nodeIndex, fileIndex, totalFiles", async () => {
    const onProgress: PipelineProgressCallback = vi.fn();
    const runNode = vi.fn(async (file: FileInput, _type: string, _params, onNodeProgress) => {
      // Simulate 50% then 100% progress
      onNodeProgress?.(50, "halfway");
      onNodeProgress?.(100, "done");
      return makeResult(file.name);
    });
    const files = [makeFile("a.jpg"), makeFile("b.jpg")];

    await executePipeline(SINGLE_NODE_PIPELINE, files, runNode, onProgress);

    // Single processing node (index 0), 2 files
    // File 0: 50%, 100%
    // File 1: 50%, 100%
    expect(onProgress).toHaveBeenCalledWith(0, 0, 2, 50, "halfway");
    expect(onProgress).toHaveBeenCalledWith(0, 0, 2, 100, "done");
    expect(onProgress).toHaveBeenCalledWith(0, 1, 2, 50, "halfway");
    expect(onProgress).toHaveBeenCalledWith(0, 1, 2, 100, "done");
  });

  it("increments nodeIndex for each processing node in progress callbacks", async () => {
    const onProgress: PipelineProgressCallback = vi.fn();
    const runNode = vi.fn(async (file: FileInput, _type: string, _params, onNodeProgress) => {
      onNodeProgress?.(100, "done");
      return makeResult(file.name);
    });
    const files = [makeFile("a.jpg")];

    await executePipeline(MULTI_NODE_PIPELINE, files, runNode, onProgress);

    // 2 processing nodes, 1 file each
    expect(onProgress).toHaveBeenCalledWith(0, 0, 1, 100, "done"); // compress
    expect(onProgress).toHaveBeenCalledWith(1, 0, 1, 100, "done"); // resize
  });

  // --- Node order guarantee ---

  it("processes nodes in definition order", async () => {
    const callOrder: string[] = [];
    const runNode = vi.fn(async (file: FileInput, nodeType: string) => {
      callOrder.push(nodeType);
      return makeResult(file.name);
    });
    const files = [makeFile("test.jpg")];

    await executePipeline(MULTI_NODE_PIPELINE, files, runNode);

    expect(callOrder).toEqual(["compress-images", "resize-images"]);
  });

  it("processes files in input order within each node", async () => {
    const callOrder: string[] = [];
    const runNode = vi.fn(async (file: FileInput, nodeType: string) => {
      callOrder.push(`${nodeType}:${file.name}`);
      return makeResult(file.name);
    });
    const files = [makeFile("first.jpg"), makeFile("second.jpg"), makeFile("third.jpg")];

    await executePipeline(SINGLE_NODE_PIPELINE, files, runNode);

    expect(callOrder).toEqual([
      "compress-images:first.jpg",
      "compress-images:second.jpg",
      "compress-images:third.jpg",
    ]);
  });

  // --- runNode receives correct args ---

  it("passes correct nodeType and params to runNode", async () => {
    const runNode = vi.fn(async (file: FileInput) => makeResult(file.name));
    const files = [makeFile("img.jpg")];

    await executePipeline(SINGLE_NODE_PIPELINE, files, runNode);

    expect(runNode).toHaveBeenCalledWith(
      expect.objectContaining({ name: "img.jpg" }),
      "compress-images",
      { quality: 80 },
      expect.any(Function),
    );
  });

  it("passes each node's own params in multi-node pipeline", async () => {
    const runNode = vi.fn(async (file: FileInput) => makeResult(file.name));
    const files = [makeFile("img.jpg")];

    await executePipeline(MULTI_NODE_PIPELINE, files, runNode);

    // First call: compress-images with quality
    expect(runNode).toHaveBeenCalledWith(
      expect.anything(),
      "compress-images",
      { quality: 80 },
      expect.any(Function),
    );
    // Second call: resize-images with width
    expect(runNode).toHaveBeenCalledWith(
      expect.anything(),
      "resize-images",
      { width: 800 },
      expect.any(Function),
    );
  });

  // --- Result structure ---

  it("returns durationMs as a positive number", async () => {
    const runNode = vi.fn(async (file: FileInput) => makeResult(file.name));
    const files = [makeFile("a.jpg")];

    const result = await executePipeline(SINGLE_NODE_PIPELINE, files, runNode);

    expect(result.durationMs).toBeGreaterThanOrEqual(0);
    expect(typeof result.durationMs).toBe("number");
  });

  it("preserves metadata from the final node's results", async () => {
    const runNode = vi.fn(async (file: FileInput) => ({
      ...makeResult(file.name),
      metadata: { compressionRatio: 0.65 },
    }));
    const files = [makeFile("a.jpg")];

    const result = await executePipeline(SINGLE_NODE_PIPELINE, files, runNode);

    expect(result.files[0].metadata).toEqual({ compressionRatio: 0.65 });
  });

  // --- Edge case: no onProgress callback ---

  it("works without an onProgress callback", async () => {
    const runNode = vi.fn(async (file: FileInput) => makeResult(file.name));
    const files = [makeFile("a.jpg")];

    // No onProgress — should not throw
    const result = await executePipeline(SINGLE_NODE_PIPELINE, files, runNode);

    expect(result.files).toHaveLength(1);
  });
});
