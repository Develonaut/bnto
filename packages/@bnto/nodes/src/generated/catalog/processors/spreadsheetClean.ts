/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { ProcessorDef } from "../types";

/** Processor definition for spreadsheet-clean. */
export const spreadsheetCleanProcessor: ProcessorDef = {
  nodeType: "spreadsheet-clean",
  name: "Clean CSV",
  description: "Remove empty rows, trim whitespace, and deduplicate CSV data",
  category: "spreadsheet",
  accepts: ["text/csv"] as const,
  platforms: ["browser"] as const,
  parameters: [
  {
    name: "trimWhitespace",
    label: "Trim Whitespace",
    description: "Remove leading and trailing whitespace from every cell",
    type: "boolean" as const,
    default: true,
  },
  {
    name: "removeEmptyRows",
    label: "Remove Empty Rows",
    description: "Skip rows where every cell is blank",
    type: "boolean" as const,
    default: true,
  },
  {
    name: "removeDuplicates",
    label: "Remove Duplicates",
    description: "Remove duplicate rows, keeping the first occurrence",
    type: "boolean" as const,
    default: true,
  },
  ],
  inputCardinality: "perFile" as const,
};
