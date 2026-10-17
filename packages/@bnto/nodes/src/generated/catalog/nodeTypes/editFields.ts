/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { NodeTypeInfo } from "../types";

/** Node type info for edit-fields. */
export const editFieldsNodeType: NodeTypeInfo = {
  name: "edit-fields",
  label: "Edit Fields",
  description: "Set fields on each item from templates over its other fields; files pass through.",
  category: "data",
  isContainer: false,
  platforms: ["browser"] as const,
  icon: "pen-line",
};
