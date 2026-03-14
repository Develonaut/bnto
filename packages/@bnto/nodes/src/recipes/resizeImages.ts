/** Resize Images recipe — resize to exact dimensions or percentages. */

import type { Recipe } from "../recipe";
import { CURRENT_FORMAT_VERSION } from "../formatVersion";
import { getProcessorDefaults } from "../generated/catalog";

export const resizeImages: Recipe = {
  slug: "resize-images",
  name: "Resize Images",
  description: "Resize images to exact dimensions or percentages. Free, no signup required.",
  category: "image",
  accept: {
    mimeTypes: ["image/jpeg", "image/png", "image/webp"],
    extensions: [".jpg", ".jpeg", ".png", ".webp"],
    label: "JPEG, PNG, or WebP images",
    mimePrefix: "image/",
  },
  features: ["PNG", "JPEG", "WebP", "Custom dimensions", "Browser-based"],
  seo: {
    title: "Resize Images Online Free -- bnto",
    h1: "Resize Images Online Free",
  },
  definition: {
    id: "resize-images",
    type: "group",
    version: CURRENT_FORMAT_VERSION,
    name: "Resize Images",
    position: { x: 0, y: 0 },
    metadata: {
      description: "Accepts image files and resizes each one.",
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
        id: "resize-loop",
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
            id: "resize-image",
            type: "image",
            version: CURRENT_FORMAT_VERSION,
            name: "Resize",
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
        name: "Output",
        position: { x: 500, y: 100 },
        metadata: {},
        parameters: {
          mode: "download",
          label: "Resized Images",
          zip: true,
          autoDownload: true,
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
