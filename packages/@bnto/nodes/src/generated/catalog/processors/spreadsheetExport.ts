/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { ProcessorDef } from "../types";

/** Processor definition for spreadsheet-export. */
export const spreadsheetExportProcessor: ProcessorDef = {
  nodeType: "spreadsheet-export",
  name: "Export Items as CSV",
  description: "Write the files passing through, with their metadata, as one CSV row each",
  category: "spreadsheet",
  accepts: [] as const,
  platforms: ["cli", "desktop"] as const,
  parameters: [
    {
      name: "path",
      label: "CSV Path",
      description: "File to write, one row per file passing through; its folder must exist",
      type: "string" as const,
      constraints: { required: true },
      placeholder: "items.csv",
    },
    {
      name: "name_column",
      label: "Name Column",
      description: "Header of the first column, which holds each file's name",
      type: "string" as const,
      default: "file",
    },
    {
      name: "overwrite",
      label: "Overwrite",
      description:
        "Replace a CSV already at the path; off keeps it (it may be an earlier run's record) and fails the run",
      type: "boolean" as const,
      default: false,
    },
  ],
  inputCardinality: "batch" as const,
};
