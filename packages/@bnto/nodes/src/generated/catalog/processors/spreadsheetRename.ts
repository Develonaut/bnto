/**
 * AUTO-GENERATED from engine/catalog.snapshot.json — DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { ProcessorDef } from "../types";

/** Processor definition for spreadsheet-rename. */
export const spreadsheetRenameProcessor: ProcessorDef = {
  nodeType: "spreadsheet-rename",
  name: "Rename CSV Columns",
  description: "Rename column headers in a CSV file",
  category: "spreadsheet",
  accepts: ["text/csv"] as const,
  platforms: ["browser"] as const,
  parameters: [
  {
    name: "columns",
    label: "Column Mapping",
    description: "Map of old column names to new names (e.g., {\"Name\": \"full_name\"})",
    type: "object" as const,
  },
  ],
  inputCardinality: "perFile" as const,
};
