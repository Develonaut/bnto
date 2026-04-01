/** Watermark Images recipe — overlay a logo or watermark onto images. */

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
  id: "b7d2a3f1-8e4c-4d6b-9a1f-2c5e7b8d3f4a",
  slug: "watermark-images",
  name: "Watermark Images",
  description:
    "Add a logo or watermark to images. Position, size, and opacity are fully configurable. Runs in your browser — files never leave your machine.",
  category: "image",
  accept: {
    mimeTypes: ["image/jpeg", "image/png", "image/webp"],
    extensions: [".jpg", ".jpeg", ".png", ".webp"],
    label: "JPEG, PNG, or WebP images",
    mimePrefix: "image/",
  },
  features: ["PNG", "JPEG", "WebP", "Configurable position", "Adjustable opacity", "Browser-based"],
  definition: {
    id: "watermark-images",
    type: "group",
    version: CURRENT_FORMAT_VERSION,
    name: "Watermark Images",
    position: { x: 0, y: 0 },
    metadata: {
      description: "Accepts image files and overlays an image onto each one.",
    },
    parameters: {},
    inputPorts: [],
    outputPorts: [],
    settings: { iteration: "auto" },
    nodes: [
      defaultInputNode(IMAGE_INPUT),
      {
        id: "overlay",
        type: "image-overlay",
        version: CURRENT_FORMAT_VERSION,
        name: "Overlay",
        position: { x: 250, y: 100 },
        metadata: {},
        parameters: {
          ...getProcessorDefaults("image-overlay"),
          overlay: "",
        },
        inputPorts: [],
        outputPorts: [],
      },
      defaultOutputNode({ label: "Watermarked Images" }),
    ],
    edges: [
      { id: "e1", source: "input", target: "overlay" },
      { id: "e2", source: "overlay", target: "output" },
    ],
  },
};
