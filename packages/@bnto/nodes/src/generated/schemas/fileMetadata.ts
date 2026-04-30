/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchema } from "../../schemas/types";

/** Zod schema for file-metadata node parameters. */
export const fileMetadataParamsSchema = z.object({
  include_hash: z.boolean().optional().default(false),
});

/** Inferred TypeScript type for file-metadata node parameters. */
export type FileMetadataParams = z.infer<typeof fileMetadataParamsSchema>;

/** Full schema definition for the file-metadata node type. */
export const fileMetadataNodeSchema: NodeSchema = {
  nodeType: "file-metadata",
  schemaVersion: 1,
  schema: fileMetadataParamsSchema,
  params: {
    include_hash: {
      label: "Include Hash",
      description: "Compute and include a SHA-256 hash of the file content.",
    },
  },
};
