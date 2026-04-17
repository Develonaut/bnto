/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { ProcessorDef } from "../types";

/** Processor definition for image-strip-exif. */
export const imageStripExifProcessor: ProcessorDef = {
  nodeType: "image-strip-exif",
  name: "Strip EXIF",
  description: "Remove all EXIF metadata from images (GPS, camera info, timestamps)",
  category: "image",
  accepts: ["image/jpeg", "image/png", "image/webp"] as const,
  platforms: ["browser"] as const,
  parameters: [
    {
      name: "quality",
      label: "Quality",
      description:
        "Output quality (1 = lowest, 100 = highest). WebP is lossless-only; quality has no effect until lossy WebP support is added.",
      type: "number" as const,
      default: 80,
      constraints: { min: 1, max: 100, required: false },
    },
  ],
  inputCardinality: "perFile" as const,
};
