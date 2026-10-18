/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchema } from "../../schemas/types";

/** Zod schema for file-join node parameters. */
export const fileJoinParamsSchema = z.object({
  filename: z.string(),
  separator: z.string().optional().default(""),
});

/** Inferred TypeScript type for file-join node parameters. */
export type FileJoinParams = z.infer<typeof fileJoinParamsSchema>;

/** Full schema definition for the file-join node type. */
export const fileJoinNodeSchema: NodeSchema = {
  nodeType: "file-join",
  schemaVersion: 1,
  schema: fileJoinParamsSchema,
  params: {
    filename: {
      label: "File Name",
      description:
        "Name of the joined file, e.g. README.md; its MIME type follows the extension. Reads run templates such as {{vars.X}}.",
      placeholder: "joined.txt",
    },
    separator: {
      label: "Separator",
      description:
        "Put between files, as written (a newline, a blank line, ---). Not before the first or after the last. Empty joins them directly.",
    },
  },
};
