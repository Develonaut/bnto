/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { NodeTypeInfo } from "../types";

/** Node type info for file-metadata. */
export const fileMetadataNodeType: NodeTypeInfo = {
  name: "file-metadata",
  label: "File Metadata",
  description:
    "Extract file metadata (folder, size, extension, MIME type, modified time, hash) and attach to output.",
  category: "file",
  isContainer: false,
  platforms: ["browser"] as const,
  icon: "file-text",
};
