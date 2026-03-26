/** Rename Files recipe — batch rename files with patterns. */

import type { Recipe } from "../recipe";
import { CURRENT_FORMAT_VERSION, getProcessorDefaults } from "@bnto/nodes";
import { defaultInputNode } from "./defaultInputNode";
import { defaultOutputNode } from "./defaultOutputNode";

export const renameFiles: Recipe = {
  id: "404cea0b-ea2c-4d81-bd89-47f5eb3b8389",
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
    settings: { iteration: "auto" },
    nodes: [
      defaultInputNode({
        accept: ["*/*"],
        extensions: [],
        label: "any files",
      }),
      {
        id: "rename-file",
        type: "file-rename",
        version: CURRENT_FORMAT_VERSION,
        name: "Rename",
        position: { x: 250, y: 100 },
        metadata: {},
        parameters: {
          ...getProcessorDefaults("file-rename"),
          prefix: "renamed-",
        },
        inputPorts: [],
        outputPorts: [],
      },
      defaultOutputNode({ label: "Renamed Files" }),
    ],
    edges: [
      { id: "e1", source: "input", target: "rename-file" },
      { id: "e2", source: "rename-file", target: "output" },
    ],
  },
};
