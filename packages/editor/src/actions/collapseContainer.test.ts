/**
 * collapseContainer action tests — pure function, no React or store needed.
 */

import { describe, it, expect } from "vitest";
import { collapseContainer } from "./collapseContainer";
import type { EditorState } from "../store/types";
import type { Definition } from "@bnto/nodes";
import type { BentoNode } from "../adapters/types";

function makeDefinition(): Definition {
  return {
    id: "root",
    type: "group",
    version: "1.0.0",
    name: "Root",
    position: { x: 0, y: 0 },
    metadata: {},
    parameters: {},
    inputPorts: [],
    outputPorts: [],
    nodes: [
      {
        id: "loop1",
        type: "loop",
        version: "1.0.0",
        name: "Loop",
        position: { x: 0, y: 0 },
        metadata: {},
        parameters: {},
        inputPorts: [],
        outputPorts: [],
        nodes: [
          {
            id: "child1",
            type: "image",
            version: "1.0.0",
            name: "Image",
            position: { x: 0, y: 0 },
            metadata: {},
            parameters: { quality: 80 },
            inputPorts: [],
            outputPorts: [],
          },
        ],
      },
    ],
  };
}

function makeExpandedState(): EditorState {
  const parentNode: BentoNode = {
    id: "loop1",
    type: "compartment",
    position: { x: 0, y: 0 },
    data: {
      label: "Loop",
      variant: "primary",
      width: 140,
      height: 140,
      status: "idle",
      isContainer: true,
      isExpanded: true,
      depth: 0,
    },
  };

  const childNode: BentoNode = {
    id: "child1",
    type: "compartment",
    position: { x: 0, y: 200 },
    data: {
      label: "Image",
      variant: "primary",
      width: 140,
      height: 140,
      status: "idle",
      parentContainerId: "loop1",
      depth: 1,
    },
  };

  return {
    nodes: [parentNode, childNode],
    edges: [],
    configs: {
      loop1: { nodeType: "loop", name: "Loop", parameters: {} },
      child1: { nodeType: "image", name: "Image", parameters: { quality: 95 } },
    },
    definition: makeDefinition(),
    recipeMetadata: { id: "root", name: "Root", type: "group", version: "1.0.0", cloudId: null },
    isDirty: false,
    validationErrors: [],
    executionState: {},
    nodeProgress: {},
    undoStack: [],
    redoStack: [],
    selectedNodeId: null,
    panels: { config: false, palette: false, run: false, help: false },
    expandedContainerIds: new Set(["loop1"]),
    executionPhase: "idle",
    executionResults: [],
    executionErrors: [],
    executionLogs: [],
    executionFileProgress: null,
    executionInputFiles: [],
    insertAfterNodeId: null,
    insertIntoContainerId: null,
  } as EditorState;
}

describe("collapseContainer", () => {
  it("removes child nodes from store", () => {
    const result = collapseContainer(makeExpandedState(), "loop1");
    expect(result).not.toBeNull();
    const childIds = result!.nodes!.map((n) => n.id);
    expect(childIds).not.toContain("child1");
    expect(childIds).toContain("loop1");
  });

  it("removes child configs from store", () => {
    const result = collapseContainer(makeExpandedState(), "loop1");
    expect(result!.configs!.child1).toBeUndefined();
    expect(result!.configs!.loop1).toBeDefined();
  });

  it("marks the parent as collapsed", () => {
    const result = collapseContainer(makeExpandedState(), "loop1");
    const parent = result!.nodes!.find((n) => n.id === "loop1");
    expect(parent!.data.isExpanded).toBe(false);
  });

  it("removes nodeId from expandedContainerIds", () => {
    const result = collapseContainer(makeExpandedState(), "loop1");
    const expandedIds = result!.expandedContainerIds as Set<string>;
    expect(expandedIds.has("loop1")).toBe(false);
  });

  it("writes child config changes back to definition", () => {
    const state = makeExpandedState();
    // Child config has quality=95, definition has quality=80
    const result = collapseContainer(state, "loop1");
    const def = result!.definition!;
    const loopDef = def.nodes![0]!;
    const childDef = loopDef.nodes![0]!;
    expect(childDef.parameters.quality).toBe(95);
  });

  it("returns null if not expanded", () => {
    const state = makeExpandedState();
    state.expandedContainerIds = new Set();
    const result = collapseContainer(state, "loop1");
    expect(result).toBeNull();
  });

  it("pushes undo snapshot", () => {
    const result = collapseContainer(makeExpandedState(), "loop1");
    expect(result!.undoStack!.length).toBe(1);
  });

  it("marks dirty", () => {
    const result = collapseContainer(makeExpandedState(), "loop1");
    expect(result!.isDirty).toBe(true);
  });
});
