/**
 * Adapter tests — verify Definition ↔ Bento round-trip conversion.
 *
 * The adapters are pure functions. No React, no DOM — just data transforms.
 */

import { describe, it, expect } from "vitest";
import { definitionToBento, SLOTS } from "../adapters/definitionToBento";
import { bentoToDefinition } from "../adapters/bentoToDefinition";
import { createBlankDefinition, addNode } from "@bnto/nodes";
import { NODE_TYPE_INFO, NODE_TYPE_NAMES } from "@bnto/nodes";

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

  it("assigns bento slot positions, not definition positions", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image", { x: 999, y: 888 }).definition;

    const result = definitionToBento(def);
    // Should use slot[0] position, not the definition position
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
    // Image category → primary
    def = addNode(def, "image").definition;

    const result = definitionToBento(def);
    expect(result.nodes[0]!.data.variant).toBe("primary");
  });

  it("uses node name as compartment label", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;
    // addNode sets name from NODE_TYPE_INFO.label
    const expected = NODE_TYPE_INFO["image"].label;

    const result = definitionToBento(def);
    expect(result.nodes[0]!.data.label).toBe(expected);
  });

  it("links compartment back to definition node via nodeId", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;
    const nodeId = def.nodes![0]!.id;

    const result = definitionToBento(def);
    expect(result.nodes[0]!.data.nodeId).toBe(nodeId);
    expect(result.nodes[0]!.id).toBe(nodeId);
  });

  it("caps at SLOTS.length compartments", () => {
    let def = createBlankDefinition();
    // Add more nodes than available slots
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
// bentoToDefinition
// ---------------------------------------------------------------------------

describe("bentoToDefinition", () => {
  it("updates node positions from bento compartment positions", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;
    def = addNode(def, "spreadsheet").definition;

    const bento = definitionToBento(def);

    // Simulate a drag — move the first compartment
    const modified = {
      ...bento,
      nodes: bento.nodes.map((n, i) =>
        i === 0 ? { ...n, position: { x: 500, y: 300 } } : n,
      ),
    };

    const result = bentoToDefinition(def, modified.nodes);
    expect(result.nodes![0]!.position).toEqual({ x: 500, y: 300 });
    // Second node gets the bento slot position (not the original definition position)
    expect(result.nodes![1]!.position).toEqual(bento.nodes[1]!.position);
  });

  it("preserves node parameters and type", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;

    const bento = definitionToBento(def);
    const result = bentoToDefinition(def, bento.nodes);

    // All non-position data preserved
    expect(result.nodes![0]!.type).toBe("image");
    expect(result.nodes![0]!.parameters).toEqual(def.nodes![0]!.parameters);
    expect(result.nodes![0]!.name).toBe(def.nodes![0]!.name);
  });

  it("preserves root definition metadata", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;

    const bento = definitionToBento(def);
    const result = bentoToDefinition(def, bento.nodes);

    expect(result.id).toBe(def.id);
    expect(result.name).toBe(def.name);
    expect(result.type).toBe(def.type);
    expect(result.edges).toEqual(def.edges);
  });

  it("handles empty bento nodes gracefully", () => {
    const def = createBlankDefinition();
    const result = bentoToDefinition(def, []);
    expect(result.nodes).toEqual([]);
  });
});

// ---------------------------------------------------------------------------
// Round-trip: definition → bento → definition
// ---------------------------------------------------------------------------

describe("round-trip", () => {
  it("preserves node count and types through conversion", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;
    def = addNode(def, "spreadsheet").definition;
    def = addNode(def, "transform").definition;

    const bento = definitionToBento(def);
    const result = bentoToDefinition(def, bento.nodes);

    expect(result.nodes!.length).toBe(3);
    expect(result.nodes![0]!.type).toBe("image");
    expect(result.nodes![1]!.type).toBe("spreadsheet");
    expect(result.nodes![2]!.type).toBe("transform");
  });

  it("preserves node IDs through conversion", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;
    def = addNode(def, "spreadsheet").definition;

    const originalIds = def.nodes!.map((n) => n.id);
    const bento = definitionToBento(def);
    const result = bentoToDefinition(def, bento.nodes);
    const resultIds = result.nodes!.map((n) => n.id);

    expect(resultIds).toEqual(originalIds);
  });

  it("preserves parameters through conversion", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;

    const bento = definitionToBento(def);
    const result = bentoToDefinition(def, bento.nodes);

    expect(result.nodes![0]!.parameters).toEqual(def.nodes![0]!.parameters);
  });
});
