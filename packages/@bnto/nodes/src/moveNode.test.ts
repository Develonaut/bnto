import { describe, expect, it } from "vitest";

import { addNode } from "./addNode";
import { createBlankDefinition } from "./createBlankDefinition";
import { moveNode } from "./moveNode";
import { isValid } from "./definitionResult";

// Blank definition now has 2 I/O nodes (input + output).
const IO_NODE_COUNT = 2;

describe("moveNode", () => {
  it("updates a node's position", () => {
    const blank = createBlankDefinition();
    const { definition: withNode } = addNode(blank, "image");
    const nodeId = withNode.nodes![IO_NODE_COUNT]!.id;

    const result = moveNode(withNode, nodeId, { x: 300, y: 400 });
    expect(result.definition.nodes![IO_NODE_COUNT]!.position).toEqual({ x: 300, y: 400 });
    expect(isValid(result)).toBe(true);
  });

  it("does not mutate the original definition", () => {
    const blank = createBlankDefinition();
    const { definition: withNode } = addNode(blank, "image");
    const nodeId = withNode.nodes![IO_NODE_COUNT]!.id;

    moveNode(withNode, nodeId, { x: 300, y: 400 });
    expect(withNode.nodes![IO_NODE_COUNT]!.position).toEqual({ x: 0, y: 0 });
  });

  it("returns the same definition if node ID not found", () => {
    const blank = createBlankDefinition();
    const { definition: withNode } = addNode(blank, "image");

    const result = moveNode(withNode, "nonexistent", { x: 100, y: 100 });
    expect(result.definition).toBe(withNode);
  });

  it("moves the correct node when multiple nodes exist", () => {
    const blank = createBlankDefinition();
    const r1 = addNode(blank, "image", { x: 0, y: 0 });
    const r2 = addNode(r1.definition, "transform", { x: 100, y: 0 });
    const secondNodeId = r2.definition.nodes![IO_NODE_COUNT + 1]!.id;

    const result = moveNode(r2.definition, secondNodeId, { x: 500, y: 300 });

    // First added node unchanged
    expect(result.definition.nodes![IO_NODE_COUNT]!.position).toEqual({ x: 0, y: 0 });
    // Second added node moved
    expect(result.definition.nodes![IO_NODE_COUNT + 1]!.position).toEqual({ x: 500, y: 300 });
  });

  it("works on nested nodes (inside containers)", () => {
    const blank = createBlankDefinition();
    const { definition: withLoop } = addNode(blank, "loop");
    const loopNode = withLoop.nodes![IO_NODE_COUNT]!;

    const childNode = {
      id: "child-1",
      type: "image" as const,
      version: "1.0.0",
      name: "Nested Image",
      position: { x: 0, y: 0 },
      metadata: {},
      parameters: {},
      inputPorts: [],
      outputPorts: [{ id: "out-1", name: "output" }],
    };

    const withChild = {
      ...withLoop,
      nodes: [...withLoop.nodes!.slice(0, IO_NODE_COUNT), { ...loopNode, nodes: [childNode] }],
    };

    const result = moveNode(withChild, "child-1", { x: 200, y: 150 });
    expect(result.definition.nodes![IO_NODE_COUNT]!.nodes![0]!.position).toEqual({
      x: 200,
      y: 150,
    });
  });

  it("can move the root definition itself", () => {
    const blank = createBlankDefinition();
    const result = moveNode(blank, blank.id, { x: 50, y: 75 });

    expect(result.definition.position).toEqual({ x: 50, y: 75 });
  });
});
