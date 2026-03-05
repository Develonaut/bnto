/**
 * Rename Files recipe — batch rename files with patterns.
 *
 * Go source: engine/pkg/menu/recipes/rename-files.json
 */

import type { Recipe } from "../recipe";
import { CURRENT_FORMAT_VERSION } from "../formatVersion";

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
      description: "Lists files and renames each by adding a prefix.",
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
        position: { x: 300, y: 100 },
        metadata: {},
        parameters: {
          mode: "forEach",
          items: '{{index . "input" "files"}}',
        },
        inputPorts: [{ id: "in-1", name: "items" }],
        outputPorts: [],
        nodes: [
          {
            id: "rename-file",
            type: "file-system",
            version: CURRENT_FORMAT_VERSION,
            name: "Rename File",
            position: { x: 100, y: 100 },
            metadata: {},
            parameters: {
              operation: "move",
              source: "{{.item}}",
              dest: "{{.OUTPUT_DIR}}/renamed-{{basename .item}}",
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
