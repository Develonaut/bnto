/**
 * AUTO-GENERATED from engine/catalog.snapshot.json — DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { ProcessorDef } from "../types";

/** Processor definition for spreadsheet-convert. */
export const spreadsheetConvertProcessor: ProcessorDef = {
  nodeType: "spreadsheet-convert",
  name: "CSV to JSON",
  description: "Convert CSV rows to a JSON array of objects",
  category: "spreadsheet",
  accepts: ["text/csv"] as const,
  platforms: ["browser"] as const,
  parameters: [
  {
    name: "delimiter",
    label: "Delimiter",
    description: "Column separator character",
    type: "enum" as const,
    options: ["comma","semicolon","tab","pipe"] as const,
    default: "comma",
  },
  {
    name: "pretty",
    label: "Pretty Print",
    description: "Format output JSON with indentation",
    type: "boolean" as const,
    default: false,
  },
  ],
  inputCardinality: "perFile" as const,
};
