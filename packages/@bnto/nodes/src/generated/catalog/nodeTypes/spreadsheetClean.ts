/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { NodeTypeInfo } from "../types";

/** Node type info for spreadsheet-clean. */
export const spreadsheetCleanNodeType: NodeTypeInfo = {
  name: "spreadsheet-clean",
  label: "Clean CSV",
  description: "Clean CSV data by removing empty rows, trimming whitespace, and deduplicating.",
  category: "spreadsheet",
  isContainer: false,
  platforms: ["browser"] as const,
  icon: "sheet",
};
