/** Batch Compress recipe — loops over image files and compresses each one. */

import type { Recipe } from "../recipe";
import { CURRENT_FORMAT_VERSION } from "../formatVersion";
import { getProcessorDefaults } from "../generated/catalog";

export const batchCompress: Recipe = {
  slug: "batch-compress",
  name: "Batch Compress",
  description: "Compress multiple images in a single batch. Reusable building block.",
  category: "image",
  accept: {
    mimeTypes: ["image/jpeg", "image/png", "image/webp"],
    extensions: [".jpg", ".jpeg", ".png", ".webp"],
    label: "JPEG, PNG, or WebP images",
    mimePrefix: "image/",
  },
  features: ["PNG", "JPEG", "WebP", "Batch processing"],
  seo: {
    title: "Batch Compress Images -- bnto",
    h1: "Batch Compress Images",
  },
  definition: {
    id: "batch-compress",
    type: "group",
    version: CURRENT_FORMAT_VERSION,
    name: "Batch Compress",
    position: { x: 0, y: 0 },
    metadata: {
      description: "Loops over files and compresses each one.",
      customData: { displayName: "Batch Compress" },
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
      {
        id: "compress-loop",
        type: "loop",
        version: CURRENT_FORMAT_VERSION,
        name: "Compress Each Image",
        position: { x: 250, y: 100 },
        metadata: {},
        parameters: { mode: "forEach" },
        inputPorts: [{ id: "in-1", name: "items" }],
        outputPorts: [],
        nodes: [
          {
            id: "compress-image",
            type: "image",
            version: CURRENT_FORMAT_VERSION,
            name: "Compress Image",
            position: { x: 0, y: 0 },
            metadata: {},
            parameters: {
              operation: "compress",
              ...getProcessorDefaults("image", "compress"),
            },
            inputPorts: [],
            outputPorts: [],
          },
        ],
        edges: [],
      },
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
      { id: "e1", source: "input", target: "compress-loop" },
      { id: "e2", source: "compress-loop", target: "output" },
    ],
  },
};
