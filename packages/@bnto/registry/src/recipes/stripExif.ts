/** Strip EXIF recipe — remove EXIF metadata from images. */

import type { Recipe } from "../recipe";
import { CURRENT_FORMAT_VERSION, getProcessorDefaults } from "@bnto/nodes";
import { defaultInputNode } from "./defaultInputNode";
import { defaultOutputNode } from "./defaultOutputNode";

const IMAGE_INPUT = {
  accept: ["image/jpeg", "image/png", "image/webp"],
  extensions: [".jpg", ".jpeg", ".png", ".webp"],
  label: "JPEG, PNG, or WebP images",
} as const;

export const stripExif: Recipe = {
  id: "b8d4f2a1-9e53-4c0b-a6f7-4g2b3d5e7f8a",
  slug: "strip-exif",
  name: "Strip EXIF",
  description:
    "Remove EXIF metadata from images instantly in your browser. No upload limits, no signup.",
  category: "image",
  accept: {
    mimeTypes: ["image/jpeg", "image/png", "image/webp"],
    extensions: [".jpg", ".jpeg", ".png", ".webp"],
    label: "JPEG, PNG, or WebP images",
    mimePrefix: "image/",
  },
  features: ["PNG", "JPEG", "WebP", "No upload", "Browser-based"],
  definition: {
    id: "strip-exif",
    type: "group",
    version: CURRENT_FORMAT_VERSION,
    name: "Strip EXIF",
    position: { x: 0, y: 0 },
    metadata: {
      description: "Accepts image files and strips EXIF metadata from each one.",
    },
    parameters: {},
    inputPorts: [],
    outputPorts: [],
    settings: { iteration: "auto" },
    nodes: [
      defaultInputNode(IMAGE_INPUT),
      {
        id: "strip-exif-node",
        type: "image-strip-exif",
        version: CURRENT_FORMAT_VERSION,
        name: "Strip EXIF",
        position: { x: 250, y: 100 },
        metadata: {},
        parameters: {
          ...getProcessorDefaults("image-strip-exif"),
        },
        inputPorts: [],
        outputPorts: [],
      },
      defaultOutputNode({ label: "Cleaned Images" }),
    ],
    edges: [
      { id: "e1", source: "input", target: "strip-exif-node" },
      { id: "e2", source: "strip-exif-node", target: "output" },
    ],
  },
};
