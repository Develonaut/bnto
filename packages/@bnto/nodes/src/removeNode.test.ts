import { describe, expect, it } from "vitest";

import { addNode } from "./addNode";
import { createBlankDefinition } from "./createBlankDefinition";
import { removeNode } from "./removeNode";
import { isValid } from "./definitionResult";

// Blank definition now has 2 I/O nodes (input + output).
const IO_NODE_COUNT = 2;

describe("removeNode", () => {
  it("removes a node by ID", () => {
    const blank = createBlankDefinition();
    const { definition: withNode } = addNode(blank, "image");
    const nodeId = withNode.nodes![IO_NODE_COUNT]!.id;

    const result = removeNode(withNode, nodeId);
    expect(result.definition.nodes).toHaveLength(IO_NODE_COUNT);
    expect(isValid(result)).toBe(true);
  });

  it("does not mutate the original definition", () => {
    const blank = createBlankDefinition();
    const { definition: withNode } = addNode(blank, "image");
    const nodeId = withNode.nodes![IO_NODE_COUNT]!.id;

    removeNode(withNode, nodeId);
    expect(withNode.nodes).toHaveLength(IO_NODE_COUNT + 1);
  });

  it("returns the same definition if node ID not found", () => {
    const blank = createBlankDefinition();
    const { definition: withNode } = addNode(blank, "image");

    const result = removeNode(withNode, "nonexistent-id");
    expect(result.definition.nodes).toHaveLength(IO_NODE_COUNT + 1);
    expect(result.definition).toBe(withNode);
  });

  it("removes edges connected to the removed node", () => {
    const blank = createBlankDefinition();
    const r1 = addNode(blank, "image");
    const r2 = addNode(r1.definition, "transform");
    const node1Id = r2.definition.nodes![IO_NODE_COUNT]!.id;
    const node2Id = r2.definition.nodes![IO_NODE_COUNT + 1]!.id;

    // Manually add an edge between the two nodes
    const withEdge = {
      ...r2.definition,
      edges: [
        { id: "e1", source: node1Id, target: node2Id },
      ],
    };

    const result = removeNode(withEdge, node1Id);
    expect(result.definition.nodes).toHaveLength(IO_NODE_COUNT + 1);
    expect(result.definition.nodes![IO_NODE_COUNT]!.id).toBe(node2Id);
    expect(result.definition.edges).toHaveLength(0);
  });

  it("only removes edges connected to the removed node", () => {
    const blank = createBlankDefinition();
    const r1 = addNode(blank, "image");
    const r2 = addNode(r1.definition, "transform");
    const r3 = addNode(r2.definition, "spreadsheet");
    const node1Id = r3.definition.nodes![IO_NODE_COUNT]!.id;
    const node2Id = r3.definition.nodes![IO_NODE_COUNT + 1]!.id;
    const node3Id = r3.definition.nodes![IO_NODE_COUNT + 2]!.id;

    const withEdges = {
      ...r3.definition,
      edges: [
        { id: "e1", source: node1Id, target: node2Id },
        { id: "e2", source: node2Id, target: node3Id },
      ],
    };

    // Remove node1 — only e1 should be removed (it references node1 as source)
    const result = removeNode(withEdges, node1Id);
    expect(result.definition.nodes).toHaveLength(IO_NODE_COUNT + 2);
    expect(result.definition.edges).toHaveLength(1);
    expect(result.definition.edges![0]!.id).toBe("e2");
  });

  it("removes the correct node from a definition with multiple nodes", () => {
    const blank = createBlankDefinition();
    const r1 = addNode(blank, "image");
    const r2 = addNode(r1.definition, "transform");
    const r3 = addNode(r2.definition, "spreadsheet");
    const middleNodeId = r3.definition.nodes![IO_NODE_COUNT + 1]!.id;

    const result = removeNode(r3.definition, middleNodeId);
    expect(result.definition.nodes).toHaveLength(IO_NODE_COUNT + 2);
    expect(result.definition.nodes![IO_NODE_COUNT]!.type).toBe("image");
    expect(result.definition.nodes![IO_NODE_COUNT + 1]!.type).toBe("spreadsheet");
  });

  it("handles removing from a definition with only I/O nodes", () => {
    const blank = createBlankDefinition();
    const result = removeNode(blank, "nonexistent");
    expect(result.definition.nodes).toHaveLength(IO_NODE_COUNT);
    expect(result.definition).toBe(blank);
  });

  it("removes a nested node inside a container", () => {
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

    const result = removeNode(withChild, "child-1");
    // I/O nodes + loop still exists, but its child is gone
    expect(result.definition.nodes).toHaveLength(IO_NODE_COUNT + 1);
    expect(result.definition.nodes![IO_NODE_COUNT]!.nodes).toHaveLength(0);
  });

  it("cleans up edges inside a container when removing a nested node", () => {
    const blank = createBlankDefinition();
    const { definition: withLoop } = addNode(blank, "loop");
    const loopNode = withLoop.nodes![IO_NODE_COUNT]!;

    const child1 = {
      id: "child-1",
      type: "image" as const,
      version: "1.0.0",
      name: "Image",
      position: { x: 0, y: 0 },
      metadata: {},
      parameters: {},
      inputPorts: [],
      outputPorts: [{ id: "out-1", name: "output" }],
    };

    const child2 = {
      id: "child-2",
      type: "transform" as const,
      version: "1.0.0",
      name: "Transform",
      position: { x: 100, y: 0 },
      metadata: {},
      parameters: {},
      inputPorts: [],
      outputPorts: [{ id: "out-2", name: "output" }],
    };

    const withChildren = {
      ...withLoop,
      nodes: [
        ...withLoop.nodes!.slice(0, IO_NODE_COUNT),
        {
          ...loopNode,
          nodes: [child1, child2],
          edges: [{ id: "e1", source: "child-1", target: "child-2" }],
        },
      ],
    };

    // Remove child-1 — the edge inside the loop should also be cleaned
    const result = removeNode(withChildren, "child-1");
    expect(result.definition.nodes![IO_NODE_COUNT]!.nodes).toHaveLength(1);
    expect(result.definition.nodes![IO_NODE_COUNT]!.nodes![0]!.id).toBe("child-2");
    expect(result.definition.nodes![IO_NODE_COUNT]!.edges).toHaveLength(0);
  });

  it("does not affect root edges when removing a nested node", () => {
    const blank = createBlankDefinition();
    const { definition: withLoop } = addNode(blank, "loop");
    const { definition: withImage } = addNode(withLoop, "image");
    const loopNode = withImage.nodes![IO_NODE_COUNT]!;
    const imageNode = withImage.nodes![IO_NODE_COUNT + 1]!;

    const childNode = {
      id: "child-1",
      type: "transform" as const,
      version: "1.0.0",
      name: "Nested Transform",
      position: { x: 0, y: 0 },
      metadata: {},
      parameters: {},
      inputPorts: [],
      outputPorts: [{ id: "out-1", name: "output" }],
    };

    // Root has an edge between loop and image; loop has a child node
    const withAll = {
      ...withImage,
      edges: [{ id: "root-e1", source: loopNode.id, target: imageNode.id }],
      nodes: [
        ...withImage.nodes!.slice(0, IO_NODE_COUNT),
        { ...loopNode, nodes: [childNode] },
        imageNode,
      ],
    };

    // Remove the nested child — root edge should be untouched
    const result = removeNode(withAll, "child-1");
    expect(result.definition.edges).toHaveLength(1);
    expect(result.definition.edges![0]!.id).toBe("root-e1");
    expect(result.definition.nodes![IO_NODE_COUNT]!.nodes).toHaveLength(0);
  });
});
