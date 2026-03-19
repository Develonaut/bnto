import { describe, expect, it } from "vitest";
import { NODE_SCHEMA_DEFS } from "./registry";
import { extractSchemaDefaults } from "./extractSchemaDefaults";
import { getVisibleParams } from "./getVisibleParams";
import { inferFieldType } from "./inferFieldType";
import type { NodeSchemaDefinition } from "./types";

/**
 * Integration tests for the schema-to-form pipeline.
 *
 * Verifies the full chain: engine schema → Zod → defaults → visibility → field type inference.
 * Every node type with a schema must produce valid defaults, visible params,
 * and inferrable field types — the three inputs that SchemaForm needs.
 */

const schemaEntries = Object.entries(NODE_SCHEMA_DEFS) as [string, NodeSchemaDefinition][];

/**
 * Build "ready" values for a schema: defaults + first enum value for any
 * required enum field without a default. This simulates a form that just
 * opened — every required field has a sensible value.
 */
function buildReadyValues(schemaDef: NodeSchemaDefinition): Record<string, unknown> {
  const defaults = extractSchemaDefaults(schemaDef);
  const shape = schemaDef.schema.shape;

  for (const [key, zodField] of Object.entries(shape)) {
    if (key in defaults) continue;

    // Walk through ZodOptional/ZodDefault wrappers to find the inner type
    let inner = zodField;
    while (inner && "_def" in inner) {
      const def = inner._def as Record<string, unknown>;
      if (def.innerType) {
        inner = def.innerType as typeof inner;
      } else {
        break;
      }
    }

    // Check if the inner type is a ZodEnum
    const typeName = inner?._def?.typeName as string | undefined;
    if (typeName === "ZodEnum") {
      const values = (inner._def as Record<string, unknown>).values as string[];
      if (values?.length) {
        defaults[key] = values[0];
      }
    }
  }

  return defaults;
}

describe("schema form integration", () => {
  it.each(schemaEntries)(
    "%s: extractSchemaDefaults produces a valid object",
    (_typeName, schemaDef) => {
      const defaults = extractSchemaDefaults(schemaDef);
      expect(defaults).toBeDefined();
      expect(typeof defaults).toBe("object");
    },
  );

  it.each(schemaEntries)(
    "%s: getVisibleParams returns non-empty array with ready values",
    (typeName, schemaDef) => {
      const values = buildReadyValues(schemaDef);
      const visible = getVisibleParams(typeName, values);
      expect(Array.isArray(visible)).toBe(true);
      expect(visible.length).toBeGreaterThan(0);
    },
  );

  it.each(schemaEntries)("%s: every visible param has a Zod shape entry", (typeName, schemaDef) => {
    const values = buildReadyValues(schemaDef);
    const visible = getVisibleParams(typeName, values);
    const shape = schemaDef.schema.shape;

    for (const paramName of visible) {
      expect(shape[paramName]).toBeDefined();
    }
  });

  it.each(schemaEntries)(
    "%s: inferFieldType succeeds for every visible param",
    (typeName, schemaDef) => {
      const values = buildReadyValues(schemaDef);
      const visible = getVisibleParams(typeName, values);
      const shape = schemaDef.schema.shape;

      for (const paramName of visible) {
        const meta = schemaDef.params[paramName];
        expect(meta).toBeDefined();
        const fieldInfo = inferFieldType(shape[paramName], meta!);
        expect(fieldInfo).toBeDefined();
        expect(fieldInfo.control).toBeTruthy();
        expect(fieldInfo.type).toBeTruthy();
      }
    },
  );

  it.each(schemaEntries)("%s: ready values pass partial Zod validation", (_typeName, schemaDef) => {
    const values = buildReadyValues(schemaDef);
    // Use .partial() because some schemas have complex required fields
    // (records, arrays) that can't be auto-seeded from defaults alone.
    // The form always renders with whatever values are provided — partial
    // validation confirms the seeded defaults don't conflict with constraints.
    const result = schemaDef.schema.partial().safeParse(values);
    expect(result.success).toBe(true);
  });
});
