/**
 * Rename CSV Columns recipe — rename column headers in bulk.
 *
 * Simplified from Go engine's read+write multi-node structure to a flat
 * 3-node pipeline: input → spreadsheet:rename → output.
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
      description: "Renames column headers in a CSV file.",
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
        id: "rename-columns",
        type: "spreadsheet",
        version: CURRENT_FORMAT_VERSION,
        name: "Rename Columns",
        position: { x: 300, y: 100 },
        metadata: {},
        parameters: {
          operation: "rename",
        },
        inputPorts: [{ id: "in-1", name: "files" }],
        outputPorts: [{ id: "out-1", name: "files" }],
      },
      {
        id: "output",
        type: "output",
        version: CURRENT_FORMAT_VERSION,
        name: "Renamed CSV",
        position: { x: 500, y: 100 },
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
      { id: "e1", source: "input", target: "rename-columns" },
      { id: "e2", source: "rename-columns", target: "output" },
    ],
  },
};
