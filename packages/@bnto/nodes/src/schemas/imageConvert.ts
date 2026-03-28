/**
 * Image convert field configs — augments engine-generated schema with UI hints.
 *
 * The Rust engine is the single source of truth for image-convert parameters.
 * This file adds UI presentation metadata (format options, quality suffix)
 * for the editor.
 *
 * To change parameters, edit the processor `metadata()` in
 * `engine/crates/bnto-image/src/`, then run `task nodes:generate`.
 */

import { PROCESSOR_MAP } from "../generated/catalog";
import type { NodeParamFields } from "./types";

export { imageConvertParamsSchema, imageConvertNodeSchema } from "../generated/schemas";
export type { ImageConvertParams } from "../generated/schemas";

/** Derive IMAGE_FORMATS from the engine catalog for backward compat. */
const convertProc = PROCESSOR_MAP.get("image-convert");
const formatParam = convertProc?.parameters.find((p) => p.name === "format");
export const IMAGE_FORMATS = (formatParam?.options ?? ["jpeg", "png", "webp"]) as readonly string[];

/** UI presentation metadata for image-convert node fields. */
export const imageConvertFields: NodeParamFields = {
  format: {
    options: IMAGE_FORMATS.map((f) => ({
      value: f,
      label: f.toUpperCase(),
    })),
  },
  quality: {
    suffix: "%",
    presets: [
      { value: 60, label: "Draft" },
      { value: 80, label: "Balanced" },
      { value: 100, label: "Maximum" },
    ],
  },
};
