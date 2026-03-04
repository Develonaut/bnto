/**
 * Adapter tests — verify Definition ↔ Bento conversion and helpers.
 *
 * The adapters are pure functions. No React, no DOM — just data transforms.
 */

import { describe, it, expect } from "vitest";
import { definitionToBento } from "../adapters/definitionToBento";
import { SLOTS } from "../adapters/bentoSlots";
import { createCompartmentNode } from "../adapters/createCompartmentNode";
import { rfNodesToDefinition } from "../adapters/rfNodesToDefinition";
import { createBlankDefinition, addNode } from "@bnto/nodes";
import { NODE_TYPE_INFO, NODE_TYPE_NAMES, NODE_SCHEMAS } from "@bnto/nodes";

// ---------------------------------------------------------------------------
// definitionToBento
// ---------------------------------------------------------------------------

describe("definitionToBento", () => {
  it("returns empty nodes for a blank definition", () => {
    const def = createBlankDefinition();
    const result = definitionToBento(def);
    expect(result.nodes).toEqual([]);
  });

  it("maps child nodes to compartment nodes", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;
    def = addNode(def, "spreadsheet").definition;

    const result = definitionToBento(def);
    expect(result.nodes.length).toBe(2);
  });

  it("populates domain fields (nodeType, name, parameters) in node data", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;
    const childNode = def.nodes![0]!;

    const result = definitionToBento(def);
    const data = result.nodes[0]!.data;
    expect(data.nodeType).toBe(childNode.type);
    expect(data.name).toBe(childNode.name);
    expect(data.parameters).toEqual(childNode.parameters);
  });

  it("assigns bento slot positions, not definition positions", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image", { x: 999, y: 888 }).definition;

    const result = definitionToBento(def);
    expect(result.nodes[0]!.position).toEqual({ x: SLOTS[0]!.x, y: SLOTS[0]!.y });
  });

  it("assigns slot dimensions to compartment data", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;

    const result = definitionToBento(def);
    expect(result.nodes[0]!.data.width).toBe(SLOTS[0]!.w);
    expect(result.nodes[0]!.data.height).toBe(SLOTS[0]!.h);
  });

  it("sets node type as compartment", () => {
    let def = createBlankDefinition();
    def = addNode(def, "transform").definition;

    const result = definitionToBento(def);
    expect(result.nodes[0]!.type).toBe("compartment");
  });

  it("maps node category to compartment variant color", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;

    const result = definitionToBento(def);
    expect(result.nodes[0]!.data.variant).toBe("primary");
  });

  it("uses node name as compartment label", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;
    const expected = NODE_TYPE_INFO["image"].label;

    const result = definitionToBento(def);
    expect(result.nodes[0]!.data.label).toBe(expected);
  });

  it("RF node id matches definition node id", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;
    const nodeId = def.nodes![0]!.id;

    const result = definitionToBento(def);
    expect(result.nodes[0]!.id).toBe(nodeId);
  });

  it("caps at SLOTS.length compartments", () => {
    let def = createBlankDefinition();
    for (let i = 0; i < SLOTS.length + 3; i++) {
      def = addNode(def, "transform").definition;
    }

    const result = definitionToBento(def);
    expect(result.nodes.length).toBe(SLOTS.length);
  });

  it("sets all compartments to idle status", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;
    def = addNode(def, "spreadsheet").definition;

    const result = definitionToBento(def);
    for (const node of result.nodes) {
      expect(node.data.status).toBe("idle");
    }
  });

  it("handles all 10 node types", () => {
    for (const typeName of NODE_TYPE_NAMES) {
      let def = createBlankDefinition();
      def = addNode(def, typeName).definition;

      const result = definitionToBento(def);
      expect(result.nodes.length).toBe(1);
      expect(result.nodes[0]!.data.label).toBeTruthy();
      expect(result.nodes[0]!.data.variant).toBeTruthy();
    }
  });
});

// ---------------------------------------------------------------------------
// createCompartmentNode
// ---------------------------------------------------------------------------

describe("createCompartmentNode", () => {
  it("creates a BentoNode from a node type and slot index", () => {
    const result = createCompartmentNode("image", 0);
    expect(result).not.toBeNull();
    expect(result!.type).toBe("compartment");
    expect(result!.data.nodeType).toBe("image");
  });

  it("returns null when slot index exceeds available slots", () => {
    const result = createCompartmentNode("image", SLOTS.length + 5);
    expect(result).toBeNull();
  });

  it("uses slot position by default", () => {
    const result = createCompartmentNode("image", 2);
    expect(result!.position).toEqual({ x: SLOTS[2]!.x, y: SLOTS[2]!.y });
  });

  it("uses custom position when provided", () => {
    const result = createCompartmentNode("image", 0, { x: 42, y: 99 });
    expect(result!.position).toEqual({ x: 42, y: 99 });
  });

  it("maps category to variant color", () => {
    const result = createCompartmentNode("image", 0);
    expect(result!.data.variant).toBe("primary");
  });

  it("sets slot dimensions", () => {
    const result = createCompartmentNode("image", 0);
    expect(result!.data.width).toBe(SLOTS[0]!.w);
    expect(result!.data.height).toBe(SLOTS[0]!.h);
  });

  it("generates a UUID for node id", () => {
    const result = createCompartmentNode("image", 0);
    expect(result!.id).toBeTruthy();
    expect(result!.id.length).toBeGreaterThan(0);
  });

  it("builds default parameters from schema", () => {
    const result = createCompartmentNode("image", 0);
    const schema = NODE_SCHEMAS["image"];
    const expectedDefaults: Record<string, unknown> = {};
    for (const param of schema.parameters) {
      if (param.default !== undefined) {
        expectedDefaults[param.name] = param.default;
      }
    }
    expect(result!.data.parameters).toEqual(expectedDefaults);
  });

  it("works with all 10 node types", () => {
    for (const typeName of NODE_TYPE_NAMES) {
      const result = createCompartmentNode(typeName, 0);
      expect(result).not.toBeNull();
      expect(result!.data.nodeType).toBe(typeName);
      expect(result!.data.label).toBeTruthy();
    }
  });
});

// ---------------------------------------------------------------------------
// rfNodesToDefinition
// ---------------------------------------------------------------------------

describe("rfNodesToDefinition", () => {
  it("converts RF nodes back to Definition child nodes", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;
    def = addNode(def, "spreadsheet").definition;

    const bento = definitionToBento(def);
    const result = rfNodesToDefinition(bento.nodes, def);

    expect(result.nodes!.length).toBe(2);
    expect(result.nodes![0]!.type).toBe("image");
    expect(result.nodes![1]!.type).toBe("spreadsheet");
  });

  it("preserves node IDs through the round-trip", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;
    const originalId = def.nodes![0]!.id;

    const bento = definitionToBento(def);
    const result = rfNodesToDefinition(bento.nodes, def);

    expect(result.nodes![0]!.id).toBe(originalId);
  });

  it("preserves node positions from RF state", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;

    const bento = definitionToBento(def);
    // Simulate a drag — user moved the node
    const movedNodes = bento.nodes.map((n) => ({
      ...n,
      position: { x: 777, y: 333 },
    }));
    const result = rfNodesToDefinition(movedNodes, def);

    expect(result.nodes![0]!.position).toEqual({ x: 777, y: 333 });
  });

  it("preserves node parameters through the round-trip", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;
    const originalParams = def.nodes![0]!.parameters;

    const bento = definitionToBento(def);
    const result = rfNodesToDefinition(bento.nodes, def);

    expect(result.nodes![0]!.parameters).toEqual(originalParams);
  });

  it("preserves root definition metadata", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;

    const bento = definitionToBento(def);
    const result = rfNodesToDefinition(bento.nodes, def);

    expect(result.id).toBe(def.id);
    expect(result.name).toBe(def.name);
    expect(result.type).toBe(def.type);
    expect(result.version).toBe(def.version);
  });

  it("handles empty RF nodes gracefully", () => {
    const def = createBlankDefinition();
    const result = rfNodesToDefinition([], def);

    expect(result.nodes).toEqual([]);
    expect(result.id).toBe(def.id);
  });

  it("round-trips definition → bento → definition preserving domain data", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;
    def = addNode(def, "transform").definition;
    def = addNode(def, "file-system").definition;

    const bento = definitionToBento(def);
    const roundTripped = rfNodesToDefinition(bento.nodes, def);

    for (let i = 0; i < def.nodes!.length; i++) {
      const original = def.nodes![i]!;
      const result = roundTripped.nodes![i]!;
      expect(result.id).toBe(original.id);
      expect(result.type).toBe(original.type);
      expect(result.name).toBe(original.name);
      expect(result.parameters).toEqual(original.parameters);
    }
  });
});
