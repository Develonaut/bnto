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

    expect(result.nodes!.length).toBe(2);
    expect(result.nodes![0]!.type).toBe("image");
    expect(result.nodes![1]!.type).toBe("spreadsheet");
  });

  it("preserves node IDs through the round-trip", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;
    const originalId = def.nodes![0]!.id;

    const bento = definitionToBento(def);
    const result = rfNodesToDefinition(bento.nodes, def, bento.configs);

    expect(result.nodes![0]!.id).toBe(originalId);
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

    expect(result.nodes![0]!.position).toEqual({ x: 777, y: 333 });
  });

  it("preserves node parameters through the round-trip", () => {
    let def = createBlankDefinition();
    def = addNode(def, "image").definition;
    const originalParams = def.nodes![0]!.parameters;

    const bento = definitionToBento(def);
    const result = rfNodesToDefinition(bento.nodes, def, bento.configs);

    expect(result.nodes![0]!.parameters).toEqual(originalParams);
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

    const bento = definitionToBento(def);
    // Pass empty configs — adapter should use node.data.label as fallback
    const result = rfNodesToDefinition(bento.nodes, def, {});

    expect(result.nodes![0]!.name).toBe(bento.nodes[0]!.data.label);
  });
});
