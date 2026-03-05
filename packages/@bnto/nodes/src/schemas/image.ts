/**
 * Image node schema — parameters for image processing operations.
 *
 * Go source: engine/pkg/node/library/image/image.go
 * Validator: engine/pkg/validator/validators.go → validateImage
 */

import { z } from "zod";
import type { NodeSchemaDefinition } from "./types";

/**
 * Valid image processing operations.
 *
 * NOTE: "batch" is not yet implemented in any engine (Rust WASM or Go).
 * It will be added here when batch image processing is built. See ROADMAP.md.
 */
export const IMAGE_OPERATIONS = ["resize", "convert", "optimize", "composite"] as const;

/** Supported output image formats. */
export const IMAGE_FORMATS = ["png", "jpeg", "webp"] as const;

/** Zod schema for image node parameters. */
export const imageParamsSchema = z.object({
  operation: z.enum(IMAGE_OPERATIONS),
  input: z.string().optional(),
  output: z.string().optional(),
  format: z.enum(IMAGE_FORMATS).optional(),
  quality: z.number().min(1).max(100).optional().default(80),
  width: z.number().min(1).optional(),
  height: z.number().min(1).optional(),
  maintainAspect: z.boolean().optional().default(true),
  base: z.string().optional(),
  overlay: z.string().optional(),
  position: z.string().optional().default("center"),
  x: z.number().optional().default(0),
  y: z.number().optional().default(0),
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
    },
    output: {
      label: "Output",
      description: "Output image file path.",
      placeholder: "{{.OUTPUT_DIR}}/{{basename .item}}",
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
    base: {
      label: "Base Image",
      description: "Path to the base (background) image for compositing.",
      visibleWhen: { param: "operation", equals: "composite" },
      requiredWhen: { param: "operation", equals: "composite" },
    },
    overlay: {
      label: "Overlay Image",
      description: "Path to the overlay (foreground) image for compositing.",
      visibleWhen: { param: "operation", equals: "composite" },
      requiredWhen: { param: "operation", equals: "composite" },
    },
    position: {
      label: "Position",
      description: 'Overlay position — "center" or use x/y offsets.',
      visibleWhen: { param: "operation", equals: "composite" },
    },
    x: {
      label: "X Offset",
      description: "Horizontal offset for overlay placement in pixels.",
      visibleWhen: { param: "operation", equals: "composite" },
    },
    y: {
      label: "Y Offset",
      description: "Vertical offset for overlay placement in pixels.",
      visibleWhen: { param: "operation", equals: "composite" },
    },
  },
};
