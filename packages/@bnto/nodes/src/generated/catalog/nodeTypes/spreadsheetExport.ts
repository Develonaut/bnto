/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { NodeTypeInfo } from "../types";

/** Node type info for spreadsheet-export. */
export const spreadsheetExportNodeType: NodeTypeInfo = {
  name: "spreadsheet-export",
  label: "Export Items as CSV",
  description: "Write the files passing through, with their metadata, as one CSV row each.",
  category: "spreadsheet",
  isContainer: false,
  platforms: ["cli"] as const,
  icon: "table",
};
