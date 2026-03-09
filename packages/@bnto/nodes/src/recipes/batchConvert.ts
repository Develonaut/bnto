/** Batch Convert recipe — loops over image files and converts each to a target format. */

import type { Recipe } from "../recipe";
import { CURRENT_FORMAT_VERSION } from "../formatVersion";
import { getProcessorDefaults } from "../generated/catalog";

export const batchConvert: Recipe = {
  slug: "batch-convert",
  name: "Batch Convert",
  description:
    "Convert multiple images to a target format in a single batch. Reusable building block.",
  category: "image",
  accept: {
    mimeTypes: ["image/jpeg", "image/png", "image/webp", "image/gif"],
    extensions: [".jpg", ".jpeg", ".png", ".webp", ".gif"],
    label: "JPEG, PNG, WebP, or GIF images",
    mimePrefix: "image/",
  },
  features: ["PNG", "JPEG", "WebP", "GIF", "Batch processing"],
  seo: {
    title: "Batch Convert Images -- bnto",
    h1: "Batch Convert Images",
  },
  definition: {
    id: "batch-convert",
    type: "group",
    version: CURRENT_FORMAT_VERSION,
    name: "Batch Convert",
    position: { x: 0, y: 0 },
    metadata: {
      description: "Loops over files and converts each to the target format.",
      customData: { displayName: "Batch Convert" },
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
        version: CURRENT_FORMAT_VERSION,
        name: "Convert Each Image",
        position: { x: 250, y: 100 },
        metadata: {},
        parameters: { mode: "forEach" },
        inputPorts: [{ id: "in-1", name: "items" }],
        outputPorts: [],
        nodes: [
          {
            id: "convert-image",
            type: "image",
            version: CURRENT_FORMAT_VERSION,
            name: "Convert Image",
            position: { x: 0, y: 0 },
            metadata: {},
            parameters: {
              operation: "convert",
              ...getProcessorDefaults("image", "convert"),
              format: "webp",
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
