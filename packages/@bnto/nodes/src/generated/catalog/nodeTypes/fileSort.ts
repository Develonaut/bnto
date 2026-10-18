/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { NodeTypeInfo } from "../types";

/** Node type info for file-sort. */
export const fileSortNodeType: NodeTypeInfo = {
  name: "file-sort",
  label: "Sort Files",
  description: "Reorder items by one or more metadata fields, each ascending or descending.",
  category: "file",
  isContainer: false,
  platforms: ["browser"] as const,
  icon: "arrow-up-down",
};
