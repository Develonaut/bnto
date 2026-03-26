/** Compress & Rename recipe — compress images and suffix for organization. */

import type { Recipe } from "../recipe";
import { CURRENT_FORMAT_VERSION, getProcessorDefaults } from "@bnto/nodes";
import { defaultInputNode } from "./defaultInputNode";
import { defaultOutputNode } from "./defaultOutputNode";

const IMAGE_INPUT = {
  accept: ["image/jpeg", "image/png", "image/webp"],
  extensions: [".jpg", ".jpeg", ".png", ".webp"],
  label: "JPEG, PNG, or WebP images",
} as const;

export const compressAndRename: Recipe = {
  id: "b3a1c7d2-8f4e-4a6b-9c5d-1e2f3a4b5c6d",
  slug: "compress-and-rename",
  name: "Compress & Rename",
  description:
    "Compress images and add a suffix so originals and compressed versions are distinguishable. Free, no signup.",
  category: "image",
  accept: {
    mimeTypes: ["image/jpeg", "image/png", "image/webp"],
    extensions: [".jpg", ".jpeg", ".png", ".webp"],
    label: "JPEG, PNG, or WebP images",
    mimePrefix: "image/",
  },
  features: ["Compress", "Rename", "Suffix", "Multi-step", "Browser-based"],
  definition: {
    id: "compress-and-rename",
    type: "group",
    version: CURRENT_FORMAT_VERSION,
    name: "Compress & Rename",
    position: { x: 0, y: 0 },
    metadata: {
      description: "Accepts image files, compresses each one, and renames with a suffix.",
    },
    parameters: {},
    inputPorts: [],
    outputPorts: [],
    settings: { iteration: "auto" },
    nodes: [
      defaultInputNode(IMAGE_INPUT),
      {
        id: "compress",
        type: "image-compress",
        version: CURRENT_FORMAT_VERSION,
        name: "Compress",
        position: { x: 250, y: 100 },
        metadata: {},
        parameters: {
          ...getProcessorDefaults("image-compress"),
        },
        inputPorts: [],
        outputPorts: [],
      },
      {
        id: "rename",
        type: "file-rename",
        version: CURRENT_FORMAT_VERSION,
        name: "Rename",
        position: { x: 500, y: 100 },
        metadata: {},
        parameters: {
          ...getProcessorDefaults("file-rename"),
          suffix: "-min",
        },
        inputPorts: [],
        outputPorts: [],
      },
      { ...defaultOutputNode({ label: "Compressed & Renamed" }), position: { x: 750, y: 100 } },
    ],
    edges: [
      { id: "e1", source: "input", target: "compress" },
      { id: "e2", source: "compress", target: "rename" },
      { id: "e3", source: "rename", target: "output" },
    ],
  },
};
