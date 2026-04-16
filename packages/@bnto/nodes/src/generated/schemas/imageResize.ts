/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchema } from "../../schemas/types";

/** Zod schema for image-resize node parameters. */
export const imageResizeParamsSchema = z.object({
    width: z.number().min(1).optional(),
    height: z.number().min(1).optional(),
    maintainAspect: z.boolean().optional().default(true),
    quality: z.number().min(1).max(100).optional().default(80),
});

/** Inferred TypeScript type for image-resize node parameters. */
export type ImageResizeParams = z.infer<typeof imageResizeParamsSchema>;

/** Full schema definition for the image-resize node type. */
export const imageResizeNodeSchema: NodeSchema = {
  nodeType: "image-resize",
  schemaVersion: 1,
  schema: imageResizeParamsSchema,
  params: {
    width: {
      label: "Width",
      description: "Target width in pixels",
    },
    height: {
      label: "Height",
      description: "Target height in pixels",
    },
    maintainAspect: {
      label: "Maintain Aspect Ratio",
      description: "Keep the original width-to-height ratio when resizing",
    },
    quality: {
      label: "Quality",
      description: "Output quality (1 = lowest, 100 = highest). WebP is lossless-only; quality has no effect until lossy WebP support is added.",
    },
  },
};
