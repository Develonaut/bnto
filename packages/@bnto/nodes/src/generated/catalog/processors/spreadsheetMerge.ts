/**
 * AUTO-GENERATED from engine/catalog.snapshot.json — DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { ProcessorDef } from "../types";

/** Processor definition for spreadsheet-merge. */
export const spreadsheetMergeProcessor: ProcessorDef = {
  nodeType: "spreadsheet-merge",
  name: "Merge CSV",
  description: "Combine multiple CSV files into one, with header reconciliation and deduplication.",
  category: "spreadsheet",
  accepts: ["text/csv"] as const,
  platforms: ["browser"] as const,
  parameters: [
  {
    name: "headerHandling",
    label: "Header Handling",
    description: "How to reconcile headers across files",
    type: "enum" as const,
    options: ["first-file","union"] as const,
    default: "first-file",
  },
  {
    name: "deduplicate",
    label: "Remove Duplicates",
    description: "Remove duplicate rows across all files",
    type: "boolean" as const,
    default: false,
  },
  ],
  inputCardinality: "batch" as const,
};
