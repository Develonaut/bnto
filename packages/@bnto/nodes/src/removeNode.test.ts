import { describe, expect, it } from "vitest";

import { addNode } from "./addNode";
import { createBlankDefinition } from "./createBlankDefinition";
import { removeNode } from "./removeNode";
import { isValid } from "./definitionResult";

describe("removeNode", () => {
  it("removes a node by ID", () => {
    const blank = createBlankDefinition();
    const { definition: withNode } = addNode(blank, "image");
    const nodeId = withNode.nodes![0]!.id;

    const result = removeNode(withNode, nodeId);
    expect(result.definition.nodes).toHaveLength(0);
    expect(isValid(result)).toBe(true);
  });

  it("does not mutate the original definition", () => {
    const blank = createBlankDefinition();
    const { definition: withNode } = addNode(blank, "image");
    const nodeId = withNode.nodes![0]!.id;

    removeNode(withNode, nodeId);
    expect(withNode.nodes).toHaveLength(1);
  });

  it("returns the same definition if node ID not found", () => {
    const blank = createBlankDefinition();
    const { definition: withNode } = addNode(blank, "image");

    const result = removeNode(withNode, "nonexistent-id");
    expect(result.definition.nodes).toHaveLength(1);
    expect(result.definition).toBe(withNode);
  });

  it("removes edges connected to the removed node", () => {
    const blank = createBlankDefinition();
    const r1 = addNode(blank, "image");
    const r2 = addNode(r1.definition, "transform");
    const node1Id = r2.definition.nodes![0]!.id;
    const node2Id = r2.definition.nodes![1]!.id;

    // Manually add an edge between the two nodes
    const withEdge = {
      ...r2.definition,
      edges: [
        { id: "e1", source: node1Id, target: node2Id },
      ],
    };

    const result = removeNode(withEdge, node1Id);
    expect(result.definition.nodes).toHaveLength(1);
    expect(result.definition.nodes![0]!.id).toBe(node2Id);
    expect(result.definition.edges).toHaveLength(0);
  });

  it("only removes edges connected to the removed node", () => {
    const blank = createBlankDefinition();
    const r1 = addNode(blank, "image");
    const r2 = addNode(r1.definition, "transform");
    const r3 = addNode(r2.definition, "spreadsheet");
    const node1Id = r3.definition.nodes![0]!.id;
    const node2Id = r3.definition.nodes![1]!.id;
    const node3Id = r3.definition.nodes![2]!.id;

    const withEdges = {
      ...r3.definition,
      edges: [
        { id: "e1", source: node1Id, target: node2Id },
        { id: "e2", source: node2Id, target: node3Id },
      ],
    };

    // Remove node1 — only e1 should be removed (it references node1 as source)
    const result = removeNode(withEdges, node1Id);
    expect(result.definition.nodes).toHaveLength(2);
    expect(result.definition.edges).toHaveLength(1);
    expect(result.definition.edges![0]!.id).toBe("e2");
  });

  it("removes the correct node from a definition with multiple nodes", () => {
    const blank = createBlankDefinition();
    const r1 = addNode(blank, "image");
    const r2 = addNode(r1.definition, "transform");
    const r3 = addNode(r2.definition, "spreadsheet");
    const middleNodeId = r3.definition.nodes![1]!.id;

    const result = removeNode(r3.definition, middleNodeId);
    expect(result.definition.nodes).toHaveLength(2);
    expect(result.definition.nodes![0]!.type).toBe("image");
    expect(result.definition.nodes![1]!.type).toBe("spreadsheet");
  });

  it("handles removing from a definition with no nodes", () => {
    const blank = createBlankDefinition();
    const result = removeNode(blank, "nonexistent");
    expect(result.definition.nodes).toHaveLength(0);
    expect(result.definition).toBe(blank);
  });
});
