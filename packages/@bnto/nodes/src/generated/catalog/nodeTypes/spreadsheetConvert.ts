/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { NodeTypeInfo } from "../types";

/** Node type info for spreadsheet-convert. */
export const spreadsheetConvertNodeType: NodeTypeInfo = {
  name: "spreadsheet-convert",
  label: "CSV to JSON",
  description: "Convert CSV data to JSON format with configurable delimiters.",
  category: "spreadsheet",
  isContainer: false,
  platforms: ["browser"] as const,
  icon: "sheet",
};
