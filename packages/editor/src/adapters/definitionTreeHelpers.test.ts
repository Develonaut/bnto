/**
 * definitionTreeHelpers tests — pure function, no React or store needed.
 */

import { describe, it, expect } from "vitest";
import type { Definition } from "@bnto/core";
import {
  addChildToContainer,
  removeChildFromContainer,
  updateNodeInTree,
} from "./definitionTreeHelpers";

function makeContainer(id: string, children: Definition[] = []): Definition {
  return {
    id,
    type: "group",
    version: "1.0.0",
    name: `Container ${id}`,
    position: { x: 0, y: 0 },
    metadata: {},
    parameters: {},
    inputPorts: [],
    outputPorts: [],
    nodes: children,
  };
}

function makeLeaf(id: string, params: Record<string, unknown> = {}): Definition {
  return {
    id,
    type: "image-compress",
    version: "1.0.0",
    name: `Leaf ${id}`,
    position: { x: 0, y: 0 },
    metadata: {},
    parameters: params,
    inputPorts: [],
    outputPorts: [],
  };
}

describe("addChildToContainer", () => {
  it("adds a child to an empty container", () => {
    const root = makeContainer("root");
    const child = makeLeaf("child1");
    const result = addChildToContainer(root, "root", child);
    expect(result.nodes).toHaveLength(1);
    expect(result.nodes![0]!.id).toBe("child1");
  });

  it("prepends a child to existing children (no afterChildId)", () => {
    const root = makeContainer("root", [makeLeaf("a")]);
    const child = makeLeaf("b");
    const result = addChildToContainer(root, "root", child);
    expect(result.nodes).toHaveLength(2);
    expect(result.nodes![0]!.id).toBe("b");
    expect(result.nodes![1]!.id).toBe("a");
  });

  it("inserts after a specific child", () => {
    const root = makeContainer("root", [makeLeaf("a"), makeLeaf("c")]);
    const child = makeLeaf("b");
    const result = addChildToContainer(root, "root", child, "a");
    expect(result.nodes).toHaveLength(3);
    expect(result.nodes![0]!.id).toBe("a");
    expect(result.nodes![1]!.id).toBe("b");
    expect(result.nodes![2]!.id).toBe("c");
  });

  it("adds to nested container", () => {
    const inner = makeContainer("inner");
    const root = makeContainer("root", [inner]);
    const child = makeLeaf("child1");
    const result = addChildToContainer(root, "inner", child);
    expect(result.nodes![0]!.nodes).toHaveLength(1);
    expect(result.nodes![0]!.nodes![0]!.id).toBe("child1");
  });

  it("returns unchanged tree if containerId not found", () => {
    const root = makeContainer("root");
    const child = makeLeaf("child1");
    const result = addChildToContainer(root, "nonexistent", child);
    expect(result.nodes).toHaveLength(0);
  });
});

describe("removeChildFromContainer", () => {
  it("removes a child by ID", () => {
    const root = makeContainer("root", [makeLeaf("a"), makeLeaf("b")]);
    const result = removeChildFromContainer(root, "root", "a");
    expect(result.nodes).toHaveLength(1);
    expect(result.nodes![0]!.id).toBe("b");
  });

  it("removes from nested container", () => {
    const inner = makeContainer("inner", [makeLeaf("a"), makeLeaf("b")]);
    const root = makeContainer("root", [inner]);
    const result = removeChildFromContainer(root, "inner", "a");
    expect(result.nodes![0]!.nodes).toHaveLength(1);
    expect(result.nodes![0]!.nodes![0]!.id).toBe("b");
  });

  it("returns unchanged tree if childId not found", () => {
    const root = makeContainer("root", [makeLeaf("a")]);
    const result = removeChildFromContainer(root, "root", "nonexistent");
    expect(result.nodes).toHaveLength(1);
  });
});

describe("updateNodeInTree", () => {
  it("updates parameters of a root node", () => {
    const root = makeLeaf("root", { quality: 80 });
    const result = updateNodeInTree(root, "root", { quality: 95 });
    expect(result.parameters.quality).toBe(95);
  });

  it("updates a nested node's parameters", () => {
    const leaf = makeLeaf("leaf", { quality: 80 });
    const root = makeContainer("root", [leaf]);
    const result = updateNodeInTree(root, "leaf", { quality: 95 });
    expect(result.nodes![0]!.parameters.quality).toBe(95);
  });

  it("merges parameters (does not replace)", () => {
    const leaf = makeLeaf("leaf", { quality: 80, format: "jpg" });
    const root = makeContainer("root", [leaf]);
    const result = updateNodeInTree(root, "leaf", { quality: 95 });
    expect(result.nodes![0]!.parameters.quality).toBe(95);
    expect(result.nodes![0]!.parameters.format).toBe("jpg");
  });

  it("returns unchanged tree if nodeId not found", () => {
    const root = makeContainer("root", [makeLeaf("a")]);
    const result = updateNodeInTree(root, "nonexistent", { quality: 95 });
    expect(result).toEqual(root);
  });
});
