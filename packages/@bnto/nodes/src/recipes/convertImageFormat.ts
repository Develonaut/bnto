/** Convert Image Format recipe — convert between PNG, JPEG, WebP, and GIF. */

import type { Recipe } from "../recipe";
import { CURRENT_FORMAT_VERSION } from "../formatVersion";
import { getProcessorDefaults } from "../generated/catalog";
import { defaultInputNode } from "./defaultInputNode";
import { defaultOutputNode } from "./defaultOutputNode";

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
      description: "Accepts image files and converts each to a target format.",
    },
    parameters: {},
    inputPorts: [],
    outputPorts: [],
    nodes: [
      defaultInputNode({
        accept: ["image/jpeg", "image/png", "image/webp", "image/gif"],
        extensions: [".jpg", ".jpeg", ".png", ".webp", ".gif"],
        label: "JPEG, PNG, WebP, or GIF images",
      }),
      {
        id: "convert-loop",
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
            id: "convert-image",
            type: "image",
            version: CURRENT_FORMAT_VERSION,
            name: "Convert",
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
      defaultOutputNode({ label: "Converted Images" }),
    ],
    edges: [
      { id: "e1", source: "input", target: "convert-loop" },
      { id: "e2", source: "convert-loop", target: "output" },
    ],
  },
};
