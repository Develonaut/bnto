/**
 * Tests for individual node type schemas — validates Zod shapes,
 * defaults, visibility rules, and conditional requirements.
 */
import { describe, expect, it } from "vitest";

import { NODE_SCHEMA_DEFS, NODE_FIELD_CONFIGS, inferFieldType } from "./index";

describe("file-rename schema", () => {
  const def = NODE_SCHEMA_DEFS["file-rename"]!;

  it("accepts empty object (all optional)", () => {
    const result = def.schema.safeParse({});
    expect(result.success).toBe(true);
  });

  it("case enum has 3 values", () => {
    const info = inferFieldType(def.schema.shape.case);
    expect(info.type).toBe("enum");
    expect(info.enumValues).toHaveLength(3);
    expect(info.enumValues).toContain("lower");
    expect(info.enumValues).toContain("upper");
    expect(info.enumValues).toContain("title");
  });
});

describe("loop schema", () => {
  const def = NODE_SCHEMA_DEFS["loop"]!;

  it("mode is required", () => {
    const result = def.schema.safeParse({});
    expect(result.success).toBe(false);
  });

  it("mode-specific params are conditionally required via FieldConfig (items is optional)", () => {
    const fields = NODE_FIELD_CONFIGS["loop"]!;
    // items is optional — the Rust engine iterates files directly
    expect(fields.items?.requiredWhen).toBeUndefined();
    expect(fields.count?.requiredWhen).toEqual({ param: "mode", equals: "times" });
    expect(fields.condition?.requiredWhen).toEqual({ param: "mode", equals: "while" });
  });
});

describe("edit-fields schema", () => {
  const def = NODE_SCHEMA_DEFS["edit-fields"]!;

  it("values is required", () => {
    const result = def.schema.safeParse({});
    expect(result.success).toBe(false);
  });

  it("keepOnlySet defaults to false", () => {
    const result = def.schema.safeParse({ values: { name: "test" } });
    expect(result.success).toBe(true);
    if (result.success) expect(result.data.keepOnlySet).toBe(false);
  });
});

describe("image-compress schema", () => {
  const def = NODE_SCHEMA_DEFS["image-compress"]!;

  it("quality defaults to 80 with 1-100 range", () => {
    const result = def.schema.safeParse({});
    expect(result.success).toBe(true);
    if (result.success) expect(result.data.quality).toBe(80);

    const info = inferFieldType(def.schema.shape.quality);
    expect(info.min).toBe(1);
    expect(info.max).toBe(100);
  });
});

describe("image-resize schema", () => {
  const def = NODE_SCHEMA_DEFS["image-resize"]!;

  it("accepts empty object (all optional/defaulted)", () => {
    const result = def.schema.safeParse({});
    expect(result.success).toBe(true);
  });

  it("maintainAspect defaults to true", () => {
    const result = def.schema.safeParse({});
    expect(result.success).toBe(true);
    if (result.success) expect(result.data.maintainAspect).toBe(true);
  });

  it("quality defaults to 80", () => {
    const result = def.schema.safeParse({});
    expect(result.success).toBe(true);
    if (result.success) expect(result.data.quality).toBe(80);
  });
});

describe("image-convert schema", () => {
  const def = NODE_SCHEMA_DEFS["image-convert"]!;

  it("defaults format to jpeg when omitted", () => {
    const result = def.schema.safeParse({});
    expect(result.success).toBe(true);
    if (result.success) expect(result.data.format).toBe("jpeg");
  });

  it("accepts valid format", () => {
    const result = def.schema.safeParse({ format: "webp" });
    expect(result.success).toBe(true);
  });

  it("rejects invalid format", () => {
    const result = def.schema.safeParse({ format: "gif" });
    expect(result.success).toBe(false);
  });
});

describe("spreadsheet-clean schema", () => {
  const def = NODE_SCHEMA_DEFS["spreadsheet-clean"]!;

  it("accepts empty object (all defaulted)", () => {
    const result = def.schema.safeParse({});
    expect(result.success).toBe(true);
  });

  it("defaults trimWhitespace to true", () => {
    const result = def.schema.safeParse({});
    expect(result.success).toBe(true);
    if (result.success) expect(result.data.trimWhitespace).toBe(true);
  });
});

describe("spreadsheet-rename schema", () => {
  const def = NODE_SCHEMA_DEFS["spreadsheet-rename"]!;

  it("accepts empty object (columns optional)", () => {
    const result = def.schema.safeParse({});
    expect(result.success).toBe(true);
  });
});

describe("transform schema", () => {
  const def = NODE_SCHEMA_DEFS["transform"]!;

  it("has no required parameters", () => {
    const result = def.schema.safeParse({});
    expect(result.success).toBe(true);
  });
});

describe("group schema", () => {
  const def = NODE_SCHEMA_DEFS["group"]!;

  it("has no required parameters", () => {
    const result = def.schema.safeParse({});
    expect(result.success).toBe(true);
  });

  it("mode defaults to sequential", () => {
    const result = def.schema.safeParse({});
    expect(result.success).toBe(true);
    if (result.success) expect(result.data.mode).toBe("sequential");
  });
});

describe("parallel schema", () => {
  const def = NODE_SCHEMA_DEFS["parallel"]!;

  it("requires tasks", () => {
    const result = def.schema.safeParse({});
    expect(result.success).toBe(false);
  });

  it("errorStrategy defaults to failFast", () => {
    const result = def.schema.safeParse({ tasks: [{ a: 1 }] });
    expect(result.success).toBe(true);
    if (result.success) expect(result.data.errorStrategy).toBe("failFast");
  });
});
