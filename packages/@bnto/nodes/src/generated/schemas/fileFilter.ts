/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import { z } from "zod";
import type { NodeSchema } from "../../schemas/types";

/** Zod schema for file-filter node parameters. */
export const fileFilterParamsSchema = z.object({
  extensions: z.string().optional(),
  name_pattern: z.string().optional(),
  pattern_mode: z
    .enum(["glob", "regex"] as const)
    .optional()
    .default("glob"),
  match: z
    .enum(["name", "path"] as const)
    .optional()
    .default("name"),
  keep: z.boolean().optional().default(true),
  min_size: z.number().min(0).optional().default(0),
  max_size: z.number().min(0).optional().default(0),
});

/** Inferred TypeScript type for file-filter node parameters. */
export type FileFilterParams = z.infer<typeof fileFilterParamsSchema>;

/** Full schema definition for the file-filter node type. */
export const fileFilterNodeSchema: NodeSchema = {
  nodeType: "file-filter",
  schemaVersion: 1,
  schema: fileFilterParamsSchema,
  params: {
    extensions: {
      label: "Extensions",
      description:
        'Comma-separated list of file extensions to keep (e.g., "jpg,png,svg"). Leave empty to allow all extensions.',
      placeholder: "jpg,png,svg",
    },
    name_pattern: {
      label: "Name Pattern",
      description:
        'Glob pattern to match against filenames (e.g., "photo*", "*.backup.*"). Uses glob syntax with * and ? wildcards.',
      placeholder: "photo*",
    },
    pattern_mode: {
      label: "Pattern Mode",
      description: "How to interpret the name pattern.",
    },
    match: {
      label: "Match Against",
      description:
        "What the name pattern is tested against: the file's name, or its whole relative path (folders separated by /).",
    },
    keep: {
      label: "Keep Matches",
      description:
        "When on, files matching every criterion are kept and the rest dropped. When off, matching files are dropped and the rest kept.",
    },
    min_size: {
      label: "Minimum Size",
      description: "Minimum file size in bytes. Files smaller than this are dropped.",
    },
    max_size: {
      label: "Maximum Size",
      description:
        "Maximum file size in bytes. Files larger than this are dropped. 0 means no limit.",
    },
  },
};
