/**
 * addNode action tests — pure function, no React or store needed.
 */

import { describe, it, expect } from "vitest";
import { addNode } from "./addNode";
import type { EditorState } from "../store/types";
import type { Definition } from "@bnto/nodes";
import { CURRENT_FORMAT_VERSION } from "@bnto/nodes";
import type { BentoNode, NodeConfigs } from "../adapters/types";

function blankState(overrides?: Partial<EditorState>): EditorState {
  return {
    nodes: [],
    edges: [],
    configs: {},
    recipeMetadata: {
      id: "test",
      name: "Test",
      slug: "test",
      cloudId: null,
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

function stateWithIoNodes(): EditorState {
  const inputNode: BentoNode = {
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
  };
  const outputNode: BentoNode = {
    id: "output",
    type: "compartment",
    position: { x: 0, y: 0 },
    data: {
      label: "Output",
      variant: "info",
      width: 200,
      height: 200,
      status: "idle",
    },
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
    const state = blankState({
      redoStack: [{ nodes: [], configs: {}, definition: null, expandedContainerIds: new Set() }],
    });
    const result = addNode(state, "image");
    expect(result!.nextState.redoStack).toEqual([]);
  });

  it("pre-sets operation via defaultParams", () => {
    const result = addNode(blankState(), "image", null, null, { operation: "resize" });
    expect(result).not.toBeNull();
    const config = result!.nextState.configs![result!.nodeId];
    expect(config!.parameters.operation).toBe("resize");
  });

  it("uses short operation name as node label when operation pre-set", () => {
    const result = addNode(blankState(), "image", null, null, { operation: "compress" });
    const config = result!.nextState.configs![result!.nodeId];
    expect(config!.name).toBe("Compress");
  });
});

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

function stateWithContainer(): EditorState {
  const containerNode: BentoNode = {
    id: "loop1",
    type: "compartment",
    position: { x: 0, y: 0 },
    data: {
      label: "Loop",
      variant: "primary",
      width: 170,
      height: 170,
      status: "idle",
      isContainer: true,
      isExpanded: false,
      depth: 0,
    },
  };
  const def = validDef({
    nodes: [validDef({ id: "loop1", type: "loop", parameters: {} })],
  });
  return blankState({
    nodes: [containerNode],
    configs: { loop1: { nodeType: "loop", name: "Loop", parameters: {} } },
    definition: def,
  });
}

describe("addNode — child insertion (Mode 1)", () => {
  it("adds a child into a container", () => {
    const result = addNode(stateWithContainer(), "image", null, "loop1");
    expect(result).not.toBeNull();
    const child = result!.nextState.nodes!.find((n) => n.id === result!.nodeId);
    expect(child).toBeDefined();
    expect(child!.data.parentContainerId).toBe("loop1");
    expect(child!.data.depth).toBe(1);
  });

  it("auto-expands the container", () => {
    const result = addNode(stateWithContainer(), "image", null, "loop1");
    const expandedIds = result!.nextState.expandedContainerIds as Set<string>;
    expect(expandedIds.has("loop1")).toBe(true);
  });

  it("marks container as expanded in node data", () => {
    const result = addNode(stateWithContainer(), "image", null, "loop1");
    const container = result!.nextState.nodes!.find((n) => n.id === "loop1");
    expect(container!.data.isExpanded).toBe(true);
  });

  it("adds child to definition tree", () => {
    const result = addNode(stateWithContainer(), "image", null, "loop1");
    const def = result!.nextState.definition as Definition;
    const loopDef = def.nodes![0]!;
    expect(loopDef.nodes).toHaveLength(1);
    expect(loopDef.nodes![0]!.id).toBe(result!.nodeId);
  });

  it("inserts child right after container in flat array", () => {
    const result = addNode(stateWithContainer(), "image", null, "loop1");
    const nodes = result!.nextState.nodes!;
    const containerIdx = nodes.findIndex((n) => n.id === "loop1");
    const childIdx = nodes.findIndex((n) => n.id === result!.nodeId);
    expect(childIdx).toBe(containerIdx + 1);
  });

  it("returns null for unknown container", () => {
    const result = addNode(stateWithContainer(), "image", null, "nonexistent");
    expect(result).toBeNull();
  });

  it("pre-sets operation via defaultParams in child-into-container path", () => {
    const result = addNode(stateWithContainer(), "spreadsheet", null, "loop1", {
      operation: "clean",
    });
    expect(result).not.toBeNull();
    const config = result!.nextState.configs![result!.nodeId];
    expect(config!.parameters.operation).toBe("clean");
    expect(config!.name).toBe("Clean");
  });
});
