/** Watermark Images recipe — overlay a watermark onto source images. */

import type { Recipe } from "../recipe";
import { CURRENT_FORMAT_VERSION, getProcessorDefaults } from "@bnto/nodes";
import { defaultInputNode } from "./defaultInputNode";
import { defaultOutputNode } from "./defaultOutputNode";

const IMAGE_INPUT = {
  accept: ["image/jpeg", "image/png", "image/webp"],
  extensions: [".jpg", ".jpeg", ".png", ".webp"],
  label: "JPEG, PNG, or WebP images",
} as const;

export const watermarkImages: Recipe = {
  id: "f92e1c3a-8b7d-4e5f-a106-3d9c8b2e7f4a",
  slug: "watermark-images",
  name: "Watermark Images",
  description:
    "Add a watermark to your images instantly in the browser. Choose position, size, and opacity. No upload, no signup.",
  category: "image",
  accept: {
    mimeTypes: ["image/jpeg", "image/png", "image/webp"],
    extensions: [".jpg", ".jpeg", ".png", ".webp"],
    label: "JPEG, PNG, or WebP images",
    mimePrefix: "image/",
  },
  features: ["PNG", "JPEG", "WebP", "Custom position", "No upload", "Browser-based"],
  definition: {
    id: "watermark-images",
    type: "group",
    version: CURRENT_FORMAT_VERSION,
    name: "Watermark Images",
    position: { x: 0, y: 0 },
    metadata: {
      description: "Accepts image files and overlays a watermark onto each one.",
    },
    parameters: {},
    inputPorts: [],
    outputPorts: [],
    settings: { iteration: "auto" },
    nodes: [
      defaultInputNode(IMAGE_INPUT),
      {
        id: "watermark-image",
        type: "image-watermark",
        version: CURRENT_FORMAT_VERSION,
        name: "Watermark",
        position: { x: 250, y: 100 },
        metadata: {},
        parameters: {
          ...getProcessorDefaults("image-watermark"),
          watermark: "",
        },
        inputPorts: [],
        outputPorts: [],
      },
      defaultOutputNode({ label: "Watermarked Images" }),
    ],
    edges: [
      { id: "e1", source: "input", target: "watermark-image" },
      { id: "e2", source: "watermark-image", target: "output" },
    ],
  },
};
