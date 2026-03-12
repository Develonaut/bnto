/**
 * updateParams action tests — pure function, no React or store needed.
 */

import { describe, it, expect } from "vitest";
import { updateParams } from "./updateParams";
import type { Definition } from "@bnto/nodes";
import { CURRENT_FORMAT_VERSION } from "@bnto/nodes";
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
    nodeProgress: {},
    undoStack: [],
    redoStack: [],
    definition: null,
    selectedNodeId: null,
    panels: { config: false, palette: false, run: false },
    executionPhase: "idle",
    executionResults: [],
    executionErrors: [],
    executionLogs: [],
    executionFileProgress: null,
    executionInputFiles: [],
    insertAfterNodeId: null,
    insertIntoContainerId: null,
    expandedContainerIds: new Set(),
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
    state.redoStack = [{ nodes: [], configs: {}, definition: null, expandedContainerIds: new Set() }];
    const result = updateParams(state, "a", { quality: 60 });
    expect(result!.redoStack).toEqual([]);
  });

  it("writes through to definition tree for nested child", () => {
    const def: Definition = {
      id: "root",
      type: "group",
      version: CURRENT_FORMAT_VERSION,
      name: "Root",
      position: { x: 0, y: 0 },
      metadata: {},
      parameters: {},
      inputPorts: [],
      outputPorts: [],
      nodes: [
        {
          id: "loop",
          type: "loop",
          version: CURRENT_FORMAT_VERSION,
          name: "Loop",
          position: { x: 0, y: 0 },
          metadata: {},
          parameters: { mode: "forEach" },
          inputPorts: [],
          outputPorts: [],
          nodes: [
            {
              id: "child",
              type: "image",
              version: CURRENT_FORMAT_VERSION,
              name: "Image",
              position: { x: 0, y: 0 },
              metadata: {},
              parameters: { operation: "compress", compression: 20 },
              inputPorts: [],
              outputPorts: [],
            },
          ],
        },
      ],
    };
    const state: EditorState = {
      ...stateWithNode(),
      configs: {
        loop: { nodeType: "loop", name: "Loop", parameters: { mode: "forEach" } },
        child: { nodeType: "image", name: "Image", parameters: { operation: "compress", compression: 20 } },
      },
      definition: def,
    };
    const result = updateParams(state, "child", { compression: 50 });
    expect(result).not.toBeNull();
    // Configs updated
    expect(result!.configs!.child!.parameters.compression).toBe(50);
    // Definition tree updated
    const updatedDef = result!.definition as Definition;
    expect(updatedDef.nodes![0].nodes![0].parameters.compression).toBe(50);
  });
});
