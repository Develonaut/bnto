import { describe, expect, it } from "vitest";
import type { Definition } from "@bnto/core";
import { extractProcessingNodes } from "./extractProcessingNodes";

/** Minimal definition factory for testing. */
function defn(overrides: Partial<Definition> = {}): Definition {
  return {
    id: "root",
    type: "group",
    version: "1.0.0",
    name: "Test Recipe",
    position: { x: 0, y: 0 },
    metadata: {},
    parameters: {},
    inputPorts: [],
    outputPorts: [],
    nodes: [],
    edges: [],
    ...overrides,
  };
}

function node(overrides: Partial<Definition>): Definition {
  return defn({
    position: { x: 0, y: 0 },
    metadata: {},
    inputPorts: [],
    outputPorts: [],
    ...overrides,
  });
}

describe("extractProcessingNodes", () => {
  it("returns empty array for a definition with no child nodes", () => {
    const result = extractProcessingNodes(defn());
    expect(result).toEqual([]);
  });

  it("filters out input and output nodes", () => {
    const result = extractProcessingNodes(
      defn({
        nodes: [
          node({ id: "input", type: "input", name: "Input", parameters: { mode: "file-upload" } }),
          node({
            id: "compress",
            type: "image-compress",
            name: "Compress",
            parameters: { quality: 80 },
          }),
          node({ id: "output", type: "output", name: "Output", parameters: { mode: "download" } }),
        ],
      }),
    );
    expect(result).toHaveLength(1);
    expect(result[0].id).toBe("compress");
    expect(result[0].type).toBe("image-compress");
  });

  it("filters out container nodes (group, loop)", () => {
    const result = extractProcessingNodes(
      defn({
        nodes: [
          node({ id: "input", type: "input", name: "Input", parameters: {} }),
          node({ id: "loop", type: "loop", name: "Loop", parameters: {} }),
          node({
            id: "compress",
            type: "image-compress",
            name: "Compress",
            parameters: { quality: 80 },
          }),
          node({ id: "output", type: "output", name: "Output", parameters: {} }),
        ],
      }),
    );
    expect(result).toHaveLength(1);
    expect(result[0].type).toBe("image-compress");
  });

  it("returns id, type, name, and parameters for each processing node", () => {
    const result = extractProcessingNodes(
      defn({
        nodes: [
          node({ id: "input", type: "input", name: "Input", parameters: {} }),
          node({
            id: "compress",
            type: "image-compress",
            name: "Compress",
            parameters: { quality: 80 },
          }),
          node({ id: "output", type: "output", name: "Output", parameters: {} }),
        ],
      }),
    );
    expect(result[0]).toEqual({
      id: "compress",
      type: "image-compress",
      name: "Compress",
      parameters: { quality: 80 },
    });
  });

  it("preserves node order for multi-node pipelines", () => {
    const result = extractProcessingNodes(
      defn({
        nodes: [
          node({ id: "input", type: "input", name: "Input", parameters: {} }),
          node({
            id: "compress",
            type: "image-compress",
            name: "Compress",
            parameters: { quality: 80 },
          }),
          node({
            id: "rename",
            type: "file-rename",
            name: "Rename",
            parameters: { suffix: "-min" },
          }),
          node({ id: "output", type: "output", name: "Output", parameters: {} }),
        ],
      }),
    );
    expect(result).toHaveLength(2);
    expect(result[0].type).toBe("image-compress");
    expect(result[1].type).toBe("file-rename");
  });

  it("handles definitions with no nodes array", () => {
    const result = extractProcessingNodes(defn({ nodes: undefined }));
    expect(result).toEqual([]);
  });
});
