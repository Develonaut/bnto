/**
 * Compress Images recipe — optimize PNG, JPEG, and WebP images.
 *
 * Go source: engine/pkg/menu/recipes/compress-images.json
 */

import type { Recipe } from "../recipe";

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
    version: "1.0.0",
    name: "Compress Images",
    position: { x: 0, y: 0 },
    metadata: {
      description:
        "Lists image files and compresses each one to WebP format.",
    },
    parameters: {},
    inputPorts: [],
    outputPorts: [],
    nodes: [
      {
        id: "list-images",
        type: "file-system",
        version: "1.0.0",
        name: "List Images",
        position: { x: 100, y: 100 },
        metadata: {},
        parameters: { operation: "list", path: "{{.INPUT_DIR}}/*" },
        inputPorts: [],
        outputPorts: [{ id: "out-1", name: "files" }],
      },
      {
        id: "compress-loop",
        type: "loop",
        version: "1.0.0",
        name: "Compress Each Image",
        position: { x: 300, y: 100 },
        metadata: {},
        parameters: {
          mode: "forEach",
          items: '{{index . "list-images" "files"}}',
        },
        inputPorts: [{ id: "in-1", name: "items" }],
        outputPorts: [],
        nodes: [
          {
            id: "compress-image",
            type: "image",
            version: "1.0.0",
            name: "Compress Image",
            position: { x: 100, y: 100 },
            metadata: {},
            parameters: {
              operation: "optimize",
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
    ],
    edges: [{ id: "e1", source: "list-images", target: "compress-loop" }],
  },
};
