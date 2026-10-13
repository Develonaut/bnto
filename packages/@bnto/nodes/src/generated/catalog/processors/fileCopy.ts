/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { ProcessorDef } from "../types";

/** Processor definition for file-copy. */
export const fileCopyProcessor: ProcessorDef = {
  nodeType: "file-copy",
  name: "Copy Files",
  description: "Place output files in a destination directory with conflict handling.",
  category: "file",
  accepts: [] as const,
  platforms: ["cli", "desktop"] as const,
  parameters: [
    {
      name: "destination",
      label: "Destination",
      description: "Directory path to copy files into.",
      type: "string" as const,
      placeholder: "./output",
    },
    {
      name: "create_dirs",
      label: "Create Directories",
      description: "Automatically create the destination directory if it doesn't exist.",
      type: "boolean" as const,
      default: true,
    },
    {
      name: "conflict",
      label: "Conflict Resolution",
      description:
        'What to do when a file with the same name already exists. "newer" copies only when the destination is missing or older than the source (path input only), keeps the source\'s modified time, and never deletes.',
      type: "enum" as const,
      options: ["skip", "overwrite", "rename", "newer"] as const,
      default: "skip",
    },
    {
      name: "tolerance_ms",
      label: "Time Tolerance",
      description:
        'Modified-time gap treated as equal in "newer" mode; FAT and Google Drive round times.',
      type: "number" as const,
      default: 2000,
      constraints: { min: 0, required: false },
    },
  ],
  inputCardinality: "perFile" as const,
};
