/** Batch Resize sub-recipe — loops over image files and resizes each one. */

import type { Definition } from "../../definition";
import { CURRENT_FORMAT_VERSION } from "../../formatVersion";
import { getProcessorDefaults } from "../../generated/catalog";

export const batchResize: Definition = {
  id: "batch-resize",
  type: "group",
  version: CURRENT_FORMAT_VERSION,
  name: "Batch Resize",
  position: { x: 250, y: 100 },
  metadata: {
    description: "Reusable sub-recipe: loops over files and resizes each one.",
  },
  parameters: {},
  inputPorts: [{ id: "in-1", name: "files" }],
  outputPorts: [{ id: "out-1", name: "files" }],
  nodes: [
    {
      id: "resize-loop",
      type: "loop",
      version: CURRENT_FORMAT_VERSION,
      name: "Resize Each Image",
      position: { x: 0, y: 0 },
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
            width: 200, // recipe choice — default resize target
          },
          inputPorts: [],
          outputPorts: [],
        },
      ],
      edges: [],
    },
  ],
  edges: [],
};
