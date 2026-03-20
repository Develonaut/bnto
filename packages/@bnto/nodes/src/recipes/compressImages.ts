/** Compress Images recipe — optimize PNG, JPEG, and WebP images. */

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

export const compressImages: Recipe = {
  id: "04ad520d-3afe-470b-8225-6cae2b14c402",
  slug: "compress-images",
  name: "Compress Images",
  description:
    "Compress PNG, JPEG, and WebP images instantly in your browser. No upload limits, no signup.",
  category: "image",
  accept: {
    mimeTypes: ["image/jpeg", "image/png", "image/webp"],
    extensions: [".jpg", ".jpeg", ".png", ".webp"],
    label: "JPEG, PNG, or WebP images",
    mimePrefix: "image/",
  },
  features: ["PNG", "JPEG", "WebP", "No upload", "Browser-based"],
  definition: {
    id: "compress-images",
    type: "group",
    version: CURRENT_FORMAT_VERSION,
    name: "Compress Images",
    position: { x: 0, y: 0 },
    metadata: {
      description: "Accepts image files and compresses each one.",
    },
    parameters: {},
    inputPorts: [],
    outputPorts: [],
    nodes: [
      defaultInputNode(IMAGE_INPUT),
      {
        id: "compress-loop",
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
            id: "compress-image",
            type: "image-compress",
            version: CURRENT_FORMAT_VERSION,
            name: "Compress",
            position: { x: 0, y: 0 },
            metadata: {},
            parameters: {
              ...getProcessorDefaults("image-compress"),
            },
            inputPorts: [],
            outputPorts: [],
          },
        ],
        edges: [],
      },
      defaultOutputNode({ label: "Compressed Images" }),
    ],
    edges: [
      { id: "e1", source: "input", target: "compress-loop" },
      { id: "e2", source: "compress-loop", target: "output" },
    ],
  },
};
