/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { NodeTypeInfo } from "../types";

/** Node type info for file-decode. */
export const fileDecodeNodeType: NodeTypeInfo = {
  name: "file-decode",
  label: "Decode Files",
  description: "Decode each file's content from a text encoding (base64) into its bytes.",
  category: "file",
  isContainer: false,
  platforms: ["browser"] as const,
  icon: "arrow-left-right",
};
