/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchema } from "../../schemas/types";

/** Zod schema for file-collect node parameters. */
export const fileCollectParamsSchema = z.object({
  pattern: z.string().optional().default("*"),
  recursive: z.boolean().optional().default(true),
  flatten: z.boolean().optional().default(true),
});

/** Inferred TypeScript type for file-collect node parameters. */
export type FileCollectParams = z.infer<typeof fileCollectParamsSchema>;

/** Full schema definition for the file-collect node type. */
export const fileCollectNodeSchema: NodeSchema = {
  nodeType: "file-collect",
  schemaVersion: 1,
  schema: fileCollectParamsSchema,
  params: {
    pattern: {
      label: "Pattern",
      description:
        'Glob pattern to match files (e.g., "*.jpg", "**/*.svg"). Default: "*" (all files).',
      placeholder: "*.jpg",
    },
    recursive: {
      label: "Recursive",
      description: "Traverse subdirectories when collecting files.",
    },
    flatten: {
      label: "Flatten",
      description:
        "Strip directory structure from output filenames. When true, all files appear as if in the same directory.",
    },
  },
};
