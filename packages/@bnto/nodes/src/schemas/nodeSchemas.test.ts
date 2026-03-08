/**
 * Tests for individual node type schemas — validates Zod shapes,
 * defaults, visibility rules, and conditional requirements.
 */
import { describe, expect, it } from "vitest";

import { NODE_SCHEMA_DEFS, inferFieldType } from "./index";

describe("http-request schema", () => {
  const def = NODE_SCHEMA_DEFS["http-request"];

  it("url is required (not optional, not defaulted)", () => {
    const outerType = def.schema.shape.url._def.typeName;
    expect(outerType).not.toBe("ZodOptional");
    expect(outerType).not.toBe("ZodDefault");
  });

  it("method defaults to GET", () => {
    const result = def.schema.safeParse({ url: "https://example.com" });
    expect(result.success).toBe(true);
    if (result.success) expect(result.data.method).toBe("GET");
  });

  it("method enum has 7 values", () => {
    const info = inferFieldType(def.schema.shape.method);
    expect(info.type).toBe("enum");
    expect(info.enumValues).toHaveLength(7);
    expect(info.enumValues).toContain("GET");
    expect(info.enumValues).toContain("OPTIONS");
  });

  it("timeout defaults to 30", () => {
    const result = def.schema.safeParse({ url: "https://example.com" });
    expect(result.success).toBe(true);
    if (result.success) expect(result.data.timeout).toBe(30);
  });
});

describe("file-system schema", () => {
  const def = NODE_SCHEMA_DEFS["file-system"];

  it("operation is required", () => {
    const result = def.schema.safeParse({});
    expect(result.success).toBe(false);
  });

  it("operation enum has 9 values (8 legacy + rename from engine)", () => {
    const info = inferFieldType(def.schema.shape.operation);
    expect(info.type).toBe("enum");
    expect(info.enumValues).toHaveLength(9);
    expect(info.enumValues).toContain("list");
    expect(info.enumValues).toContain("delete");
    expect(info.enumValues).toContain("rename");
  });

  it("content is conditionally required for write", () => {
    expect(def.params.content.requiredWhen).toEqual({
      param: "operation",
      equals: "write",
    });
  });
});

describe("loop schema", () => {
  const def = NODE_SCHEMA_DEFS["loop"];

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

describe("shell-command schema", () => {
  const def = NODE_SCHEMA_DEFS["shell-command"];

  it("command is required", () => {
    const result = def.schema.safeParse({});
    expect(result.success).toBe(false);
  });

  it("has correct defaults for timeout, retry, stall", () => {
    const result = def.schema.safeParse({ command: "echo hello" });
    expect(result.success).toBe(true);
    if (result.success) {
      expect(result.data.timeout).toBe(120);
      expect(result.data.retry).toBe(0);
      expect(result.data.retryDelay).toBe(5);
      expect(result.data.stallTimeout).toBe(0);
    }
  });
});

describe("edit-fields schema", () => {
  const def = NODE_SCHEMA_DEFS["edit-fields"];

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
  const def = NODE_SCHEMA_DEFS["image"];

  it("operation is required", () => {
    const result = def.schema.safeParse({});
    expect(result.success).toBe(false);
  });

  it("quality defaults to 80 with 1-100 range", () => {
    const result = def.schema.safeParse({ operation: "resize" });
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

  it("composite requires base and overlay", () => {
    expect(def.params.base.requiredWhen).toEqual({
      param: "operation",
      equals: "composite",
    });
    expect(def.params.overlay.requiredWhen).toEqual({
      param: "operation",
      equals: "composite",
    });
  });
});

describe("spreadsheet schema", () => {
  const def = NODE_SCHEMA_DEFS["spreadsheet"];

  it("requires operation, format, and path", () => {
    const result = def.schema.safeParse({});
    expect(result.success).toBe(false);
  });

  it("passes with required fields", () => {
    const result = def.schema.safeParse({ operation: "read", format: "csv", path: "/file.csv" });
    expect(result.success).toBe(true);
  });

  it("rows conditionally required for write", () => {
    expect(def.params.rows.requiredWhen).toEqual({
      param: "operation",
      equals: "write",
    });
  });
});

describe("transform schema", () => {
  const def = NODE_SCHEMA_DEFS["transform"];

  it("has no required parameters", () => {
    const result = def.schema.safeParse({});
    expect(result.success).toBe(true);
  });
});

describe("group schema", () => {
  const def = NODE_SCHEMA_DEFS["group"];

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
  const def = NODE_SCHEMA_DEFS["parallel"];

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
