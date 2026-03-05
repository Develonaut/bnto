/**
 * Creates a minimal valid Definition — the "blank canvas" entry point.
 *
 * Returns a root group node with input and output child nodes,
 * no processing nodes, no edges between them. The input node defaults
 * to file-upload mode accepting any file. The output node defaults
 * to download mode.
 *
 * Users start here and add processing nodes between input and output.
 */

import type { Definition } from "./definition";

/** Creates a blank recipe definition ready for the editor. */
export function createBlankDefinition(): Definition {
  return {
    id: crypto.randomUUID(),
    type: "group",
    version: "1.0.0",
    name: "New Recipe",
    position: { x: 0, y: 0 },
    metadata: {
      createdAt: new Date().toISOString(),
    },
    parameters: {},
    inputPorts: [{ id: crypto.randomUUID(), name: "input" }],
    outputPorts: [{ id: crypto.randomUUID(), name: "output" }],
    // I/O node IDs are stable strings ("input"/"output"), not UUIDs.
    // Each definition has exactly one of each — stable IDs simplify
    // lookup (getInputNode/getOutputNode) and edge wiring.
    nodes: [
      {
        id: "input",
        type: "input",
        version: "1.0.0",
        name: "Input Files",
        position: { x: 0, y: 100 },
        metadata: {},
        parameters: {
          mode: "file-upload",
          accept: ["*/*"],
          extensions: [],
          label: "Any files",
          multiple: true,
        },
        inputPorts: [],
        outputPorts: [{ id: "out-1", name: "files" }],
      },
      {
        id: "output",
        type: "output",
        version: "1.0.0",
        name: "Output",
        position: { x: 400, y: 100 },
        metadata: {},
        parameters: {
          mode: "download",
          label: "Output Files",
          zip: true,
          autoDownload: false,
        },
        inputPorts: [{ id: "in-1", name: "files" }],
        outputPorts: [],
      },
    ],
    edges: [],
  };
}
