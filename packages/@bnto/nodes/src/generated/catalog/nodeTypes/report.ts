/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { NodeTypeInfo } from "../types";

/** Node type info for report. */
export const reportNodeType: NodeTypeInfo = {
  name: "report",
  label: "Report",
  description: "Print a templated line to the run's output; files pass through unchanged.",
  category: "control",
  isContainer: false,
  platforms: ["browser"] as const,
  icon: "message-square",
};
