/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { ProcessorDef } from "../types";

/** Processor definition for file-rename. */
export const fileRenameProcessor: ProcessorDef = {
  nodeType: "file-rename",
  name: "Rename Files",
  description: "Transform filenames using patterns, find/replace, and case rules",
  category: "file",
  accepts: [] as const,
  platforms: ["browser"] as const,
  parameters: [
    {
      name: "find",
      label: "Find",
      description: "Text or regex pattern to search for in the filename",
      type: "string" as const,
    },
    {
      name: "replace",
      label: "Replace",
      description: "Replacement text (used with Find)",
      type: "string" as const,
    },
    {
      name: "case",
      label: "Case",
      description: "Transform the filename to a specific case",
      type: "enum" as const,
      options: ["lower", "upper", "title"] as const,
    },
    {
      name: "prefix",
      label: "Prefix",
      description: "Text to prepend to the filename",
      type: "string" as const,
    },
    {
      name: "suffix",
      label: "Suffix",
      description: "Text to append before the file extension",
      type: "string" as const,
    },
    {
      name: "pattern",
      label: "Pattern",
      description:
        "Template for the output filename (supports {{name}}, {{ext}}, {{index}}, {{date}})",
      type: "string" as const,
      placeholder: "{{name}}-compressed.{{ext}}",
    },
  ],
  inputCardinality: "perFile" as const,
};
