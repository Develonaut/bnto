/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { NodeTypeInfo } from "../types";

/** Node type info for file-join. */
export const fileJoinNodeType: NodeTypeInfo = {
  name: "file-join",
  label: "Join Files",
  description:
    "Join the files flowing in into one file, in arrival order (edge order when several steps feed it), a separator between them.",
  category: "file",
  isContainer: false,
  platforms: ["browser"] as const,
  icon: "merge",
};
