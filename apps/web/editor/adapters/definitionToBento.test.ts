/**
 * definitionToBento tests — verify Definition → BentoNode[] conversion.
 */

import { describe, it, expect } from "vitest";
import { definitionToBento } from "./definitionToBento";
import { SLOTS } from "./bentoSlots";
import { createBlankDefinition, addNode } from "@bnto/nodes";
import { NODE_TYPE_INFO, NODE_TYPE_NAMES } from "@bnto/nodes";

describe("definitionToBento", () => {
  it("returns I/O nodes for a blank definition", () => {
    const def = createBlankDefinition();
    const result = definitionToBento(def);
    // Blank definition has 2 I/O nodes (input + output)
    expect(result.nodes.length).toBe(2);
    expect(Object.keys(result.configs).length).toBe(2);
  });

  it("maps child nodes to compartment nodes", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;
    def = addNode(def, "spreadsheet").definition;

    const result = definitionToBento(def);
    // 2 I/O nodes + 2 added = 4
    expect(result.nodes.length).toBe(4);
  });

  it("stores domain fields in configs map, not in node.data", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;
    const childNode = def.nodes!.find((n) => n.type === "image")!;

    const result = definitionToBento(def);
    const config = result.configs[childNode.id]!;
    expect(config.nodeType).toBe(childNode.type);
    expect(config.name).toBe(childNode.name);
    expect(config.parameters).toEqual(childNode.parameters);

    // node.data should NOT have domain fields
    const rfNode = result.nodes.find((n) => n.id === childNode.id)!;
    expect("nodeType" in rfNode.data).toBe(false);
    expect("parameters" in rfNode.data).toBe(false);
  });

  it("assigns bento slot positions, not definition positions", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image", { x: 999, y: 888 }).definition;
    const imageNode = def.nodes!.find((n) => n.type === "image")!;

    const result = definitionToBento(def);
    const rfNode = result.nodes.find((n) => n.id === imageNode.id)!;
    // Image is 3rd node (after input + output), so slot index 2
    expect(rfNode.position).toEqual({ x: SLOTS[2]!.x, y: SLOTS[2]!.y });
  });

  it("assigns slot dimensions to compartment data", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;
    const imageNode = def.nodes!.find((n) => n.type === "image")!;

    const result = definitionToBento(def);
    const rfNode = result.nodes.find((n) => n.id === imageNode.id)!;
    expect(rfNode.data.width).toBe(SLOTS[2]!.w);
    expect(rfNode.data.height).toBe(SLOTS[2]!.h);
  });

  it("sets node type as compartment", () => {
    let def = createBlankDefinition();
    def = addNode(def, "transform").definition;

    const result = definitionToBento(def);
    // All nodes (including I/O) are compartments
    for (const node of result.nodes) {
      expect(node.type).toBe("compartment");
    }
  });

  it("maps node category to compartment variant color", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;
    const imageNode = def.nodes!.find((n) => n.type === "image")!;

    const result = definitionToBento(def);
    const rfNode = result.nodes.find((n) => n.id === imageNode.id)!;
    expect(rfNode.data.variant).toBe("primary");
  });

  it("uses node name as compartment label", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;
    const expected = NODE_TYPE_INFO["image"].label;
    const imageNode = def.nodes!.find((n) => n.type === "image")!;

    const result = definitionToBento(def);
    const rfNode = result.nodes.find((n) => n.id === imageNode.id)!;
    expect(rfNode.data.label).toBe(expected);
  });

  it("RF node id matches definition node id", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;
    const imageNode = def.nodes!.find((n) => n.type === "image")!;

    const result = definitionToBento(def);
    const rfNode = result.nodes.find((n) => n.id === imageNode.id)!;
    expect(rfNode.id).toBe(imageNode.id);
  });

  it("configs map keys match RF node ids", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;
    def = addNode(def, "transform").definition;

    const result = definitionToBento(def);
    for (const node of result.nodes) {
      expect(result.configs[node.id]).toBeDefined();
    }
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

  it("handles all 12 node types", () => {
    for (const typeName of NODE_TYPE_NAMES) {
      let def = createBlankDefinition();
      def = addNode(def, typeName).definition;

      const result = definitionToBento(def);
      // 2 I/O base + 1 added = 3
      expect(result.nodes.length).toBe(3);
      const addedNode = def.nodes![def.nodes!.length - 1]!;
      const rfNode = result.nodes.find((n) => n.id === addedNode.id)!;
      expect(rfNode.data.label).toBeTruthy();
      expect(rfNode.data.variant).toBeTruthy();
      expect(result.configs[rfNode.id]!.nodeType).toBe(typeName);
    }
  });
});
