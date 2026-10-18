/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { ProcessorDef } from "../types";

/** Processor definition for file-delete. */
export const fileDeleteProcessor: ProcessorDef = {
  nodeType: "file-delete",
  name: "Delete Files",
  description:
    "Delete the files flowing in, each under a base folder; missing files are skipped and counted, and a folder or a file outside the base refuses the run.",
  category: "file",
  accepts: [] as const,
  platforms: ["cli", "desktop"] as const,
  parameters: [
    {
      name: "base",
      label: "Base Folder",
      description:
        'Folder every deleted file must be under. A file outside it (including through ".." or a link) refuses the run, so nothing is deleted.',
      type: "string" as const,
      constraints: { required: true },
      placeholder: "./build",
    },
  ],
  inputCardinality: "batch" as const,
};
