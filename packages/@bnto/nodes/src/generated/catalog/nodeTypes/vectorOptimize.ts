/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { NodeTypeInfo } from "../types";

/** Node type info for vector-optimize. */
export const vectorOptimizeNodeType: NodeTypeInfo = {
  name: "vector-optimize",
  label: "Optimize SVG",
  description: "Remove editor metadata, comments, and unnecessary elements from SVG files.",
  category: "vector",
  isContainer: false,
  platforms: ["browser"] as const,
  icon: "file-minus-2",
};
