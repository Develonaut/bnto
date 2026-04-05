/**
 * Tests for the NODE_SCHEMAS registry — completeness, structural invariants,
 * Zod schema parsing, and enum constant exports.
 */

import { describe, expect, it } from "vitest";

import {
  NODE_SCHEMAS,
  inferFieldType,
  LOOP_MODES,
  IMAGE_FORMATS,
  GROUP_MODES,
  ERROR_STRATEGIES,
  INPUT_MODES,
  OUTPUT_MODES,
} from "./index";
import { NODE_TYPE_NAMES } from "../generated/catalog";

// ---------- Registry completeness ----------

describe("NODE_SCHEMAS", () => {
  it("has a schema for every node type that has one", () => {
    // Types without engine processors — no schemas generated
    const TYPES_WITHOUT_SCHEMAS = new Set(["http-request", "shell-command"]);
    const expected = NODE_TYPE_NAMES.filter((n: string) => !TYPES_WITHOUT_SCHEMAS.has(n)).length;
    expect(Object.keys(NODE_SCHEMAS)).toHaveLength(expected);
  });

  it("every schema entry matches its nodeType key", () => {
    for (const [name, def] of Object.entries(NODE_SCHEMAS)) {
      expect(def!.nodeType).toBe(name);
    }
  });

  it("http-request and shell-command have no schema (no engine processor)", () => {
    expect(NODE_SCHEMAS["http-request"]).toBeUndefined();
    expect(NODE_SCHEMAS["shell-command"]).toBeUndefined();
  });

  it("video-download has a generated schema (CLI-only but codegen-propagated)", () => {
    expect(NODE_SCHEMAS["video-download"]).toBeDefined();
  });
});

// ---------- Structural invariants ----------

describe("every schema definition", () => {
  const allDefs = Object.values(NODE_SCHEMAS).filter(Boolean);

  it("has a non-empty nodeType", () => {
    for (const def of allDefs) {
      expect(def!.nodeType).toBeTruthy();
    }
  });

  it("has a schemaVersion >= 1", () => {
    for (const def of allDefs) {
      expect(def!.schemaVersion).toBeGreaterThanOrEqual(1);
    }
  });

  it("has a Zod schema with a shape", () => {
    for (const def of allDefs) {
      expect(def!.schema).toBeDefined();
      expect(def!.schema.shape).toBeDefined();
    }
  });

  it("has params metadata for every field in the Zod shape", () => {
    for (const def of allDefs) {
      const shapeKeys = Object.keys(def!.schema.shape);
      for (const key of shapeKeys) {
        expect(def!.params[key]).toBeDefined();
        expect(def!.params[key].label).toBeTruthy();
        expect(def!.params[key].description).toBeTruthy();
      }
    }
  });

  it("has no extra params keys beyond the Zod shape", () => {
    for (const def of allDefs) {
      const shapeKeys = new Set(Object.keys(def!.schema.shape));
      for (const key of Object.keys(def!.params)) {
        expect(shapeKeys.has(key)).toBe(true);
      }
    }
  });

  it("enum fields inferred via inferFieldType have enumValues", () => {
    for (const def of allDefs) {
      for (const [, zodField] of Object.entries(def!.schema.shape)) {
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
      for (const [, zodField] of Object.entries(def!.schema.shape)) {
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
  it("image-compress accepts empty object (quality defaults)", () => {
    const result = NODE_SCHEMAS["image-compress"]!.schema.safeParse({});
    expect(result.success).toBe(true);
  });

  it("image-resize accepts empty object (all optional/defaulted)", () => {
    const result = NODE_SCHEMAS["image-resize"]!.schema.safeParse({});
    expect(result.success).toBe(true);
  });

  it("image-convert defaults format to jpeg when omitted", () => {
    const result = NODE_SCHEMAS["image-convert"]!.schema.safeParse({});
    expect(result.success).toBe(true);
    if (result.success) expect(result.data.format).toBe("jpeg");
  });

  it("image-convert accepts valid format", () => {
    const result = NODE_SCHEMAS["image-convert"]!.schema.safeParse({ format: "webp" });
    expect(result.success).toBe(true);
  });

  it("loop requires mode", () => {
    const result = NODE_SCHEMAS["loop"]!.schema.safeParse({});
    expect(result.success).toBe(false);
  });

  it("group defaults mode to sequential", () => {
    const result = NODE_SCHEMAS["group"]!.schema.safeParse({});
    expect(result.success).toBe(true);
    if (result.success) {
      expect(result.data.mode).toBe("sequential");
    }
  });

  it("edit-fields requires values", () => {
    const result = NODE_SCHEMAS["edit-fields"]!.schema.safeParse({});
    expect(result.success).toBe(false);
  });

  it("input defaults mode to file-upload", () => {
    const result = NODE_SCHEMAS["input"]!.schema.safeParse({});
    expect(result.success).toBe(true);
    if (result.success) {
      expect(result.data.mode).toBe("file-upload");
    }
  });

  it("output defaults mode to download", () => {
    const result = NODE_SCHEMAS["output"]!.schema.safeParse({});
    expect(result.success).toBe(true);
    if (result.success) {
      expect(result.data.mode).toBe("download");
    }
  });
});

// ---------- Enum constant exports ----------

describe("enum constants", () => {
  it("LOOP_MODES has 3 modes", () => {
    expect(LOOP_MODES).toHaveLength(3);
  });

  it("IMAGE_FORMATS has 3 formats", () => {
    expect(IMAGE_FORMATS).toHaveLength(3);
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
