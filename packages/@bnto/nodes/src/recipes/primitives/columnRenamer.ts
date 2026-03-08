/** Column Renamer sub-recipe — renames column headers in a CSV file. */

import type { Definition } from "../../definition";
import { CURRENT_FORMAT_VERSION } from "../../formatVersion";
import { getProcessorDefaults } from "../../generated/catalog";

export const columnRenamer: Definition = {
  id: "column-renamer",
  type: "group",
  version: CURRENT_FORMAT_VERSION,
  name: "Column Renamer",
  position: { x: 250, y: 100 },
  metadata: {
    description: "Reusable sub-recipe: renames column headers in a CSV file.",
  },
  parameters: {},
  inputPorts: [{ id: "in-1", name: "files" }],
  outputPorts: [{ id: "out-1", name: "files" }],
  nodes: [
    {
      id: "rename-columns",
      type: "spreadsheet",
      version: CURRENT_FORMAT_VERSION,
      name: "Rename Columns",
      position: { x: 0, y: 0 },
      metadata: {},
      parameters: {
        operation: "rename",
        ...getProcessorDefaults("spreadsheet", "rename"),
        columns: {}, // recipe choice — empty by default, user fills in
      },
      inputPorts: [{ id: "in-1", name: "files" }],
      outputPorts: [{ id: "out-1", name: "files" }],
    },
  ],
  edges: [],
};
