/** Standardize CSV recipe — clean up and rename column headers in one step. */

import type { Recipe } from "../recipe";
import { CURRENT_FORMAT_VERSION, getProcessorDefaults } from "@bnto/nodes";
import { defaultInputNode } from "./defaultInputNode";
import { defaultOutputNode } from "./defaultOutputNode";

const CSV_INPUT = {
  accept: ["text/csv"],
  extensions: [".csv"],
  label: "CSV files",
  multiple: false,
} as const;

export const standardizeCsv: Recipe = {
  id: "a2b3c4d5-6e7f-8a9b-0c1d-2e3f4a5b6c7d",
  slug: "standardize-csv",
  name: "Standardize CSV",
  description: "Clean up messy CSV data and rename column headers in one step. Free, no signup.",
  category: "spreadsheet",
  accept: {
    mimeTypes: ["text/csv"],
    extensions: [".csv"],
    label: "CSV files",
  },
  features: ["CSV", "Clean", "Rename columns", "Multi-step", "Browser-based"],
  definition: {
    id: "standardize-csv",
    type: "group",
    version: CURRENT_FORMAT_VERSION,
    name: "Standardize CSV",
    position: { x: 0, y: 0 },
    metadata: {
      description: "Accepts a CSV file, cleans it, and renames the column headers.",
    },
    parameters: {},
    inputPorts: [],
    outputPorts: [],
    settings: { iteration: "auto" },
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
      {
        id: "rename-columns",
        type: "spreadsheet-rename",
        version: CURRENT_FORMAT_VERSION,
        name: "Rename",
        position: { x: 500, y: 100 },
        metadata: {},
        parameters: {
          ...getProcessorDefaults("spreadsheet-rename"),
          columns: {},
        },
        inputPorts: [{ id: "in-1", name: "files" }],
        outputPorts: [{ id: "out-1", name: "files" }],
      },
      defaultOutputNode({ label: "Standardized CSV", zip: false }),
    ],
    edges: [
      { id: "e1", source: "input", target: "clean" },
      { id: "e2", source: "clean", target: "rename-columns" },
      { id: "e3", source: "rename-columns", target: "output" },
    ],
  },
};
