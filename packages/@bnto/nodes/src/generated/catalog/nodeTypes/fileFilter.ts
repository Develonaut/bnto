/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { NodeTypeInfo } from "../types";

/** Node type info for file-filter. */
export const fileFilterNodeType: NodeTypeInfo = {
  name: "file-filter",
  label: "Filter Files",
  description: "Drop files that don't match extension, pattern, or size criteria.",
  category: "file",
  isContainer: false,
  platforms: ["browser"] as const,
  icon: "filter",
};
