/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { ProcessorDef } from "../types";

/** Processor definition for spreadsheet-read. */
export const spreadsheetReadProcessor: ProcessorDef = {
  nodeType: "spreadsheet-read",
  name: "Read Rows",
  description:
    "Explode a CSV or a JSON array into one item per row for loop iteration. Nested JSON fields read as dotted keys (meta.id)",
  category: "spreadsheet",
  accepts: ["text/csv", "application/json"] as const,
  platforms: ["browser"] as const,
  parameters: [
    {
      name: "format",
      label: "Format",
      description: "How the file is read: auto picks JSON for a .json file and CSV otherwise",
      type: "enum" as const,
      options: ["auto", "csv", "json"] as const,
      default: "auto",
    },
    {
      name: "hasHeaders",
      label: "Has Headers",
      description: "First row contains column headers (CSV only)",
      type: "boolean" as const,
      default: true,
    },
    {
      name: "delimiter",
      label: "Delimiter",
      description: "Column separator character (CSV only)",
      type: "enum" as const,
      options: ["comma", "semicolon", "tab", "pipe"] as const,
      default: "comma",
    },
    {
      name: "maxRows",
      label: "Max Rows",
      description:
        "Maximum rows (CSV rows or JSON array elements) to process. Error if the file has more.",
      type: "number" as const,
      default: 100000,
      constraints: { min: 1, max: 10000000, required: false },
    },
  ],
  inputCardinality: "perFile" as const,
};
