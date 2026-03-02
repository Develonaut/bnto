/**
 * Creates a minimal valid Definition — the "blank canvas" entry point.
 *
 * Returns a root group node with one input port and one output port,
 * no children, no edges. This is the starting point for building a
 * recipe from scratch in the editor.
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
    nodes: [],
    edges: [],
  };
}
