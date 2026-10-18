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
    "Extract file metadata (folder, size, extension, MIME type, modified time, hash) and attach it to the pipeline output. `folder` is the item's folder in its relative name, `/`-separated (`a/b/c.mp4` gives `a/b`), empty at the top level.",
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
