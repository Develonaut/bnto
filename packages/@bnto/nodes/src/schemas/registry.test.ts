/**
 * Tests for the NODE_SCHEMA_DEFS registry, helpers, and structural invariants.
 */

import { describe, expect, it } from "vitest";

import { NODE_TYPE_NAMES } from "../nodeTypes";

import {
  NODE_SCHEMA_DEFS,
  getNodeSchema,
  getRequiredParams,
  getConditionallyRequired,
  getVisibleParams,
  inferFieldType,
  HTTP_METHODS,
  FILE_OPERATIONS,
  LOOP_MODES,
  IMAGE_OPERATIONS,
  IMAGE_FORMATS,
  SPREADSHEET_OPERATIONS,
  SPREADSHEET_FORMATS,
  GROUP_MODES,
  ERROR_STRATEGIES,
  INPUT_MODES,
  OUTPUT_MODES,
} from "./index";

// ---------- Registry completeness ----------

describe("NODE_SCHEMA_DEFS", () => {
  it("has a schema for every registered node type", () => {
    for (const name of NODE_TYPE_NAMES) {
      expect(NODE_SCHEMA_DEFS[name]).toBeDefined();
      expect(NODE_SCHEMA_DEFS[name].nodeType).toBe(name);
    }
  });

  it("has exactly 12 entries", () => {
    expect(Object.keys(NODE_SCHEMA_DEFS)).toHaveLength(12);
  });
});

// ---------- Structural invariants ----------

describe("every schema definition", () => {
  const allDefs = Object.values(NODE_SCHEMA_DEFS);

  it("has a non-empty nodeType", () => {
    for (const def of allDefs) {
      expect(def.nodeType).toBeTruthy();
    }
  });

  it("has a schemaVersion >= 1", () => {
    for (const def of allDefs) {
      expect(def.schemaVersion).toBeGreaterThanOrEqual(1);
    }
  });

  it("has a Zod schema with a shape", () => {
    for (const def of allDefs) {
      expect(def.schema).toBeDefined();
      expect(def.schema.shape).toBeDefined();
    }
  });

  it("has params metadata for every field in the Zod shape", () => {
    for (const def of allDefs) {
      const shapeKeys = Object.keys(def.schema.shape);
      for (const key of shapeKeys) {
        expect(def.params[key]).toBeDefined();
        expect(def.params[key].label).toBeTruthy();
        expect(def.params[key].description).toBeTruthy();
      }
    }
  });

  it("has no extra params keys beyond the Zod shape", () => {
    for (const def of allDefs) {
      const shapeKeys = new Set(Object.keys(def.schema.shape));
      for (const key of Object.keys(def.params)) {
        expect(shapeKeys.has(key)).toBe(true);
      }
    }
  });

  it("enum fields inferred via inferFieldType have enumValues", () => {
    for (const def of allDefs) {
      for (const [key, zodField] of Object.entries(def.schema.shape)) {
        const info = inferFieldType(zodField);
        if (info.type === "enum") {
          expect(info.enumValues).toBeDefined();
          expect(info.enumValues!.length).toBeGreaterThan(0);
        }
      }
    }
  });

  it("number fields with min/max have min <= max", () => {
    for (const def of allDefs) {
      for (const [, zodField] of Object.entries(def.schema.shape)) {
        const info = inferFieldType(zodField);
        if (info.type === "number" && info.min !== undefined && info.max !== undefined) {
          expect(info.min).toBeLessThanOrEqual(info.max);
        }
      }
    }
  });
});

// ---------- Zod schema parsing ----------

describe("Zod schemas parse correctly", () => {
  it("image accepts empty object (all optional except operation via enum)", () => {
    const result = NODE_SCHEMA_DEFS["image"].schema.safeParse({ operation: "resize" });
    expect(result.success).toBe(true);
  });

  it("image rejects invalid operation", () => {
    const result = NODE_SCHEMA_DEFS["image"].schema.safeParse({ operation: "explode" });
    expect(result.success).toBe(false);
  });

  it("http-request requires url", () => {
    const result = NODE_SCHEMA_DEFS["http-request"].schema.safeParse({ method: "GET" });
    expect(result.success).toBe(false);
  });

  it("http-request passes with url and defaults method", () => {
    const result = NODE_SCHEMA_DEFS["http-request"].schema.safeParse({
      url: "https://example.com",
    });
    expect(result.success).toBe(true);
    if (result.success) {
      expect(result.data.method).toBe("GET");
    }
  });

  it("shell-command requires command", () => {
    const result = NODE_SCHEMA_DEFS["shell-command"].schema.safeParse({});
    expect(result.success).toBe(false);
  });

  it("loop requires mode", () => {
    const result = NODE_SCHEMA_DEFS["loop"].schema.safeParse({});
    expect(result.success).toBe(false);
  });

  it("group defaults mode to sequential", () => {
    const result = NODE_SCHEMA_DEFS["group"].schema.safeParse({});
    expect(result.success).toBe(true);
    if (result.success) {
      expect(result.data.mode).toBe("sequential");
    }
  });

  it("edit-fields requires values", () => {
    const result = NODE_SCHEMA_DEFS["edit-fields"].schema.safeParse({});
    expect(result.success).toBe(false);
  });

  it("input defaults mode to file-upload", () => {
    const result = NODE_SCHEMA_DEFS["input"].schema.safeParse({});
    expect(result.success).toBe(true);
    if (result.success) {
      expect(result.data.mode).toBe("file-upload");
    }
  });

  it("output defaults mode to download", () => {
    const result = NODE_SCHEMA_DEFS["output"].schema.safeParse({});
    expect(result.success).toBe(true);
    if (result.success) {
      expect(result.data.mode).toBe("download");
    }
  });
});

// ---------- Helper functions ----------

describe("getNodeSchema", () => {
  it("returns schema for valid type", () => {
    const schema = getNodeSchema("image");
    expect(schema).toBeDefined();
    expect(schema!.nodeType).toBe("image");
  });

  it("returns undefined for unknown type", () => {
    expect(getNodeSchema("unknown")).toBeUndefined();
  });
});

describe("getRequiredParams", () => {
  it("returns required param names for http-request", () => {
    const required = getRequiredParams("http-request");
    expect(required).toContain("url");
  });

  it("returns empty array for unknown type", () => {
    expect(getRequiredParams("unknown")).toEqual([]);
  });

  it("returns empty for group (all optional/defaulted)", () => {
    const required = getRequiredParams("group");
    expect(required).toHaveLength(0);
  });
});

describe("getConditionallyRequired", () => {
  it("finds items when loop mode is forEach", () => {
    const params = getConditionallyRequired("loop", "mode", "forEach");
    expect(params).toContain("items");
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
    const shape = NODE_SCHEMA_DEFS["image"].schema.shape;
    const info = inferFieldType(shape.operation);
    expect(info.type).toBe("enum");
    expect(info.enumValues).toEqual(IMAGE_OPERATIONS);
  });

  it("detects number type with min/max", () => {
    const shape = NODE_SCHEMA_DEFS["image"].schema.shape;
    const info = inferFieldType(shape.quality);
    expect(info.type).toBe("number");
    expect(info.min).toBe(1);
    expect(info.max).toBe(100);
  });

  it("detects boolean type", () => {
    const shape = NODE_SCHEMA_DEFS["image"].schema.shape;
    const info = inferFieldType(shape.maintainAspect);
    expect(info.type).toBe("boolean");
  });

  it("detects string type for plain strings", () => {
    const shape = NODE_SCHEMA_DEFS["http-request"].schema.shape;
    const info = inferFieldType(shape.url);
    expect(info.type).toBe("string");
  });

  it("unwraps optional/default wrappers", () => {
    // quality is z.number().min(1).max(100).optional().default(80)
    const shape = NODE_SCHEMA_DEFS["image"].schema.shape;
    const info = inferFieldType(shape.quality);
    expect(info.type).toBe("number");
    expect(info.min).toBe(1);
    expect(info.max).toBe(100);
  });

  it("detects enum inside default wrapper", () => {
    // method is z.enum(HTTP_METHODS).default("GET")
    const shape = NODE_SCHEMA_DEFS["http-request"].schema.shape;
    const info = inferFieldType(shape.method);
    expect(info.type).toBe("enum");
    expect(info.enumValues).toEqual(HTTP_METHODS);
  });
});

// ---------- Enum constant exports ----------

describe("enum constants", () => {
  it("HTTP_METHODS has 7 methods", () => {
    expect(HTTP_METHODS).toHaveLength(7);
  });

  it("FILE_OPERATIONS has 8 operations", () => {
    expect(FILE_OPERATIONS).toHaveLength(8);
  });

  it("LOOP_MODES has 3 modes", () => {
    expect(LOOP_MODES).toHaveLength(3);
  });

  it("IMAGE_OPERATIONS has 4 operations", () => {
    expect(IMAGE_OPERATIONS).toHaveLength(4);
  });

  it("IMAGE_FORMATS has 3 formats", () => {
    expect(IMAGE_FORMATS).toHaveLength(3);
  });

  it("SPREADSHEET_OPERATIONS has 2 operations", () => {
    expect(SPREADSHEET_OPERATIONS).toHaveLength(2);
  });

  it("SPREADSHEET_FORMATS has 2 formats", () => {
    expect(SPREADSHEET_FORMATS).toHaveLength(2);
  });

  it("GROUP_MODES has 2 modes", () => {
    expect(GROUP_MODES).toHaveLength(2);
  });

  it("ERROR_STRATEGIES has 2 strategies", () => {
    expect(ERROR_STRATEGIES).toHaveLength(2);
  });

  it("INPUT_MODES has 3 modes", () => {
    expect(INPUT_MODES).toHaveLength(3);
    expect(INPUT_MODES).toContain("file-upload");
    expect(INPUT_MODES).toContain("text");
    expect(INPUT_MODES).toContain("url");
  });

  it("OUTPUT_MODES has 3 modes", () => {
    expect(OUTPUT_MODES).toHaveLength(3);
    expect(OUTPUT_MODES).toContain("download");
    expect(OUTPUT_MODES).toContain("display");
    expect(OUTPUT_MODES).toContain("preview");
  });
});
