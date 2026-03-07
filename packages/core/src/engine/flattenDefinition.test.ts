import { describe, it, expect } from "vitest";
import { flattenDefinition } from "./flattenDefinition";
import type { Definition } from "@bnto/nodes";

/** Minimal valid Definition for testing. */
function makeDef(overrides: Partial<Definition> = {}): Definition {
  return {
    id: "root",
    type: "group",
    version: "1.0.0",
    name: "Test",
    position: { x: 0, y: 0 },
    metadata: {},
    parameters: {},
    inputPorts: [],
    outputPorts: [],
    ...overrides,
  };
}

describe("flattenDefinition", () => {
  it("converts a flat 3-node definition to PipelineDefinition", () => {
    const def = makeDef({
      nodes: [
        makeDef({ id: "input", type: "input", parameters: { mode: "file-upload" } }),
        makeDef({
          id: "compress",
          type: "image",
          parameters: { operation: "compress", quality: 80 },
        }),
        makeDef({ id: "output", type: "output", parameters: { mode: "download" } }),
      ],
    });

    const pipeline = flattenDefinition(def);

    expect(pipeline.nodes).toHaveLength(3);
    expect(pipeline.nodes[0]).toEqual({
      id: "input",
      type: "input",
      params: { mode: "file-upload" },
    });
    expect(pipeline.nodes[1]).toEqual({
      id: "compress",
      type: "image",
      params: { operation: "compress", quality: 80 },
    });
    expect(pipeline.nodes[2]).toEqual({
      id: "output",
      type: "output",
      params: { mode: "download" },
    });
  });

  it("preserves children for container nodes", () => {
    const def = makeDef({
      nodes: [
        makeDef({ id: "input", type: "input" }),
        makeDef({
          id: "my-loop",
          type: "loop",
          parameters: { mode: "forEach" },
          nodes: [makeDef({ id: "child-1", type: "image", parameters: { operation: "compress" } })],
        }),
        makeDef({ id: "output", type: "output" }),
      ],
    });

    const pipeline = flattenDefinition(def);

    expect(pipeline.nodes).toHaveLength(3);
    const loopNode = pipeline.nodes[1];
    expect(loopNode.type).toBe("loop");
    expect(loopNode.children).toHaveLength(1);
    expect(loopNode.children![0]).toEqual({
      id: "child-1",
      type: "image",
      params: { operation: "compress" },
    });
  });

  it("handles empty nodes array", () => {
    const def = makeDef({ nodes: [] });
    const pipeline = flattenDefinition(def);
    expect(pipeline.nodes).toEqual([]);
  });

  it("handles missing nodes property", () => {
    const def = makeDef();
    delete def.nodes;
    const pipeline = flattenDefinition(def);
    expect(pipeline.nodes).toEqual([]);
  });

  it("handles nested groups recursively", () => {
    const def = makeDef({
      nodes: [
        makeDef({
          id: "outer-group",
          type: "group",
          nodes: [
            makeDef({
              id: "inner-loop",
              type: "loop",
              nodes: [
                makeDef({ id: "deep-node", type: "image", parameters: { operation: "resize" } }),
              ],
            }),
          ],
        }),
      ],
    });

    const pipeline = flattenDefinition(def);

    expect(pipeline.nodes).toHaveLength(1);
    const outer = pipeline.nodes[0];
    expect(outer.children).toHaveLength(1);
    const inner = outer.children![0];
    expect(inner.children).toHaveLength(1);
    expect(inner.children![0].id).toBe("deep-node");
  });

  it("does not add children for primitive nodes", () => {
    const def = makeDef({
      nodes: [makeDef({ id: "img", type: "image", parameters: { operation: "compress" } })],
    });

    const pipeline = flattenDefinition(def);

    expect(pipeline.nodes[0].children).toBeUndefined();
  });
});
