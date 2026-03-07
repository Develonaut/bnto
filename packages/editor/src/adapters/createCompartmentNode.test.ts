/**
 * createCompartmentNode tests — verify node + config tuple creation.
 */

import { describe, it, expect } from "vitest";
import { createCompartmentNode } from "./createCompartmentNode";
import { SLOTS } from "./bentoSlots";
import { NODE_TYPE_NAMES, NODE_SCHEMA_DEFS } from "@bnto/nodes";

describe("createCompartmentNode", () => {
  it("creates a BentoNode + NodeConfig from a node type and slot index", () => {
    const result = createCompartmentNode("image", 0);
    expect(result).not.toBeNull();
    expect(result!.node.type).toBe("compartment"); // processing → "compartment"
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

  it("sets slot dimensions for processing nodes", () => {
    const result = createCompartmentNode("image", 0);
    expect(result!.node.data.width).toBe(SLOTS[0]!.w);
    expect(result!.node.data.height).toBe(SLOTS[0]!.h);
  });

  it("sets I/O-specific dimensions for input nodes", () => {
    const result = createCompartmentNode("input", 0);
    expect(result!.node.data.width).toBe(100);
    expect(result!.node.data.height).toBe(100);
  });

  it("sets I/O-specific dimensions for output nodes", () => {
    const result = createCompartmentNode("output", 0);
    expect(result!.node.data.width).toBe(100);
    expect(result!.node.data.height).toBe(100);
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
    const schemaDef = NODE_SCHEMA_DEFS["image"];
    // Extract defaults from Zod schema
    const shape = schemaDef.schema.shape as Record<
      string,
      { _def?: { typeName?: string; defaultValue?: () => unknown } }
    >;
    const expectedDefaults: Record<string, unknown> = {};
    for (const [name, field] of Object.entries(shape)) {
      if (field?._def?.typeName === "ZodDefault" && typeof field._def.defaultValue === "function") {
        expectedDefaults[name] = field._def.defaultValue();
      }
    }
    expect(result!.config.parameters).toEqual(expectedDefaults);
  });

  it("sets static icon for processing nodes via getNodeIcon", () => {
    const result = createCompartmentNode("image", 0);
    expect(result!.node.data.icon).toBe("image");
  });

  it("sets contextual icon for I/O nodes via getNodeIcon", () => {
    // Input defaults to file-upload mode → "file-up"
    const inputResult = createCompartmentNode("input", 0);
    expect(inputResult!.node.data.icon).toBe("file-up");

    // Output defaults to download mode → "download"
    const outputResult = createCompartmentNode("output", 0);
    expect(outputResult!.node.data.icon).toBe("download");
  });

  it("sets human-readable sublabel", () => {
    const inputResult = createCompartmentNode("input", 0);
    expect(inputResult!.node.data.sublabel).toBe("File Upload");

    const imageResult = createCompartmentNode("image", 0);
    expect(imageResult!.node.data.sublabel).toBe("Image");
  });

  it("sets isIoNode true for input nodes", () => {
    const result = createCompartmentNode("input", 0);
    expect(result!.node.data.isIoNode).toBe(true);
  });

  it("sets isIoNode true for output nodes", () => {
    const result = createCompartmentNode("output", 0);
    expect(result!.node.data.isIoNode).toBe(true);
  });

  it("sets isIoNode false for processing nodes", () => {
    const result = createCompartmentNode("image", 0);
    expect(result!.node.data.isIoNode).toBe(false);
  });

  it("works with all 12 node types", () => {
    for (const typeName of NODE_TYPE_NAMES) {
      const result = createCompartmentNode(typeName, 0);
      expect(result).not.toBeNull();
      expect(result!.config.nodeType).toBe(typeName);
      expect(result!.node.data.label).toBeTruthy();
    }
  });
});
