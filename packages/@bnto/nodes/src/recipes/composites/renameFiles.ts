/** Rename Files recipe — batch rename files with patterns. */

import type { Recipe } from "../../recipe";
import { CURRENT_FORMAT_VERSION } from "../../formatVersion";
import { batchRename } from "../primitives/batchRename";

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
      description: "Accepts files and renames each one using a reusable batch rename sub-recipe.",
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
      batchRename,
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
      { id: "e1", source: "input", target: "batch-rename" },
      { id: "e2", source: "batch-rename", target: "output" },
    ],
  },
};
