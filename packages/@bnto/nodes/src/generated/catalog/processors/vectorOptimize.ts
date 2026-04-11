/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { ProcessorDef } from "../types";

/** Processor definition for vector-optimize. */
export const vectorOptimizeProcessor: ProcessorDef = {
  nodeType: "vector-optimize",
  name: "Optimize SVG",
  description: "Reduce SVG file size by removing unnecessary data and optimizing paths",
  category: "vector",
  accepts: ["image/svg+xml"] as const,
  platforms: ["browser"] as const,
  parameters: [
    {
      name: "precision",
      label: "Numeric Precision",
      description: "Decimal places for numeric values in paths and transforms (1-10)",
      type: "number" as const,
      default: 3,
      constraints: { min: 1, max: 10, required: false },
    },
    {
      name: "removeComments",
      label: "Remove Comments",
      description: "Strip XML comments",
      type: "boolean" as const,
      default: true,
    },
    {
      name: "removeMetadata",
      label: "Remove Metadata",
      description: "Strip <metadata> elements",
      type: "boolean" as const,
      default: true,
    },
    {
      name: "collapseGroups",
      label: "Collapse Groups",
      description: "Merge redundant nested <g> elements",
      type: "boolean" as const,
      default: true,
    },
    {
      name: "minify",
      label: "Minify",
      description: "Remove unnecessary whitespace and line breaks",
      type: "boolean" as const,
      default: true,
    },
  ],
  inputCardinality: "perFile" as const,
};
