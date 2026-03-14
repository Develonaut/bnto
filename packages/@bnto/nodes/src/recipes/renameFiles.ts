/** Rename Files recipe — batch rename files with patterns. */

import type { Recipe } from "../recipe";
import { CURRENT_FORMAT_VERSION } from "../formatVersion";
import { getProcessorDefaults } from "../generated/catalog";

export const renameFiles: Recipe = {
  slug: "rename-files",
  name: "Rename Files",
  description: "Batch rename files with patterns. Free, no signup required.",
  category: "file",
  accept: {
    mimeTypes: ["*/*"],
    extensions: [],
    label: "any files",
  },
  features: ["Batch rename", "Pattern matching", "Browser-based"],
  seo: {
    title: "Rename Files Online Free -- bnto",
    h1: "Rename Files Online Free",
  },
  definition: {
    id: "rename-files",
    type: "group",
    version: CURRENT_FORMAT_VERSION,
    name: "Rename Files",
    position: { x: 0, y: 0 },
    metadata: {
      description: "Accepts files and renames each one.",
    },
    parameters: {},
    inputPorts: [],
    outputPorts: [],
    nodes: [
      {
        id: "input",
        type: "input",
        version: CURRENT_FORMAT_VERSION,
        name: "Input",
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
        name: "For Each",
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
            name: "Rename",
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
        name: "Output",
        position: { x: 500, y: 100 },
        metadata: {},
        parameters: {
          mode: "download",
          label: "Renamed Files",
          zip: true,
          autoDownload: true,
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
