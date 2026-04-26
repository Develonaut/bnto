/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { NodeTypeInfo } from "../types";

/** Node type info for file-sanitize. */
export const fileSanitizeNodeType: NodeTypeInfo = {
  name: "file-sanitize",
  label: "Sanitize Filenames",
  description: "Clean filenames for web-safe or cross-platform use.",
  category: "file",
  isContainer: false,
  platforms: ["browser"] as const,
  icon: "folder-check",
};
