/** Batch Rename recipe — loops over files and renames each one. */

import type { Recipe } from "../recipe";
import { CURRENT_FORMAT_VERSION } from "../formatVersion";
import { getProcessorDefaults } from "../generated/catalog";

export const batchRename: Recipe = {
  slug: "batch-rename",
  name: "Batch Rename",
  description: "Rename multiple files in a single batch. Reusable building block.",
  category: "file",
  accept: {
    mimeTypes: ["*/*"],
    extensions: [],
    label: "any files",
  },
  features: ["Batch rename", "Pattern matching"],
  seo: {
    title: "Batch Rename Files -- bnto",
    h1: "Batch Rename Files",
  },
  definition: {
    id: "batch-rename",
    type: "group",
    version: CURRENT_FORMAT_VERSION,
    name: "Batch Rename",
    position: { x: 0, y: 0 },
    metadata: {
      description: "Loops over files and renames each one.",
      customData: { displayName: "Batch Rename" },
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
          accept: ["*/*"],
          extensions: [],
          label: "any files",
          multiple: true,
        },
        inputPorts: [],
        outputPorts: [{ id: "out-1", name: "files" }],
      },
      {
        id: "rename-loop",
        type: "loop",
        version: CURRENT_FORMAT_VERSION,
        name: "Rename Each File",
        position: { x: 250, y: 100 },
        metadata: {},
        parameters: { mode: "forEach" },
        inputPorts: [{ id: "in-1", name: "items" }],
        outputPorts: [],
        nodes: [
          {
            id: "rename-file",
            type: "file-system",
            version: CURRENT_FORMAT_VERSION,
            name: "Rename File",
            position: { x: 0, y: 0 },
            metadata: {},
            parameters: {
              operation: "rename",
              ...getProcessorDefaults("file-system", "rename"),
              prefix: "renamed-",
            },
            inputPorts: [],
            outputPorts: [],
          },
        ],
        edges: [],
      },
      {
        id: "output",
        type: "output",
        version: CURRENT_FORMAT_VERSION,
        name: "Renamed Files",
        position: { x: 500, y: 100 },
        metadata: {},
        parameters: {
          mode: "download",
          label: "Renamed Files",
          zip: true,
          autoDownload: false,
        },
        inputPorts: [{ id: "in-1", name: "files" }],
        outputPorts: [],
      },
    ],
    edges: [
      { id: "e1", source: "input", target: "rename-loop" },
      { id: "e2", source: "rename-loop", target: "output" },
    ],
  },
};
