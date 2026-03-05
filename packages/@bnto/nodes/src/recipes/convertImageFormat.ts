/**
 * Convert Image Format recipe — convert between PNG, JPEG, WebP, and GIF.
 *
 * Go source: engine/pkg/menu/recipes/convert-image-format.json
 */

import type { Recipe } from "../recipe";

export const convertImageFormat: Recipe = {
  slug: "convert-image-format",
  name: "Convert Image Format",
  description:
    "Convert between PNG, JPEG, WebP, and GIF formats instantly. Free, no signup.",
  category: "image",
  accept: {
    mimeTypes: ["image/jpeg", "image/png", "image/webp", "image/gif"],
    extensions: [".jpg", ".jpeg", ".png", ".webp", ".gif"],
    label: "JPEG, PNG, WebP, or GIF images",
    mimePrefix: "image/",
  },
  features: ["PNG", "JPEG", "WebP", "GIF", "Browser-based"],
  seo: {
    title: "Convert Image Format Online Free -- bnto",
    h1: "Convert Image Format Online Free",
  },
  definition: {
    id: "convert-image-format",
    type: "group",
    version: "1.0.0",
    name: "Convert Image Format",
    position: { x: 0, y: 0 },
    metadata: {
      description:
        "Lists image files and converts each to a target format.",
    },
    parameters: {},
    inputPorts: [],
    outputPorts: [],
    nodes: [
      {
        id: "input",
        type: "input",
        version: "1.0.0",
        name: "Input Files",
        position: { x: 0, y: 100 },
        metadata: {},
        parameters: {
          mode: "file-upload",
          accept: ["image/jpeg", "image/png", "image/webp", "image/gif"],
          extensions: [".jpg", ".jpeg", ".png", ".webp", ".gif"],
          label: "JPEG, PNG, WebP, or GIF images",
          multiple: true,
        },
        inputPorts: [],
        outputPorts: [{ id: "out-1", name: "files" }],
      },
      {
        id: "convert-loop",
        type: "loop",
        version: "1.0.0",
        name: "Convert Each Image",
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
            id: "convert-image",
            type: "image",
            version: "1.0.0",
            name: "Convert Image",
            position: { x: 100, y: 100 },
            metadata: {},
            parameters: {
              operation: "convert",
              input: "{{.item}}",
              output: "{{.OUTPUT_DIR}}/{{basename .item}}.webp",
              format: "webp",
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
        version: "1.0.0",
        name: "Converted Images",
        position: { x: 500, y: 100 },
        metadata: {},
        parameters: {
          mode: "download",
          label: "Converted Images",
          zip: true,
          autoDownload: false,
        },
        inputPorts: [{ id: "in-1", name: "files" }],
        outputPorts: [],
      },
    ],
    edges: [
      { id: "e1", source: "input", target: "convert-loop" },
      { id: "e2", source: "convert-loop", target: "output" },
    ],
  },
};
