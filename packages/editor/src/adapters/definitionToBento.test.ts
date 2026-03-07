/**
 * definitionToBento tests — verify Definition → BentoNode[] conversion.
 */

import { describe, it, expect } from "vitest";
import { definitionToBento } from "./definitionToBento";
import { CELL, SLOTS } from "./bentoSlots";
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

  it("assigns computed positions, not definition positions", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image", { x: 999, y: 888 }).definition;
    const imageNode = def.nodes!.find((n) => n.type === "image")!;

    const result = definitionToBento(def);
    const rfNode = result.nodes.find((n) => n.id === imageNode.id)!;
    // addNode appends to end: [input, output, image]. Slot index 2 → x = 2 * STRIDE
    expect(rfNode.position).toEqual({ x: SLOTS[2]!.x, y: SLOTS[2]!.y });
  });

  it("assigns slot dimensions to processing nodes", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;
    const imageNode = def.nodes!.find((n) => n.type === "image")!;

    const result = definitionToBento(def);
    const rfNode = result.nodes.find((n) => n.id === imageNode.id)!;
    expect(rfNode.data.width).toBe(SLOTS[2]!.w);
    expect(rfNode.data.height).toBe(SLOTS[2]!.h);
  });

  it("assigns I/O-specific dimensions to I/O nodes", () => {
    const def = createBlankDefinition();
    const result = definitionToBento(def);

    const inputNode = result.nodes.find((n) => n.id === "input")!;
    const outputNode = result.nodes.find((n) => n.id === "output")!;
    expect(inputNode.data.width).toBe(100);
    expect(inputNode.data.height).toBe(100);
    expect(outputNode.data.width).toBe(100);
    expect(outputNode.data.height).toBe(100);
  });

  it("places I/O nodes at slot position (renderer handles visual centering)", () => {
    const def = createBlankDefinition();
    const result = definitionToBento(def);

    const inputNode = result.nodes.find((n) => n.id === "input")!;
    // All nodes use uniform slot positions — no y-offset in data.
    // Visual centering is handled by the CompartmentNode renderer.
    expect(inputNode.position).toEqual({ x: SLOTS[0]!.x, y: SLOTS[0]!.y });
  });

  it("sets RF node type based on domain node type", () => {
    let def = createBlankDefinition();
    def = addNode(def, "transform").definition;

    const result = definitionToBento(def);
    const inputNode = result.nodes.find((n) => n.id === "input")!;
    const outputNode = result.nodes.find((n) => n.id === "output")!;
    const transformNode = result.nodes.find((n) => n.data.label === "Transform")!;
    // I/O nodes use "io" type, processing nodes use "compartment"
    expect(inputNode.type).toBe("io");
    expect(outputNode.type).toBe("io");
    expect(transformNode.type).toBe("compartment");
  });

  it("maps node category to compartment variant color", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;
    const imageNode = def.nodes!.find((n) => n.type === "image")!;

    const result = definitionToBento(def);
    const rfNode = result.nodes.find((n) => n.id === imageNode.id)!;
    expect(rfNode.data.variant).toBe("primary");
  });

  it("maps I/O nodes to info variant", () => {
    const def = createBlankDefinition();
    const result = definitionToBento(def);

    const inputNode = result.nodes.find((n) => n.id === "input")!;
    const outputNode = result.nodes.find((n) => n.id === "output")!;
    expect(inputNode.data.variant).toBe("info");
    expect(outputNode.data.variant).toBe("info");
  });

  it("uses node name as label for processing nodes", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;
    const expected = NODE_TYPE_INFO["image"].label;
    const imageNode = def.nodes!.find((n) => n.type === "image")!;

    const result = definitionToBento(def);
    const rfNode = result.nodes.find((n) => n.id === imageNode.id)!;
    expect(rfNode.data.label).toBe(expected);
  });

  it("uses simple labels for I/O nodes", () => {
    const def = createBlankDefinition();
    const result = definitionToBento(def);

    const inputNode = result.nodes.find((n) => n.id === "input")!;
    const outputNode = result.nodes.find((n) => n.id === "output")!;
    expect(inputNode.data.label).toBe("Input");
    expect(outputNode.data.label).toBe("Output");
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

  it("sets contextual icon for I/O nodes via getNodeIcon", () => {
    const def = createBlankDefinition();
    const result = definitionToBento(def);

    // Blank definition: input is file-upload mode → "file-up"
    const inputNode = result.nodes.find((n) => n.id === "input")!;
    expect(inputNode.data.icon).toBe("file-up");
    // Output is download mode → "download"
    const outputNode = result.nodes.find((n) => n.id === "output")!;
    expect(outputNode.data.icon).toBe("download");
  });

  it("sets static icon for processing nodes via getNodeIcon", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;

    const result = definitionToBento(def);
    const imageNode = result.nodes.find((n) => n.data.label === "Image")!;
    expect(imageNode.data.icon).toBe("image");
  });

  it("sets human-readable sublabel for I/O nodes", () => {
    const def = createBlankDefinition();
    const result = definitionToBento(def);

    const inputNode = result.nodes.find((n) => n.id === "input")!;
    expect(inputNode.data.sublabel).toBe("File Upload");
    const outputNode = result.nodes.find((n) => n.id === "output")!;
    expect(outputNode.data.sublabel).toBe("Download");
  });

  it("sets category sublabel for processing nodes", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;
    const imageNode = def.nodes!.find((n) => n.type === "image")!;

    const result = definitionToBento(def);
    const rfNode = result.nodes.find((n) => n.id === imageNode.id)!;
    expect(rfNode.data.sublabel).toBe("Image");
  });

  it("sets isIoNode true for I/O nodes", () => {
    const def = createBlankDefinition();
    const result = definitionToBento(def);
    const inputNode = result.nodes.find((n) => n.id === "input")!;
    const outputNode = result.nodes.find((n) => n.id === "output")!;
    expect(inputNode.data.isIoNode).toBe(true);
    expect(outputNode.data.isIoNode).toBe(true);
  });

  it("sets isIoNode false for processing nodes", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;
    const imageNode = def.nodes!.find((n) => n.type === "image")!;

    const result = definitionToBento(def);
    const rfNode = result.nodes.find((n) => n.id === imageNode.id)!;
    expect(rfNode.data.isIoNode).toBe(false);
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
