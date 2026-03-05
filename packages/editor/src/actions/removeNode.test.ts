/**
 * removeNode action tests — pure function, no React or store needed.
 */

import { describe, it, expect } from "vitest";
import { removeNode } from "./removeNode";
import type { EditorState } from "../store/types";
import type { BentoNode, NodeConfigs } from "../adapters/types";

function stateWithNodes(): EditorState {
  const nodes: BentoNode[] = [
    {
      id: "a",
      type: "compartment",
      position: { x: 0, y: 0 },
      data: { label: "A", variant: "primary", width: 200, height: 200, status: "idle" },
    },
    {
      id: "b",
      type: "compartment",
      position: { x: 0, y: 0 },
      data: { label: "B", variant: "primary", width: 200, height: 200, status: "idle" },
    },
  ];
  const configs: NodeConfigs = {
    a: { nodeType: "image", name: "A", parameters: {} },
    b: { nodeType: "transform", name: "B", parameters: {} },
  };
  return {
    nodes,
    edges: [],
    configs,
    recipeMetadata: { id: "test", name: "Test", type: "group", version: "1.0.0" },
    isDirty: false,
    validationErrors: [],
    executionState: {},
    undoStack: [],
    redoStack: [],
    selectedNodeId: null,
    layersOpen: false,
    configOpen: false,
  } as EditorState;
}

function stateWithIoNode(): EditorState {
  const state = stateWithNodes();
  state.nodes.push({
    id: "input",
    type: "compartment",
    position: { x: 0, y: 0 },
    data: { label: "Input", variant: "info", width: 200, height: 200, status: "idle" },
  });
  state.configs.input = { nodeType: "input", name: "Input", parameters: {} };
  return state;
}

describe("removeNode", () => {
  it("removes a node and its config", () => {
    const result = removeNode(stateWithNodes(), "a");
    expect(result).not.toBeNull();
    expect(result!.nodes!.length).toBe(1);
    expect(result!.configs!.a).toBeUndefined();
  });

  it("returns null when trying to remove an I/O node", () => {
    const result = removeNode(stateWithIoNode(), "input");
    expect(result).toBeNull();
  });

  it("auto-selects nearest node after removal", () => {
    const result = removeNode(stateWithNodes(), "b");
    const selected = result!.nodes!.find((n) => n.selected);
    expect(selected).toBeDefined();
    expect(selected!.id).toBe("a");
  });

  it("pushes undo snapshot", () => {
    const result = removeNode(stateWithNodes(), "a");
    expect(result!.undoStack!.length).toBe(1);
  });

  it("marks dirty", () => {
    const result = removeNode(stateWithNodes(), "a");
    expect(result!.isDirty).toBe(true);
  });
});
