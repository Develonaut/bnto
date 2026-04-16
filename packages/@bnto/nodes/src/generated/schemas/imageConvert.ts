/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchema } from "../../schemas/types";

/** Zod schema for image-convert node parameters. */
export const imageConvertParamsSchema = z.object({
  format: z.enum(["jpeg", "png", "webp"] as const).default("jpeg"),
  quality: z.number().min(1).max(100).optional().default(80),
});

/** Inferred TypeScript type for image-convert node parameters. */
export type ImageConvertParams = z.infer<typeof imageConvertParamsSchema>;

/** Full schema definition for the image-convert node type. */
export const imageConvertNodeSchema: NodeSchema = {
  nodeType: "image-convert",
  schemaVersion: 1,
  schema: imageConvertParamsSchema,
  params: {
    format: {
      label: "Output Format",
      description: "The target image format to convert to",
    },
    quality: {
      label: "Quality",
      description:
        "Output quality (1 = lowest, 100 = highest). WebP is lossless-only; quality has no effect until lossy WebP support is added.",
    },
  },
};
