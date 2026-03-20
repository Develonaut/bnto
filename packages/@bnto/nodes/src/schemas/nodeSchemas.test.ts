/**
 * Tests for individual node type schemas — validates Zod shapes,
 * defaults, visibility rules, and conditional requirements.
 */
import { describe, expect, it } from "vitest";

import { NODE_SCHEMA_DEFS, inferFieldType } from "./index";

describe("file-system schema", () => {
  const def = NODE_SCHEMA_DEFS["file-system"]!;

  it("operation is required", () => {
    const result = def.schema.safeParse({});
    expect(result.success).toBe(false);
  });

  it("operation enum has 1 value (engine-only: rename)", () => {
    const info = inferFieldType(def.schema.shape.operation);
    expect(info.type).toBe("enum");
    expect(info.enumValues).toHaveLength(1);
    expect(info.enumValues).toContain("rename");
  });
});

describe("loop schema", () => {
  const def = NODE_SCHEMA_DEFS["loop"]!;

  it("mode is required", () => {
    const result = def.schema.safeParse({});
    expect(result.success).toBe(false);
  });

  it("mode-specific params are conditionally required (items is optional)", () => {
    // items is optional — the Rust engine iterates files directly
    expect(def.params.items.requiredWhen).toBeUndefined();
    expect(def.params.count.requiredWhen).toEqual({ param: "mode", equals: "times" });
    expect(def.params.condition.requiredWhen).toEqual({ param: "mode", equals: "while" });
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

describe("image schema", () => {
  const def = NODE_SCHEMA_DEFS["image"]!;

  it("operation is required", () => {
    const result = def.schema.safeParse({});
    expect(result.success).toBe(false);
  });

  it("quality defaults to 80 with 1-100 range", () => {
    const result = def.schema.safeParse({ operation: "compress" });
    expect(result.success).toBe(true);
    if (result.success) expect(result.data.quality).toBe(80);

    const info = inferFieldType(def.schema.shape.quality);
    expect(info.min).toBe(1);
    expect(info.max).toBe(100);
  });

  it("resize params visible only for resize", () => {
    for (const name of ["width", "height", "maintainAspect"]) {
      expect(def.params[name].visibleWhen).toEqual({
        param: "operation",
        equals: "resize",
      });
    }
  });

  it("rejects composite (removed — no engine processor)", () => {
    const result = def.schema.safeParse({ operation: "composite" });
    expect(result.success).toBe(false);
  });
});

describe("spreadsheet schema", () => {
  const def = NODE_SCHEMA_DEFS["spreadsheet"]!;

  it("requires operation", () => {
    const result = def.schema.safeParse({});
    expect(result.success).toBe(false);
  });

  it("passes with engine operation clean", () => {
    const result = def.schema.safeParse({ operation: "clean" });
    expect(result.success).toBe(true);
  });

  it("passes with engine operation rename", () => {
    const result = def.schema.safeParse({ operation: "rename" });
    expect(result.success).toBe(true);
  });

  it("rejects removed legacy operation read", () => {
    const result = def.schema.safeParse({ operation: "read" });
    expect(result.success).toBe(false);
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
