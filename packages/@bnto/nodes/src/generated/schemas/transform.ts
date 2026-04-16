/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchema } from "../../schemas/types";

/** Zod schema for transform node parameters. */
export const transformParamsSchema = z.object({
  expression: z.string().optional(),
  mappings: z.record(z.unknown()).optional(),
});

/** Inferred TypeScript type for transform node parameters. */
export type TransformParams = z.infer<typeof transformParamsSchema>;

/** Full schema definition for the transform node type. */
export const transformNodeSchema: NodeSchema = {
  nodeType: "transform",
  schemaVersion: 1,
  schema: transformParamsSchema,
  params: {
    expression: {
      label: "Expression",
      description: "Expr expression for a single transformation. Mutually exclusive with mappings.",
      placeholder: 'firstName + " " + lastName',
    },
    mappings: {
      label: "Mappings",
      description:
        "Map of field names to expr expressions for multi-field transformations. Mutually exclusive with expression.",
    },
  },
};
