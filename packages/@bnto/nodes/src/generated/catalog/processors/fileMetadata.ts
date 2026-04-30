/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { ProcessorDef } from "../types";

/** Processor definition for file-metadata. */
export const fileMetadataProcessor: ProcessorDef = {
  nodeType: "file-metadata",
  name: "File Metadata",
  description:
    "Extract file metadata (size, extension, MIME type, hash) and attach it to the pipeline output.",
  category: "file",
  accepts: [] as const,
  platforms: ["browser", "cli", "desktop"] as const,
  parameters: [
    {
      name: "include_hash",
      label: "Include Hash",
      description: "Compute and include a SHA-256 hash of the file content.",
      type: "boolean" as const,
      default: false,
    },
  ],
  inputCardinality: "perFile" as const,
};
