/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { ProcessorDef } from "../types";

/** Processor definition for file-collect. */
export const fileCollectProcessor: ProcessorDef = {
  nodeType: "file-collect",
  name: "Collect Files",
  description: "Traverse a directory and collect files matching a glob pattern into the pipeline.",
  category: "file",
  accepts: [] as const,
  platforms: ["cli", "desktop"] as const,
  parameters: [
    {
      name: "pattern",
      label: "Pattern",
      description:
        'Glob pattern to match files (e.g., "*.jpg", "**/*.svg"). Default: "*" (all files).',
      type: "string" as const,
      default: "*",
      placeholder: "*.jpg",
    },
    {
      name: "recursive",
      label: "Recursive",
      description: "Traverse subdirectories when collecting files.",
      type: "boolean" as const,
      default: true,
    },
    {
      name: "flatten",
      label: "Flatten",
      description:
        "Strip directory structure from output filenames. When true, all files appear as if in the same directory.",
      type: "boolean" as const,
      default: true,
    },
  ],
  inputCardinality: "perFile" as const,
};
