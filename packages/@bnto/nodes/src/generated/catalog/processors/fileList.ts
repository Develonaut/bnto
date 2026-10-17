/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { ProcessorDef } from "../types";

/** Processor definition for file-list. */
export const fileListProcessor: ProcessorDef = {
  nodeType: "file-list",
  name: "List Files",
  description:
    "Turn lists of paths into files: each listed path under a base folder that exists joins the pipeline; missing ones are skipped and counted. `entries` lists every path in order with its full path and status.",
  category: "file",
  accepts: [] as const,
  platforms: ["cli", "desktop"] as const,
  parameters: [
    {
      name: "paths",
      label: "Paths",
      description:
        "Paths to list, relative to the base folder, added to any read from input files (one per line, or NUL-separated).",
      type: "object" as const,
    },
    {
      name: "base",
      label: "Base Folder",
      description:
        'Folder the listed paths are relative to. Defaults to the working directory; absolute paths and ".." are refused.',
      type: "string" as const,
      placeholder: ".",
    },
  ],
  inputCardinality: "batch" as const,
};
