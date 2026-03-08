/** CSV Cleaner sub-recipe — trims whitespace, removes empty rows, deduplicates. */

import type { Definition } from "../../definition";
import { CURRENT_FORMAT_VERSION } from "../../formatVersion";
import { getProcessorDefaults } from "../../generated/catalog";

export const csvCleaner: Definition = {
  id: "csv-cleaner",
  type: "group",
  version: CURRENT_FORMAT_VERSION,
  name: "CSV Cleaner",
  position: { x: 250, y: 100 },
  metadata: {
    description: "Reusable sub-recipe: trims whitespace, removes empty rows, deduplicates.",
  },
  parameters: {},
  inputPorts: [{ id: "in-1", name: "files" }],
  outputPorts: [{ id: "out-1", name: "files" }],
  nodes: [
    {
      id: "clean",
      type: "spreadsheet",
      version: CURRENT_FORMAT_VERSION,
      name: "Clean CSV",
      position: { x: 0, y: 0 },
      metadata: {},
      parameters: {
        operation: "clean",
        ...getProcessorDefaults("spreadsheet", "clean"),
      },
      inputPorts: [{ id: "in-1", name: "files" }],
      outputPorts: [{ id: "out-1", name: "files" }],
    },
  ],
  edges: [],
};
