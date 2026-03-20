/**
 * Image resize field configs — augments engine-generated schema with UI hints.
 *
 * The Rust engine is the single source of truth for image-resize parameters.
 * This file adds UI presentation metadata (groups, suffixes) for the editor.
 *
 * To change parameters, edit the processor `metadata()` in
 * `engine/crates/bnto-image/src/`, then run `task nodes:generate`.
 */

import type { FieldConfigMap } from "./types";

export { imageResizeParamsSchema, imageResizeNodeSchema } from "../generated/schemas";
export type { ImageResizeParams } from "../generated/schemas";

/** UI presentation metadata for image-resize node fields. */
export const imageResizeFields: FieldConfigMap = {
  width: { group: "dimensions", suffix: "px" },
  height: { group: "dimensions", suffix: "px" },
  maintainAspect: { group: "dimensions", label: "Maintain Aspect Ratio" },
  quality: {
    suffix: "%",
    presets: [
      { value: 60, label: "Draft" },
      { value: 80, label: "Balanced" },
      { value: 100, label: "Maximum" },
    ],
  },
};
