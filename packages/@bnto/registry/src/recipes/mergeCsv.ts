/** Merge CSV recipe — combine multiple CSV files into a single merged file. */

import type { Recipe } from "../recipe";
import { CURRENT_FORMAT_VERSION, getProcessorDefaults } from "@bnto/nodes";
import { defaultInputNode } from "./defaultInputNode";
import { defaultOutputNode } from "./defaultOutputNode";

const CSV_INPUT = {
  accept: ["text/csv"],
  extensions: [".csv"],
  label: "CSV files",
  multiple: true,
} as const;

export const mergeCsv: Recipe = {
  id: "b4d5e6f7-8a9b-0c1d-2e3f-4a5b6c7d8e9f",
  slug: "merge-csv",
  name: "Merge CSV",
  description: "Combine multiple CSV files into one with header reconciliation. Free, no signup.",
  category: "spreadsheet",
  accept: {
    mimeTypes: ["text/csv"],
    extensions: [".csv"],
    label: "CSV files",
  },
  features: ["CSV", "Merge", "Header reconciliation", "Deduplication", "Browser-based"],
  definition: {
    id: "merge-csv",
    type: "group",
    version: CURRENT_FORMAT_VERSION,
    name: "Merge CSV",
    position: { x: 0, y: 0 },
    metadata: {
      description: "Accepts multiple CSV files and merges them into one.",
    },
    parameters: {},
    inputPorts: [],
    outputPorts: [],
    settings: { iteration: "auto" },
    nodes: [
      defaultInputNode(CSV_INPUT),
      {
        id: "merge",
        type: "spreadsheet-merge",
        version: CURRENT_FORMAT_VERSION,
        name: "Merge",
        position: { x: 250, y: 100 },
        metadata: {},
        parameters: {
          ...getProcessorDefaults("spreadsheet-merge"),
        },
        inputPorts: [{ id: "in-1", name: "files" }],
        outputPorts: [{ id: "out-1", name: "files" }],
      },
      defaultOutputNode({ label: "Merged CSV", zip: false }),
    ],
    edges: [
      { id: "e1", source: "input", target: "merge" },
      { id: "e2", source: "merge", target: "output" },
    ],
  },
};
