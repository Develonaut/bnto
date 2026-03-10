import { describe, expect, it } from "vitest";

import type { Definition } from "@bnto/nodes";
import { CURRENT_FORMAT_VERSION } from "@bnto/nodes";
import type { EditorState } from "../store/types";
import { updateSurfacedParam } from "./updateSurfacedParam";

/** Creates a minimal valid definition for testing. */
function validDef(overrides: Partial<Definition> = {}): Definition {
  return {
    id: "test",
    type: "group",
    version: CURRENT_FORMAT_VERSION,
    name: "Test",
    position: { x: 0, y: 0 },
    metadata: {},
    parameters: {},
    inputPorts: [],
    outputPorts: [],
    ...overrides,
  };
}

function baseState(def: Definition): EditorState {
  return {
    nodes: [],
    edges: [],
    configs: {},
    definition: def,
    recipeMetadata: { id: def.id, name: def.name, type: def.type, version: def.version },
    isDirty: false,
    validationErrors: [],
    executionState: {},
    nodeProgress: {},
    undoStack: [],
    redoStack: [],
    selectedNodeId: null,
    panels: { config: false, palette: false, run: false },
    executionPhase: "idle",
    executionResults: [],
    executionErrors: [],
    executionLogs: [],
    executionFileProgress: null,
    executionInputFiles: [],
    insertAfterNodeId: null,
  };
}

describe("updateSurfacedParam", () => {
  it("updates leaf node params in definition", () => {
    const def = validDef({
      id: "root",
      nodes: [
        validDef({
          id: "leaf",
          type: "image",
          parameters: { operation: "compress", quality: 80 },
        }),
      ],
    });
    const state = baseState(def);
    const result = updateSurfacedParam(state, "leaf", { quality: 60 });
    expect(result).not.toBeNull();
    const updatedDef = result!.definition as Definition;
    expect(updatedDef.nodes![0].parameters.quality).toBe(60);
    // operation should be unchanged
    expect(updatedDef.nodes![0].parameters.operation).toBe("compress");
  });

  it("updates deeply nested leaf nodes", () => {
    const def = validDef({
      id: "root",
      nodes: [
        validDef({
          id: "loop",
          type: "loop",
          parameters: { mode: "forEach" },
          nodes: [
            validDef({
              id: "deep-leaf",
              type: "image",
              parameters: { operation: "compress", quality: 80 },
            }),
          ],
        }),
      ],
    });
    const state = baseState(def);
    const result = updateSurfacedParam(state, "deep-leaf", { quality: 50 });
    expect(result).not.toBeNull();
    const updatedDef = result!.definition as Definition;
    expect(updatedDef.nodes![0].nodes![0].parameters.quality).toBe(50);
  });

  it("sets dirty flag via withUndo", () => {
    const def = validDef({
      nodes: [
        validDef({ id: "leaf", type: "image", parameters: { operation: "compress", quality: 80 } }),
      ],
    });
    const state = baseState(def);
    const result = updateSurfacedParam(state, "leaf", { quality: 60 });
    expect(result!.isDirty).toBe(true);
  });

  it("returns null when definition is null", () => {
    const state = baseState(validDef());
    state.definition = null;
    const result = updateSurfacedParam(state, "leaf", { quality: 60 });
    expect(result).toBeNull();
  });

  it("preserves other nodes when updating one leaf", () => {
    const def = validDef({
      id: "root",
      nodes: [
        validDef({
          id: "leaf-a",
          type: "image",
          parameters: { operation: "compress", quality: 80 },
        }),
        validDef({ id: "leaf-b", type: "image", parameters: { operation: "resize", width: 200 } }),
      ],
    });
    const state = baseState(def);
    const result = updateSurfacedParam(state, "leaf-a", { quality: 40 });
    const updatedDef = result!.definition as Definition;
    expect(updatedDef.nodes![0].parameters.quality).toBe(40);
    expect(updatedDef.nodes![1].parameters.width).toBe(200);
  });
});
