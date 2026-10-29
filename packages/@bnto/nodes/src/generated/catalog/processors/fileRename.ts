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
  description: "Rename files using patterns, find/replace, case rules, and counters",
  category: "file",
  accepts: [] as const,
  platforms: ["browser"] as const,
  parameters: [
    {
      name: "sanitize",
      label: "Sanitize",
      description:
        'Clean the file name (not its folders) before other transforms. To fold folders into the name, replace "/" with "-" using Find in an earlier rename step',
      type: "enum" as const,
      options: ["slugify", "strip", "normalize"] as const,
    },
    {
      name: "separator",
      label: "Separator",
      description:
        "Character to replace spaces and special characters in the file name (used with Sanitize)",
      type: "string" as const,
      default: "-",
    },
    {
      name: "maxLength",
      label: "Max Length",
      description: "Maximum file name length after sanitize, folders not counted (0 = no limit)",
      type: "number" as const,
      default: 0,
      constraints: { min: 0, required: false },
    },
    {
      name: "find",
      label: "Find",
      description:
        "Text or regex pattern to search for in the whole relative path, folders included",
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
      description: "Transform the file name (not its folders) to a specific case",
      type: "enum" as const,
      options: ["lower", "upper", "title"] as const,
    },
    {
      name: "prefix",
      label: "Prefix",
      description: "Text to prepend to the file name (not its folders)",
      type: "string" as const,
    },
    {
      name: "suffix",
      label: "Suffix",
      description: "Text to append to the file name, before its extension",
      type: "string" as const,
    },
    {
      name: "pattern",
      label: "Pattern",
      description:
        "Template for the whole relative path, folders included (supports {{name}}, {{ext}}, {{index}}, {{date}}, {{counter}}). Replaces the other name transforms; without {{name}} it drops the folders",
      type: "string" as const,
      placeholder: "{{name}}-{{counter}}.{{ext}}",
    },
    {
      name: "counterStart",
      label: "Counter Start",
      description: "Starting number for the {{counter}} variable",
      type: "number" as const,
      default: 1,
      constraints: { min: 0, required: false },
    },
    {
      name: "counterPad",
      label: "Counter Padding",
      description: "Zero-pad width for the counter (e.g., 3 → 001, 002)",
      type: "number" as const,
      default: 0,
      constraints: { min: 0, max: 10, required: false },
    },
    {
      name: "extension",
      label: "Extension",
      description: "Replace the file extension (without dot)",
      type: "string" as const,
      placeholder: "png",
    },
  ],
  inputCardinality: "perFile" as const,
};
