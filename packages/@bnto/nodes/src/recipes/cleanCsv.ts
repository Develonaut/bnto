/** Clean CSV recipe — remove empty rows, trim whitespace, deduplicate. */

import type { Recipe } from "../recipe";
import { CURRENT_FORMAT_VERSION } from "../formatVersion";
import { getProcessorDefaults } from "../generated/catalog";
import { defaultInputNode } from "./defaultInputNode";
import { defaultOutputNode } from "./defaultOutputNode";

const CSV_INPUT = {
  accept: ["text/csv"],
  extensions: [".csv"],
  label: "CSV files",
  multiple: false,
} as const;

export const cleanCsv: Recipe = {
  id: "5beec7db-f553-4e99-90f3-07be1d419db8",
  slug: "clean-csv",
  name: "Clean CSV",
  description: "Remove empty rows, trim whitespace, deduplicate CSV data. Free, no signup.",
  category: "spreadsheet",
  accept: {
    mimeTypes: ["text/csv"],
    extensions: [".csv"],
    label: "CSV files",
  },
  features: ["CSV", "Remove duplicates", "Trim whitespace", "Browser-based"],
  definition: {
    id: "clean-csv",
    type: "group",
    version: CURRENT_FORMAT_VERSION,
    name: "Clean CSV",
    position: { x: 0, y: 0 },
    metadata: {
      description: "Accepts a CSV file and cleans it.",
    },
    parameters: {},
    inputPorts: [],
    outputPorts: [],
    nodes: [
      defaultInputNode(CSV_INPUT),
      {
        id: "clean",
        type: "spreadsheet-clean",
        version: CURRENT_FORMAT_VERSION,
        name: "Clean",
        position: { x: 250, y: 100 },
        metadata: {},
        parameters: {
          ...getProcessorDefaults("spreadsheet-clean"),
        },
        inputPorts: [{ id: "in-1", name: "files" }],
        outputPorts: [{ id: "out-1", name: "files" }],
      },
      defaultOutputNode({ label: "Cleaned CSV", zip: false }),
    ],
    edges: [
      { id: "e1", source: "input", target: "clean" },
      { id: "e2", source: "clean", target: "output" },
    ],
  },
};
