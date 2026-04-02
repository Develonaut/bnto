/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchema } from "../../schemas/types";

/** Zod schema for image-strip-exif node parameters. */
export const imageStripExifParamsSchema = z.object({
    quality: z.number().min(1).max(100).optional().default(80),
});

/** Inferred TypeScript type for image-strip-exif node parameters. */
export type ImageStripExifParams = z.infer<typeof imageStripExifParamsSchema>;

/** Full schema definition for the image-strip-exif node type. */
export const imageStripExifNodeSchema: NodeSchema = {
  nodeType: "image-strip-exif",
  schemaVersion: 1,
  schema: imageStripExifParamsSchema,
  params: {
    quality: {
      label: "Quality",
      description: "Output quality (1 = lowest, 100 = highest). WebP is lossless-only; quality has no effect until lossy WebP support is added.",
    },
  },
};
