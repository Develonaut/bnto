/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchema } from "../../schemas/types";

/** Zod schema for file-delete node parameters. */
export const fileDeleteParamsSchema = z.object({
  base: z.string(),
});

/** Inferred TypeScript type for file-delete node parameters. */
export type FileDeleteParams = z.infer<typeof fileDeleteParamsSchema>;

/** Full schema definition for the file-delete node type. */
export const fileDeleteNodeSchema: NodeSchema = {
  nodeType: "file-delete",
  schemaVersion: 1,
  schema: fileDeleteParamsSchema,
  params: {
    base: {
      label: "Base Folder",
      description:
        'Folder every deleted file must be under. A file outside it (including through ".." or a link) refuses the run, so nothing is deleted.',
      placeholder: "./build",
    },
  },
};
