/** CSV to JSON recipe — convert CSV data to a JSON array of objects. */

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

export const csvToJson: Recipe = {
  id: "a7c3e1f0-8d42-4b9a-b5e6-3f1a2c4d5e6f",
  slug: "csv-to-json",
  name: "CSV to JSON",
  description: "Convert CSV files to JSON format with configurable delimiters. Free, no signup.",
  category: "spreadsheet",
  accept: {
    mimeTypes: ["text/csv"],
    extensions: [".csv"],
    label: "CSV files",
  },
  features: ["CSV", "JSON", "Configurable delimiter", "Browser-based"],
  definition: {
    id: "csv-to-json",
    type: "group",
    version: CURRENT_FORMAT_VERSION,
    name: "CSV to JSON",
    position: { x: 0, y: 0 },
    metadata: {
      description: "Accepts a CSV file and converts it to JSON.",
    },
    parameters: {},
    inputPorts: [],
    outputPorts: [],
    settings: { iteration: "auto" },
    nodes: [
      defaultInputNode(CSV_INPUT),
      {
        id: "convert",
        type: "spreadsheet-convert",
        version: CURRENT_FORMAT_VERSION,
        name: "Convert to JSON",
        position: { x: 250, y: 100 },
        metadata: {},
        parameters: {
          ...getProcessorDefaults("spreadsheet-convert"),
        },
        inputPorts: [{ id: "in-1", name: "files" }],
        outputPorts: [{ id: "out-1", name: "files" }],
      },
      defaultOutputNode({ label: "JSON file", zip: false }),
    ],
    edges: [
      { id: "e1", source: "input", target: "convert" },
      { id: "e2", source: "convert", target: "output" },
    ],
  },
};
