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
  description: "Optimize SVG files by removing editor metadata, comments, and unnecessary elements",
  category: "vector",
  accepts: ["image/svg+xml"] as const,
  platforms: ["browser","native"] as const,
  parameters: [
  {
    name: "removeComments",
    label: "Remove Comments",
    description: "Strip XML comments from the SVG",
    type: "boolean" as const,
    default: true,
  },
  {
    name: "removeMetadata",
    label: "Remove Metadata",
    description: "Strip <metadata> elements (RDF, Dublin Core, etc.)",
    type: "boolean" as const,
    default: true,
  },
  {
    name: "collapseGroups",
    label: "Collapse Groups",
    description: "Remove empty groups and collapse single-child wrapper groups",
    type: "boolean" as const,
    default: true,
  },
  {
    name: "minify",
    label: "Minify",
    description: "Remove unnecessary whitespace and newlines",
    type: "boolean" as const,
    default: true,
  },
  {
    name: "precision",
    label: "Numeric Precision",
    description: "Decimal places for numeric values (reserved for Tier 2)",
    type: "number" as const,
    default: 3,
    constraints: { min: 0, max: 8, required: false },
  },
  ],
  inputCardinality: "perFile" as const,
};
