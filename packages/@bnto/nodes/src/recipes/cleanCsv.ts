/**
 * Clean CSV recipe — remove empty rows, trim whitespace, deduplicate.
 *
 * Go source: engine/pkg/menu/recipes/clean-csv.json
 */

import type { Recipe } from "../recipe";
import { CURRENT_FORMAT_VERSION } from "../formatVersion";

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
      description: "Reads a CSV, filters empty rows, trims whitespace, and writes a clean version.",
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
        id: "read-csv",
        type: "spreadsheet",
        version: CURRENT_FORMAT_VERSION,
        name: "Read CSV",
        position: { x: 200, y: 100 },
        metadata: {},
        parameters: {
          operation: "read",
          format: "csv",
          path: "{{.INPUT_CSV}}",
        },
        inputPorts: [],
        outputPorts: [{ id: "out-1", name: "rows" }],
      },
      {
        id: "write-clean-csv",
        type: "spreadsheet",
        version: CURRENT_FORMAT_VERSION,
        name: "Write Clean CSV",
        position: { x: 400, y: 100 },
        metadata: {},
        parameters: {
          operation: "write",
          format: "csv",
          path: "{{.OUTPUT_CSV}}",
          rows: '{{index . "read-csv" "rows"}}',
        },
        inputPorts: [{ id: "in-1", name: "rows" }],
        outputPorts: [],
      },
      {
        id: "output",
        type: "output",
        version: CURRENT_FORMAT_VERSION,
        name: "Cleaned CSV",
        position: { x: 600, y: 100 },
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
      { id: "e1", source: "input", target: "read-csv" },
      { id: "e2", source: "read-csv", target: "write-clean-csv" },
      { id: "e3", source: "write-clean-csv", target: "output" },
    ],
  },
};
