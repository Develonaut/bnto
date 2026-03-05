/**
 * Transform node schema — parameters for data transformations via expr.
 *
 * Go source: engine/pkg/node/library/transform/transform.go
 */

import { z } from "zod";
import type { NodeSchemaDefinition } from "./types";

/** Zod schema for transform node parameters. */
export const transformParamsSchema = z.object({
  expression: z.string().optional(),
  mappings: z.record(z.string()).optional(),
});

/** Inferred TypeScript type for transform node parameters. */
export type TransformParams = z.infer<typeof transformParamsSchema>;

/** Full schema definition for the transform node type. */
export const transformNodeSchema: NodeSchemaDefinition = {
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
