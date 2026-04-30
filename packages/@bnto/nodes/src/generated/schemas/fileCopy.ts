/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchema } from "../../schemas/types";

/** Zod schema for file-copy node parameters. */
export const fileCopyParamsSchema = z.object({
  destination: z.string().optional(),
  create_dirs: z.boolean().optional().default(true),
  conflict: z
    .enum(["skip", "overwrite", "rename"] as const)
    .optional()
    .default("skip"),
});

/** Inferred TypeScript type for file-copy node parameters. */
export type FileCopyParams = z.infer<typeof fileCopyParamsSchema>;

/** Full schema definition for the file-copy node type. */
export const fileCopyNodeSchema: NodeSchema = {
  nodeType: "file-copy",
  schemaVersion: 1,
  schema: fileCopyParamsSchema,
  params: {
    destination: {
      label: "Destination",
      description: "Directory path to copy files into.",
      placeholder: "./output",
    },
    create_dirs: {
      label: "Create Directories",
      description: "Automatically create the destination directory if it doesn't exist.",
    },
    conflict: {
      label: "Conflict Resolution",
      description: "What to do when a file with the same name already exists.",
    },
  },
};
