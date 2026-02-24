/**
 * Rename CSV Columns recipe — rename column headers in bulk.
 *
 * Go source: engine/pkg/menu/recipes/rename-csv-columns.json
 */

import type { Recipe } from "../recipe";

export const renameCsvColumns: Recipe = {
  slug: "rename-csv-columns",
  name: "Rename CSV Columns",
  description:
    "Rename CSV column headers in bulk. Free, no signup required.",
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
    version: "1.0.0",
    name: "Rename CSV Columns",
    position: { x: 0, y: 0 },
    metadata: {
      description:
        "Reads a CSV and writes it with column data preserved.",
    },
    parameters: {},
    inputPorts: [],
    outputPorts: [],
    nodes: [
      {
        id: "read-csv",
        type: "spreadsheet",
        version: "1.0.0",
        name: "Read CSV",
        position: { x: 100, y: 100 },
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
        version: "1.0.0",
        name: "Write CSV",
        position: { x: 300, y: 100 },
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
    ],
    edges: [{ id: "e1", source: "read-csv", target: "write-csv" }],
  },
};
