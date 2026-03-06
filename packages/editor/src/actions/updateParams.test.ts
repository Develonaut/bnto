/**
 * updateParams action tests — pure function, no React or store needed.
 */

import { describe, it, expect } from "vitest";
import { updateParams } from "./updateParams";
import type { EditorState } from "../store/types";
import type { BentoNode, NodeConfigs } from "../adapters/types";

function stateWithNode(): EditorState {
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
  ];
  const configs: NodeConfigs = {
    a: { nodeType: "image", name: "A", parameters: { quality: 80 } },
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
    paletteOpen: false,
  } as EditorState;
}

describe("updateParams", () => {
  it("merges params into existing config", () => {
    const result = updateParams(stateWithNode(), "a", { format: "webp" });
    expect(result).not.toBeNull();
    expect(result!.configs!.a!.parameters.quality).toBe(80);
    expect(result!.configs!.a!.parameters.format).toBe("webp");
  });

  it("overwrites existing params", () => {
    const result = updateParams(stateWithNode(), "a", { quality: 60 });
    expect(result!.configs!.a!.parameters.quality).toBe(60);
  });

  it("returns null for unknown node", () => {
    const result = updateParams(stateWithNode(), "nonexistent", {
      quality: 60,
    });
    expect(result).toBeNull();
  });

  it("does not modify nodes array", () => {
    const state = stateWithNode();
    const result = updateParams(state, "a", { quality: 60 });
    expect(result!.nodes).toBeUndefined();
  });

  it("marks dirty", () => {
    const result = updateParams(stateWithNode(), "a", { quality: 60 });
    expect(result!.isDirty).toBe(true);
  });

  it("pushes undo snapshot", () => {
    const result = updateParams(stateWithNode(), "a", { quality: 60 });
    expect(result!.undoStack!.length).toBe(1);
  });

  it("clears redo stack", () => {
    const state = stateWithNode();
    state.redoStack = [{ nodes: [], configs: {} }];
    const result = updateParams(state, "a", { quality: 60 });
    expect(result!.redoStack).toEqual([]);
  });
});
