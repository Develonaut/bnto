/**
 * Transform node schema — parameters for data transformations via expr.
 *
 * Go source: engine/pkg/node/library/transform/transform.go
 */

import type { NodeSchema } from "./types";

export const transformSchema: NodeSchema = {
  nodeType: "transform",
  schemaVersion: 1,
  parameters: [
    {
      name: "expression",
      type: "string",
      required: false,
      label: "Expression",
      description: "Expr expression for a single transformation. Mutually exclusive with mappings.",
      placeholder: 'firstName + " " + lastName',
    },
    {
      name: "mappings",
      type: "object",
      required: false,
      label: "Mappings",
      description:
        "Map of field names to expr expressions for multi-field transformations. Mutually exclusive with expression.",
    },
  ],
} as const;
