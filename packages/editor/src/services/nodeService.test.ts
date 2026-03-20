/**
 * Node service tests — verifies service wrapping of pure actions.
 */

import { describe, it, expect, beforeEach } from "vitest";
import { createEditorStore } from "../store/createEditorStore";
import { createNodeService } from "./nodeService";
import type { NodeService } from "../editorTypes";

describe("createNodeService", () => {
  let service: NodeService;

  beforeEach(() => {
    const storeApi = createEditorStore();
    service = createNodeService(storeApi);
  });

  it("addNode returns a node ID on success", () => {
    const id = service.addNode("image-compress");
    expect(id).toBeTruthy();
  });

  it("addNode returns null for duplicate I/O node", () => {
    const id = service.addNode("input");
    expect(id).toBeNull();
  });

  it("removeNode returns true on success", () => {
    const id = service.addNode("image-compress")!;
    expect(service.removeNode(id)).toBe(true);
  });

  it("removeNode returns false for I/O node", () => {
    // Need to find the input node ID from the store
    const storeApi = createEditorStore();
    const svc = createNodeService(storeApi);
    const inputNode = storeApi
      .getState()
      .nodes.find((n) => storeApi.getState().configs[n.id]?.nodeType === "input");
    expect(svc.removeNode(inputNode!.id)).toBe(false);
  });

  it("selectNode changes selection", () => {
    const storeApi = createEditorStore();
    const svc = createNodeService(storeApi);
    const id = svc.addNode("image-compress")!;
    svc.selectNode(null);
    svc.selectNode(id);
    const node = storeApi.getState().nodes.find((n) => n.id === id);
    expect(node?.selected).toBe(true);
  });

  it("setInsertAfterNodeId sets insertion context", () => {
    const storeApi = createEditorStore();
    const svc = createNodeService(storeApi);
    svc.setInsertAfterNodeId("some-id");
    expect(storeApi.getState().insertAfterNodeId).toBe("some-id");
    svc.setInsertAfterNodeId(null);
    expect(storeApi.getState().insertAfterNodeId).toBeNull();
  });

  it("setInsertIntoContainerId sets insertion context", () => {
    const storeApi = createEditorStore();
    const svc = createNodeService(storeApi);
    svc.setInsertIntoContainerId("container-id");
    expect(storeApi.getState().insertIntoContainerId).toBe("container-id");
  });
});
