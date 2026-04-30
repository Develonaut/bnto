/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { NodeTypeInfo } from "../types";

/** Node type info for file-collect. */
export const fileCollectNodeType: NodeTypeInfo = {
  name: "file-collect",
  label: "Collect Files",
  description: "Traverse a directory and collect files matching a glob pattern into the pipeline.",
  category: "file",
  isContainer: false,
  platforms: ["cli"] as const,
  icon: "folder-search",
};
