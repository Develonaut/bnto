/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { NodeTypeInfo } from "../types";

/** Node type info for file-inspect. */
export const fileInspectNodeType: NodeTypeInfo = {
  name: "file-inspect",
  label: "Inspect Files",
  description:
    "Inspect files and attach their metadata (folder, size, extension, MIME type, modified time, hash) to the output.",
  category: "file",
  isContainer: false,
  platforms: ["browser"] as const,
  icon: "file-text",
};
