import { describe, it, expect } from "vitest";
import { buildNodeListTree } from "./buildNodeListTree";
import type { BentoNode, NodeConfigs } from "../adapters/types";

function makeNode(id: string, overrides: Partial<BentoNode["data"]> = {}): BentoNode {
  return {
    id,
    type: "compartment",
    position: { x: 0, y: 0 },
    data: {
      label: id,
      variant: "muted",
      width: 200,
      height: 80,
      status: "idle",
      ...overrides,
    },
  };
}

function makeConfig(name: string) {
  return { nodeType: "image", name, parameters: {} };
}

describe("buildNodeListTree", () => {
  it("returns isEmpty when only IO nodes exist", () => {
    const nodes = [makeNode("input", { isIoNode: true }), makeNode("output", { isIoNode: true })];
    const configs: NodeConfigs = {
      input: makeConfig("Input"),
      output: makeConfig("Output"),
    };
    const tree = buildNodeListTree(nodes, configs);
    expect(tree.isEmpty).toBe(true);
    expect(tree.entries).toHaveLength(2);
  });

  it("places placeholderIndex before the last IO node (Output)", () => {
    const nodes = [makeNode("input", { isIoNode: true }), makeNode("output", { isIoNode: true })];
    const configs: NodeConfigs = {
      input: makeConfig("Input"),
      output: makeConfig("Output"),
    };
    const tree = buildNodeListTree(nodes, configs);
    // Output is at index 1, placeholder should go before it
    expect(tree.placeholderIndex).toBe(1);
  });

  it("sets placeholderIndex to -1 when processing nodes exist", () => {
    const nodes = [
      makeNode("input", { isIoNode: true }),
      makeNode("compress", {}),
      makeNode("output", { isIoNode: true }),
    ];
    const configs: NodeConfigs = {
      input: makeConfig("Input"),
      compress: makeConfig("Compress"),
      output: makeConfig("Output"),
    };
    const tree = buildNodeListTree(nodes, configs);
    expect(tree.placeholderIndex).toBe(-1);
  });

  it("preserves original array order (Input, processing, Output)", () => {
    const nodes = [
      makeNode("input", { isIoNode: true }),
      makeNode("compress", {}),
      makeNode("output", { isIoNode: true }),
    ];
    const configs: NodeConfigs = {
      input: makeConfig("Input"),
      compress: makeConfig("Compress"),
      output: makeConfig("Output"),
    };
    const tree = buildNodeListTree(nodes, configs);
    const ids = tree.entries.map((e) => e.node.id);
    expect(ids).toEqual(["input", "compress", "output"]);
    expect(tree.isEmpty).toBe(false);
  });

  it("nests children under their parent container", () => {
    const nodes = [
      makeNode("input", { isIoNode: true }),
      makeNode("loop1", { isContainer: true }),
      makeNode("convert", { parentContainerId: "loop1" }),
      makeNode("output", { isIoNode: true }),
    ];
    const configs: NodeConfigs = {
      input: makeConfig("Input"),
      loop1: makeConfig("For Each"),
      convert: makeConfig("Convert"),
      output: makeConfig("Output"),
    };
    const tree = buildNodeListTree(nodes, configs);
    expect(tree.entries).toHaveLength(3); // input, loop1, output (convert nested)
    const container = tree.entries[1]!;
    expect(container.isContainer).toBe(true);
    expect(container.children).toHaveLength(1);
    expect(container.children[0]!.node.id).toBe("convert");
  });

  it("handles nested containers (container inside container)", () => {
    const nodes = [
      makeNode("outer", { isContainer: true }),
      makeNode("inner", { isContainer: true, parentContainerId: "outer" }),
      makeNode("leaf", { parentContainerId: "inner" }),
    ];
    const configs: NodeConfigs = {
      outer: makeConfig("Outer"),
      inner: makeConfig("Inner"),
      leaf: makeConfig("Leaf"),
    };
    const tree = buildNodeListTree(nodes, configs);
    expect(tree.entries).toHaveLength(1);
    const outer = tree.entries[0]!;
    expect(outer.children).toHaveLength(1);
    const inner = outer.children[0]!;
    expect(inner.isContainer).toBe(true);
    expect(inner.children).toHaveLength(1);
    expect(inner.children[0]!.node.id).toBe("leaf");
  });

  it("preserves array order within each group", () => {
    const nodes = [makeNode("a"), makeNode("b"), makeNode("c")];
    const configs: NodeConfigs = {
      a: makeConfig("A"),
      b: makeConfig("B"),
      c: makeConfig("C"),
    };
    const tree = buildNodeListTree(nodes, configs);
    const ids = tree.entries.map((e) => e.node.id);
    expect(ids).toEqual(["a", "b", "c"]);
  });

  it("skips nodes without configs", () => {
    const nodes = [makeNode("a"), makeNode("b")];
    const configs: NodeConfigs = { a: makeConfig("A") };
    const tree = buildNodeListTree(nodes, configs);
    expect(tree.entries).toHaveLength(1);
    expect(tree.entries[0]!.node.id).toBe("a");
  });

  it("returns empty tree when no nodes exist", () => {
    const tree = buildNodeListTree([], {});
    expect(tree.isEmpty).toBe(true);
    expect(tree.entries).toHaveLength(0);
  });

  it("container with no children has empty children array", () => {
    const nodes = [makeNode("loop1", { isContainer: true })];
    const configs: NodeConfigs = { loop1: makeConfig("For Each") };
    const tree = buildNodeListTree(nodes, configs);
    const container = tree.entries[0]!;
    expect(container.isContainer).toBe(true);
    expect(container.children).toEqual([]);
  });
});
