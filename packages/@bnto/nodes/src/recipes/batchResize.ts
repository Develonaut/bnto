/** Batch Resize recipe — loops over image files and resizes each one. */

import type { Recipe } from "../recipe";
import { CURRENT_FORMAT_VERSION } from "../formatVersion";
import { getProcessorDefaults } from "../generated/catalog";

export const batchResize: Recipe = {
  slug: "batch-resize",
  name: "Batch Resize",
  description: "Resize multiple images in a single batch. Reusable building block.",
  category: "image",
  accept: {
    mimeTypes: ["image/jpeg", "image/png", "image/webp"],
    extensions: [".jpg", ".jpeg", ".png", ".webp"],
    label: "JPEG, PNG, or WebP images",
    mimePrefix: "image/",
  },
  features: ["PNG", "JPEG", "WebP", "Custom dimensions"],
  seo: {
    title: "Batch Resize Images -- bnto",
    h1: "Batch Resize Images",
  },
  definition: {
    id: "batch-resize",
    type: "group",
    version: CURRENT_FORMAT_VERSION,
    name: "Batch Resize",
    position: { x: 0, y: 0 },
    metadata: {
      description: "Loops over files and resizes each one.",
      customData: { displayName: "Batch Resize" },
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
        id: "resize-loop",
        type: "loop",
        version: CURRENT_FORMAT_VERSION,
        name: "Resize Each Image",
        position: { x: 250, y: 100 },
        metadata: {},
        parameters: { mode: "forEach" },
        inputPorts: [{ id: "in-1", name: "items" }],
        outputPorts: [],
        nodes: [
          {
            id: "resize-image",
            type: "image",
            version: CURRENT_FORMAT_VERSION,
            name: "Resize Image",
            position: { x: 0, y: 0 },
            metadata: {},
            parameters: {
              operation: "resize",
              ...getProcessorDefaults("image", "resize"),
              width: 200,
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
        name: "Resized Images",
        position: { x: 500, y: 100 },
        metadata: {},
        parameters: {
          mode: "download",
          label: "Resized Images",
          zip: true,
          autoDownload: false,
        },
        inputPorts: [{ id: "in-1", name: "files" }],
        outputPorts: [],
      },
    ],
    edges: [
      { id: "e1", source: "input", target: "resize-loop" },
      { id: "e2", source: "resize-loop", target: "output" },
    ],
  },
};
