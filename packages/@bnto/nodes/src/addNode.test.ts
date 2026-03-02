import { describe, expect, it } from "vitest";

import { addNode } from "./addNode";
import { createBlankDefinition } from "./createBlankDefinition";
import { isValid } from "./definitionResult";
import { NODE_TYPE_NAMES, NODE_TYPE_INFO } from "./nodeTypes";
import type { NodeTypeName } from "./nodeTypes";
import { NODE_SCHEMAS } from "./schemas/registry";

describe("addNode", () => {
  it("adds a node to an empty definition", () => {
    const blank = createBlankDefinition();
    const result = addNode(blank, "image");

    expect(result.definition.nodes).toHaveLength(1);
    expect(result.definition.nodes![0]!.type).toBe("image");
    expect(isValid(result)).toBe(true);
  });

  it("does not mutate the original definition", () => {
    const blank = createBlankDefinition();
    const originalNodes = blank.nodes;
    addNode(blank, "image");

    expect(blank.nodes).toBe(originalNodes);
    expect(blank.nodes).toHaveLength(0);
  });

  it("generates a unique UUID for each node", () => {
    const blank = createBlankDefinition();
    const r1 = addNode(blank, "image");
    const r2 = addNode(blank, "image");

    expect(r1.definition.nodes![0]!.id).toBeTruthy();
    expect(r2.definition.nodes![0]!.id).toBeTruthy();
    expect(r1.definition.nodes![0]!.id).not.toBe(r2.definition.nodes![0]!.id);
  });

  it("uses the provided position", () => {
    const blank = createBlankDefinition();
    const result = addNode(blank, "image", { x: 100, y: 200 });

    expect(result.definition.nodes![0]!.position).toEqual({ x: 100, y: 200 });
  });

  it("defaults to position {x: 0, y: 0} when no position given", () => {
    const blank = createBlankDefinition();
    const result = addNode(blank, "image");

    expect(result.definition.nodes![0]!.position).toEqual({ x: 0, y: 0 });
  });

  it("sets the node name from NODE_TYPE_INFO label", () => {
    const blank = createBlankDefinition();
    const result = addNode(blank, "image");

    expect(result.definition.nodes![0]!.name).toBe("Image");
  });

  it("populates default parameters from schema", () => {
    const blank = createBlankDefinition();
    const result = addNode(blank, "image");
    const params = result.definition.nodes![0]!.parameters;

    // Image schema has quality: 80 as default
    expect(params.quality).toBe(80);
    // Image schema has maintainAspect: true as default
    expect(params.maintainAspect).toBe(true);
  });

  it("adds multiple nodes sequentially", () => {
    const blank = createBlankDefinition();
    const r1 = addNode(blank, "image");
    const r2 = addNode(r1.definition, "transform");

    expect(r2.definition.nodes).toHaveLength(2);
    expect(r2.definition.nodes![0]!.type).toBe("image");
    expect(r2.definition.nodes![1]!.type).toBe("transform");
  });

  // Test every node type can be added
  describe("works for all 10 node types", () => {
    for (const typeName of NODE_TYPE_NAMES) {
      it(`adds ${typeName} node`, () => {
        const blank = createBlankDefinition();
        const result = addNode(blank, typeName);

        expect(result.definition.nodes).toHaveLength(1);
        const node = result.definition.nodes![0]!;
        expect(node.type).toBe(typeName);
        expect(node.id).toBeTruthy();
        expect(node.version).toBe("1.0.0");
        expect(node.name).toBe(NODE_TYPE_INFO[typeName].label);
      });
    }
  });

  describe("container nodes get proper structure", () => {
    const containerTypes: NodeTypeName[] = ["group", "loop", "parallel"];

    for (const typeName of containerTypes) {
      it(`${typeName} has empty nodes and edges arrays`, () => {
        const blank = createBlankDefinition();
        const result = addNode(blank, typeName);
        const node = result.definition.nodes![0]!;

        expect(node.nodes).toEqual([]);
        expect(node.edges).toEqual([]);
      });

      it(`${typeName} has an input port`, () => {
        const blank = createBlankDefinition();
        const result = addNode(blank, typeName);
        const node = result.definition.nodes![0]!;

        expect(node.inputPorts).toHaveLength(1);
        expect(node.inputPorts[0]!.name).toBe("input");
      });
    }
  });

  describe("non-container nodes get output port", () => {
    const nonContainerTypes = NODE_TYPE_NAMES.filter(
      (t) => !NODE_TYPE_INFO[t].isContainer,
    );

    for (const typeName of nonContainerTypes) {
      it(`${typeName} has an output port`, () => {
        const blank = createBlankDefinition();
        const result = addNode(blank, typeName);
        const node = result.definition.nodes![0]!;

        expect(node.outputPorts).toHaveLength(1);
        expect(node.outputPorts[0]!.name).toBe("output");
      });
    }
  });

  describe("default parameters match schema", () => {
    for (const typeName of NODE_TYPE_NAMES) {
      const schema = NODE_SCHEMAS[typeName];
      const defaultParams = schema.parameters.filter(
        (p) => p.default !== undefined,
      );

      if (defaultParams.length > 0) {
        it(`${typeName} includes all schema defaults`, () => {
          const blank = createBlankDefinition();
          const result = addNode(blank, typeName);
          const params = result.definition.nodes![0]!.parameters;

          for (const param of defaultParams) {
            expect(params[param.name]).toBe(param.default);
          }
        });
      }
    }
  });
});
