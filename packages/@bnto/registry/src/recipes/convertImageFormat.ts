/** Convert Image Format recipe — convert between PNG, JPEG, WebP, and GIF. */

import type { Recipe } from "../recipe";
import { CURRENT_FORMAT_VERSION, getProcessorDefaults } from "@bnto/nodes";
import { defaultInputNode } from "./defaultInputNode";
import { defaultOutputNode } from "./defaultOutputNode";

export const convertImageFormat: Recipe = {
  id: "9cf4fa85-9145-49f7-b838-dae759261ba2",
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
    settings: { iteration: "auto" },
    nodes: [
      defaultInputNode({
        accept: ["image/jpeg", "image/png", "image/webp", "image/gif"],
        extensions: [".jpg", ".jpeg", ".png", ".webp", ".gif"],
        label: "JPEG, PNG, WebP, or GIF images",
      }),
      {
        id: "convert-image",
        type: "image-convert",
        version: CURRENT_FORMAT_VERSION,
        name: "Convert",
        position: { x: 250, y: 100 },
        metadata: {},
        parameters: {
          ...getProcessorDefaults("image-convert"),
          format: "webp",
        },
        inputPorts: [],
        outputPorts: [],
      },
      defaultOutputNode({ label: "Converted Images" }),
    ],
    edges: [
      { id: "e1", source: "input", target: "convert-image" },
      { id: "e2", source: "convert-image", target: "output" },
    ],
  },
};
