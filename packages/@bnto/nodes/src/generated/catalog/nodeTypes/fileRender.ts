/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { NodeTypeInfo } from "../types";

/** Node type info for file-render. */
export const fileRenderNodeType: NodeTypeInfo = {
  name: "file-render",
  label: "Render Text File",
  description: "Render one text file from a template: a header, one row per item, then a footer.",
  category: "file",
  isContainer: false,
  platforms: ["browser"] as const,
  icon: "file-text",
};
