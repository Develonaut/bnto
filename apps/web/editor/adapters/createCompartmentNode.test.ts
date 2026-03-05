/**
 * createCompartmentNode tests — verify node + config tuple creation.
 */

import { describe, it, expect } from "vitest";
import { createCompartmentNode } from "./createCompartmentNode";
import { SLOTS } from "./bentoSlots";
import { NODE_TYPE_NAMES, NODE_SCHEMAS } from "@bnto/nodes";

describe("createCompartmentNode", () => {
  it("creates a BentoNode + NodeConfig from a node type and slot index", () => {
    const result = createCompartmentNode("image", 0);
    expect(result).not.toBeNull();
    expect(result!.node.type).toBe("compartment");
    expect(result!.config.nodeType).toBe("image");
  });

  it("returns null when slot index exceeds available slots", () => {
    const result = createCompartmentNode("image", SLOTS.length + 5);
    expect(result).toBeNull();
  });

  it("uses slot position by default", () => {
    const result = createCompartmentNode("image", 2);
    expect(result!.node.position).toEqual({ x: SLOTS[2]!.x, y: SLOTS[2]!.y });
  });

  it("uses custom position when provided", () => {
    const result = createCompartmentNode("image", 0, { x: 42, y: 99 });
    expect(result!.node.position).toEqual({ x: 42, y: 99 });
  });

  it("maps category to variant color", () => {
    const result = createCompartmentNode("image", 0);
    expect(result!.node.data.variant).toBe("primary");
  });

  it("sets slot dimensions", () => {
    const result = createCompartmentNode("image", 0);
    expect(result!.node.data.width).toBe(SLOTS[0]!.w);
    expect(result!.node.data.height).toBe(SLOTS[0]!.h);
  });

  it("generates a UUID for node id", () => {
    const result = createCompartmentNode("image", 0);
    expect(result!.node.id).toBeTruthy();
    expect(result!.node.id.length).toBeGreaterThan(0);
  });

  it("keeps domain data in config, not in node.data", () => {
    const result = createCompartmentNode("image", 0);
    expect(result!.config.nodeType).toBe("image");
    expect(result!.config.parameters).toBeDefined();

    const data = result!.node.data;
    expect("nodeType" in data).toBe(false);
    expect("parameters" in data).toBe(false);
  });

  it("builds default parameters from schema in config", () => {
    const result = createCompartmentNode("image", 0);
    const schema = NODE_SCHEMAS["image"];
    const expectedDefaults: Record<string, unknown> = {};
    for (const param of schema.parameters) {
      if (param.default !== undefined) {
        expectedDefaults[param.name] = param.default;
      }
    }
    expect(result!.config.parameters).toEqual(expectedDefaults);
  });

  it("works with all 10 node types", () => {
    for (const typeName of NODE_TYPE_NAMES) {
      const result = createCompartmentNode(typeName, 0);
      expect(result).not.toBeNull();
      expect(result!.config.nodeType).toBe(typeName);
      expect(result!.node.data.label).toBeTruthy();
    }
  });
});
