/** Resize Images recipe — resize to exact dimensions or percentages. */

import type { Recipe } from "../../recipe";
import { CURRENT_FORMAT_VERSION } from "../../formatVersion";
import { batchResize } from "../primitives/batchResize";

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
      description:
        "Accepts image files and resizes each one using a reusable batch resize sub-recipe.",
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
      batchResize,
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
      { id: "e1", source: "input", target: "batch-resize" },
      { id: "e2", source: "batch-resize", target: "output" },
    ],
  },
};
