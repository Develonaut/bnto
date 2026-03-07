/**
 * Clean CSV recipe — remove empty rows, trim whitespace, deduplicate.
 *
 * Simplified from Go engine's read+write multi-node structure to a flat
 * 3-node pipeline: input → spreadsheet:clean → output.
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
      description: "Filters empty rows, trims whitespace, and writes a clean CSV.",
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
        id: "clean-csv",
        type: "spreadsheet",
        version: CURRENT_FORMAT_VERSION,
        name: "Clean CSV",
        position: { x: 300, y: 100 },
        metadata: {},
        parameters: {
          operation: "clean",
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
      { id: "e1", source: "input", target: "clean-csv" },
      { id: "e2", source: "clean-csv", target: "output" },
    ],
  },
};
