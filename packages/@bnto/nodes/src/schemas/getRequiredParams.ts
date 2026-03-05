/**
 * Returns the names of required parameters for a node type.
 *
 * A parameter is "required" if the Zod schema does NOT wrap it
 * in .optional() or .default().
 */

import { getNodeSchema } from "./getNodeSchema";

/** Returns names of required parameters for a node type. */
export function getRequiredParams(typeName: string): string[] {
  const schemaDef = getNodeSchema(typeName);
  if (!schemaDef) return [];

  const shape = schemaDef.schema.shape as Record<string, { _def?: { typeName?: string } }>;
  const required: string[] = [];

  for (const [name, field] of Object.entries(shape)) {
    const fieldType = field?._def?.typeName ?? "";
    if (fieldType !== "ZodOptional" && fieldType !== "ZodDefault") {
      required.push(name);
    }
  }

  return required;
}
