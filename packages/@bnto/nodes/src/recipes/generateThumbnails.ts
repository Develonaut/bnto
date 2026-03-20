/** Generate Thumbnails recipe — resize, convert to WebP, and rename with prefix. */

import type { Recipe } from "../recipe";
import { CURRENT_FORMAT_VERSION } from "../formatVersion";
import { getProcessorDefaults } from "../generated/catalog";
import { defaultInputNode } from "./defaultInputNode";
import { defaultOutputNode } from "./defaultOutputNode";

const IMAGE_INPUT = {
  accept: ["image/jpeg", "image/png", "image/webp"],
  extensions: [".jpg", ".jpeg", ".png", ".webp"],
  label: "JPEG, PNG, or WebP images",
} as const;

export const generateThumbnails: Recipe = {
  id: "4cf63f58-327a-4e6a-a03c-caa9679c7152",
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
      defaultInputNode(IMAGE_INPUT),
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
            type: "image-resize",
            version: CURRENT_FORMAT_VERSION,
            name: "Resize",
            position: { x: 0, y: 0 },
            metadata: {},
            parameters: {
              ...getProcessorDefaults("image-resize"),
              width: 150,
            },
            inputPorts: [],
            outputPorts: [],
          },
          {
            id: "convert",
            type: "image-convert",
            version: CURRENT_FORMAT_VERSION,
            name: "Convert",
            position: { x: 250, y: 0 },
            metadata: {},
            parameters: {
              ...getProcessorDefaults("image-convert"),
              format: "webp",
            },
            inputPorts: [],
            outputPorts: [],
          },
          {
            id: "rename",
            type: "file-rename",
            version: CURRENT_FORMAT_VERSION,
            name: "Rename",
            position: { x: 500, y: 0 },
            metadata: {},
            parameters: {
              ...getProcessorDefaults("file-rename"),
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
      defaultOutputNode({ label: "Thumbnails" }),
    ],
    edges: [
      { id: "e1", source: "input", target: "process-loop" },
      { id: "e2", source: "process-loop", target: "output" },
    ],
  },
};
