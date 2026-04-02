/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { NodeTypeInfo } from "../types";

/** Node type info for image-convert. */
export const imageConvertNodeType: NodeTypeInfo = {
  name: "image-convert",
  label: "Convert Image Format",
  description: "Convert images between JPEG, PNG, and WebP formats.",
  category: "image",
  isContainer: false,
  platforms: ["browser"] as const,
  icon: "image",
};
