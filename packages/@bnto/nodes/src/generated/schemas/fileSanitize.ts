/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchema } from "../../schemas/types";

/** Zod schema for file-sanitize node parameters. */
export const fileSanitizeParamsSchema = z.object({
  mode: z
    .enum(["slugify", "strip", "normalize"] as const)
    .optional()
    .default("slugify"),
  separator: z.string().optional().default("-"),
  max_length: z.number().min(0).optional().default(0),
});

/** Inferred TypeScript type for file-sanitize node parameters. */
export type FileSanitizeParams = z.infer<typeof fileSanitizeParamsSchema>;

/** Full schema definition for the file-sanitize node type. */
export const fileSanitizeNodeSchema: NodeSchema = {
  nodeType: "file-sanitize",
  schemaVersion: 1,
  schema: fileSanitizeParamsSchema,
  params: {
    mode: {
      label: "Mode",
      description: "Sanitization strategy",
    },
    separator: {
      label: "Separator",
      description: "Character to replace spaces and special characters",
    },
    max_length: {
      label: "Max Length",
      description: "Maximum filename length (0 = no limit)",
    },
  },
};
