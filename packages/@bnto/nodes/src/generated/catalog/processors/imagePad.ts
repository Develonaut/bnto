/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { ProcessorDef } from "../types";

/** Processor definition for image-pad. */
export const imagePadProcessor: ProcessorDef = {
  nodeType: "image-pad",
  name: "Pad Images",
  description:
    "Pad images onto a larger, centered canvas, filling the added space with transparency or a color.",
  category: "image",
  accepts: ["image/jpeg", "image/png", "image/webp"] as const,
  platforms: ["browser"] as const,
  parameters: [
    {
      name: "width",
      label: "Width",
      description: "Canvas width in pixels. Leave empty to keep the image's width.",
      type: "number" as const,
      constraints: { min: 1, max: 16384, required: false },
    },
    {
      name: "height",
      label: "Height",
      description: "Canvas height in pixels. Leave empty to keep the image's height.",
      type: "number" as const,
      constraints: { min: 1, max: 16384, required: false },
    },
    {
      name: "background",
      label: "Background",
      description:
        'Fill for the added space: "transparent" or a hex color (#fff, #ffffff, #ffffff80). JPEG has no transparency, so a transparent fill comes out white.',
      type: "string" as const,
      default: "transparent",
      placeholder: "#ffffff",
    },
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
