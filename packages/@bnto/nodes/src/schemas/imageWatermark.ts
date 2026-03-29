/**
 * Image watermark field configs — augments engine-generated schema with UI hints.
 *
 * The Rust engine is the single source of truth for image-watermark parameters.
 * This file adds UI presentation metadata (file control, presets) for the editor.
 *
 * To change parameters, edit the processor `metadata()` in
 * `engine/crates/bnto-image/src/watermark.rs`, then run `task nodes:generate`.
 */

import type { NodeParamFields } from "./types";

export { imageWatermarkParamsSchema, imageWatermarkNodeSchema } from "../generated/schemas";
export type { ImageWatermarkParams } from "../generated/schemas";

/** UI presentation metadata for image-watermark node fields. */
export const imageWatermarkFields: NodeParamFields = {
  watermark: {
    control: "file",
    accept: ["image/jpeg", "image/png", "image/webp"],
  },
  position: {
    control: "watermarkPreview",
  },
  size: { suffix: "%" },
  opacity: { suffix: "%" },
  offsetX: { suffix: "px", label: "Horizontal Offset" },
  offsetY: { suffix: "px", label: "Vertical Offset" },
  quality: {
    suffix: "%",
    presets: [
      { value: 60, label: "Draft" },
      { value: 80, label: "Balanced" },
      { value: 100, label: "Maximum" },
    ],
  },
};
