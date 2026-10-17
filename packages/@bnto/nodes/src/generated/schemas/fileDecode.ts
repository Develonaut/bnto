/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchema } from "../../schemas/types";

/** Zod schema for file-decode node parameters. */
export const fileDecodeParamsSchema = z.object({
  encoding: z
    .enum(["base64"] as const)
    .optional()
    .default("base64"),
  filename: z.string().optional(),
});

/** Inferred TypeScript type for file-decode node parameters. */
export type FileDecodeParams = z.infer<typeof fileDecodeParamsSchema>;

/** Full schema definition for the file-decode node type. */
export const fileDecodeNodeSchema: NodeSchema = {
  nodeType: "file-decode",
  schemaVersion: 1,
  schema: fileDecodeParamsSchema,
  params: {
    encoding: {
      label: "Encoding",
      description:
        "The text encoding each file is in. base64 is the standard alphabet (+ and /), padded or not; whitespace and line breaks are ignored. URL-safe base64 (- and _) is not accepted.",
    },
    filename: {
      label: "Output Filename",
      description:
        "Name for each decoded file. {{name}} is the input's name without its last extension and {{ext}} that extension, e.g. {{name}}.json. Leave empty to keep the input's name.",
      placeholder: "{{name}}.json",
    },
  },
};
