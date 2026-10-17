/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchema } from "../../schemas/types";

/** Zod schema for file-list node parameters. */
export const fileListParamsSchema = z.object({
  paths: z.record(z.unknown()).optional(),
  base: z.string().optional(),
});

/** Inferred TypeScript type for file-list node parameters. */
export type FileListParams = z.infer<typeof fileListParamsSchema>;

/** Full schema definition for the file-list node type. */
export const fileListNodeSchema: NodeSchema = {
  nodeType: "file-list",
  schemaVersion: 1,
  schema: fileListParamsSchema,
  params: {
    paths: {
      label: "Paths",
      description:
        "Paths to list, relative to the base folder, added to any read from input files (one per line, or NUL-separated).",
    },
    base: {
      label: "Base Folder",
      description:
        'Folder the listed paths are relative to. Defaults to the working directory; absolute paths and ".." are refused.',
      placeholder: ".",
    },
  },
};
