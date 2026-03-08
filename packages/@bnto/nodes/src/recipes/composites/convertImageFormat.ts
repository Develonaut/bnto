/** Convert Image Format recipe — convert between PNG, JPEG, WebP, and GIF. */

import type { Recipe } from "../../recipe";
import { CURRENT_FORMAT_VERSION } from "../../formatVersion";
import { batchConvert } from "../primitives/batchConvert";

export const convertImageFormat: Recipe = {
  slug: "convert-image-format",
  name: "Convert Image Format",
  description: "Convert between PNG, JPEG, WebP, and GIF formats instantly. Free, no signup.",
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
    version: CURRENT_FORMAT_VERSION,
    name: "Convert Image Format",
    position: { x: 0, y: 0 },
    metadata: {
      description:
        "Accepts image files and converts each to a target format using a reusable batch conversion sub-recipe.",
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
      batchConvert,
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
      { id: "e1", source: "input", target: "batch-convert" },
      { id: "e2", source: "batch-convert", target: "output" },
    ],
  },
};
