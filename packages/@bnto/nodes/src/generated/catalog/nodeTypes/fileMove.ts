/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { NodeTypeInfo } from "../types";

/** Node type info for file-move. */
export const fileMoveNodeType: NodeTypeInfo = {
  name: "file-move",
  label: "Move Files",
  description: "Move files into folders under a destination, by the path in each file's name.",
  category: "file",
  isContainer: false,
  platforms: ["cli"] as const,
  icon: "folder-open",
};
