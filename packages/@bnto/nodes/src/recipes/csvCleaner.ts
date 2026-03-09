/** CSV Cleaner recipe — trims whitespace, removes empty rows, deduplicates. */

import type { Recipe } from "../recipe";
import { CURRENT_FORMAT_VERSION } from "../formatVersion";
import { getProcessorDefaults } from "../generated/catalog";

export const csvCleaner: Recipe = {
  slug: "csv-cleaner",
  name: "CSV Cleaner",
  description:
    "Clean a CSV file by trimming whitespace, removing empty rows, and deduplicating. Reusable building block.",
  category: "spreadsheet",
  accept: {
    mimeTypes: ["text/csv"],
    extensions: [".csv"],
    label: "CSV files",
  },
  features: ["CSV", "Trim whitespace", "Remove duplicates"],
  seo: {
    title: "CSV Cleaner -- bnto",
    h1: "CSV Cleaner",
  },
  definition: {
    id: "csv-cleaner",
    type: "group",
    version: CURRENT_FORMAT_VERSION,
    name: "CSV Cleaner",
    position: { x: 0, y: 0 },
    metadata: {
      description: "Trims whitespace, removes empty rows, deduplicates.",
      customData: { displayName: "CSV Cleaner" },
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
      {
        id: "clean",
        type: "spreadsheet",
        version: CURRENT_FORMAT_VERSION,
        name: "Clean CSV",
        position: { x: 250, y: 100 },
        metadata: {},
        parameters: {
          operation: "clean",
          ...getProcessorDefaults("spreadsheet", "clean"),
        },
        inputPorts: [{ id: "in-1", name: "files" }],
        outputPorts: [{ id: "out-1", name: "files" }],
      },
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
      { id: "e1", source: "input", target: "clean" },
      { id: "e2", source: "clean", target: "output" },
    ],
  },
};
