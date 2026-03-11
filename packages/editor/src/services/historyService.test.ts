/**
 * History service tests — verifies undo/redo through the service layer.
 */

import { describe, it, expect, beforeEach } from "vitest";
import { createEditorStore } from "../store/createEditorStore";
import { createHistoryService } from "./historyService";
import { createNodeService } from "./nodeService";
import type { HistoryService } from "../editorTypes";

describe("createHistoryService", () => {
  let service: HistoryService;
  let storeApi: ReturnType<typeof createEditorStore>;

  beforeEach(() => {
    storeApi = createEditorStore();
    service = createHistoryService(storeApi);
  });

  it("undo reverses the last change", () => {
    const nodeSvc = createNodeService(storeApi);
    const before = storeApi.getState().nodes.length;
    nodeSvc.addNode("image");
    expect(storeApi.getState().nodes.length).toBe(before + 1);
    service.undo();
    expect(storeApi.getState().nodes.length).toBe(before);
  });

  it("redo restores an undone change", () => {
    const nodeSvc = createNodeService(storeApi);
    const before = storeApi.getState().nodes.length;
    nodeSvc.addNode("image");
    service.undo();
    service.redo();
    expect(storeApi.getState().nodes.length).toBe(before + 1);
  });

  it("undo is a no-op when stack is empty", () => {
    const before = storeApi.getState().nodes.length;
    service.undo();
    expect(storeApi.getState().nodes.length).toBe(before);
  });

  it("redo is a no-op when stack is empty", () => {
    const before = storeApi.getState().nodes.length;
    service.redo();
    expect(storeApi.getState().nodes.length).toBe(before);
  });

  it("resetHistory clears both stacks", () => {
    const nodeSvc = createNodeService(storeApi);
    nodeSvc.addNode("image");
    expect(storeApi.getState().undoStack.length).toBeGreaterThan(0);
    service.resetHistory();
    expect(storeApi.getState().undoStack.length).toBe(0);
    expect(storeApi.getState().redoStack.length).toBe(0);
  });

  it("pushUndo captures current state", () => {
    service.pushUndo();
    expect(storeApi.getState().undoStack.length).toBe(1);
    expect(storeApi.getState().redoStack.length).toBe(0);
  });
});
