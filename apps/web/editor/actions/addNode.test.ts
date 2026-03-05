/**
 * addNode action tests — pure function, no React or store needed.
 */

import { describe, it, expect } from "vitest";
import { addNode } from "./addNode";
import type { EditorState } from "../store/types";
import type { BentoNode, NodeConfigs } from "../adapters/types";

function blankState(overrides?: Partial<EditorState>): EditorState {
  return {
    nodes: [],
    edges: [],
    configs: {},
    recipeMetadata: { id: "test", name: "Test", type: "group", version: "1.0.0" },
    isDirty: false,
    validationErrors: [],
    executionState: {},
    undoStack: [],
    redoStack: [],
    ...overrides,
  } as EditorState;
}

function stateWithIoNodes(): EditorState {
  const inputNode: BentoNode = {
    id: "input",
    type: "compartment",
    position: { x: 0, y: 0 },
    data: { label: "Input", variant: "info", width: 200, height: 200, status: "idle" },
  };
  const outputNode: BentoNode = {
    id: "output",
    type: "compartment",
    position: { x: 0, y: 0 },
    data: { label: "Output", variant: "info", width: 200, height: 200, status: "idle" },
  };
  const configs: NodeConfigs = {
    input: { nodeType: "input", name: "Input", parameters: {} },
    output: { nodeType: "output", name: "Output", parameters: {} },
  };
  return blankState({ nodes: [inputNode, outputNode], configs });
}

describe("addNode", () => {
  it("returns next state and nodeId on success", () => {
    const result = addNode(blankState(), "image");
    expect(result).not.toBeNull();
    expect(result!.nodeId).toBeTruthy();
    expect(result!.nextState.nodes).toBeDefined();
    expect(result!.nextState.isDirty).toBe(true);
  });

  it("blocks duplicate input node", () => {
    const result = addNode(stateWithIoNodes(), "input");
    expect(result).toBeNull();
  });

  it("blocks duplicate output node", () => {
    const result = addNode(stateWithIoNodes(), "output");
    expect(result).toBeNull();
  });

  it("allows adding non-I/O node when I/O nodes exist", () => {
    const result = addNode(stateWithIoNodes(), "image");
    expect(result).not.toBeNull();
  });

  it("auto-selects the new node", () => {
    const result = addNode(blankState(), "image");
    const newNode = result!.nextState.nodes!.find((n) => n.id === result!.nodeId);
    expect(newNode!.selected).toBe(true);
  });

  it("pushes undo snapshot", () => {
    const result = addNode(blankState(), "image");
    expect(result!.nextState.undoStack!.length).toBe(1);
  });

  it("clears redo stack", () => {
    const state = blankState({ redoStack: [{ nodes: [], configs: {} }] });
    const result = addNode(state, "image");
    expect(result!.nextState.redoStack).toEqual([]);
  });
});
