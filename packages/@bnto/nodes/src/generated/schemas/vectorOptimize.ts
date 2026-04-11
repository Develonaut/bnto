/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchema } from "../../schemas/types";

/** Zod schema for vector-optimize node parameters. */
export const vectorOptimizeParamsSchema = z.object({
  precision: z.number().min(1).max(10).optional().default(3),
  removeComments: z.boolean().optional().default(true),
  removeMetadata: z.boolean().optional().default(true),
  collapseGroups: z.boolean().optional().default(true),
  minify: z.boolean().optional().default(true),
});

/** Inferred TypeScript type for vector-optimize node parameters. */
export type VectorOptimizeParams = z.infer<typeof vectorOptimizeParamsSchema>;

/** Full schema definition for the vector-optimize node type. */
export const vectorOptimizeNodeSchema: NodeSchema = {
  nodeType: "vector-optimize",
  schemaVersion: 1,
  schema: vectorOptimizeParamsSchema,
  params: {
    precision: {
      label: "Numeric Precision",
      description: "Decimal places for numeric values in paths and transforms (1-10)",
    },
    removeComments: {
      label: "Remove Comments",
      description: "Strip XML comments",
    },
    removeMetadata: {
      label: "Remove Metadata",
      description: "Strip <metadata> elements",
    },
    collapseGroups: {
      label: "Collapse Groups",
      description: "Merge redundant nested <g> elements",
    },
    minify: {
      label: "Minify",
      description: "Remove unnecessary whitespace and line breaks",
    },
  },
};
