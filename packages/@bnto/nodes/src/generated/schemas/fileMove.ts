/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchema } from "../../schemas/types";

/** Zod schema for file-move node parameters. */
export const fileMoveParamsSchema = z.object({
  destination: z.string(),
  create_dirs: z.boolean().optional().default(true),
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
      description:
        'Folder to move files under. Each file\'s name is its path inside it (e.g. Pistol/Slide/shot.wav); absolute paths and ".." are refused.',
      placeholder: "./sorted",
    },
    create_dirs: {
      label: "Create Folders",
      description:
        "Create missing folders under the destination. Off refuses the run when a folder is missing.",
    },
  },
};
