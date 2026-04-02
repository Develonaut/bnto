/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchema } from "../../schemas/types";

/** Zod schema for image-compress node parameters. */
export const imageCompressParamsSchema = z.object({
    quality: z.number().min(1).max(100).optional().default(80),
});

/** Inferred TypeScript type for image-compress node parameters. */
export type ImageCompressParams = z.infer<typeof imageCompressParamsSchema>;

/** Full schema definition for the image-compress node type. */
export const imageCompressNodeSchema: NodeSchema = {
  nodeType: "image-compress",
  schemaVersion: 1,
  schema: imageCompressParamsSchema,
  params: {
    quality: {
      label: "Quality",
      description: "Output quality (1 = lowest, 100 = highest). WebP is lossless-only; quality has no effect until lossy WebP support is added.",
    },
  },
};
