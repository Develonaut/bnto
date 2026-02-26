/**
 * Image node schema — parameters for image processing operations.
 *
 * Go source: engine/pkg/node/library/image/image.go
 * Validator: engine/pkg/validator/validators.go → validateImage
 */

import type { NodeSchema } from "./types";

/**
 * Valid image processing operations.
 *
 * NOTE: "batch" is not yet implemented in any engine (Rust WASM or Go).
 * It will be added here when batch image processing is built. See ROADMAP.md.
 */
export const IMAGE_OPERATIONS = [
  "resize",
  "convert",
  "optimize",
  "composite",
] as const;

/** Supported output image formats. */
export const IMAGE_FORMATS = ["png", "jpeg", "webp"] as const;

export const imageSchema: NodeSchema = {
  nodeType: "image",
  parameters: [
    {
      name: "operation",
      type: "enum",
      required: true,
      label: "Operation",
      description: "The image processing operation to perform.",
      enumValues: IMAGE_OPERATIONS,
    },
    {
      name: "input",
      type: "string",
      required: false,
      label: "Input",
      description: "Input image file path.",
      placeholder: "{{.item}}",
    },
    {
      name: "output",
      type: "string",
      required: false,
      label: "Output",
      description: "Output image file path.",
      placeholder: "{{.OUTPUT_DIR}}/{{basename .item}}",
    },
    {
      name: "format",
      type: "enum",
      required: false,
      label: "Format",
      description: "Output image format.",
      enumValues: IMAGE_FORMATS,
      visibleWhen: { param: "operation", equals: "convert" },
    },
    {
      name: "quality",
      type: "number",
      required: false,
      label: "Quality",
      description: "Output quality (1-100). Higher is better quality but larger file size.",
      default: 80,
      min: 1,
      max: 100,
    },
    {
      name: "width",
      type: "number",
      required: false,
      label: "Width",
      description: "Target width in pixels for resize.",
      min: 1,
      visibleWhen: { param: "operation", equals: "resize" },
    },
    {
      name: "height",
      type: "number",
      required: false,
      label: "Height",
      description: "Target height in pixels for resize (optional if maintainAspect is true).",
      min: 1,
      visibleWhen: { param: "operation", equals: "resize" },
    },
    {
      name: "maintainAspect",
      type: "boolean",
      required: false,
      label: "Maintain Aspect Ratio",
      description: "Preserve the original aspect ratio when resizing.",
      default: true,
      visibleWhen: { param: "operation", equals: "resize" },
    },
    {
      name: "base",
      type: "string",
      required: false,
      label: "Base Image",
      description: "Path to the base (background) image for compositing.",
      visibleWhen: { param: "operation", equals: "composite" },
      requiredWhen: { param: "operation", equals: "composite" },
    },
    {
      name: "overlay",
      type: "string",
      required: false,
      label: "Overlay Image",
      description: "Path to the overlay (foreground) image for compositing.",
      visibleWhen: { param: "operation", equals: "composite" },
      requiredWhen: { param: "operation", equals: "composite" },
    },
    {
      name: "position",
      type: "string",
      required: false,
      label: "Position",
      description: 'Overlay position — "center" or use x/y offsets.',
      default: "center",
      visibleWhen: { param: "operation", equals: "composite" },
    },
    {
      name: "x",
      type: "number",
      required: false,
      label: "X Offset",
      description: "Horizontal offset for overlay placement in pixels.",
      default: 0,
      visibleWhen: { param: "operation", equals: "composite" },
    },
    {
      name: "y",
      type: "number",
      required: false,
      label: "Y Offset",
      description: "Vertical offset for overlay placement in pixels.",
      default: 0,
      visibleWhen: { param: "operation", equals: "composite" },
    },
  ],
} as const;
