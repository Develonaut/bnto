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
  description: "Copy output files into a destination folder, with conflict handling.",
  category: "file",
  accepts: [] as const,
  platforms: ["cli", "desktop"] as const,
  parameters: [
    {
      name: "destination",
      label: "Destination",
      description: "Folder to copy files into.",
      type: "string" as const,
      placeholder: "./output",
    },
    {
      name: "createDirs",
      label: "Create Folders",
      description: "Automatically create the destination folder if it doesn't exist.",
      type: "boolean" as const,
      default: true,
    },
    {
      name: "conflict",
      label: "Conflict Resolution",
      description:
        "What to do when a file with the same name already exists. \"newer\" copies only when the destination is missing or older than the source (path input only), keeps the source's modified time, and never deletes. A copy that would replace a file the run still reads (another incoming file's, another loop iteration's or another branch's) is refused before the step copies anything.",
      type: "enum" as const,
      options: ["skip", "overwrite", "rename", "newer"] as const,
      default: "skip",
    },
    {
      name: "toleranceMs",
      label: "Time Tolerance",
      description:
        'Modified-time gap treated as equal in "newer" mode; FAT and Google Drive round times.',
      type: "number" as const,
      default: 2000,
      constraints: { min: 0, required: false },
    },
  ],
  inputCardinality: "batch" as const,
};
