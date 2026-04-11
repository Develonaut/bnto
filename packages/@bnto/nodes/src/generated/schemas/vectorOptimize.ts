/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchema } from "../../schemas/types";

/** Zod schema for vector-optimize node parameters. */
export const vectorOptimizeParamsSchema = z.object({
    removeComments: z.boolean().optional().default(true),
    removeMetadata: z.boolean().optional().default(true),
    collapseGroups: z.boolean().optional().default(true),
    minify: z.boolean().optional().default(true),
    precision: z.number().min(0).max(8).optional().default(3),
});

/** Inferred TypeScript type for vector-optimize node parameters. */
export type VectorOptimizeParams = z.infer<typeof vectorOptimizeParamsSchema>;

/** Full schema definition for the vector-optimize node type. */
export const vectorOptimizeNodeSchema: NodeSchema = {
  nodeType: "vector-optimize",
  schemaVersion: 1,
  schema: vectorOptimizeParamsSchema,
  params: {
    removeComments: {
      label: "Remove Comments",
      description: "Strip XML comments from the SVG",
    },
    removeMetadata: {
      label: "Remove Metadata",
      description: "Strip <metadata> elements (RDF, Dublin Core, etc.)",
    },
    collapseGroups: {
      label: "Collapse Groups",
      description: "Remove empty groups and collapse single-child wrapper groups",
    },
    minify: {
      label: "Minify",
      description: "Remove unnecessary whitespace and newlines",
    },
    precision: {
      label: "Numeric Precision",
      description: "Decimal places for numeric values (reserved for Tier 2)",
    },
  },
};
