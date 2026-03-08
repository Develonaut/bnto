/**
 * Image node schema — parameters for image processing operations.
 *
 * Engine-level metadata (defaults, constraints, MIME types) comes from
 * the generated catalog module. UI-only metadata (visibleWhen, hidden,
 * placeholder) stays here.
 *
 * Operations are derived from the engine catalog — only engine-backed
 * operations are valid.
 */

import { z } from "zod";
import type { NodeSchemaDefinition } from "./types";
import { getProcessorDefaults, getParamConstraints, PROCESSOR_MAP } from "../generated/catalog";
import { getEngineOperations } from "./deriveOperations";

/**
 * Valid image processing operations — derived from engine PROCESSORS.
 *
 * Currently: ["compress", "convert", "resize"]
 */
export const IMAGE_OPERATIONS = getEngineOperations("image");

/**
 * Supported output image formats — derived from engine's image:convert
 * processor format param options.
 */
const convertProc = PROCESSOR_MAP.get("image:convert");
const formatParam = convertProc?.parameters.find((p) => p.name === "format");
export const IMAGE_FORMATS = (formatParam?.options ?? ["jpeg", "png", "webp"]) as readonly string[];

// --- Engine-sourced constraints ---

const compressDefaults = getProcessorDefaults("image", "compress");
const resizeDefaults = getProcessorDefaults("image", "resize");
const qualityConstraints = getParamConstraints("image", "compress", "quality");
const widthConstraints = getParamConstraints("image", "resize", "width");
const heightConstraints = getParamConstraints("image", "resize", "height");

/** Zod schema for image node parameters. */
export const imageParamsSchema = z.object({
  operation: z.enum(IMAGE_OPERATIONS as [string, ...string[]]),
  input: z.string().optional(),
  output: z.string().optional(),
  format: z.enum(IMAGE_FORMATS as [string, ...string[]]).optional(),
  quality: z
    .number()
    .min(qualityConstraints?.min ?? 1)
    .max(qualityConstraints?.max ?? 100)
    .optional()
    .default(compressDefaults.quality as number),
  width: z
    .number()
    .min(widthConstraints?.min ?? 1)
    .optional(),
  height: z
    .number()
    .min(heightConstraints?.min ?? 1)
    .optional(),
  maintainAspect: z
    .boolean()
    .optional()
    .default(resizeDefaults.maintainAspect as boolean),
});

/** Inferred TypeScript type for image node parameters. */
export type ImageParams = z.infer<typeof imageParamsSchema>;

/** Full schema definition for the image node type. */
export const imageNodeSchema: NodeSchemaDefinition = {
  nodeType: "image",
  schemaVersion: 1,
  schema: imageParamsSchema,
  params: {
    operation: {
      label: "Operation",
      description: "The image processing operation to perform.",
    },
    input: {
      label: "Input",
      description: "Input image file path.",
      placeholder: "{{.item}}",
      hidden: true,
    },
    output: {
      label: "Output",
      description: "Output image file path.",
      placeholder: "{{.OUTPUT_DIR}}/{{basename .item}}",
      hidden: true,
    },
    format: {
      label: "Format",
      description: "Output image format.",
      visibleWhen: { param: "operation", equals: "convert" },
    },
    quality: {
      label: "Quality",
      description: "Output quality (1-100). Higher is better quality but larger file size.",
    },
    width: {
      label: "Width",
      description: "Target width in pixels for resize.",
      visibleWhen: { param: "operation", equals: "resize" },
    },
    height: {
      label: "Height",
      description: "Target height in pixels for resize (optional if maintainAspect is true).",
      visibleWhen: { param: "operation", equals: "resize" },
    },
    maintainAspect: {
      label: "Maintain Aspect Ratio",
      description: "Preserve the original aspect ratio when resizing.",
      visibleWhen: { param: "operation", equals: "resize" },
    },
  },
};
