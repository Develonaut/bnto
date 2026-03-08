/** Clean CSV recipe — remove empty rows, trim whitespace, deduplicate. */

import type { Recipe } from "../../recipe";
import { CURRENT_FORMAT_VERSION } from "../../formatVersion";
import { csvCleaner } from "../primitives/csvCleaner";

export const cleanCsv: Recipe = {
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
  seo: {
    title: "Clean CSV Online Free -- bnto",
    h1: "Clean CSV Online Free",
  },
  definition: {
    id: "clean-csv",
    type: "group",
    version: CURRENT_FORMAT_VERSION,
    name: "Clean CSV",
    position: { x: 0, y: 0 },
    metadata: {
      description: "Accepts a CSV file and cleans it using a reusable CSV cleaner sub-recipe.",
    },
    parameters: {},
    inputPorts: [],
    outputPorts: [],
    nodes: [
      {
        id: "input",
        type: "input",
        version: CURRENT_FORMAT_VERSION,
        name: "Input Files",
        position: { x: 0, y: 100 },
        metadata: {},
        parameters: {
          mode: "file-upload",
          accept: ["text/csv"],
          extensions: [".csv"],
          label: "CSV files",
          multiple: false,
        },
        inputPorts: [],
        outputPorts: [{ id: "out-1", name: "files" }],
      },
      csvCleaner,
      {
        id: "output",
        type: "output",
        version: CURRENT_FORMAT_VERSION,
        name: "Cleaned CSV",
        position: { x: 500, y: 100 },
        metadata: {},
        parameters: {
          mode: "download",
          label: "Cleaned CSV",
          zip: false,
          autoDownload: false,
        },
        inputPorts: [{ id: "in-1", name: "files" }],
        outputPorts: [],
      },
    ],
    edges: [
      { id: "e1", source: "input", target: "csv-cleaner" },
      { id: "e2", source: "csv-cleaner", target: "output" },
    ],
  },
};
