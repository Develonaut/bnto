/** Batch Compress sub-recipe — loops over image files and compresses each one. */

import type { Definition } from "../../definition";
import { CURRENT_FORMAT_VERSION } from "../../formatVersion";
import { getProcessorDefaults } from "../../generated/catalog";

export const batchCompress: Definition = {
  id: "batch-compress",
  type: "group",
  version: CURRENT_FORMAT_VERSION,
  name: "Batch Compress",
  position: { x: 250, y: 100 },
  metadata: {
    description: "Reusable sub-recipe: loops over files and compresses each one.",
  },
  parameters: {},
  inputPorts: [{ id: "in-1", name: "files" }],
  outputPorts: [{ id: "out-1", name: "files" }],
  nodes: [
    {
      id: "compress-loop",
      type: "loop",
      version: CURRENT_FORMAT_VERSION,
      name: "Compress Each Image",
      position: { x: 0, y: 0 },
      metadata: {},
      parameters: { mode: "forEach" },
      inputPorts: [{ id: "in-1", name: "items" }],
      outputPorts: [],
      nodes: [
        {
          id: "compress-image",
          type: "image",
          version: CURRENT_FORMAT_VERSION,
          name: "Compress Image",
          position: { x: 0, y: 0 },
          metadata: {},
          parameters: {
            operation: "compress",
            ...getProcessorDefaults("image", "compress"),
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
