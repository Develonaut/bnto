/**
 * createEditor factory tests — imperative API, no React.
 *
 * Tests the full editor instance: factory creation, domain client
 * operations, undo/redo, state access, and subscription.
 */

import { describe, it, expect, vi } from "vitest";
import { getRecipeBySlug } from "@bnto/nodes";
import { createEditor } from "./createEditor";

function compressImagesDefinition() {
  const recipe = getRecipeBySlug("compress-images");
  if (!recipe) throw new Error("compress-images recipe not found");
  return recipe.definition;
}

describe("createEditor", () => {
  // --- Factory creation ---

  it("creates an instance with all domain clients", () => {
    const editor = createEditor();
    expect(editor.nodes).toBeDefined();
    expect(editor.definition).toBeDefined();
    expect(editor.execution).toBeDefined();
    expect(editor.history).toBeDefined();
    expect(editor.panels).toBeDefined();
    expect(editor.getState).toBeTypeOf("function");
    expect(editor.subscribe).toBeTypeOf("function");
    expect(editor.destroy).toBeTypeOf("function");
    expect(editor._storeApi).toBeDefined();
  });

  it("creates a blank editor with I/O nodes when no definition given", () => {
    const editor = createEditor();
    const state = editor.getState();
    expect(state.nodes.length).toBe(2); // input + output
    expect(state.isDirty).toBe(false);
    expect(state.recipeMetadata.type).toBe("group");
  });

  it("initializes from a definition with recipe nodes", () => {
    const def = compressImagesDefinition();
    const editor = createEditor(def);
    const state = editor.getState();
    expect(state.recipeMetadata.name).toBe("Compress Images");
    expect(state.nodes.length).toBeGreaterThan(2);
  });

  // --- nodes client ---

  it("adds a node via nodes client", () => {
    const editor = createEditor();
    const before = editor.getState().nodes.length;
    const id = editor.nodes.addNode("image");
    expect(id).toBeTruthy();
    expect(editor.getState().nodes.length).toBe(before + 1);
  });

  it("returns null when adding a duplicate I/O node", () => {
    const editor = createEditor();
    // Already has input node from blank init
    const id = editor.nodes.addNode("input");
    expect(id).toBeNull();
  });

  it("removes a node via nodes client", () => {
    const editor = createEditor();
    const nodeId = editor.nodes.addNode("image")!;
    const before = editor.getState().nodes.length;
    const removed = editor.nodes.removeNode(nodeId);
    expect(removed).toBe(true);
    expect(editor.getState().nodes.length).toBe(before - 1);
  });

  it("returns false when removing an I/O node", () => {
    const editor = createEditor();
    const inputNode = editor.getState().nodes.find(
      (n) => editor.getState().configs[n.id]?.nodeType === "input",
    );
    const removed = editor.nodes.removeNode(inputNode!.id);
    expect(removed).toBe(false);
  });

  it("selects a node via nodes client", () => {
    const editor = createEditor();
    const nodeId = editor.nodes.addNode("image")!;
    editor.nodes.selectNode(null);
    editor.nodes.selectNode(nodeId);
    const selected = editor.getState().nodes.find((n) => n.id === nodeId);
    expect(selected?.selected).toBe(true);
  });

  // --- definition client ---

  it("updates params via definition client", () => {
    const editor = createEditor();
    const nodeId = editor.nodes.addNode("image")!;
    const ok = editor.definition.updateParams(nodeId, { quality: 80 });
    expect(ok).toBe(true);
    expect(editor.getState().configs[nodeId]?.parameters.quality).toBe(80);
  });

  it("returns false updating params for nonexistent node", () => {
    const editor = createEditor();
    const ok = editor.definition.updateParams("nonexistent", { quality: 80 });
    expect(ok).toBe(false);
  });

  it("exports as definition via definition client", () => {
    const editor = createEditor();
    editor.nodes.addNode("image");
    const def = editor.definition.exportAsDefinition();
    expect(def.nodes).toBeDefined();
    expect(def.nodes!.length).toBeGreaterThan(0);
  });

  it("exports as recipe via definition client", () => {
    const editor = createEditor();
    editor.nodes.addNode("image");
    const result = editor.definition.exportAsRecipe();
    // May have validation errors for incomplete recipe — that's ok
    expect(result).toBeDefined();
    expect(result.errors).toBeDefined();
  });

  it("marks and resets dirty flag via definition client", () => {
    const editor = createEditor();
    expect(editor.getState().isDirty).toBe(false);
    editor.definition.markDirty();
    expect(editor.getState().isDirty).toBe(true);
    editor.definition.resetDirty();
    expect(editor.getState().isDirty).toBe(false);
  });

  // --- history client ---

  it("undo reverses addNode", () => {
    const editor = createEditor();
    const before = editor.getState().nodes.length;
    editor.nodes.addNode("image");
    expect(editor.getState().nodes.length).toBe(before + 1);
    editor.history.undo();
    expect(editor.getState().nodes.length).toBe(before);
  });

  it("redo restores undone addNode", () => {
    const editor = createEditor();
    const before = editor.getState().nodes.length;
    editor.nodes.addNode("image");
    editor.history.undo();
    expect(editor.getState().nodes.length).toBe(before);
    editor.history.redo();
    expect(editor.getState().nodes.length).toBe(before + 1);
  });

  it("undo is a no-op when stack is empty", () => {
    const editor = createEditor();
    const before = editor.getState().nodes.length;
    editor.history.undo();
    expect(editor.getState().nodes.length).toBe(before);
  });

  it("resetHistory clears both stacks", () => {
    const editor = createEditor();
    editor.nodes.addNode("image");
    expect(editor.getState().undoStack.length).toBeGreaterThan(0);
    editor.history.resetHistory();
    expect(editor.getState().undoStack.length).toBe(0);
    expect(editor.getState().redoStack.length).toBe(0);
  });

  // --- panels client ---

  it("opens and closes panels", () => {
    const editor = createEditor();
    editor.panels.openPanel("run");
    expect(editor.getState().panels.run).toBe(true);
    editor.panels.closePanel("run");
    expect(editor.getState().panels.run).toBe(false);
  });

  it("toggles a panel", () => {
    const editor = createEditor();
    editor.panels.togglePanel("palette");
    expect(editor.getState().panels.palette).toBe(true);
    editor.panels.togglePanel("palette");
    expect(editor.getState().panels.palette).toBe(false);
  });

  // --- getState and subscribe ---

  it("getState returns current state", () => {
    const editor = createEditor();
    const state = editor.getState();
    expect(state.nodes).toBeDefined();
    expect(state.configs).toBeDefined();
    expect(state.recipeMetadata).toBeDefined();
  });

  it("subscribe fires on state changes", () => {
    const editor = createEditor();
    const listener = vi.fn();
    const unsub = editor.subscribe(listener);
    editor.nodes.addNode("image");
    expect(listener).toHaveBeenCalled();
    unsub();
  });

  it("unsubscribe stops firing", () => {
    const editor = createEditor();
    const listener = vi.fn();
    const unsub = editor.subscribe(listener);
    unsub();
    editor.nodes.addNode("image");
    expect(listener).not.toHaveBeenCalled();
  });

  // --- destroy ---

  it("destroy does not throw", () => {
    const editor = createEditor();
    expect(() => editor.destroy()).not.toThrow();
  });
});
