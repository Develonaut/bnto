/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { NodeTypeInfo } from "../types";

/** Node type info for image-resize. */
export const imageResizeNodeType: NodeTypeInfo = {
  name: "image-resize",
  label: "Resize Images",
  description: "Change image dimensions while maintaining quality.",
  category: "image",
  isContainer: false,
  platforms: ["browser"] as const,
  icon: "image",
};
