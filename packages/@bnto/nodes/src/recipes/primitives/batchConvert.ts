/** Batch Convert sub-recipe — loops over image files and converts each to a target format. */

import type { Definition } from "../../definition";
import { CURRENT_FORMAT_VERSION } from "../../formatVersion";
import { getProcessorDefaults } from "../../generated/catalog";

export const batchConvert: Definition = {
  id: "batch-convert",
  type: "group",
  version: CURRENT_FORMAT_VERSION,
  name: "Batch Convert",
  position: { x: 250, y: 100 },
  metadata: {
    description: "Reusable sub-recipe: loops over files and converts each to the target format.",
  },
  parameters: {},
  inputPorts: [{ id: "in-1", name: "files" }],
  outputPorts: [{ id: "out-1", name: "files" }],
  nodes: [
    {
      id: "convert-loop",
      type: "loop",
      version: CURRENT_FORMAT_VERSION,
      name: "Convert Each Image",
      position: { x: 0, y: 0 },
      metadata: {},
      parameters: { mode: "forEach" },
      inputPorts: [{ id: "in-1", name: "items" }],
      outputPorts: [],
      nodes: [
        {
          id: "convert-image",
          type: "image",
          version: CURRENT_FORMAT_VERSION,
          name: "Convert Image",
          position: { x: 0, y: 0 },
          metadata: {},
          parameters: {
            operation: "convert",
            ...getProcessorDefaults("image", "convert"),
            format: "webp", // recipe choice — override engine default
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
