/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { NodeTypeInfo } from "../types";

/** Node type info for parallel. */
export const parallelNodeType: NodeTypeInfo = {
  name: "parallel",
  label: "Parallel",
  description: "Execute tasks concurrently with configurable worker pool and error strategy.",
  category: "control",
  isContainer: true,
  browserCapable: true,
  icon: "git-fork",
};
