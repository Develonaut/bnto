import { describe, expect, it } from "vitest";

import { addNode } from "./addNode";
import { createBlankDefinition } from "./createBlankDefinition";
import { isValid } from "./definitionResult";
import { CURRENT_FORMAT_VERSION } from "./formatVersion";
import { NODE_TYPE_NAMES, NODE_TYPE_INFO } from "./nodeTypes";
import type { NodeTypeName } from "./nodeTypes";
import { NODE_SCHEMA_DEFS } from "./schemas/registry";

// Blank definition now has 2 I/O nodes (input + output).
// Newly added nodes appear after them.
const IO_NODE_COUNT = 2;

describe("addNode", () => {
  it("adds a node to a definition with I/O nodes", () => {
    const blank = createBlankDefinition();
    const result = addNode(blank, "image");

    expect(result.definition.nodes).toHaveLength(IO_NODE_COUNT + 1);
    expect(result.definition.nodes![IO_NODE_COUNT]!.type).toBe("image");
    expect(isValid(result)).toBe(true);
  });

  it("does not mutate the original definition", () => {
    const blank = createBlankDefinition();
    const originalNodes = blank.nodes;
    addNode(blank, "image");

    expect(blank.nodes).toBe(originalNodes);
    expect(blank.nodes).toHaveLength(IO_NODE_COUNT);
  });

  it("generates a unique UUID for each node", () => {
    const blank = createBlankDefinition();
    const r1 = addNode(blank, "image");
    const r2 = addNode(blank, "image");

    const n1 = r1.definition.nodes![IO_NODE_COUNT]!;
    const n2 = r2.definition.nodes![IO_NODE_COUNT]!;
    expect(n1.id).toBeTruthy();
    expect(n2.id).toBeTruthy();
    expect(n1.id).not.toBe(n2.id);
  });

  it("uses the provided position", () => {
    const blank = createBlankDefinition();
    const result = addNode(blank, "image", { x: 100, y: 200 });

    expect(result.definition.nodes![IO_NODE_COUNT]!.position).toEqual({ x: 100, y: 200 });
  });

  it("defaults to position {x: 0, y: 0} when no position given", () => {
    const blank = createBlankDefinition();
    const result = addNode(blank, "image");

    expect(result.definition.nodes![IO_NODE_COUNT]!.position).toEqual({ x: 0, y: 0 });
  });

  it("sets the node name from NODE_TYPE_INFO label", () => {
    const blank = createBlankDefinition();
    const result = addNode(blank, "image");

    expect(result.definition.nodes![IO_NODE_COUNT]!.name).toBe("Image");
  });

  it("populates default parameters from schema", () => {
    const blank = createBlankDefinition();
    const result = addNode(blank, "image");
    const params = result.definition.nodes![IO_NODE_COUNT]!.parameters;

    // Image schema has quality: 80 as default
    expect(params.quality).toBe(80);
    // Image schema has maintainAspect: true as default
    expect(params.maintainAspect).toBe(true);
  });

  it("adds multiple nodes sequentially", () => {
    const blank = createBlankDefinition();
    const r1 = addNode(blank, "image");
    const r2 = addNode(r1.definition, "transform");

    expect(r2.definition.nodes).toHaveLength(IO_NODE_COUNT + 2);
    expect(r2.definition.nodes![IO_NODE_COUNT]!.type).toBe("image");
    expect(r2.definition.nodes![IO_NODE_COUNT + 1]!.type).toBe("transform");
  });

  // Test every node type can be added
  describe("works for all 12 node types", () => {
    for (const typeName of NODE_TYPE_NAMES) {
      it(`adds ${typeName} node`, () => {
        const blank = createBlankDefinition();
        const result = addNode(blank, typeName);

        expect(result.definition.nodes).toHaveLength(IO_NODE_COUNT + 1);
        const node = result.definition.nodes![IO_NODE_COUNT]!;
        expect(node.type).toBe(typeName);
        expect(node.id).toBeTruthy();
        expect(node.version).toBe(CURRENT_FORMAT_VERSION);
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
        const node = result.definition.nodes![IO_NODE_COUNT]!;

        expect(node.nodes).toEqual([]);
        expect(node.edges).toEqual([]);
      });

      it(`${typeName} has an input port with unique ID`, () => {
        const blank = createBlankDefinition();
        const result = addNode(blank, typeName);
        const node = result.definition.nodes![IO_NODE_COUNT]!;

        expect(node.inputPorts).toHaveLength(1);
        expect(node.inputPorts[0]!.name).toBe("input");
        expect(node.inputPorts[0]!.id).toBeTruthy();
      });
    }
  });

  describe("non-container nodes get output port", () => {
    const nonContainerTypes = NODE_TYPE_NAMES.filter((t) => !NODE_TYPE_INFO[t].isContainer);

    for (const typeName of nonContainerTypes) {
      it(`${typeName} has an output port with unique ID`, () => {
        const blank = createBlankDefinition();
        const result = addNode(blank, typeName);
        const node = result.definition.nodes![IO_NODE_COUNT]!;

        expect(node.outputPorts).toHaveLength(1);
        expect(node.outputPorts[0]!.name).toBe("output");
        expect(node.outputPorts[0]!.id).toBeTruthy();
      });
    }
  });

  describe("default parameters match schema", () => {
    for (const typeName of NODE_TYPE_NAMES) {
      const schemaDef = NODE_SCHEMA_DEFS[typeName];
      // Parse empty object to discover Zod defaults
      const parsed = schemaDef.schema.safeParse({});
      if (!parsed.success) continue;
      const defaults = parsed.data as Record<string, unknown>;
      const defaultKeys = Object.keys(defaults);

      if (defaultKeys.length > 0) {
        it(`${typeName} includes all schema defaults`, () => {
          const blank = createBlankDefinition();
          const result = addNode(blank, typeName);
          const params = result.definition.nodes![IO_NODE_COUNT]!.parameters;

          for (const key of defaultKeys) {
            expect(params[key]).toBe(defaults[key]);
          }
        });
      }
    }
  });
});
