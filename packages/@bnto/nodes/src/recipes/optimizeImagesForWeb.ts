/** Optimize Images for Web recipe — resize, convert to WebP, and compress. */

import type { Recipe } from "../recipe";
import { CURRENT_FORMAT_VERSION } from "../formatVersion";
import { getProcessorDefaults } from "../generated/catalog";
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
    nodes: [
      defaultInputNode(IMAGE_INPUT),
      {
        id: "process-loop",
        type: "loop",
        version: CURRENT_FORMAT_VERSION,
        name: "For Each",
        position: { x: 250, y: 100 },
        metadata: {},
        parameters: { mode: "forEach" },
        inputPorts: [{ id: "in-1", name: "items" }],
        outputPorts: [],
        nodes: [
          {
            id: "resize",
            type: "image",
            version: CURRENT_FORMAT_VERSION,
            name: "Resize",
            position: { x: 0, y: 0 },
            metadata: {},
            parameters: {
              operation: "resize",
              ...getProcessorDefaults("image", "resize"),
              width: 800,
            },
            inputPorts: [],
            outputPorts: [],
          },
          {
            id: "convert",
            type: "image",
            version: CURRENT_FORMAT_VERSION,
            name: "Convert",
            position: { x: 250, y: 0 },
            metadata: {},
            parameters: {
              operation: "convert",
              ...getProcessorDefaults("image", "convert"),
              format: "webp",
            },
            inputPorts: [],
            outputPorts: [],
          },
          {
            id: "compress",
            type: "image",
            version: CURRENT_FORMAT_VERSION,
            name: "Compress",
            position: { x: 500, y: 0 },
            metadata: {},
            parameters: {
              operation: "compress",
              ...getProcessorDefaults("image", "compress"),
            },
            inputPorts: [],
            outputPorts: [],
          },
        ],
        edges: [
          { id: "le1", source: "resize", target: "convert" },
          { id: "le2", source: "convert", target: "compress" },
        ],
      },
      defaultOutputNode({ label: "Optimized Images" }),
    ],
    edges: [
      { id: "e1", source: "input", target: "process-loop" },
      { id: "e2", source: "process-loop", target: "output" },
    ],
  },
};
