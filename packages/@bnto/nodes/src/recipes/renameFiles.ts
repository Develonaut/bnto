/**
 * Rename Files recipe — batch rename files with patterns.
 *
 * Go source: engine/pkg/menu/recipes/rename-files.json
 */

import type { Recipe } from "../recipe";

export const renameFiles: Recipe = {
  slug: "rename-files",
  name: "Rename Files",
  description:
    "Batch rename files with patterns. Free, no signup required.",
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
    version: "1.0.0",
    name: "Rename Files",
    position: { x: 0, y: 0 },
    metadata: {
      description:
        "Lists files and renames each by adding a prefix.",
    },
    parameters: {},
    inputPorts: [],
    outputPorts: [],
    nodes: [
      {
        id: "list-files",
        type: "file-system",
        version: "1.0.0",
        name: "List Files",
        position: { x: 100, y: 100 },
        metadata: {},
        parameters: { operation: "list", path: "{{.INPUT_DIR}}/*" },
        inputPorts: [],
        outputPorts: [{ id: "out-1", name: "files" }],
      },
      {
        id: "rename-loop",
        type: "loop",
        version: "1.0.0",
        name: "Rename Each File",
        position: { x: 300, y: 100 },
        metadata: {},
        parameters: {
          mode: "forEach",
          items: '{{index . "list-files" "files"}}',
        },
        inputPorts: [{ id: "in-1", name: "items" }],
        outputPorts: [],
        nodes: [
          {
            id: "rename-file",
            type: "file-system",
            version: "1.0.0",
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
    ],
    edges: [{ id: "e1", source: "list-files", target: "rename-loop" }],
  },
};
