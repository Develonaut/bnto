import { describe, expect, it } from "vitest";

import { addNode } from "./addNode";
import { createBlankDefinition } from "./createBlankDefinition";
import { updateNodeParams } from "./updateNodeParams";

describe("updateNodeParams", () => {
  it("merges new parameters into an existing node", () => {
    const blank = createBlankDefinition();
    const { definition: withNode } = addNode(blank, "image");
    const nodeId = withNode.nodes![0]!.id;

    const result = updateNodeParams(withNode, nodeId, { quality: 50 });
    expect(result.definition.nodes![0]!.parameters.quality).toBe(50);
  });

  it("preserves existing parameters not in the update", () => {
    const blank = createBlankDefinition();
    const { definition: withNode } = addNode(blank, "image");
    const nodeId = withNode.nodes![0]!.id;

    // Image defaults include maintainAspect: true
    const result = updateNodeParams(withNode, nodeId, { quality: 50 });
    expect(result.definition.nodes![0]!.parameters.maintainAspect).toBe(true);
    expect(result.definition.nodes![0]!.parameters.quality).toBe(50);
  });

  it("does not mutate the original definition", () => {
    const blank = createBlankDefinition();
    const { definition: withNode } = addNode(blank, "image");
    const nodeId = withNode.nodes![0]!.id;
    const originalQuality = withNode.nodes![0]!.parameters.quality;

    updateNodeParams(withNode, nodeId, { quality: 10 });
    expect(withNode.nodes![0]!.parameters.quality).toBe(originalQuality);
  });

  it("returns the same definition if node ID not found", () => {
    const blank = createBlankDefinition();
    const { definition: withNode } = addNode(blank, "image");

    const result = updateNodeParams(withNode, "nonexistent", { quality: 50 });
    expect(result.definition).toBe(withNode);
  });

  it("can set multiple parameters at once", () => {
    const blank = createBlankDefinition();
    const { definition: withNode } = addNode(blank, "image");
    const nodeId = withNode.nodes![0]!.id;

    const result = updateNodeParams(withNode, nodeId, {
      operation: "resize",
      width: 800,
      height: 600,
    });

    const params = result.definition.nodes![0]!.parameters;
    expect(params.operation).toBe("resize");
    expect(params.width).toBe(800);
    expect(params.height).toBe(600);
  });

  it("can add new parameters that didn't exist before", () => {
    const blank = createBlankDefinition();
    const { definition: withNode } = addNode(blank, "image");
    const nodeId = withNode.nodes![0]!.id;

    const result = updateNodeParams(withNode, nodeId, {
      operation: "optimize",
      input: "{{.item}}",
    });

    expect(result.definition.nodes![0]!.parameters.input).toBe("{{.item}}");
  });

  it("can overwrite existing parameter values", () => {
    const blank = createBlankDefinition();
    const { definition: withNode } = addNode(blank, "image");
    const nodeId = withNode.nodes![0]!.id;

    // Quality defaults to 80, overwrite it
    const result = updateNodeParams(withNode, nodeId, { quality: 95 });
    expect(result.definition.nodes![0]!.parameters.quality).toBe(95);
  });

  it("updates the correct node when multiple nodes exist", () => {
    const blank = createBlankDefinition();
    const r1 = addNode(blank, "image");
    const r2 = addNode(r1.definition, "transform");
    const secondNodeId = r2.definition.nodes![1]!.id;

    const result = updateNodeParams(r2.definition, secondNodeId, {
      expression: "{{.item}}",
    });

    // First node unchanged
    expect(result.definition.nodes![0]!.parameters).toEqual(
      r2.definition.nodes![0]!.parameters,
    );
    // Second node updated
    expect(result.definition.nodes![1]!.parameters.expression).toBe(
      "{{.item}}",
    );
  });

  it("works on nested nodes (inside containers)", () => {
    const blank = createBlankDefinition();
    const { definition: withLoop } = addNode(blank, "loop");
    const loopNode = withLoop.nodes![0]!;

    // Manually add a child node inside the loop
    const childNode = {
      id: "child-1",
      type: "image" as const,
      version: "1.0.0",
      name: "Nested Image",
      position: { x: 0, y: 0 },
      metadata: {},
      parameters: { quality: 80 },
      inputPorts: [],
      outputPorts: [{ id: "out-1", name: "output" }],
    };

    const withChild = {
      ...withLoop,
      nodes: [{ ...loopNode, nodes: [childNode] }],
    };

    const result = updateNodeParams(withChild, "child-1", { quality: 50 });
    expect(result.definition.nodes![0]!.nodes![0]!.parameters.quality).toBe(50);
  });
});
