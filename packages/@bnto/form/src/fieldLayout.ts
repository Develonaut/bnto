import type { NodeParamControl } from "@bnto/core";

/** Controls that render inline (label left, control right). */
const INLINE_CONTROLS = new Set<NodeParamControl>(["switch", "select"]);

/** Controls that render their own label (Slider owns its header row). */
const SELF_LABELED_CONTROLS = new Set<NodeParamControl>(["slider"]);

/** Layout mode for a schema field based on its control type. */
export type FieldLayout = "inline" | "self-labeled" | "column";

/** Determine layout mode for a control type. */
export function getFieldLayout(control: NodeParamControl): FieldLayout {
  if (SELF_LABELED_CONTROLS.has(control)) return "self-labeled";
  if (INLINE_CONTROLS.has(control)) return "inline";
  return "column";
}
