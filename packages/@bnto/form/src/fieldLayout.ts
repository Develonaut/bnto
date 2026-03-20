import type { NodeParamControl } from "@bnto/core";
import { INLINE_CONTROLS, SELF_LABELED_CONTROLS } from "./controlCategories";

/** Layout mode for a schema field based on its control type. */
export type FieldLayout = "inline" | "self-labeled" | "column";

/** Determine layout mode for a control type. */
export function getFieldLayout(control: NodeParamControl): FieldLayout {
  if (SELF_LABELED_CONTROLS.has(control)) return "self-labeled";
  if (INLINE_CONTROLS.has(control)) return "inline";
  return "column";
}
