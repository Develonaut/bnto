/**
 * expandContainer action tests — pure function, no React or store needed.
 */

import { describe, it, expect } from "vitest";
import { expandContainer } from "./expandContainer";
import type { EditorState } from "../store/types";
import type { Definition } from "@bnto/core";
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

function makeState(overrides?: Partial<EditorState>): EditorState {
  const loopNode: BentoNode = {
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
      isExpanded: false,
      depth: 0,
    },
  };

  return {
    nodes: [loopNode],
    edges: [],
    configs: {
      loop1: { nodeType: "loop", name: "Loop", parameters: {} },
    },
    definition: makeDefinition(),
    recipeMetadata: { id: "root", name: "Root", slug: "root", cloudId: null },
    isDirty: false,
    validationErrors: [],
    executionState: {},
    nodeProgress: {},
    undoStack: [],
    redoStack: [],
    selectedNodeId: null,
    panels: { config: false, palette: false, run: false, help: false },
    expandedContainerIds: new Set(),
    executionPhase: "idle",
    executionResults: [],
    executionErrors: [],
    executionLogs: [],
    executionFileProgress: null,
    executionInputFiles: [],
    insertAfterNodeId: null,
    insertIntoContainerId: null,
    ...overrides,
  } as EditorState;
}

describe("expandContainer", () => {
  it("materializes children into store nodes", () => {
    const result = expandContainer(makeState(), "loop1");
    expect(result).not.toBeNull();
    const newNodes = result!.nodes!;
    const childNode = newNodes.find((n) => n.id === "child1");
    expect(childNode).toBeDefined();
    expect(childNode!.data.parentContainerId).toBe("loop1");
    expect(childNode!.data.depth).toBe(1);
  });

  it("creates configs for child nodes", () => {
    const result = expandContainer(makeState(), "loop1");
    const configs = result!.configs!;
    expect(configs.child1).toBeDefined();
    expect(configs.child1!.nodeType).toBe("image");
    expect(configs.child1!.parameters.quality).toBe(80);
  });

  it("marks the parent as expanded", () => {
    const result = expandContainer(makeState(), "loop1");
    const parent = result!.nodes!.find((n) => n.id === "loop1");
    expect(parent!.data.isExpanded).toBe(true);
  });

  it("adds nodeId to expandedContainerIds", () => {
    const result = expandContainer(makeState(), "loop1");
    const expandedIds = result!.expandedContainerIds as Set<string>;
    expect(expandedIds.has("loop1")).toBe(true);
  });

  it("returns null if already expanded", () => {
    const state = makeState({ expandedContainerIds: new Set(["loop1"]) });
    const result = expandContainer(state, "loop1");
    expect(result).toBeNull();
  });

  it("returns null if no definition", () => {
    const state = makeState({ definition: null });
    const result = expandContainer(state, "loop1");
    expect(result).toBeNull();
  });

  it("returns null if node not found", () => {
    const result = expandContainer(makeState(), "nonexistent");
    expect(result).toBeNull();
  });

  it("pushes undo snapshot", () => {
    const result = expandContainer(makeState(), "loop1");
    expect(result!.undoStack!.length).toBe(1);
  });

  it("marks dirty", () => {
    const result = expandContainer(makeState(), "loop1");
    expect(result!.isDirty).toBe(true);
  });
});
