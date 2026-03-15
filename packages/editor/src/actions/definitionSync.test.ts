/**
 * Definition tree sync tests — verifies that all editor actions keep
 * `state.definition` in sync with the flat graph (nodes + configs).
 *
 * This is the critical integration surface: the definition tree is
 * what gets exported and sent to the engine. If it falls out of sync
 * with the flat graph, the engine sees stale/missing nodes.
 *
 * Covers the full lifecycle: add → update params → remove, for both
 * top-level and container-child nodes. Also verifies round-trip
 * through rfNodesToDefinition (the export adapter).
 */

import { describe, it, expect } from "vitest";
import { addNode } from "./addNode";
import { removeNode } from "./removeNode";
import { updateParams } from "./updateParams";
import { rfNodesToDefinition } from "../adapters/rfNodesToDefinition";
import { findDefinitionById } from "../adapters/findDefinitionById";
import type { EditorState } from "../store/types";
import type { Definition } from "@bnto/nodes";
import { CURRENT_FORMAT_VERSION } from "@bnto/nodes";
import type { BentoNode, NodeConfigs } from "../adapters/types";

/* ── Helpers ──────────────────────────────────────────────────────── */

function validDef(overrides: Partial<Definition> = {}): Definition {
  return {
    id: "root",
    type: "group",
    version: CURRENT_FORMAT_VERSION,
    name: "Root",
    position: { x: 0, y: 0 },
    metadata: {},
    parameters: {},
    inputPorts: [],
    outputPorts: [],
    ...overrides,
  };
}

function blankState(overrides?: Partial<EditorState>): EditorState {
  return {
    nodes: [],
    edges: [],
    configs: {},
    recipeMetadata: {
      id: "root",
      name: "Test Recipe",
      type: "group",
      version: CURRENT_FORMAT_VERSION,
    },
    isDirty: false,
    validationErrors: [],
    executionState: {},
    nodeProgress: {},
    undoStack: [],
    redoStack: [],
    definition: null,
    selectedNodeId: null,
    panels: { config: false, palette: false, run: false, help: false },
    executionPhase: "idle",
    executionResults: [],
    executionErrors: [],
    executionLogs: [],
    executionFileProgress: null,
    executionInputFiles: [],
    insertAfterNodeId: null,
    insertIntoContainerId: null,
    expandedContainerIds: new Set(),
    ...overrides,
  } as EditorState;
}

/** State with Input + Output nodes and a root definition. */
function stateWithIoPipeline(): EditorState {
  const inputNode: BentoNode = {
    id: "input-1",
    type: "compartment",
    position: { x: 0, y: 0 },
    data: { label: "Input", variant: "info", width: 170, height: 170, status: "idle", depth: 0 },
  };
  const outputNode: BentoNode = {
    id: "output-1",
    type: "compartment",
    position: { x: 240, y: 0 },
    data: { label: "Output", variant: "info", width: 170, height: 170, status: "idle", depth: 0 },
  };
  const configs: NodeConfigs = {
    "input-1": { nodeType: "input", name: "Input", parameters: {} },
    "output-1": { nodeType: "output", name: "Output", parameters: {} },
  };
  const def = validDef({
    nodes: [
      validDef({ id: "input-1", type: "input", name: "Input" }),
      validDef({ id: "output-1", type: "output", name: "Output" }),
    ],
  });
  return blankState({ nodes: [inputNode, outputNode], configs, definition: def });
}

/** Merge partial state into an EditorState (simulates store.setState). */
function applyResult(state: EditorState, partial: Partial<EditorState>): EditorState {
  return { ...state, ...partial } as EditorState;
}

/* ── Top-level definition sync ────────────────────────────────────── */

describe("definition sync — top-level nodes", () => {
  it("adds a top-level node to the definition tree", () => {
    const state = stateWithIoPipeline();
    const result = addNode(state, "image");
    expect(result).not.toBeNull();

    const def = result!.nextState.definition as Definition;
    expect(def).not.toBeNull();
    // Root should have 3 nodes: input, new image, output
    expect(def.nodes).toHaveLength(3);
    const imageNode = def.nodes!.find((n) => n.id === result!.nodeId);
    expect(imageNode).toBeDefined();
    expect(imageNode!.type).toBe("image");
  });

  it("inserts before the output node by default", () => {
    const state = stateWithIoPipeline();
    const result = addNode(state, "image");
    const def = result!.nextState.definition as Definition;
    // Order: input, image, output
    expect(def.nodes![0]!.type).toBe("input");
    expect(def.nodes![1]!.id).toBe(result!.nodeId);
    expect(def.nodes![2]!.type).toBe("output");
  });

  it("inserts after a specific node when afterNodeId is provided", () => {
    const state = stateWithIoPipeline();

    // First add an image node (inserted before output)
    const first = addNode(state, "image");
    const state2 = applyResult(state, first!.nextState);

    // Then add a transform node after the image
    const second = addNode(state2, "transform", first!.nodeId);
    const def = second!.nextState.definition as Definition;

    // Order: input, image, transform, output
    expect(def.nodes).toHaveLength(4);
    expect(def.nodes![1]!.id).toBe(first!.nodeId);
    expect(def.nodes![2]!.id).toBe(second!.nodeId);
    expect(def.nodes![3]!.type).toBe("output");
  });

  it("removes a top-level node from the definition tree", () => {
    const state = stateWithIoPipeline();

    // Add a node
    const added = addNode(state, "image");
    const state2 = applyResult(state, added!.nextState);

    // Remove it
    const removed = removeNode(state2, added!.nodeId);
    expect(removed).not.toBeNull();

    const def = removed!.definition as Definition;
    // Should be back to just input + output
    expect(def.nodes).toHaveLength(2);
    expect(def.nodes!.find((n) => n.id === added!.nodeId)).toBeUndefined();
  });

  it("updates top-level node params in the definition tree", () => {
    const state = stateWithIoPipeline();

    // Add an image node
    const added = addNode(state, "image");
    const state2 = applyResult(state, added!.nextState);

    // Update its params
    const updated = updateParams(state2, added!.nodeId, { quality: 75 });
    expect(updated).not.toBeNull();

    const def = updated!.definition as Definition;
    const imageDef = def?.nodes?.find((n) => n.id === added!.nodeId);
    expect(imageDef?.parameters?.quality).toBe(75);
  });
});

/* ── Container definition sync ────────────────────────────────────── */

describe("definition sync — container nodes", () => {
  it("adds a container node to the definition tree", () => {
    const state = stateWithIoPipeline();
    const result = addNode(state, "loop");
    expect(result).not.toBeNull();

    const def = result!.nextState.definition as Definition;
    const loopNode = def.nodes!.find((n) => n.id === result!.nodeId);
    expect(loopNode).toBeDefined();
    expect(loopNode!.type).toBe("loop");
  });

  it("adds a child into a container and updates the definition tree", () => {
    const state = stateWithIoPipeline();

    // Add a loop container
    const loopResult = addNode(state, "loop");
    const state2 = applyResult(state, loopResult!.nextState);

    // Add an image child inside the loop
    const childResult = addNode(state2, "image", null, loopResult!.nodeId);
    expect(childResult).not.toBeNull();

    const def = childResult!.nextState.definition as Definition;
    const loopDef = findDefinitionById(def, loopResult!.nodeId);
    expect(loopDef).not.toBeNull();
    expect(loopDef!.nodes).toHaveLength(1);
    expect(loopDef!.nodes![0]!.id).toBe(childResult!.nodeId);
  });

  it("preserves container children through export round-trip", () => {
    const state = stateWithIoPipeline();

    // Add loop + image child
    const loopResult = addNode(state, "loop");
    const state2 = applyResult(state, loopResult!.nextState);
    const childResult = addNode(state2, "image", null, loopResult!.nodeId);
    const state3 = applyResult(state2, childResult!.nextState);

    // Export via rfNodesToDefinition — this is what the engine receives
    const exported = rfNodesToDefinition(
      state3.nodes,
      state3.recipeMetadata,
      state3.configs,
      state3.definition,
    );

    // The loop node should have the image child
    const loopExported = exported.nodes!.find((n) => n.id === loopResult!.nodeId);
    expect(loopExported).toBeDefined();
    expect(loopExported!.nodes).toHaveLength(1);
    expect(loopExported!.nodes![0]!.id).toBe(childResult!.nodeId);
    expect(loopExported!.nodes![0]!.type).toBe("image");
  });

  it("removes a container and its definition from the tree", () => {
    const state = stateWithIoPipeline();

    // Add a loop container
    const loopResult = addNode(state, "loop");
    const state2 = applyResult(state, loopResult!.nextState);

    // Remove it
    const removed = removeNode(state2, loopResult!.nodeId);
    const def = removed!.definition as Definition;
    expect(def.nodes).toHaveLength(2); // input + output only
    expect(def.nodes!.find((n) => n.id === loopResult!.nodeId)).toBeUndefined();
  });

  it("updates child node params in the definition tree", () => {
    const state = stateWithIoPipeline();

    // Add loop + image child
    const loopResult = addNode(state, "loop");
    const state2 = applyResult(state, loopResult!.nextState);
    const childResult = addNode(state2, "image", null, loopResult!.nodeId);
    const state3 = applyResult(state2, childResult!.nextState);

    // Update child params
    const updated = updateParams(state3, childResult!.nodeId, { quality: 50 });
    const def = updated!.definition as Definition;
    const loopDef = findDefinitionById(def, loopResult!.nodeId);
    const childDef = loopDef!.nodes![0]!;
    expect(childDef.parameters?.quality).toBe(50);
  });
});

/* ── Multi-step pipeline definition sync ──────────────────────────── */

describe("definition sync — full pipeline scenarios", () => {
  it("builds Input → Loop(Image) → Output and exports correctly", () => {
    let state = stateWithIoPipeline();

    // Add loop before output
    const loopResult = addNode(state, "loop");
    state = applyResult(state, loopResult!.nextState);

    // Add image child inside loop
    const imageResult = addNode(state, "image", null, loopResult!.nodeId);
    state = applyResult(state, imageResult!.nextState);

    // Update image params (this is what the user does in the config panel)
    const paramResult = updateParams(state, imageResult!.nodeId, {
      operation: "compress",
      quality: 80,
    });
    state = applyResult(state, paramResult!);

    // Export
    const exported = rfNodesToDefinition(
      state.nodes,
      state.recipeMetadata,
      state.configs,
      state.definition,
    );

    // Verify pipeline structure
    expect(exported.nodes).toHaveLength(3); // input, loop, output
    const inputDef = exported.nodes![0]!;
    const loopDef = exported.nodes![1]!;
    const outputDef = exported.nodes![2]!;

    expect(inputDef.type).toBe("input");
    expect(loopDef.type).toBe("loop");
    expect(outputDef.type).toBe("output");

    // Loop has image child with correct params
    expect(loopDef.nodes).toHaveLength(1);
    expect(loopDef.nodes![0]!.type).toBe("image");
    expect(loopDef.nodes![0]!.parameters?.operation).toBe("compress");
    expect(loopDef.nodes![0]!.parameters?.quality).toBe(80);
  });

  it("handles add → remove → re-add without stale definition artifacts", () => {
    let state = stateWithIoPipeline();

    // Add an image node
    const first = addNode(state, "image");
    state = applyResult(state, first!.nextState);
    expect(state.definition!.nodes).toHaveLength(3);

    // Remove it
    const removed = removeNode(state, first!.nodeId);
    state = applyResult(state, removed!);
    expect(state.definition!.nodes).toHaveLength(2);

    // Add a different node type
    const second = addNode(state, "transform");
    state = applyResult(state, second!.nextState);
    expect(state.definition!.nodes).toHaveLength(3);

    // The old image node should not be in the tree
    expect(state.definition!.nodes!.find((n) => n.id === first!.nodeId)).toBeUndefined();
    // The new transform node should be
    expect(state.definition!.nodes!.find((n) => n.id === second!.nodeId)).toBeDefined();
  });

  it("definition stays null when no definition exists", () => {
    const state = blankState();
    const result = addNode(state, "image");
    // No definition was set, so it should stay null
    expect(result!.nextState.definition).toBeUndefined();
  });

  it("multiple top-level nodes maintain correct definition order", () => {
    let state = stateWithIoPipeline();

    // Add nodes in sequence: image, then edit-fields after image, then transform
    const img = addNode(state, "image");
    state = applyResult(state, img!.nextState);
    const edit = addNode(state, "edit-fields", img!.nodeId);
    state = applyResult(state, edit!.nextState);
    const xform = addNode(state, "transform", edit!.nodeId);
    state = applyResult(state, xform!.nextState);

    const def = state.definition!;
    // Order: input, image, edit-fields, transform, output
    expect(def.nodes).toHaveLength(5);
    expect(def.nodes![0]!.type).toBe("input");
    expect(def.nodes![1]!.id).toBe(img!.nodeId);
    expect(def.nodes![2]!.id).toBe(edit!.nodeId);
    expect(def.nodes![3]!.id).toBe(xform!.nodeId);
    expect(def.nodes![4]!.type).toBe("output");
  });
});
