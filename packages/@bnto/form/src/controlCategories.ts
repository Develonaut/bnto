import type { NodeParamControl } from "@bnto/core";

/** Controls that render inline (label left, control right). */
export const INLINE_CONTROLS = new Set<NodeParamControl>(["switch", "select"]);

/** Controls that render their own label (Slider owns its header row). */
export const SELF_LABELED_CONTROLS = new Set<NodeParamControl>(["slider"]);
