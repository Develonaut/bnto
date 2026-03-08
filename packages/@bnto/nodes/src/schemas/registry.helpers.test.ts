/**
 * Tests for schema registry helper functions:
 * getNodeSchema, getRequiredParams, getConditionallyRequired,
 * getVisibleParams, inferFieldType.
 */

import { describe, expect, it } from "vitest";

import {
  NODE_SCHEMA_DEFS,
  getNodeSchema,
  getRequiredParams,
  getConditionallyRequired,
  getVisibleParams,
  inferFieldType,
  IMAGE_OPERATIONS,
  GROUP_MODES,
} from "./index";

// ---------- getNodeSchema ----------

describe("getNodeSchema", () => {
  it("returns schema for valid type", () => {
    const schema = getNodeSchema("image");
    expect(schema).toBeDefined();
    expect(schema!.nodeType).toBe("image");
  });

  it("returns undefined for unknown type", () => {
    expect(getNodeSchema("unknown")).toBeUndefined();
  });

  it("returns undefined for types without schemas", () => {
    expect(getNodeSchema("http-request")).toBeUndefined();
    expect(getNodeSchema("shell-command")).toBeUndefined();
  });
});

// ---------- getRequiredParams ----------

describe("getRequiredParams", () => {
  it("returns empty array for unknown type", () => {
    expect(getRequiredParams("unknown")).toEqual([]);
  });

  it("returns empty for group (all optional/defaulted)", () => {
    const required = getRequiredParams("group");
    expect(required).toHaveLength(0);
  });
});

// ---------- getConditionallyRequired ----------

describe("getConditionallyRequired", () => {
  it("items is not conditionally required for forEach (engine iterates files directly)", () => {
    const params = getConditionallyRequired("loop", "mode", "forEach");
    expect(params).not.toContain("items");
  });

  it("finds count when loop mode is times", () => {
    const params = getConditionallyRequired("loop", "mode", "times");
    expect(params).toContain("count");
  });

  it("returns empty for non-matching value", () => {
    const params = getConditionallyRequired("loop", "mode", "nonexistent");
    expect(params).toHaveLength(0);
  });

  it("returns empty for unknown type", () => {
    expect(getConditionallyRequired("unknown", "mode", "forEach")).toEqual([]);
  });
});

// ---------- getVisibleParams ----------

describe("getVisibleParams", () => {
  it("returns resize-specific params for image resize", () => {
    const names = getVisibleParams("image", "operation", "resize");
    expect(names).toContain("width");
    expect(names).toContain("height");
    expect(names).toContain("maintainAspect");
  });

  it("excludes resize params when operation is convert", () => {
    const names = getVisibleParams("image", "operation", "convert");
    expect(names).not.toContain("width");
    expect(names).not.toContain("height");
    expect(names).not.toContain("maintainAspect");
  });

  it("includes params without visibleWhen (always visible)", () => {
    const names = getVisibleParams("image", "operation", "resize");
    expect(names).toContain("operation");
    expect(names).toContain("quality");
  });

  it("excludes hidden params (engine wiring fields)", () => {
    const names = getVisibleParams("image", "operation", "resize");
    expect(names).not.toContain("input");
    expect(names).not.toContain("output");
  });

  it("returns empty for unknown type", () => {
    expect(getVisibleParams("unknown", "op", "val")).toEqual([]);
  });

  // --- parameters-map overload (used by editor config panel) ---

  it("parameters-map: returns visible params for current values", () => {
    const names = getVisibleParams("image", { operation: "resize", quality: 80 });
    expect(names).toContain("width");
    expect(names).toContain("height");
    expect(names).toContain("maintainAspect");
    expect(names).toContain("operation");
    expect(names).toContain("quality");
  });

  it("parameters-map: excludes hidden params", () => {
    const names = getVisibleParams("image", { operation: "compress" });
    expect(names).not.toContain("input");
    expect(names).not.toContain("output");
  });

  it("parameters-map: evaluates visibleWhen against current values", () => {
    const convert = getVisibleParams("image", { operation: "convert" });
    expect(convert).toContain("format");
    expect(convert).not.toContain("width");

    const resize = getVisibleParams("image", { operation: "resize" });
    expect(resize).toContain("width");
    expect(resize).not.toContain("format");
  });

  it("parameters-map: returns empty for unknown type", () => {
    expect(getVisibleParams("unknown", { op: "val" })).toEqual([]);
  });
});

// ---------- inferFieldType ----------

describe("inferFieldType", () => {
  it("detects enum type from Zod enum", () => {
    const shape = NODE_SCHEMA_DEFS["image"]!.schema.shape;
    const info = inferFieldType(shape.operation);
    expect(info.type).toBe("enum");
    expect(info.enumValues).toEqual(IMAGE_OPERATIONS);
  });

  it("detects number type with min/max", () => {
    const shape = NODE_SCHEMA_DEFS["image"]!.schema.shape;
    const info = inferFieldType(shape.quality);
    expect(info.type).toBe("number");
    expect(info.min).toBe(1);
    expect(info.max).toBe(100);
  });

  it("detects boolean type", () => {
    const shape = NODE_SCHEMA_DEFS["image"]!.schema.shape;
    const info = inferFieldType(shape.maintainAspect);
    expect(info.type).toBe("boolean");
  });

  it("detects string type for plain strings", () => {
    const shape = NODE_SCHEMA_DEFS["image"]!.schema.shape;
    const info = inferFieldType(shape.input);
    expect(info.type).toBe("string");
  });

  it("unwraps optional/default wrappers", () => {
    // quality is z.number().min(1).max(100).optional().default(80)
    const shape = NODE_SCHEMA_DEFS["image"]!.schema.shape;
    const info = inferFieldType(shape.quality);
    expect(info.type).toBe("number");
    expect(info.min).toBe(1);
    expect(info.max).toBe(100);
  });

  it("detects enum inside default wrapper", () => {
    // mode is z.enum(GROUP_MODES).default("sequential")
    const shape = NODE_SCHEMA_DEFS["group"]!.schema.shape;
    const info = inferFieldType(shape.mode);
    expect(info.type).toBe("enum");
    expect(info.enumValues).toEqual(GROUP_MODES);
  });
});
