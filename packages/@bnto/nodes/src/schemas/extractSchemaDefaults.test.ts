import { describe, expect, it } from "vitest";
import { z } from "zod";
import { extractSchemaDefaults } from "./extractSchemaDefaults";
import type { NodeSchemaDefinition } from "./types";
import { NODE_SCHEMA_DEFS } from "./registry";

/** Helper to build a minimal NodeSchemaDefinition from a Zod object. */
function makeSchema(schema: z.ZodObject<z.ZodRawShape>): NodeSchemaDefinition {
  return { nodeType: "test", schemaVersion: 1, schema, params: {} };
}

describe("extractSchemaDefaults", () => {
  it("extracts .default() values from Zod fields", () => {
    const schema = makeSchema(
      z.object({
        compression: z.number().min(1).max(100).optional().default(20),
        quality: z.number().min(1).max(100).optional().default(80),
        maintainAspect: z.boolean().optional().default(true),
      }),
    );

    expect(extractSchemaDefaults(schema)).toEqual({
      compression: 20,
      quality: 80,
      maintainAspect: true,
    });
  });

  it("omits fields without defaults", () => {
    const schema = makeSchema(
      z.object({
        name: z.string(),
        label: z.string().optional(),
        count: z.number().default(5),
      }),
    );

    expect(extractSchemaDefaults(schema)).toEqual({ count: 5 });
  });

  it("returns empty object for schema with no defaults", () => {
    const schema = makeSchema(
      z.object({
        operation: z.enum(["rename"]),
        find: z.string().optional(),
        replace: z.string().optional(),
      }),
    );

    expect(extractSchemaDefaults(schema)).toEqual({});
  });

  it("extracts image schema defaults", () => {
    const imageDef = NODE_SCHEMA_DEFS["image"];
    expect(imageDef).toBeDefined();
    const defaults = extractSchemaDefaults(imageDef!);
    expect(defaults).toEqual({
      compression: 20,
      quality: 80,
      maintainAspect: true,
    });
  });

  it("extracts spreadsheet schema defaults", () => {
    const spreadsheetDef = NODE_SCHEMA_DEFS["spreadsheet"];
    expect(spreadsheetDef).toBeDefined();
    const defaults = extractSchemaDefaults(spreadsheetDef!);
    expect(defaults).toEqual({
      trimWhitespace: true,
      removeEmptyRows: true,
      removeDuplicates: true,
    });
  });

  it("handles string enum defaults", () => {
    const schema = makeSchema(
      z.object({
        mode: z.enum(["a", "b", "c"]).default("b"),
      }),
    );

    expect(extractSchemaDefaults(schema)).toEqual({ mode: "b" });
  });
});
