/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { ProcessorDef } from "../types";

/** Processor definition for file-move. */
export const fileMoveProcessor: ProcessorDef = {
  nodeType: "file-move",
  name: "Move Files",
  description: "Move output files to a destination directory with conflict handling.",
  category: "file",
  accepts: [] as const,
  platforms: ["cli", "desktop"] as const,
  parameters: [
    {
      name: "destination",
      label: "Destination",
      description: "Directory path to move files into.",
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
      description: "What to do when a file with the same name already exists.",
      type: "enum" as const,
      options: ["skip", "overwrite", "rename"] as const,
      default: "skip",
    },
  ],
  inputCardinality: "perFile" as const,
};
