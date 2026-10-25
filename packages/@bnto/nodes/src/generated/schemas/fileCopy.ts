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
  createDirs: z.boolean().optional().default(true),
  conflict: z
    .enum(["skip", "overwrite", "rename", "newer"] as const)
    .optional()
    .default("skip"),
  toleranceMs: z.number().min(0).optional().default(2000),
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
      description: "Folder to copy files into.",
      placeholder: "./output",
    },
    createDirs: {
      label: "Create Folders",
      description: "Automatically create the destination folder if it doesn't exist.",
    },
    conflict: {
      label: "Conflict Resolution",
      description:
        "What to do when a file with the same name already exists. \"newer\" copies only when the destination is missing or older than the source (path input only), keeps the source's modified time, and never deletes. A copy that would replace a file another incoming file is read from is refused before anything is copied (inside a loop, each iteration's files are checked on their own).",
    },
    toleranceMs: {
      label: "Time Tolerance",
      description:
        'Modified-time gap treated as equal in "newer" mode; FAT and Google Drive round times.',
    },
  },
};
