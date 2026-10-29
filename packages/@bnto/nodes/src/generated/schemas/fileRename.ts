/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchema } from "../../schemas/types";

/** Zod schema for file-rename node parameters. */
export const fileRenameParamsSchema = z.object({
  sanitize: z.enum(["slugify", "strip", "normalize"] as const).optional(),
  separator: z.string().optional().default("-"),
  maxLength: z.number().min(0).optional().default(0),
  find: z.string().optional(),
  replace: z.string().optional(),
  case: z.enum(["lower", "upper", "title"] as const).optional(),
  prefix: z.string().optional(),
  suffix: z.string().optional(),
  pattern: z.string().optional(),
  counterStart: z.number().min(0).optional().default(1),
  counterPad: z.number().min(0).max(10).optional().default(0),
  extension: z.string().optional(),
});

/** Inferred TypeScript type for file-rename node parameters. */
export type FileRenameParams = z.infer<typeof fileRenameParamsSchema>;

/** Full schema definition for the file-rename node type. */
export const fileRenameNodeSchema: NodeSchema = {
  nodeType: "file-rename",
  schemaVersion: 1,
  schema: fileRenameParamsSchema,
  params: {
    sanitize: {
      label: "Sanitize",
      description:
        'Clean the file name (not its folders) before other transforms. To fold folders into the name, replace "/" with "-" using Find in an earlier rename step',
    },
    separator: {
      label: "Separator",
      description:
        "Character to replace spaces and special characters in the file name (used with Sanitize)",
    },
    maxLength: {
      label: "Max Length",
      description: "Maximum file name length after sanitize, folders not counted (0 = no limit)",
    },
    find: {
      label: "Find",
      description:
        "Text or regex pattern to search for in the whole relative path, folders included",
    },
    replace: {
      label: "Replace",
      description: "Replacement text (used with Find)",
    },
    case: {
      label: "Case",
      description: "Transform the file name (not its folders) to a specific case",
    },
    prefix: {
      label: "Prefix",
      description: "Text to prepend to the file name (not its folders)",
    },
    suffix: {
      label: "Suffix",
      description: "Text to append to the file name, before its extension",
    },
    pattern: {
      label: "Pattern",
      description:
        "Template for the whole relative path, folders included (supports {{name}}, {{ext}}, {{index}}, {{date}}, {{counter}}). Replaces the other name transforms; without {{name}} it drops the folders",
      placeholder: "{{name}}-{{counter}}.{{ext}}",
    },
    counterStart: {
      label: "Counter Start",
      description: "Starting number for the {{counter}} variable",
    },
    counterPad: {
      label: "Counter Padding",
      description: "Zero-pad width for the counter (e.g., 3 → 001, 002)",
    },
    extension: {
      label: "Extension",
      description: "Replace the file extension (without dot)",
      placeholder: "png",
    },
  },
};
