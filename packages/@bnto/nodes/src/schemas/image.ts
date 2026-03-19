/**
 * Image node schema — augments engine-generated schemas with UI layout hints.
 *
 * The Rust engine is the single source of truth for image node parameters.
 * Zod schemas, UI metadata (visibleWhen, labels, descriptions), and
 * constraints are all generated from the engine catalog snapshot.
 *
 * This wrapper adds `group` annotations for the editor's FieldGroup layout.
 * To change image parameters, edit the processor `metadata()` in
 * `engine/crates/bnto-image/src/`, then run `task nodes:generate`.
 */

import {
  IMAGE_OPERATIONS,
  imageParamsSchema,
  imageNodeSchema as generated,
} from "../generated/schemas";
import type { NodeSchemaDefinition } from "./types";

export { IMAGE_OPERATIONS, imageParamsSchema };
export type { ImageParams } from "../generated/schemas";

// Re-derive IMAGE_FORMATS from the generated schema for backward compat
import { PROCESSOR_MAP } from "../generated/catalog";
const convertProc = PROCESSOR_MAP.get("image:convert");
const formatParam = convertProc?.parameters.find((p) => p.name === "format");
export const IMAGE_FORMATS = (formatParam?.options ?? ["jpeg", "png", "webp"]) as readonly string[];

/** Image schema with dimension group annotations and compression presets. */
export const imageNodeSchema: NodeSchemaDefinition = {
  ...generated,
  params: {
    ...generated.params,
    operation: {
      ...generated.params.operation,
      hidden: true,
      options: [
        { value: "compress", label: "Compress" },
        { value: "convert", label: "Convert" },
        { value: "resize", label: "Resize" },
      ],
    },
    format: {
      ...generated.params.format,
      options: [
        { value: "jpeg", label: "JPEG" },
        { value: "png", label: "PNG" },
        { value: "webp", label: "WebP" },
      ],
    },
    compression: {
      ...generated.params.compression,
      displayInverted: true,
      presets: [
        { value: 20, label: "Maximum" },
        { value: 50, label: "Balanced" },
        { value: 80, label: "Light" },
      ],
    },
    quality: {
      ...generated.params.quality,
      presets: [
        { value: 60, label: "Draft" },
        { value: 80, label: "Balanced" },
        { value: 100, label: "Maximum" },
      ],
    },
    width: { ...generated.params.width, group: "dimensions", suffix: "px" },
    height: { ...generated.params.height, group: "dimensions", suffix: "px" },
    maintainAspect: { ...generated.params.maintainAspect, group: "dimensions" },
  },
};
