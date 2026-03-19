/**
 * Extract default values from a NodeSchemaDefinition's Zod shape.
 *
 * Walks each field in the schema shape, unwraps ZodDefault/Optional/Nullable
 * layers, and collects `_def.defaultValue()` from ZodDefault wrappers.
 * Fields without a ZodDefault wrapper are omitted from the result.
 */

import type { z } from "zod";
import type { NodeSchemaDefinition } from "./types";

/**
 * Extract all Zod `.default()` values from a schema definition.
 *
 * Returns a plain object of { paramName: defaultValue } for every field
 * that has a ZodDefault wrapper. Fields that are only optional (no default)
 * or required are excluded.
 */
export function extractSchemaDefaults(schema: NodeSchemaDefinition): Record<string, unknown> {
  const defaults: Record<string, unknown> = {};
  const shape = schema.schema.shape as Record<string, z.ZodTypeAny>;

  for (const [name, zodType] of Object.entries(shape)) {
    const defaultValue = findDefault(zodType);
    if (defaultValue !== undefined) {
      defaults[name] = defaultValue;
    }
  }

  return defaults;
}

/** Walk up the wrapper chain looking for a ZodDefault node. */
function findDefault(zodType: z.ZodTypeAny): unknown | undefined {
  const typeName = (zodType._def.typeName ?? "") as string;

  if (typeName === "ZodDefault") {
    return zodType._def.defaultValue();
  }

  // ZodOptional and ZodNullable wrap an inner type that may contain a default
  if (typeName === "ZodOptional" || typeName === "ZodNullable") {
    return findDefault(zodType._def.innerType);
  }

  return undefined;
}
