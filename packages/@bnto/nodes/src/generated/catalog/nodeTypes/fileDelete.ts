/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { NodeTypeInfo } from "../types";

/** Node type info for file-delete. */
export const fileDeleteNodeType: NodeTypeInfo = {
  name: "file-delete",
  label: "Delete Files",
  description:
    "Delete the files flowing in, each under a base folder; missing files are skipped and counted.",
  category: "file",
  isContainer: false,
  platforms: ["cli"] as const,
  icon: "file-minus-2",
};
