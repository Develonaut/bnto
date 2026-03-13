/** Generate Thumbnails recipe — resize, convert to WebP, and rename with prefix. */

import type { Recipe } from "../recipe";
import { CURRENT_FORMAT_VERSION } from "../formatVersion";
import { getProcessorDefaults } from "../generated/catalog";

export const generateThumbnails: Recipe = {
  slug: "generate-thumbnails",
  name: "Generate Thumbnails",
  description:
    "Resize images to thumbnail size, convert to WebP, and add a prefix. Free, no signup.",
  category: "image",
  accept: {
    mimeTypes: ["image/jpeg", "image/png", "image/webp"],
    extensions: [".jpg", ".jpeg", ".png", ".webp"],
    label: "JPEG, PNG, or WebP images",
    mimePrefix: "image/",
  },
  features: ["Thumbnails", "Resize", "WebP", "Multi-step", "Browser-based"],
  seo: {
    title: "Generate Thumbnails Online Free -- bnto",
    h1: "Generate Thumbnails Online Free",
  },
  definition: {
    id: "generate-thumbnails",
    type: "group",
    version: CURRENT_FORMAT_VERSION,
    name: "Generate Thumbnails",
    position: { x: 0, y: 0 },
    metadata: {
      description:
        "Accepts image files, resizes to thumbnail dimensions, converts to WebP, and renames with a prefix.",
    },
    parameters: {},
    inputPorts: [],
    outputPorts: [],
    nodes: [
      {
        id: "input",
        type: "input",
        version: CURRENT_FORMAT_VERSION,
        name: "Input",
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
        id: "process-loop",
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
            id: "resize",
            type: "image",
            version: CURRENT_FORMAT_VERSION,
            name: "Resize",
            position: { x: 0, y: 0 },
            metadata: {},
            parameters: {
              operation: "resize",
              ...getProcessorDefaults("image", "resize"),
              width: 150,
            },
            inputPorts: [],
            outputPorts: [],
          },
          {
            id: "convert",
            type: "image",
            version: CURRENT_FORMAT_VERSION,
            name: "Convert",
            position: { x: 250, y: 0 },
            metadata: {},
            parameters: {
              operation: "convert",
              ...getProcessorDefaults("image", "convert"),
              format: "webp",
            },
            inputPorts: [],
            outputPorts: [],
          },
          {
            id: "rename",
            type: "file-system",
            version: CURRENT_FORMAT_VERSION,
            name: "Rename",
            position: { x: 500, y: 0 },
            metadata: {},
            parameters: {
              operation: "rename",
              ...getProcessorDefaults("file-system", "rename"),
              prefix: "thumb_",
            },
            inputPorts: [],
            outputPorts: [],
          },
        ],
        edges: [
          { id: "le1", source: "resize", target: "convert" },
          { id: "le2", source: "convert", target: "rename" },
        ],
      },
      {
        id: "output",
        type: "output",
        version: CURRENT_FORMAT_VERSION,
        name: "Output",
        position: { x: 500, y: 100 },
        metadata: {},
        parameters: {
          mode: "download",
          label: "Thumbnails",
          zip: true,
          autoDownload: false,
        },
        inputPorts: [{ id: "in-1", name: "files" }],
        outputPorts: [],
      },
    ],
    edges: [
      { id: "e1", source: "input", target: "process-loop" },
      { id: "e2", source: "process-loop", target: "output" },
    ],
  },
};
