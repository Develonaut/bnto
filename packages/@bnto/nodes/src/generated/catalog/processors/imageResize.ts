/**
 * AUTO-GENERATED from engine/catalog.snapshot.json — DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { ProcessorDef } from "../types";

/** Processor definition for image-resize. */
export const imageResizeProcessor: ProcessorDef = {
  nodeType: "image-resize",
  name: "Resize Images",
  description: "Change image dimensions while maintaining quality",
  category: "image",
  accepts: ["image/jpeg","image/png","image/webp"] as const,
  platforms: ["browser"] as const,
  parameters: [
  {
    name: "width",
    label: "Width",
    description: "Target width in pixels",
    type: "number" as const,
    constraints: { min: 1, required: false },
  },
  {
    name: "height",
    label: "Height",
    description: "Target height in pixels",
    type: "number" as const,
    constraints: { min: 1, required: false },
  },
  {
    name: "maintainAspect",
    label: "Maintain Aspect Ratio",
    description: "Keep the original width-to-height ratio when resizing",
    type: "boolean" as const,
    default: true,
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
