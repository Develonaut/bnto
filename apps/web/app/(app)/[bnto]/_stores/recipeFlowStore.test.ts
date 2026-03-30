import { describe, it, expect, beforeEach } from "vitest";
import type { StoreApi } from "zustand/vanilla";
import { createRecipeFlowStore } from "./recipeFlowStore";
import type { RecipeFlowState } from "./recipeFlowStore";

const DEFAULT_CONFIG = { "compress-image": { quality: 80 } };

function mockFile(name: string, size = 1024): File {
  return new File([new ArrayBuffer(size)], name, { type: "image/jpeg" });
}

describe("recipeFlowStore", () => {
  let store: StoreApi<RecipeFlowState>;

  beforeEach(() => {
    store = createRecipeFlowStore(DEFAULT_CONFIG);
  });

  describe("initial state", () => {
    it("starts with empty files and default per-node config", () => {
      const state = store.getState();
      expect(state.files).toEqual([]);
      expect(state.config).toEqual({ "compress-image": { quality: 80 } });
      expect(state.executionId).toBeNull();
      expect(state.cloudPhase).toBe("idle");
      expect(state.clientError).toBeNull();
      expect(state.activeStep).toBe(1);
      expect(state.resolvedPhase).toBe("idle");
      expect(state.isProcessing).toBe(false);
    });

    it("creates independent config copies from defaults", () => {
      const shared = { "compress-image": { quality: 80 } };
      const store1 = createRecipeFlowStore(shared);
      const store2 = createRecipeFlowStore(shared);

      store1.getState().setNodeParam("compress-image", "quality", 50);

      expect(store1.getState().config).toEqual({ "compress-image": { quality: 50 } });
      expect(store2.getState().config).toEqual({ "compress-image": { quality: 80 } });
      expect(shared).toEqual({ "compress-image": { quality: 80 } });
    });

    it("defaults to empty config when reset with empty object", () => {
      store.getState().reset({});
      expect(store.getState().config).toEqual({});
    });
  });

  describe("setFiles", () => {
    it("replaces the file list", () => {
      const files = [mockFile("a.jpg"), mockFile("b.jpg")];
      store.getState().setFiles(files);

      expect(store.getState().files).toBe(files);
      expect(store.getState().files).toHaveLength(2);
    });

    it("clears client error when files are set", () => {
      store.getState().failCloud("upload timed out");
      expect(store.getState().clientError).toBe("upload timed out");

      store.getState().setFiles([mockFile("fresh.jpg")]);
      expect(store.getState().clientError).toBeNull();
    });

    it("does not affect other state fields", () => {
      store.getState().setNodeParam("compress-image", "quality", 50);
      store.getState().setFiles([mockFile("x.jpg")]);

      expect(store.getState().config).toEqual({ "compress-image": { quality: 50 } });
      expect(store.getState().cloudPhase).toBe("idle");
      expect(store.getState().executionId).toBeNull();
    });

    it("can set an empty file list", () => {
      store.getState().setFiles([mockFile("a.jpg")]);
      store.getState().setFiles([]);

      expect(store.getState().files).toEqual([]);
    });
  });

  describe("setNodeParam", () => {
    it("updates a single param on a single node", () => {
      store.getState().setNodeParam("compress-image", "quality", 50);
      expect(store.getState().config).toEqual({ "compress-image": { quality: 50 } });
    });

    it("preserves other params on the same node", () => {
      const multiStore = createRecipeFlowStore({
        resize: { width: 800, maintainAspectRatio: true },
      });
      multiStore.getState().setNodeParam("resize", "width", 1200);

      expect(multiStore.getState().config).toEqual({
        resize: { width: 1200, maintainAspectRatio: true },
      });
    });

    it("preserves other nodes when updating one", () => {
      const multiStore = createRecipeFlowStore({
        resize: { width: 800 },
        compress: { quality: 80 },
      });
      multiStore.getState().setNodeParam("resize", "width", 640);

      expect(multiStore.getState().config).toEqual({
        resize: { width: 640 },
        compress: { quality: 80 },
      });
    });

    it("does not affect files or phase", () => {
      store.getState().setFiles([mockFile("a.jpg")]);
      store.getState().setNodeParam("compress-image", "quality", 30);

      expect(store.getState().files).toHaveLength(1);
      expect(store.getState().cloudPhase).toBe("idle");
    });
  });

  describe("startUpload", () => {
    it("transitions cloudPhase to uploading", () => {
      store.getState().startUpload();
      expect(store.getState().cloudPhase).toBe("uploading");
    });

    it("clears client error", () => {
      store.getState().failCloud("previous error");
      store.getState().startUpload();

      expect(store.getState().clientError).toBeNull();
      expect(store.getState().cloudPhase).toBe("uploading");
    });

    it("preserves files and config", () => {
      const files = [mockFile("a.jpg")];
      store.getState().setFiles(files);
      store.getState().setNodeParam("compress-image", "quality", 60);
      store.getState().startUpload();

      expect(store.getState().files).toBe(files);
      expect(store.getState().config).toEqual({ "compress-image": { quality: 60 } });
    });
  });

  describe("startExecution", () => {
    it("sets executionId and transitions to running", () => {
      store.getState().startUpload();
      store.getState().startExecution("exec-42");

      expect(store.getState().executionId).toBe("exec-42");
      expect(store.getState().cloudPhase).toBe("running");
    });

    it("preserves files and config", () => {
      store.getState().setFiles([mockFile("a.jpg")]);
      store.getState().setNodeParam("compress-image", "quality", 70);
      store.getState().startExecution("exec-1");

      expect(store.getState().files).toHaveLength(1);
      expect(store.getState().config).toEqual({ "compress-image": { quality: 70 } });
    });
  });

  describe("failCloud", () => {
    it("sets cloudPhase to failed and records error", () => {
      store.getState().startUpload();
      store.getState().failCloud("R2 upload rejected");

      expect(store.getState().cloudPhase).toBe("failed");
      expect(store.getState().clientError).toBe("R2 upload rejected");
    });

    it("preserves executionId from a started execution", () => {
      store.getState().startExecution("exec-99");
      store.getState().failCloud("Server timeout");

      expect(store.getState().executionId).toBe("exec-99");
      expect(store.getState().cloudPhase).toBe("failed");
      expect(store.getState().clientError).toBe("Server timeout");
    });

    it("can fail from idle (client-side validation error)", () => {
      store.getState().failCloud("No files selected");

      expect(store.getState().cloudPhase).toBe("failed");
      expect(store.getState().clientError).toBe("No files selected");
    });
  });

  describe("reset", () => {
    it("returns to initial state with given config", () => {
      store.getState().setFiles([mockFile("a.jpg"), mockFile("b.jpg")]);
      store.getState().setNodeParam("compress-image", "quality", 30);
      store.getState().startUpload();
      store.getState().startExecution("exec-5");
      store.getState().setDerivedStep(3, "running");

      store.getState().reset(DEFAULT_CONFIG);
      const state = store.getState();

      expect(state.files).toEqual([]);
      expect(state.config).toEqual({ "compress-image": { quality: 80 } });
      expect(state.executionId).toBeNull();
      expect(state.cloudPhase).toBe("idle");
      expect(state.clientError).toBeNull();
      expect(state.activeStep).toBe(1);
      expect(state.resolvedPhase).toBe("idle");
      expect(state.isProcessing).toBe(false);
    });

    it("clears error state", () => {
      store.getState().failCloud("something broke");
      store.getState().reset(DEFAULT_CONFIG);

      expect(store.getState().clientError).toBeNull();
      expect(store.getState().cloudPhase).toBe("idle");
    });

    it("reset config is independent from prior mutations", () => {
      store.getState().setNodeParam("compress-image", "quality", 10);
      store.getState().reset();

      expect(store.getState().config).toEqual({ "compress-image": { quality: 80 } });
    });

    it("can be called multiple times safely", () => {
      store.getState().reset(DEFAULT_CONFIG);
      store.getState().reset(DEFAULT_CONFIG);
      store.getState().reset(DEFAULT_CONFIG);

      expect(store.getState().cloudPhase).toBe("idle");
      expect(store.getState().files).toEqual([]);
    });
  });

  describe("setDerivedStep", () => {
    it("sets activeStep, resolvedPhase, and isProcessing", () => {
      store.getState().setDerivedStep(3, "running");

      expect(store.getState().activeStep).toBe(3);
      expect(store.getState().resolvedPhase).toBe("running");
      expect(store.getState().isProcessing).toBe(true);
    });

    it("marks isProcessing false for non-processing phases", () => {
      store.getState().setDerivedStep(2, "idle");

      expect(store.getState().isProcessing).toBe(false);
    });

    it("marks isProcessing true for uploading", () => {
      store.getState().setDerivedStep(3, "uploading");

      expect(store.getState().isProcessing).toBe(true);
    });
  });

  describe("full cloud execution flow", () => {
    it("idle -> uploading -> running -> failed -> reset", () => {
      store.getState().setFiles([mockFile("photo.jpg")]);
      store.getState().startUpload();
      store.getState().startExecution("exec-123");
      store.getState().failCloud("Server returned 500");

      expect(store.getState().cloudPhase).toBe("failed");
      expect(store.getState().clientError).toBe("Server returned 500");

      store.getState().reset();
      expect(store.getState().cloudPhase).toBe("idle");
      expect(store.getState().config).toEqual({ "compress-image": { quality: 80 } });
    });
  });

  describe("factory isolation", () => {
    it("different factory calls produce independent stores", () => {
      const store2 = createRecipeFlowStore({ format: { value: "webp" } });

      store.getState().setFiles([mockFile("a.jpg")]);
      expect(store2.getState().files).toEqual([]);
      expect(store2.getState().config).toEqual({ format: { value: "webp" } });
    });
  });
});
