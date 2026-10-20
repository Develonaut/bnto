/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchema } from "../../schemas/types";

/** Zod schema for image-pad node parameters. */
export const imagePadParamsSchema = z.object({
  width: z.number().min(1).max(16384).optional(),
  height: z.number().min(1).max(16384).optional(),
  background: z.string().optional().default("transparent"),
  quality: z.number().min(1).max(100).optional().default(80),
});

/** Inferred TypeScript type for image-pad node parameters. */
export type ImagePadParams = z.infer<typeof imagePadParamsSchema>;

/** Full schema definition for the image-pad node type. */
export const imagePadNodeSchema: NodeSchema = {
  nodeType: "image-pad",
  schemaVersion: 1,
  schema: imagePadParamsSchema,
  params: {
    width: {
      label: "Width",
      description: "Canvas width in pixels. Leave empty to keep the image's width.",
    },
    height: {
      label: "Height",
      description: "Canvas height in pixels. Leave empty to keep the image's height.",
    },
    background: {
      label: "Background",
      description:
        'Fill for the added space: "transparent" or a hex color (#fff, #ffffff, #ffffff80). JPEG has no transparency, so a transparent fill comes out white.',
      placeholder: "#ffffff",
    },
    quality: {
      label: "Quality",
      description:
        "Output quality (1 = lowest, 100 = highest). WebP is lossless-only; quality has no effect until lossy WebP support is added.",
    },
  },
};
