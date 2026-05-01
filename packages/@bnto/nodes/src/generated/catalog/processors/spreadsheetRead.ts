/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { ProcessorDef } from "../types";

/** Processor definition for spreadsheet-read. */
export const spreadsheetReadProcessor: ProcessorDef = {
  nodeType: "spreadsheet-read",
  name: "Read CSV Rows",
  description: "Explode a CSV into one item per row for loop iteration",
  category: "spreadsheet",
  accepts: ["text/csv"] as const,
  platforms: ["browser"] as const,
  parameters: [
    {
      name: "hasHeaders",
      label: "Has Headers",
      description: "First row contains column headers",
      type: "boolean" as const,
      default: true,
    },
    {
      name: "delimiter",
      label: "Delimiter",
      description: "Column separator character",
      type: "enum" as const,
      options: ["comma", "semicolon", "tab", "pipe"] as const,
      default: "comma",
    },
    {
      name: "maxRows",
      label: "Max Rows",
      description: "Maximum rows to process. Error if CSV exceeds this limit.",
      type: "number" as const,
      default: 100000,
      constraints: { min: 1, max: 10000000, required: false },
    },
  ],
  inputCardinality: "perFile" as const,
};
