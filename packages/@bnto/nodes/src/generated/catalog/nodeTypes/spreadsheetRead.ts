/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { NodeTypeInfo } from "../types";

/** Node type info for spreadsheet-read. */
export const spreadsheetReadNodeType: NodeTypeInfo = {
  name: "spreadsheet-read",
  label: "Read CSV Rows",
  description: "Explode a CSV into one item per row for loop iteration.",
  category: "spreadsheet",
  isContainer: false,
  platforms: ["browser"] as const,
  icon: "table",
};
