/** All Operations test recipe — all 6 Tier 1 ops in one pipeline (test-only). */

import type { Recipe } from "../../recipe";
import { CURRENT_FORMAT_VERSION, getProcessorDefaults } from "@bnto/nodes";
import { defaultInputNode } from "../defaultInputNode";
import { defaultOutputNode } from "../defaultOutputNode";

const IMAGE_INPUT = {
  accept: ["image/jpeg", "image/png", "image/webp"],
  extensions: [".jpg", ".jpeg", ".png", ".webp"],
  label: "JPEG, PNG, or WebP images",
} as const;

export const allOperations: Recipe = {
  id: "test-all-operations",
  slug: "all-operations",
  name: "All Operations",
  description: "Test recipe combining all 6 Tier 1 operations. Not user-facing.",
  category: "image",
  accept: {
    mimeTypes: ["image/jpeg", "image/png", "image/webp"],
    extensions: [".jpg", ".jpeg", ".png", ".webp"],
    label: "JPEG, PNG, or WebP images",
    mimePrefix: "image/",
  },
  features: [],
  definition: {
    id: "all-operations",
    type: "group",
    version: CURRENT_FORMAT_VERSION,
    name: "All Operations",
    position: { x: 0, y: 0 },
    metadata: {
      description: "Test-only recipe with all 6 browser operations.",
    },
    parameters: {},
    inputPorts: [],
    outputPorts: [],
    nodes: [
      defaultInputNode(IMAGE_INPUT),
      {
        id: "process-loop",
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
            id: "compress",
            type: "image-compress",
            version: CURRENT_FORMAT_VERSION,
            name: "Compress",
            position: { x: 0, y: 0 },
            metadata: {},
            parameters: {
              ...getProcessorDefaults("image-compress"),
            },
            inputPorts: [],
            outputPorts: [],
          },
          {
            id: "resize",
            type: "image-resize",
            version: CURRENT_FORMAT_VERSION,
            name: "Resize",
            position: { x: 250, y: 0 },
            metadata: {},
            parameters: {
              ...getProcessorDefaults("image-resize"),
            },
            inputPorts: [],
            outputPorts: [],
          },
          {
            id: "convert",
            type: "image-convert",
            version: CURRENT_FORMAT_VERSION,
            name: "Convert",
            position: { x: 500, y: 0 },
            metadata: {},
            parameters: {
              ...getProcessorDefaults("image-convert"),
              format: "webp",
            },
            inputPorts: [],
            outputPorts: [],
          },
          {
            id: "rename",
            type: "file-rename",
            version: CURRENT_FORMAT_VERSION,
            name: "Rename",
            position: { x: 750, y: 0 },
            metadata: {},
            parameters: {
              ...getProcessorDefaults("file-rename"),
            },
            inputPorts: [],
            outputPorts: [],
          },
          {
            id: "clean",
            type: "spreadsheet-clean",
            version: CURRENT_FORMAT_VERSION,
            name: "Clean",
            position: { x: 1000, y: 0 },
            metadata: {},
            parameters: {
              ...getProcessorDefaults("spreadsheet-clean"),
            },
            inputPorts: [],
            outputPorts: [],
          },
          {
            id: "rename-columns",
            type: "spreadsheet-rename",
            version: CURRENT_FORMAT_VERSION,
            name: "Rename Columns",
            position: { x: 1250, y: 0 },
            metadata: {},
            parameters: {
              ...getProcessorDefaults("spreadsheet-rename"),
              columns: {},
            },
            inputPorts: [],
            outputPorts: [],
          },
        ],
        edges: [
          { id: "le1", source: "compress", target: "resize" },
          { id: "le2", source: "resize", target: "convert" },
          { id: "le3", source: "convert", target: "rename" },
          { id: "le4", source: "rename", target: "clean" },
          { id: "le5", source: "clean", target: "rename-columns" },
        ],
      },
      defaultOutputNode({ label: "Processed Files" }),
    ],
    edges: [
      { id: "e1", source: "input", target: "process-loop" },
      { id: "e2", source: "process-loop", target: "output" },
    ],
  },
};
