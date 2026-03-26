/** Optimize Images for Web recipe — resize, convert to WebP, and compress. */

import type { Recipe } from "../recipe";
import { CURRENT_FORMAT_VERSION, getProcessorDefaults } from "@bnto/nodes";
import { defaultInputNode } from "./defaultInputNode";
import { defaultOutputNode } from "./defaultOutputNode";

const IMAGE_INPUT = {
  accept: ["image/jpeg", "image/png", "image/webp"],
  extensions: [".jpg", ".jpeg", ".png", ".webp"],
  label: "JPEG, PNG, or WebP images",
} as const;

export const optimizeImagesForWeb: Recipe = {
  id: "e8c3a3ad-1e7e-40f6-ab60-00f405514f6f",
  slug: "optimize-images-for-web",
  name: "Optimize Images for Web",
  description:
    "Resize, convert to WebP, and compress images for fast web loading. Free, no signup.",
  category: "image",
  accept: {
    mimeTypes: ["image/jpeg", "image/png", "image/webp"],
    extensions: [".jpg", ".jpeg", ".png", ".webp"],
    label: "JPEG, PNG, or WebP images",
    mimePrefix: "image/",
  },
  features: ["Resize", "WebP", "Compress", "Multi-step", "Browser-based"],
  definition: {
    id: "optimize-images-for-web",
    type: "group",
    version: CURRENT_FORMAT_VERSION,
    name: "Optimize Images for Web",
    position: { x: 0, y: 0 },
    metadata: {
      description: "Accepts image files, resizes, converts to WebP, and compresses each one.",
    },
    parameters: {},
    inputPorts: [],
    outputPorts: [],
    settings: { iteration: "auto" },
    nodes: [
      defaultInputNode(IMAGE_INPUT),
      {
        id: "resize",
        type: "image-resize",
        version: CURRENT_FORMAT_VERSION,
        name: "Resize",
        position: { x: 250, y: 100 },
        metadata: {},
        parameters: {
          ...getProcessorDefaults("image-resize"),
          width: 800,
        },
        inputPorts: [],
        outputPorts: [],
      },
      {
        id: "convert",
        type: "image-convert",
        version: CURRENT_FORMAT_VERSION,
        name: "Convert",
        position: { x: 500, y: 100 },
        metadata: {},
        parameters: {
          ...getProcessorDefaults("image-convert"),
          format: "webp",
        },
        inputPorts: [],
        outputPorts: [],
      },
      {
        id: "compress",
        type: "image-compress",
        version: CURRENT_FORMAT_VERSION,
        name: "Compress",
        position: { x: 750, y: 100 },
        metadata: {},
        parameters: {
          ...getProcessorDefaults("image-compress"),
        },
        inputPorts: [],
        outputPorts: [],
      },
      { ...defaultOutputNode({ label: "Optimized Images" }), position: { x: 1000, y: 100 } },
    ],
    edges: [
      { id: "e1", source: "input", target: "resize" },
      { id: "e2", source: "resize", target: "convert" },
      { id: "e3", source: "convert", target: "compress" },
      { id: "e4", source: "compress", target: "output" },
    ],
  },
};
