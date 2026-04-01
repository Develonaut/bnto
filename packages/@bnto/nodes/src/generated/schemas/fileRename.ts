/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchema } from "../../schemas/types";

/** Zod schema for file-rename node parameters. */
export const fileRenameParamsSchema = z.object({
  find: z.string().optional(),
  replace: z.string().optional(),
  case: z.enum(["lower", "upper", "title"] as const).optional(),
  prefix: z.string().optional(),
  suffix: z.string().optional(),
  pattern: z.string().optional(),
});

/** Inferred TypeScript type for file-rename node parameters. */
export type FileRenameParams = z.infer<typeof fileRenameParamsSchema>;

/** Full schema definition for the file-rename node type. */
export const fileRenameNodeSchema: NodeSchema = {
  nodeType: "file-rename",
  schemaVersion: 1,
  schema: fileRenameParamsSchema,
  params: {
    find: {
      label: "Find",
      description: "Text or regex pattern to search for in the filename",
    },
    replace: {
      label: "Replace",
      description: "Replacement text (used with Find)",
    },
    case: {
      label: "Case",
      description: "Transform the filename to a specific case",
    },
    prefix: {
      label: "Prefix",
      description: "Text to prepend to the filename",
    },
    suffix: {
      label: "Suffix",
      description: "Text to append before the file extension",
    },
    pattern: {
      label: "Pattern",
      description:
        "Template for the output filename (supports {{name}}, {{ext}}, {{index}}, {{date}})",
      placeholder: "{{name}}-compressed.{{ext}}",
    },
  },
};
