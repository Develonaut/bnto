/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchema } from "../../schemas/types";

/** Zod schema for vector-rasterize node parameters. */
export const vectorRasterizeParamsSchema = z.object({
  format: z.enum(["png", "jpeg", "webp"] as const).default("png"),
  quality: z.number().min(1).max(100).optional().default(80),
});

/** Inferred TypeScript type for vector-rasterize node parameters. */
export type VectorRasterizeParams = z.infer<typeof vectorRasterizeParamsSchema>;

/** Full schema definition for the vector-rasterize node type. */
export const vectorRasterizeNodeSchema: NodeSchema = {
  nodeType: "vector-rasterize",
  schemaVersion: 1,
  schema: vectorRasterizeParamsSchema,
  params: {
    format: {
      label: "Output Format",
      description: "The target raster format to convert SVG to",
    },
    quality: {
      label: "Quality",
      description:
        "Output quality (1 = lowest, 100 = highest). Applies to JPEG; PNG is lossless; WebP is lossless-only.",
    },
  },
};
