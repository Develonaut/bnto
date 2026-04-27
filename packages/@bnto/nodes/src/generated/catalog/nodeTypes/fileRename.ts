/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { NodeTypeInfo } from "../types";

/** Node type info for file-rename. */
export const fileRenameNodeType: NodeTypeInfo = {
  name: "file-rename",
  label: "Rename Files",
  description:
    "Transform filenames using patterns, find/replace, case rules, counters, and sanitization.",
  category: "file",
  isContainer: false,
  platforms: ["browser"] as const,
  icon: "folder-open",
};
