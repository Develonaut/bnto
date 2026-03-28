/**
 * Image strip-exif field configs — augments engine-generated schema with UI hints.
 *
 * The Rust engine is the single source of truth for image-strip-exif parameters.
 * This file adds UI presentation metadata (presets, suffixes) for the editor.
 *
 * To change parameters, edit the processor `metadata()` in
 * `engine/crates/bnto-image/src/`, then run `task nodes:generate`.
 */

import type { NodeParamFields } from "./types";

export { imageStripExifParamsSchema, imageStripExifNodeSchema } from "../generated/schemas";
export type { ImageStripExifParams } from "../generated/schemas";

/** UI presentation metadata for image-strip-exif node fields. */
export const imageStripExifFields: NodeParamFields = {
  quality: {
    suffix: "%",
    label: "Quality",
    presets: [
      { value: 60, label: "Draft" },
      { value: 80, label: "Balanced" },
      { value: 100, label: "Maximum" },
    ],
  },
};
