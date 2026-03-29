/**
 * AUTO-GENERATED from engine/catalog.snapshot.json — DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { ProcessorDef } from "../types";

/** Processor definition for image-watermark. */
export const imageWatermarkProcessor: ProcessorDef = {
  nodeType: "image-watermark",
  name: "Watermark Images",
  description: "Overlay a watermark image onto source images.",
  category: "image",
  accepts: ["image/jpeg","image/png","image/webp"] as const,
  platforms: ["browser"] as const,
  parameters: [
  {
    name: "watermark",
    label: "Watermark Image",
    description: "The watermark image to overlay (base64-encoded).",
    type: "file" as const,
    constraints: { required: true },
  },
  {
    name: "position",
    label: "Position",
    description: "Where to place the watermark on the image.",
    type: "enum" as const,
    options: ["top-left","top-center","top-right","middle-left","center","middle-right","bottom-left","bottom-center","bottom-right"] as const,
    default: "bottom-right",
  },
  {
    name: "size",
    label: "Size",
    description: "Watermark width as a percentage of the source image width.",
    type: "number" as const,
    default: 25,
    constraints: { min: 1, max: 100, required: false },
  },
  {
    name: "opacity",
    label: "Opacity",
    description: "Watermark transparency (0 = invisible, 100 = fully opaque).",
    type: "number" as const,
    default: 80,
    constraints: { min: 0, max: 100, required: false },
  },
  {
    name: "offsetX",
    label: "Offset X",
    description: "Horizontal offset from the edge in pixels.",
    type: "number" as const,
    default: 10,
    constraints: { min: 0, max: 500, required: false },
  },
  {
    name: "offsetY",
    label: "Offset Y",
    description: "Vertical offset from the edge in pixels.",
    type: "number" as const,
    default: 10,
    constraints: { min: 0, max: 500, required: false },
  },
  {
    name: "quality",
    label: "Quality",
    description: "Output quality (1 = lowest, 100 = highest). WebP is lossless-only; quality has no effect until lossy WebP support is added.",
    type: "number" as const,
    default: 80,
    constraints: { min: 1, max: 100, required: false },
  },
  ],
  inputCardinality: "perFile" as const,
};
