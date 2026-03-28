/**
 * AUTO-GENERATED from engine/catalog.snapshot.json — DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { NodeTypeInfo } from "../types";

/** Node type info for image-strip-exif. */
export const imageStripExifNodeType: NodeTypeInfo = {
  name: "image-strip-exif",
  label: "Strip EXIF",
  description: "Remove all EXIF metadata from images (GPS, camera info, timestamps).",
  category: "image",
  isContainer: false,
  browserCapable: true,
  icon: "image",
};
