/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { NodeTypeInfo } from "../types";

/** Node type info for file-copy. */
export const fileCopyNodeType: NodeTypeInfo = {
  name: "file-copy",
  label: "Copy Files",
  description: "Place output files in a destination directory with conflict handling.",
  category: "file",
  isContainer: false,
  platforms: ["cli"] as const,
  icon: "copy",
};
