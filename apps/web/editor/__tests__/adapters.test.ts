/**
 * Adapter tests — verify Definition ↔ Bento conversion and helpers.
 *
 * The adapters are pure functions. No React, no DOM — just data transforms.
 */

import { describe, it, expect } from "vitest";
import { definitionToBento } from "../adapters/definitionToBento";
import { SLOTS } from "../adapters/bentoSlots";
import { syncPositionsToDefinition } from "../adapters/syncPositionsToDefinition";
import { definitionNodeToRfNode } from "../adapters/definitionNodeToRfNode";
import { rfNodesToDefinition } from "../adapters/rfNodesToDefinition";
import { createBlankDefinition, addNode } from "@bnto/nodes";
import type { Definition } from "@bnto/nodes";
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
// syncPositionsToDefinition
// ---------------------------------------------------------------------------

describe("syncPositionsToDefinition", () => {
  it("patches node positions from bento compartments", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;
    def = addNode(def, "spreadsheet").definition;

    const bento = definitionToBento(def);
    const modified = {
      ...bento,
      nodes: bento.nodes.map((n, i) =>
        i === 0 ? { ...n, position: { x: 500, y: 300 } } : n,
      ),
    };

    const result = syncPositionsToDefinition(def, modified.nodes);
    expect(result.nodes![0]!.position).toEqual({ x: 500, y: 300 });
    expect(result.nodes![1]!.position).toEqual(bento.nodes[1]!.position);
  });

  it("preserves node parameters and type", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;

    const bento = definitionToBento(def);
    const result = syncPositionsToDefinition(def, bento.nodes);

    expect(result.nodes![0]!.type).toBe("image");
    expect(result.nodes![0]!.parameters).toEqual(def.nodes![0]!.parameters);
    expect(result.nodes![0]!.name).toBe(def.nodes![0]!.name);
  });

  it("preserves root definition metadata", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;

    const bento = definitionToBento(def);
    const result = syncPositionsToDefinition(def, bento.nodes);

    expect(result.id).toBe(def.id);
    expect(result.name).toBe(def.name);
    expect(result.type).toBe(def.type);
    expect(result.edges).toEqual(def.edges);
  });

  it("handles empty bento nodes gracefully", () => {
    const def = createBlankDefinition();
    const result = syncPositionsToDefinition(def, []);
    expect(result.nodes).toEqual([]);
  });
});

// ---------------------------------------------------------------------------
// definitionNodeToRfNode
// ---------------------------------------------------------------------------

describe("definitionNodeToRfNode", () => {
  it("converts a definition child node to a BentoNode", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;
    const node = def.nodes![0]!;

    const result = definitionNodeToRfNode(node, 0);
    expect(result).not.toBeNull();
    expect(result!.id).toBe(node.id);
    expect(result!.type).toBe("compartment");
  });

  it("uses node position when available, falls back to slot position", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image", { x: 42, y: 99 }).definition;
    const node = def.nodes![0]!;

    const result = definitionNodeToRfNode(node, 0);
    expect(result!.position).toEqual({ x: 42, y: 99 });
  });

  it("falls back to slot position when node has no position", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;
    // Simulate a node without position by omitting it
    const { position: _, ...nodeWithoutPosition } = def.nodes![0]!;
    const node = nodeWithoutPosition as Definition;

    const result = definitionNodeToRfNode(node, 0);
    expect(result!.position).toEqual({ x: SLOTS[0]!.x, y: SLOTS[0]!.y });
  });

  it("returns null when slot index exceeds available slots", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;
    const node = def.nodes![0]!;

    const result = definitionNodeToRfNode(node, SLOTS.length + 5);
    expect(result).toBeNull();
  });

  it("maps category to variant color", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;
    const node = def.nodes![0]!;

    const result = definitionNodeToRfNode(node, 0);
    expect(result!.data.variant).toBe("primary");
  });

  it("sets compartment dimensions from slot", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;
    const node = def.nodes![0]!;

    const result = definitionNodeToRfNode(node, 0);
    expect(result!.data.width).toBe(SLOTS[0]!.w);
    expect(result!.data.height).toBe(SLOTS[0]!.h);
  });

  it("produces same output as definitionToBento for matching index", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;
    const node = def.nodes![0]!;

    const fromBatch = definitionToBento(def).nodes[0]!;
    const fromSingle = definitionNodeToRfNode(node, 0)!;

    expect(fromSingle.id).toBe(fromBatch.id);
    expect(fromSingle.type).toBe(fromBatch.type);
    expect(fromSingle.data.label).toBe(fromBatch.data.label);
    expect(fromSingle.data.variant).toBe(fromBatch.data.variant);
    expect(fromSingle.data.nodeId).toBe(fromBatch.data.nodeId);
  });

  it("populates domain fields (nodeType, name, parameters) in node data", () => {
    let def = createBlankDefinition();
    def = addNode(def, "spreadsheet").definition;
    const node = def.nodes![0]!;

    const result = definitionNodeToRfNode(node, 0)!;
    expect(result.data.nodeType).toBe(node.type);
    expect(result.data.name).toBe(node.name);
    expect(result.data.parameters).toEqual(node.parameters);
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
