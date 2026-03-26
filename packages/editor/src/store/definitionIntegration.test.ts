/**
 * Definition integration tests — store-level tests that verify the
 * definition tree stays in sync through all editor operations.
 *
 * These tests use the real store (createEditorStore) and exercise
 * the full pipeline: add node → definition updates → undo → definition
 * restores → redo → definition restores → export is correct.
 *
 * This is the missing integration layer between unit-level action tests
 * and E2E tests. It catches sync bugs that unit tests miss because they
 * don't exercise undo/redo or multi-step workflows through the store.
 */

import { describe, it, expect, beforeEach } from "vitest";
import type { StoreApi } from "zustand";
import type { Definition } from "@bnto/core";
import { createEditorStore } from "./createEditorStore";
import type { EditorStore } from "./types";
import { state, addNodeViaStore, removeNodeViaStore, updateParamsViaStore } from "./testHelpers";
import { rfNodesToDefinition } from "../adapters/rfNodesToDefinition";
import { findDefinitionById } from "../adapters/findDefinitionById";
import { addNode } from "../actions/addNode";

/* ── Helpers ──────────────────────────────────────────────────────── */

/** Export the store's current state as a Definition (what the engine sees). */
function exportDefinition(store: StoreApi<EditorStore>): Definition {
  const s = state(store);
  return rfNodesToDefinition(s.nodes, s.recipeMetadata, s.configs, s.definition);
}

/** Find a top-level node in the definition by type. */
function findNodeByType(def: Definition, type: string): Definition | undefined {
  return def.nodes?.find((n) => n.type === type);
}

/* ── Tests ─────────────────────────────────────────────────────────── */

describe("store — definition sync through undo/redo", () => {
  let store: StoreApi<EditorStore>;

  beforeEach(() => {
    store = createEditorStore();
  });

  it("definition exists after store initialization", () => {
    expect(state(store).definition).not.toBeNull();
    // Blank store creates input + output in definition
    expect(state(store).definition!.nodes).toHaveLength(2);
  });

  it("adding a top-level node updates the definition", () => {
    const nodeId = addNodeViaStore(store, "image-compress");
    expect(nodeId).not.toBeNull();

    const def = state(store).definition!;
    expect(def.nodes).toHaveLength(3); // input + image + output
    const imageDef = def.nodes!.find((n) => n.id === nodeId);
    expect(imageDef).toBeDefined();
    expect(imageDef!.type).toBe("image-compress");
  });

  it("undo restores the definition to the previous state", () => {
    addNodeViaStore(store, "image-compress");
    expect(state(store).definition!.nodes).toHaveLength(3);

    state(store).undo();
    expect(state(store).definition!.nodes).toHaveLength(2); // back to input + output
  });

  it("redo restores the definition to the forward state", () => {
    const nodeId = addNodeViaStore(store, "image-compress");
    state(store).undo();
    expect(state(store).definition!.nodes).toHaveLength(2);

    state(store).redo();
    expect(state(store).definition!.nodes).toHaveLength(3);
    expect(state(store).definition!.nodes!.find((n) => n.id === nodeId)).toBeDefined();
  });

  it("multiple undo/redo cycles preserve definition integrity", () => {
    addNodeViaStore(store, "image-compress");
    addNodeViaStore(store, "transform");
    expect(state(store).definition!.nodes).toHaveLength(4); // input + image + transform + output

    state(store).undo(); // remove transform
    expect(state(store).definition!.nodes).toHaveLength(3);

    state(store).undo(); // remove image
    expect(state(store).definition!.nodes).toHaveLength(2);

    state(store).redo(); // re-add image
    expect(state(store).definition!.nodes).toHaveLength(3);

    state(store).redo(); // re-add transform
    expect(state(store).definition!.nodes).toHaveLength(4);
  });

  it("removing a node updates the definition and undo restores it", () => {
    const nodeId = addNodeViaStore(store, "image-compress")!;
    expect(state(store).definition!.nodes).toHaveLength(3);

    removeNodeViaStore(store, nodeId);
    expect(state(store).definition!.nodes).toHaveLength(2);

    state(store).undo(); // undo remove
    expect(state(store).definition!.nodes).toHaveLength(3);
    expect(state(store).definition!.nodes!.find((n) => n.id === nodeId)).toBeDefined();
  });

  it("param updates flow through to definition and undo restores old params", () => {
    const nodeId = addNodeViaStore(store, "image-compress")!;
    const originalQuality = state(store).configs[nodeId]!.parameters.quality;
    updateParamsViaStore(store, nodeId, { quality: 50 });

    const defNode = state(store).definition!.nodes!.find((n) => n.id === nodeId);
    expect(defNode?.parameters?.quality).toBe(50);

    state(store).undo(); // undo param update
    const prevNode = state(store).definition!.nodes!.find((n) => n.id === nodeId);
    expect(prevNode?.parameters?.quality).toBe(originalQuality);
  });
});

describe("store — container definition sync through undo/redo", () => {
  let store: StoreApi<EditorStore>;

  beforeEach(() => {
    store = createEditorStore();
  });

  it("adding a container preserves it through undo/redo", () => {
    const loopId = addNodeViaStore(store, "loop")!;
    expect(findNodeByType(state(store).definition!, "loop")).toBeDefined();

    state(store).undo();
    expect(findNodeByType(state(store).definition!, "loop")).toBeUndefined();

    state(store).redo();
    expect(findNodeByType(state(store).definition!, "loop")).toBeDefined();
    expect(state(store).definition!.nodes!.find((n) => n.id === loopId)).toBeDefined();
  });

  it("adding a child into a container survives undo/redo", () => {
    const loopId = addNodeViaStore(store, "loop")!;

    // Add child using the addNode action with intoContainerId
    const childResult = addNode(state(store), "image-compress", null, loopId);
    expect(childResult).not.toBeNull();
    store.setState(childResult!.nextState);

    const loopDef = findDefinitionById(state(store).definition!, loopId);
    expect(loopDef?.nodes).toHaveLength(1);

    state(store).undo(); // undo child add
    const loopAfterUndo = findDefinitionById(state(store).definition!, loopId);
    expect(loopAfterUndo?.nodes ?? []).toHaveLength(0);

    state(store).redo(); // redo child add
    const loopAfterRedo = findDefinitionById(state(store).definition!, loopId);
    expect(loopAfterRedo?.nodes).toHaveLength(1);
  });
});

describe("store — export adapter round-trip", () => {
  let store: StoreApi<EditorStore>;

  beforeEach(() => {
    store = createEditorStore();
  });

  it("blank store exports a valid definition with input and output", () => {
    const exported = exportDefinition(store);
    expect(exported.nodes).toHaveLength(2);
    expect(exported.nodes![0]!.type).toBe("input");
    expect(exported.nodes![1]!.type).toBe("output");
  });

  it("exported definition includes all top-level nodes in order", () => {
    addNodeViaStore(store, "image-compress");
    addNodeViaStore(store, "transform");

    const exported = exportDefinition(store);
    expect(exported.nodes).toHaveLength(4);
    expect(exported.nodes![0]!.type).toBe("input");
    expect(exported.nodes![1]!.type).toBe("image-compress");
    expect(exported.nodes![2]!.type).toBe("transform");
    expect(exported.nodes![3]!.type).toBe("output");
  });

  it("exported definition includes container children", () => {
    const loopId = addNodeViaStore(store, "loop")!;
    const childResult = addNode(state(store), "image-compress", null, loopId);
    store.setState(childResult!.nextState);

    const exported = exportDefinition(store);
    const loopExported = exported.nodes!.find((n) => n.id === loopId);
    expect(loopExported).toBeDefined();
    expect(loopExported!.nodes).toHaveLength(1);
    expect(loopExported!.nodes![0]!.type).toBe("image-compress");
  });

  it("exported definition reflects latest param values", () => {
    const nodeId = addNodeViaStore(store, "image-compress")!;
    updateParamsViaStore(store, nodeId, { quality: 90, format: "webp" });

    const exported = exportDefinition(store);
    const imageNode = exported.nodes!.find((n) => n.id === nodeId);
    expect(imageNode?.parameters?.quality).toBe(90);
    expect(imageNode?.parameters?.format).toBe("webp");
  });

  it("exported definition after undo matches the previous state", () => {
    addNodeViaStore(store, "image-compress");
    const beforeTransform = exportDefinition(store);

    addNodeViaStore(store, "transform");
    state(store).undo();

    const afterUndo = exportDefinition(store);
    expect(afterUndo.nodes).toHaveLength(beforeTransform.nodes!.length);
  });

  it("full pipeline: Input → Loop(ImageCompress) → Output exports correctly", () => {
    const loopId = addNodeViaStore(store, "loop")!;
    const childResult = addNode(state(store), "image-compress", null, loopId);
    store.setState(childResult!.nextState);
    updateParamsViaStore(store, childResult!.nodeId, { quality: 80 });

    const exported = exportDefinition(store);

    // 3 top-level: input, loop, output
    expect(exported.nodes).toHaveLength(3);
    const loopExported = exported.nodes![1]!;
    expect(loopExported.type).toBe("loop");
    expect(loopExported.nodes).toHaveLength(1);

    const imageChild = loopExported.nodes![0]!;
    expect(imageChild.type).toBe("image-compress");
    expect(imageChild.parameters?.quality).toBe(80);
  });
});

describe("store — createBlank resets definition", () => {
  let store: StoreApi<EditorStore>;

  beforeEach(() => {
    store = createEditorStore();
  });

  it("createBlank resets definition to blank with I/O nodes", () => {
    addNodeViaStore(store, "image-compress");
    addNodeViaStore(store, "loop");
    expect(state(store).definition!.nodes!.length).toBeGreaterThan(2);

    state(store).createBlank();

    expect(state(store).definition).not.toBeNull();
    expect(state(store).definition!.nodes).toHaveLength(2);
    expect(state(store).definition!.nodes![0]!.type).toBe("input");
    expect(state(store).definition!.nodes![1]!.type).toBe("output");
  });

  it("createBlank clears undo/redo history", () => {
    addNodeViaStore(store, "image-compress");
    expect(state(store).undoStack.length).toBeGreaterThan(0);

    state(store).createBlank();
    expect(state(store).undoStack).toHaveLength(0);
    expect(state(store).redoStack).toHaveLength(0);
  });
});
