/**
 * rfNodesToDefinition tests — verify RF nodes → Definition conversion.
 */

import { describe, it, expect } from "vitest";
import { rfNodesToDefinition } from "./rfNodesToDefinition";
import { definitionToBento } from "./definitionToBento";
import { createBlankDefinition, addNode } from "@bnto/nodes";

describe("rfNodesToDefinition", () => {
  it("converts RF nodes + configs back to Definition child nodes", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;
    def = addNode(def, "spreadsheet").definition;

    const bento = definitionToBento(def);
    const result = rfNodesToDefinition(bento.nodes, def, bento.configs);

    // 2 I/O nodes + 2 added = 4
    expect(result.nodes!.length).toBe(4);
    const nonIo = result.nodes!.filter((n) => n.type !== "input" && n.type !== "output");
    expect(nonIo[0]!.type).toBe("image");
    expect(nonIo[1]!.type).toBe("spreadsheet");
  });

  it("preserves node IDs through the round-trip", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;
    const imageNode = def.nodes!.find((n) => n.type === "image")!;

    const bento = definitionToBento(def);
    const result = rfNodesToDefinition(bento.nodes, def, bento.configs);

    const resultImage = result.nodes!.find((n) => n.type === "image")!;
    expect(resultImage.id).toBe(imageNode.id);
  });

  it("preserves node positions from RF state", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;

    const bento = definitionToBento(def);
    const movedNodes = bento.nodes.map((n) => ({
      ...n,
      position: { x: 777, y: 333 },
    }));
    const result = rfNodesToDefinition(movedNodes, def, bento.configs);

    const resultImage = result.nodes!.find((n) => n.type === "image")!;
    expect(resultImage.position).toEqual({ x: 777, y: 333 });
  });

  it("preserves node parameters through the round-trip", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;
    const imageNode = def.nodes!.find((n) => n.type === "image")!;

    const bento = definitionToBento(def);
    const result = rfNodesToDefinition(bento.nodes, def, bento.configs);

    const resultImage = result.nodes!.find((n) => n.type === "image")!;
    expect(resultImage.parameters).toEqual(imageNode.parameters);
  });

  it("preserves root definition metadata", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;

    const bento = definitionToBento(def);
    const result = rfNodesToDefinition(bento.nodes, def, bento.configs);

    expect(result.id).toBe(def.id);
    expect(result.name).toBe(def.name);
    expect(result.type).toBe(def.type);
    expect(result.version).toBe(def.version);
  });

  it("handles empty RF nodes gracefully", () => {
    const def = createBlankDefinition();
    const result = rfNodesToDefinition([], def, {});

    expect(result.nodes).toEqual([]);
    expect(result.id).toBe(def.id);
  });

  it("round-trips definition → bento → definition preserving domain data", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;
    def = addNode(def, "transform").definition;
    def = addNode(def, "file-system").definition;

    const bento = definitionToBento(def);
    const roundTripped = rfNodesToDefinition(bento.nodes, def, bento.configs);

    for (let i = 0; i < def.nodes!.length; i++) {
      const original = def.nodes![i]!;
      const result = roundTripped.nodes![i]!;
      expect(result.id).toBe(original.id);
      expect(result.type).toBe(original.type);
      expect(result.name).toBe(original.name);
      expect(result.parameters).toEqual(original.parameters);
    }
  });

  it("uses label as fallback name when config is missing", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;
    const imageNode = def.nodes!.find((n) => n.type === "image")!;

    const bento = definitionToBento(def);
    const rfImage = bento.nodes.find((n) => n.id === imageNode.id)!;
    // Pass empty configs — adapter should use node.data.label as fallback
    const result = rfNodesToDefinition(bento.nodes, def, {});

    const resultImage = result.nodes!.find((n) => n.id === imageNode.id)!;
    expect(resultImage.name).toBe(rfImage.data.label);
  });
});
