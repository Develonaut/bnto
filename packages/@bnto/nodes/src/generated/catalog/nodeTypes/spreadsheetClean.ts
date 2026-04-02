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
  description: "Remove empty rows, trim whitespace, and deduplicate CSV data.",
  category: "spreadsheet",
  isContainer: false,
  platforms: ["browser"] as const,
  icon: "sheet",
};
