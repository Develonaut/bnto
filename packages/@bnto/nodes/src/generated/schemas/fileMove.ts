/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchema } from "../../schemas/types";

/** Zod schema for file-move node parameters. */
export const fileMoveParamsSchema = z.object({
  destination: z.string().optional(),
  create_dirs: z.boolean().optional().default(true),
  conflict: z
    .enum(["skip", "overwrite", "rename"] as const)
    .optional()
    .default("skip"),
});

/** Inferred TypeScript type for file-move node parameters. */
export type FileMoveParams = z.infer<typeof fileMoveParamsSchema>;

/** Full schema definition for the file-move node type. */
export const fileMoveNodeSchema: NodeSchema = {
  nodeType: "file-move",
  schemaVersion: 1,
  schema: fileMoveParamsSchema,
  params: {
    destination: {
      label: "Destination",
      description: "Directory path to move files into.",
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
