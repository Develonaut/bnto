/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { ProcessorDef } from "../types";

/** Processor definition for vector-rasterize. */
export const vectorRasterizeProcessor: ProcessorDef = {
  nodeType: "vector-rasterize",
  name: "SVG to Image",
  description: "Convert SVG files to raster images (PNG, JPEG, WebP)",
  category: "vector",
  accepts: ["image/svg+xml"] as const,
  platforms: ["browser"] as const,
  parameters: [
  {
    name: "format",
    label: "Output Format",
    description: "The target raster format to convert SVG to",
    type: "enum" as const,
    options: [{"value":"png","label":"PNG"},{"value":"jpeg","label":"JPEG"},{"value":"webp","label":"WebP"}] as const,
    default: "png",
    constraints: { required: true },
  },
  {
    name: "quality",
    label: "Quality",
    description: "Output quality (1 = lowest, 100 = highest). Applies to JPEG; PNG is lossless; WebP is lossless-only.",
    type: "number" as const,
    default: 80,
    constraints: { min: 1, max: 100, required: false },
  },
  ],
  inputCardinality: "perFile" as const,
};
