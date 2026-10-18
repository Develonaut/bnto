/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { NodeTypeInfo } from "../types";

/** Node type info for summarize. */
export const summarizeNodeType: NodeTypeInfo = {
  name: "summarize",
  label: "Summarize",
  description: "Group items by fields; output one item per group with its count and field totals.",
  category: "data",
  isContainer: false,
  platforms: ["browser"] as const,
  icon: "sigma",
};
