/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { NodeTypeInfo } from "../types";

/** Node type info for spreadsheet-rename. */
export const spreadsheetRenameNodeType: NodeTypeInfo = {
  name: "spreadsheet-rename",
  label: "Rename CSV Columns",
  description: "Rename column headers in a CSV file.",
  category: "spreadsheet",
  isContainer: false,
  platforms: ["browser"] as const,
  icon: "sheet",
};
