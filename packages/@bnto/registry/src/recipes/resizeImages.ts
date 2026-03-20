/** Resize Images recipe — resize to exact dimensions or percentages. */

import type { Recipe } from "../recipe";
import { CURRENT_FORMAT_VERSION, getProcessorDefaults } from "@bnto/nodes";
import { defaultInputNode } from "./defaultInputNode";
import { defaultOutputNode } from "./defaultOutputNode";

const IMAGE_INPUT = {
  accept: ["image/jpeg", "image/png", "image/webp"],
  extensions: [".jpg", ".jpeg", ".png", ".webp"],
  label: "JPEG, PNG, or WebP images",
} as const;

export const resizeImages: Recipe = {
  id: "ea8bec03-b732-4bc6-aeec-1097f75c0b87",
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
      defaultInputNode(IMAGE_INPUT),
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
            type: "image-resize",
            version: CURRENT_FORMAT_VERSION,
            name: "Resize",
            position: { x: 0, y: 0 },
            metadata: {},
            parameters: {
              ...getProcessorDefaults("image-resize"),
              width: 200,
            },
            inputPorts: [],
            outputPorts: [],
          },
        ],
        edges: [],
      },
      defaultOutputNode({ label: "Resized Images" }),
    ],
    edges: [
      { id: "e1", source: "input", target: "resize-loop" },
      { id: "e2", source: "resize-loop", target: "output" },
    ],
  },
};
