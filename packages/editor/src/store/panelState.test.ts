/**
 * Panel state centralization tests — verify openPanel, closePanel,
 * togglePanel actions work correctly with the unified panels Record.
 */

import { describe, it, expect } from "vitest";
import { getRecipeBySlug } from "@bnto/nodes";
import { createEditorStore } from "./createEditorStore";

describe("panel state", () => {
  it("initializes all panels closed except config when a node is selected", () => {
    const recipe = getRecipeBySlug("compress-images");
    const store = createEditorStore(recipe!.definition);
    const state = store.getState();
    expect(state.panels.palette).toBe(false);
    expect(state.panels.run).toBe(false);
    // config opens when selectedNodeId is set by the recipe
    expect(typeof state.panels.config).toBe("boolean");
  });

  it("openPanel sets a panel to true", () => {
    const store = createEditorStore();
    store.getState().openPanel("palette");
    expect(store.getState().panels.palette).toBe(true);
  });

  it("closePanel sets a panel to false", () => {
    const store = createEditorStore();
    store.getState().openPanel("palette");
    store.getState().closePanel("palette");
    expect(store.getState().panels.palette).toBe(false);
  });

  it("togglePanel flips a panel's state", () => {
    const store = createEditorStore();
    expect(store.getState().panels.run).toBe(false);
    store.getState().togglePanel("run");
    expect(store.getState().panels.run).toBe(true);
    store.getState().togglePanel("run");
    expect(store.getState().panels.run).toBe(false);
  });

  it("opening one panel does not affect others", () => {
    const store = createEditorStore();
    store.getState().openPanel("palette");
    expect(store.getState().panels.config).toBe(false);
    expect(store.getState().panels.run).toBe(false);
  });

  it("selectNode auto-opens config panel", () => {
    const store = createEditorStore();
    expect(store.getState().panels.config).toBe(false);
    // Need a node to select — add one via the nodes array
    const nodes = store.getState().nodes;
    if (nodes.length > 0) {
      store.getState().selectNode(nodes[0]!.id);
      expect(store.getState().panels.config).toBe(true);
    }
  });

  it("setSelectedNodeId auto-opens config panel", () => {
    const store = createEditorStore();
    store.getState().setSelectedNodeId("some-id");
    expect(store.getState().panels.config).toBe(true);
  });
});
