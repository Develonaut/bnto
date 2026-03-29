/**
 * AUTO-GENERATED from engine/catalog.snapshot.json — DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchema } from "../../schemas/types";

/** Zod schema for image-watermark node parameters. */
export const imageWatermarkParamsSchema = z.object({
    watermark: z.string(),
    position: z.enum(["top-left","top-center","top-right","middle-left","center","middle-right","bottom-left","bottom-center","bottom-right"] as const).optional().default("bottom-right"),
    size: z.number().min(1).max(500).optional().default(25),
    opacity: z.number().min(0).max(100).optional().default(80),
    offsetX: z.number().min(-500).max(500).optional().default(0),
    offsetY: z.number().min(-500).max(500).optional().default(0),
    quality: z.number().min(1).max(100).optional().default(80),
});

/** Inferred TypeScript type for image-watermark node parameters. */
export type ImageWatermarkParams = z.infer<typeof imageWatermarkParamsSchema>;

/** Full schema definition for the image-watermark node type. */
export const imageWatermarkNodeSchema: NodeSchema = {
  nodeType: "image-watermark",
  schemaVersion: 1,
  schema: imageWatermarkParamsSchema,
  params: {
    watermark: {
      label: "Watermark Image",
      description: "The watermark image to overlay (base64-encoded).",
    },
    position: {
      label: "Position",
      description: "Where to place the watermark on the image.",
    },
    size: {
      label: "Size",
      description: "Watermark width as a percentage of the source image width.",
    },
    opacity: {
      label: "Opacity",
      description: "Watermark transparency (0 = invisible, 100 = fully opaque).",
    },
    offsetX: {
      label: "Offset X",
      description: "Horizontal pixel offset from the position. Positive = right, negative = left.",
    },
    offsetY: {
      label: "Offset Y",
      description: "Vertical pixel offset from the position. Positive = down, negative = up.",
    },
    quality: {
      label: "Quality",
      description: "Output quality (1 = lowest, 100 = highest). WebP is lossless-only; quality has no effect until lossy WebP support is added.",
    },
  },
};
