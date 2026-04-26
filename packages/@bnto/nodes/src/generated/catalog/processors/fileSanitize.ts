/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { ProcessorDef } from "../types";

/** Processor definition for file-sanitize. */
export const fileSanitizeProcessor: ProcessorDef = {
  nodeType: "file-sanitize",
  name: "Sanitize Filenames",
  description: "Clean filenames for web-safe or cross-platform use",
  category: "file",
  accepts: [] as const,
  platforms: ["browser"] as const,
  parameters: [
    {
      name: "mode",
      label: "Mode",
      description: "Sanitization strategy",
      type: "enum" as const,
      options: ["slugify", "strip", "normalize"] as const,
      default: "slugify",
    },
    {
      name: "separator",
      label: "Separator",
      description: "Character to replace spaces and special characters",
      type: "string" as const,
      default: "-",
    },
    {
      name: "max_length",
      label: "Max Length",
      description: "Maximum filename length (0 = no limit)",
      type: "number" as const,
      default: 0,
      constraints: { min: 0, required: false },
    },
  ],
  inputCardinality: "perFile" as const,
};
