/**
 * Image overlay field configs — augments engine-generated schema with UI hints.
 *
 * The Rust engine is the single source of truth for image-overlay parameters.
 * This file adds UI presentation metadata (control overrides, presets) for the editor.
 *
 * To change parameters, edit the processor `metadata()` in
 * `engine/crates/bnto-image/src/watermark.rs`, then run `task nodes:generate`.
 */

import type { NodeParamFields } from "./types";

export { imageOverlayParamsSchema, imageOverlayNodeSchema } from "../generated/schemas";
export type { ImageOverlayParams } from "../generated/schemas";

/** UI presentation metadata for image-overlay node fields. */
export const imageOverlayFields: NodeParamFields = {
  overlay: {
    control: "file",
    accept: ["image/png", "image/jpeg", "image/webp"],
    label: "Overlay Image",
  },
  position: {
    control: "watermarkPreview",
  },
  size: {
    suffix: "%",
    label: "Size",
  },
  opacity: {
    suffix: "%",
    label: "Opacity",
  },
  offsetX: {
    suffix: "px",
    group: "offset",
  },
  offsetY: {
    suffix: "px",
    group: "offset",
  },
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
