/**
 * Compress Images recipe — optimize PNG, JPEG, and WebP images.
 *
 * Go source: engine/pkg/menu/recipes/compress-images.json
 */

import type { Recipe } from "../recipe";
import { CURRENT_FORMAT_VERSION } from "../formatVersion";

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
      description: "Lists image files and compresses each one to WebP format.",
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
        position: { x: 300, y: 100 },
        metadata: {},
        parameters: {
          mode: "forEach",
          items: '{{index . "input" "files"}}',
        },
        inputPorts: [{ id: "in-1", name: "items" }],
        outputPorts: [],
        nodes: [
          {
            id: "compress-image",
            type: "image",
            version: CURRENT_FORMAT_VERSION,
            name: "Compress Image",
            position: { x: 100, y: 100 },
            metadata: {},
            parameters: {
              operation: "compress",
              input: "{{.item}}",
              output: "{{.OUTPUT_DIR}}/{{basename .item}}",
              quality: 80,
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
