/** Rename CSV Columns recipe — rename column headers in bulk. */

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

export const renameCsvColumns: Recipe = {
  id: "5be92d39-fa8e-49ce-a0f3-c55d01a48c7b",
  slug: "rename-csv-columns",
  name: "Rename CSV Columns",
  description: "Rename CSV column headers in bulk. Free, no signup required.",
  category: "spreadsheet",
  accept: {
    mimeTypes: ["text/csv"],
    extensions: [".csv"],
    label: "CSV files",
  },
  features: ["CSV", "Column rename", "Bulk edit", "Browser-based"],
  definition: {
    id: "rename-csv-columns",
    type: "group",
    version: CURRENT_FORMAT_VERSION,
    name: "Rename CSV Columns",
    position: { x: 0, y: 0 },
    metadata: {
      description: "Accepts a CSV file and renames its column headers.",
    },
    parameters: {},
    inputPorts: [],
    outputPorts: [],
    settings: { iteration: "auto" },
    nodes: [
      defaultInputNode(CSV_INPUT),
      {
        id: "rename-columns",
        type: "spreadsheet-rename",
        version: CURRENT_FORMAT_VERSION,
        name: "Rename",
        position: { x: 250, y: 100 },
        metadata: {},
        parameters: {
          ...getProcessorDefaults("spreadsheet-rename"),
          columns: {},
        },
        inputPorts: [{ id: "in-1", name: "files" }],
        outputPorts: [{ id: "out-1", name: "files" }],
      },
      defaultOutputNode({ label: "Renamed CSV", zip: false }),
    ],
    edges: [
      { id: "e1", source: "input", target: "rename-columns" },
      { id: "e2", source: "rename-columns", target: "output" },
    ],
  },
};
