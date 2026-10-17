/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { NodeTypeInfo } from "../types";

/** Node type info for assert. */
export const assertNodeType: NodeTypeInfo = {
  name: "assert",
  label: "Assert",
  description: "Fail the run when a comparison doesn't hold; otherwise pass files through.",
  category: "control",
  isContainer: false,
  platforms: ["browser"] as const,
  icon: "circle-off",
};
