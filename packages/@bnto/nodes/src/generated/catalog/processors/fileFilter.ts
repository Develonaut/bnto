/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { ProcessorDef } from "../types";

/** Processor definition for file-filter. */
export const fileFilterProcessor: ProcessorDef = {
  nodeType: "file-filter",
  name: "Filter Files",
  description:
    "Keep or drop files by extension, name or path pattern, and size. Files that match every criterion are kept (or dropped, with keep off).",
  category: "file",
  accepts: [] as const,
  platforms: ["browser", "cli"] as const,
  parameters: [
    {
      name: "extensions",
      label: "Extensions",
      description:
        'Comma-separated list of file extensions to keep (e.g., "jpg,png,svg"). Leave empty to allow all extensions.',
      type: "string" as const,
      placeholder: "jpg,png,svg",
    },
    {
      name: "name_pattern",
      label: "Name Pattern",
      description:
        'Glob pattern to match against filenames (e.g., "photo*", "*.backup.*"). Uses glob syntax with * and ? wildcards.',
      type: "string" as const,
      placeholder: "photo*",
    },
    {
      name: "pattern_mode",
      label: "Pattern Mode",
      description: "How to interpret the name pattern.",
      type: "enum" as const,
      options: ["glob", "regex"] as const,
      default: "glob",
    },
    {
      name: "match",
      label: "Match Against",
      description:
        "What the name pattern is tested against: the file's name, or its whole relative path (folders separated by /).",
      type: "enum" as const,
      options: ["name", "path"] as const,
      default: "name",
    },
    {
      name: "keep",
      label: "Keep Matches",
      description:
        "When on, files matching every criterion are kept and the rest dropped. When off, matching files are dropped and the rest kept.",
      type: "boolean" as const,
      default: true,
    },
    {
      name: "min_size",
      label: "Minimum Size",
      description: "Minimum file size in bytes. Files smaller than this are dropped.",
      type: "number" as const,
      default: 0,
      constraints: { min: 0, required: false },
    },
    {
      name: "max_size",
      label: "Maximum Size",
      description:
        "Maximum file size in bytes. Files larger than this are dropped. 0 means no limit.",
      type: "number" as const,
      default: 0,
      constraints: { min: 0, required: false },
    },
  ],
  inputCardinality: "perFile" as const,
};
