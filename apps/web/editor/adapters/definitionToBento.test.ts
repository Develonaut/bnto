/**
 * definitionToBento tests — verify Definition → BentoNode[] conversion.
 */

import { describe, it, expect } from "vitest";
import { definitionToBento } from "./definitionToBento";
import { SLOTS } from "./bentoSlots";
import { createBlankDefinition, addNode } from "@bnto/nodes";
import { NODE_TYPE_INFO, NODE_TYPE_NAMES } from "@bnto/nodes";

describe("definitionToBento", () => {
  it("returns empty nodes and configs for a blank definition", () => {
    const def = createBlankDefinition();
    const result = definitionToBento(def);
    expect(result.nodes).toEqual([]);
    expect(result.configs).toEqual({});
  });

  it("maps child nodes to compartment nodes", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;
    def = addNode(def, "spreadsheet").definition;

    const result = definitionToBento(def);
    expect(result.nodes.length).toBe(2);
  });

  it("stores domain fields in configs map, not in node.data", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;
    const childNode = def.nodes![0]!;

    const result = definitionToBento(def);
    const config = result.configs[childNode.id]!;
    expect(config.nodeType).toBe(childNode.type);
    expect(config.name).toBe(childNode.name);
    expect(config.parameters).toEqual(childNode.parameters);

    // node.data should NOT have domain fields
    const data = result.nodes[0]!.data;
    expect("nodeType" in data).toBe(false);
    expect("parameters" in data).toBe(false);
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

  it("handles all 10 node types", () => {
    for (const typeName of NODE_TYPE_NAMES) {
      let def = createBlankDefinition();
      def = addNode(def, typeName).definition;

      const result = definitionToBento(def);
      expect(result.nodes.length).toBe(1);
      expect(result.nodes[0]!.data.label).toBeTruthy();
      expect(result.nodes[0]!.data.variant).toBeTruthy();
      expect(result.configs[result.nodes[0]!.id]!.nodeType).toBe(typeName);
    }
  });
});
