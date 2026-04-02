/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { NodeTypeInfo } from "../types";

/** Node type info for spreadsheet-merge. */
export const spreadsheetMergeNodeType: NodeTypeInfo = {
  name: "spreadsheet-merge",
  label: "Merge CSV",
  description: "Combine multiple CSV files into one with header reconciliation and deduplication.",
  category: "spreadsheet",
  isContainer: false,
  platforms: ["browser"] as const,
  icon: "sheet",
};
