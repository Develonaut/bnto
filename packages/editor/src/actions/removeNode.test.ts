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
      data: {
        label: "A",
        variant: "primary",
        width: 200,
        height: 200,
        status: "idle",
      },
    },
    {
      id: "b",
      type: "compartment",
      position: { x: 0, y: 0 },
      data: {
        label: "B",
        variant: "primary",
        width: 200,
        height: 200,
        status: "idle",
      },
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
    recipeMetadata: {
      id: "test",
      name: "Test",
      type: "group",
      version: "1.0.0",
    },
    isDirty: false,
    validationErrors: [],
    executionState: {},
    undoStack: [],
    redoStack: [],
    slug: null,
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
    data: {
      label: "Input",
      variant: "info",
      width: 200,
      height: 200,
      status: "idle",
    },
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

  it("reflows positions to close gap after removal", () => {
    const state = stateWithNodes();
    // Place nodes at stride positions: a=0, b=220
    state.nodes[0] = { ...state.nodes[0]!, position: { x: 0, y: 0 } };
    state.nodes[1] = { ...state.nodes[1]!, position: { x: 220, y: 0 } };
    // Add a third node at x=440
    state.nodes.push({
      id: "c",
      type: "compartment",
      position: { x: 440, y: 0 },
      data: { label: "C", variant: "primary", width: 200, height: 200, status: "idle" },
    });
    state.configs.c = { nodeType: "edit-fields", name: "C", parameters: {} };

    // Remove the middle node "b" — "c" should slide left to x=220
    const result = removeNode(state, "b");
    expect(result!.nodes!).toHaveLength(2);
    expect(result!.nodes![0]!.position.x).toBe(0);
    expect(result!.nodes![1]!.position.x).toBe(220);
  });
});
