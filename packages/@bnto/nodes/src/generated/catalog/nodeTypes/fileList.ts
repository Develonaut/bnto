/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { NodeTypeInfo } from "../types";

/** Node type info for file-list. */
export const fileListNodeType: NodeTypeInfo = {
  name: "file-list",
  label: "List Files",
  description:
    "List files from lists of paths under a base folder; every path is listed, found or missing.",
  category: "file",
  isContainer: false,
  platforms: ["cli"] as const,
  icon: "file-text",
};
