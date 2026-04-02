/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { NodeTypeInfo } from "../types";

/** Node type info for image-overlay. */
export const imageOverlayNodeType: NodeTypeInfo = {
  name: "image-overlay",
  label: "Overlay Image",
  description: "Overlay an image onto source images at a configurable position, size, and opacity.",
  category: "image",
  isContainer: false,
  platforms: ["browser"] as const,
  icon: "stamp",
};
