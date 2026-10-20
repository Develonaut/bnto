/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { NodeTypeInfo } from "../types";

/** Node type info for image-pad. */
export const imagePadNodeType: NodeTypeInfo = {
  name: "image-pad",
  label: "Pad Images",
  description:
    "Center images on a larger canvas, filling the added space with transparency or a color.",
  category: "image",
  isContainer: false,
  platforms: ["browser"] as const,
  icon: "expand",
};
