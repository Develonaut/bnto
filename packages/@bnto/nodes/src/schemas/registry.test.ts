/**
 * Tests for the NODE_SCHEMAS registry, helpers, and structural invariants.
 */

import { describe, expect, it } from "vitest";

import { NODE_TYPE_NAMES } from "../nodeTypes";

import {
  NODE_SCHEMAS,
  getNodeSchema,
  getRequiredParams,
  getConditionallyRequired,
  getVisibleParams,
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

describe("NODE_SCHEMAS", () => {
  it("has a schema for every registered node type", () => {
    for (const name of NODE_TYPE_NAMES) {
      expect(NODE_SCHEMAS[name]).toBeDefined();
      expect(NODE_SCHEMAS[name].nodeType).toBe(name);
    }
  });

  it("has exactly 12 entries", () => {
    expect(Object.keys(NODE_SCHEMAS)).toHaveLength(12);
  });
});

// ---------- Structural invariants ----------

describe("every schema", () => {
  const allSchemas = Object.values(NODE_SCHEMAS);

  it("has a non-empty nodeType", () => {
    for (const schema of allSchemas) {
      expect(schema.nodeType).toBeTruthy();
    }
  });

  it("has a parameters array", () => {
    for (const schema of allSchemas) {
      expect(Array.isArray(schema.parameters)).toBe(true);
    }
  });

  it("every parameter has required fields", () => {
    for (const schema of allSchemas) {
      for (const param of schema.parameters) {
        expect(param.name).toBeTruthy();
        expect(param.type).toBeTruthy();
        expect(typeof param.required).toBe("boolean");
        expect(param.label).toBeTruthy();
        expect(param.description).toBeTruthy();
      }
    }
  });

  it("enum parameters always have enumValues", () => {
    for (const schema of allSchemas) {
      for (const param of schema.parameters) {
        if (param.type === "enum") {
          expect(param.enumValues).toBeDefined();
          expect(param.enumValues!.length).toBeGreaterThan(0);
        }
      }
    }
  });

  it("number parameters with min/max have min <= max", () => {
    for (const schema of allSchemas) {
      for (const param of schema.parameters) {
        if (
          param.type === "number" &&
          param.min !== undefined &&
          param.max !== undefined
        ) {
          expect(param.min).toBeLessThanOrEqual(param.max);
        }
      }
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
  it("returns only required params", () => {
    const required = getRequiredParams("http-request");
    expect(required.length).toBe(2);
    for (const p of required) {
      expect(p.required).toBe(true);
    }
  });

  it("returns empty array for unknown type", () => {
    expect(getRequiredParams("unknown")).toEqual([]);
  });
});

describe("getConditionallyRequired", () => {
  it("finds items when loop mode is forEach", () => {
    const params = getConditionallyRequired("loop", "mode", "forEach");
    expect(params).toHaveLength(1);
    expect(params[0].name).toBe("items");
  });

  it("finds count when loop mode is times", () => {
    const params = getConditionallyRequired("loop", "mode", "times");
    expect(params).toHaveLength(1);
    expect(params[0].name).toBe("count");
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
    const params = getVisibleParams("image", "operation", "resize");
    const names = params.map((p) => p.name);
    expect(names).toContain("width");
    expect(names).toContain("height");
    expect(names).toContain("maintainAspect");
  });

  it("excludes resize params when operation is convert", () => {
    const params = getVisibleParams("image", "operation", "convert");
    const names = params.map((p) => p.name);
    expect(names).not.toContain("width");
    expect(names).not.toContain("height");
    expect(names).not.toContain("maintainAspect");
  });

  it("includes params without visibleWhen (always visible)", () => {
    const params = getVisibleParams("image", "operation", "resize");
    const names = params.map((p) => p.name);
    expect(names).toContain("operation");
    expect(names).toContain("input");
    expect(names).toContain("output");
    expect(names).toContain("quality");
  });

  it("returns empty for unknown type", () => {
    expect(getVisibleParams("unknown", "op", "val")).toEqual([]);
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
