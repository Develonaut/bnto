/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchema } from "../../schemas/types";

/** Zod schema for file-inspect node parameters. */
export const fileInspectParamsSchema = z.object({
  includeHash: z.boolean().optional().default(false),
});

/** Inferred TypeScript type for file-inspect node parameters. */
export type FileInspectParams = z.infer<typeof fileInspectParamsSchema>;

/** Full schema definition for the file-inspect node type. */
export const fileInspectNodeSchema: NodeSchema = {
  nodeType: "file-inspect",
  schemaVersion: 1,
  schema: fileInspectParamsSchema,
  params: {
    includeHash: {
      label: "Include Hash",
      description: "Compute and include a SHA-256 hash of the file content.",
    },
  },
};
