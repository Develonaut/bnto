/** Compress Images recipe — optimize PNG, JPEG, and WebP images. */

import type { Recipe } from "../../recipe";
import { CURRENT_FORMAT_VERSION } from "../../formatVersion";
import { batchCompress } from "../primitives/batchCompress";

export const compressImages: Recipe = {
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
  seo: {
    title: "Compress Images Online Free -- bnto",
    h1: "Compress Images Online Free",
  },
  definition: {
    id: "compress-images",
    type: "group",
    version: CURRENT_FORMAT_VERSION,
    name: "Compress Images",
    position: { x: 0, y: 0 },
    metadata: {
      description:
        "Accepts image files and compresses each one using a reusable batch compression sub-recipe.",
    },
    parameters: {},
    inputPorts: [],
    outputPorts: [],
    nodes: [
      {
        id: "input",
        type: "input",
        version: CURRENT_FORMAT_VERSION,
        name: "Input Files",
        position: { x: 0, y: 100 },
        metadata: {},
        parameters: {
          mode: "file-upload",
          accept: ["image/jpeg", "image/png", "image/webp"],
          extensions: [".jpg", ".jpeg", ".png", ".webp"],
          label: "JPEG, PNG, or WebP images",
          multiple: true,
        },
        inputPorts: [],
        outputPorts: [{ id: "out-1", name: "files" }],
      },
      batchCompress,
      {
        id: "output",
        type: "output",
        version: CURRENT_FORMAT_VERSION,
        name: "Compressed Images",
        position: { x: 500, y: 100 },
        metadata: {},
        parameters: {
          mode: "download",
          label: "Compressed Images",
          zip: true,
          autoDownload: false,
        },
        inputPorts: [{ id: "in-1", name: "files" }],
        outputPorts: [],
      },
    ],
    edges: [
      { id: "e1", source: "input", target: "batch-compress" },
      { id: "e2", source: "batch-compress", target: "output" },
    ],
  },
};
