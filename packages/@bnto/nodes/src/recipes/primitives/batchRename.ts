/** Batch Rename sub-recipe — loops over files and renames each one. */

import type { Definition } from "../../definition";
import { CURRENT_FORMAT_VERSION } from "../../formatVersion";
import { getProcessorDefaults } from "../../generated/catalog";

export const batchRename: Definition = {
  id: "batch-rename",
  type: "group",
  version: CURRENT_FORMAT_VERSION,
  name: "Batch Rename",
  position: { x: 250, y: 100 },
  metadata: {
    description: "Reusable sub-recipe: loops over files and renames each one.",
  },
  parameters: {},
  inputPorts: [{ id: "in-1", name: "files" }],
  outputPorts: [{ id: "out-1", name: "files" }],
  nodes: [
    {
      id: "rename-loop",
      type: "loop",
      version: CURRENT_FORMAT_VERSION,
      name: "Rename Each File",
      position: { x: 0, y: 0 },
      metadata: {},
      parameters: { mode: "forEach" },
      inputPorts: [{ id: "in-1", name: "items" }],
      outputPorts: [],
      nodes: [
        {
          id: "rename-file",
          type: "file-system",
          version: CURRENT_FORMAT_VERSION,
          name: "Rename File",
          position: { x: 0, y: 0 },
          metadata: {},
          parameters: {
            operation: "rename",
            ...getProcessorDefaults("file-system", "rename"),
            prefix: "renamed-", // recipe choice — default rename prefix
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
