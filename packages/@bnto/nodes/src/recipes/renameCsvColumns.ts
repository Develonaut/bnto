/**
 * Rename CSV Columns recipe — rename column headers in bulk.
 *
 * Go source: engine/pkg/menu/recipes/rename-csv-columns.json
 */

import type { Recipe } from "../recipe";
import { CURRENT_FORMAT_VERSION } from "../formatVersion";

export const renameCsvColumns: Recipe = {
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
  seo: {
    title: "Rename CSV Columns Online Free -- bnto",
    h1: "Rename CSV Columns Online Free",
  },
  definition: {
    id: "rename-csv-columns",
    type: "group",
    version: CURRENT_FORMAT_VERSION,
    name: "Rename CSV Columns",
    position: { x: 0, y: 0 },
    metadata: {
      description: "Reads a CSV and writes it with column data preserved.",
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
        id: "write-csv",
        type: "spreadsheet",
        version: CURRENT_FORMAT_VERSION,
        name: "Write CSV",
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
        name: "Renamed CSV",
        position: { x: 600, y: 100 },
        metadata: {},
        parameters: {
          mode: "download",
          label: "Renamed CSV",
          zip: false,
          autoDownload: false,
        },
        inputPorts: [{ id: "in-1", name: "files" }],
        outputPorts: [],
      },
    ],
    edges: [
      { id: "e1", source: "input", target: "read-csv" },
      { id: "e2", source: "read-csv", target: "write-csv" },
      { id: "e3", source: "write-csv", target: "output" },
    ],
  },
};
