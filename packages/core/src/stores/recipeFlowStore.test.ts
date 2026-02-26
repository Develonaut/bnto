import { describe, it, expect, beforeEach } from "vitest";
import { createRecipeFlowStore } from "./recipeFlowStore";

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

const DEFAULT_CONFIG = { quality: 80 };

function mockFile(name: string, size = 1024): File {
  return new File([new ArrayBuffer(size)], name, { type: "image/jpeg" });
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

describe("recipeFlowStore", () => {
  let store: ReturnType<typeof createRecipeFlowStore>;

  beforeEach(() => {
    store = createRecipeFlowStore(DEFAULT_CONFIG);
  });

  describe("initial state", () => {
    it("starts with empty files and default config", () => {
      const state = store.getState();
      expect(state.files).toEqual([]);
      expect(state.config).toEqual({ quality: 80 });
      expect(state.executionId).toBeNull();
      expect(state.cloudPhase).toBe("idle");
      expect(state.clientError).toBeNull();
    });

    it("creates independent config copies from defaults", () => {
      const shared = { quality: 80 };
      const store1 = createRecipeFlowStore(shared);
      const store2 = createRecipeFlowStore(shared);

      store1.getState().setConfig({ quality: 50 });

      expect(store1.getState().config).toEqual({ quality: 50 });
      expect(store2.getState().config).toEqual({ quality: 80 });
      expect(shared).toEqual({ quality: 80 });
    });

    it("defaults to empty config when no defaults provided", () => {
      const bare = createRecipeFlowStore();
      expect(bare.getState().config).toEqual({});
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
      store.getState().setConfig({ quality: 50 });
      store.getState().setFiles([mockFile("x.jpg")]);

      expect(store.getState().config).toEqual({ quality: 50 });
      expect(store.getState().cloudPhase).toBe("idle");
      expect(store.getState().executionId).toBeNull();
    });

    it("can set an empty file list", () => {
      store.getState().setFiles([mockFile("a.jpg")]);
      store.getState().setFiles([]);

      expect(store.getState().files).toEqual([]);
    });
  });

  describe("setConfig", () => {
    it("replaces the config object", () => {
      store.getState().setConfig({ quality: 50 });
      expect(store.getState().config).toEqual({ quality: 50 });
    });

    it("can set a completely different config shape", () => {
      store.getState().setConfig({ width: 800, maintainAspectRatio: true });
      expect(store.getState().config).toEqual({
        width: 800,
        maintainAspectRatio: true,
      });
    });

    it("does not affect files or phase", () => {
      store.getState().setFiles([mockFile("a.jpg")]);
      store.getState().setConfig({ quality: 30 });

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
      store.getState().setConfig({ quality: 60 });
      store.getState().startUpload();

      expect(store.getState().files).toBe(files);
      expect(store.getState().config).toEqual({ quality: 60 });
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
      store.getState().setConfig({ quality: 70 });
      store.getState().startExecution("exec-1");

      expect(store.getState().files).toHaveLength(1);
      expect(store.getState().config).toEqual({ quality: 70 });
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
      store.getState().failCloud("Go API timeout");

      expect(store.getState().executionId).toBe("exec-99");
      expect(store.getState().cloudPhase).toBe("failed");
      expect(store.getState().clientError).toBe("Go API timeout");
    });

    it("can fail from idle (client-side validation error)", () => {
      store.getState().failCloud("No files selected");

      expect(store.getState().cloudPhase).toBe("failed");
      expect(store.getState().clientError).toBe("No files selected");
    });
  });

  describe("reset", () => {
    it("returns to initial state with default config", () => {
      store.getState().setFiles([mockFile("a.jpg"), mockFile("b.jpg")]);
      store.getState().setConfig({ quality: 30 });
      store.getState().startUpload();
      store.getState().startExecution("exec-5");

      store.getState().reset();
      const state = store.getState();

      expect(state.files).toEqual([]);
      expect(state.config).toEqual({ quality: 80 });
      expect(state.executionId).toBeNull();
      expect(state.cloudPhase).toBe("idle");
      expect(state.clientError).toBeNull();
    });

    it("clears error state", () => {
      store.getState().failCloud("something broke");
      store.getState().reset();

      expect(store.getState().clientError).toBeNull();
      expect(store.getState().cloudPhase).toBe("idle");
    });

    it("reset config is independent from prior mutations", () => {
      store.getState().setConfig({ quality: 10 });
      store.getState().reset();

      expect(store.getState().config).toEqual({ quality: 80 });
    });

    it("can be called multiple times safely", () => {
      store.getState().reset();
      store.getState().reset();
      store.getState().reset();

      expect(store.getState().cloudPhase).toBe("idle");
      expect(store.getState().files).toEqual([]);
    });
  });

  describe("full cloud execution flow", () => {
    it("tracks idle → uploading → running → failed → reset", () => {
      const files = [mockFile("photo.jpg")];
      store.getState().setFiles(files);
      store.getState().setConfig({ quality: 60 });

      // Start upload
      store.getState().startUpload();
      expect(store.getState().cloudPhase).toBe("uploading");

      // Execution starts
      store.getState().startExecution("exec-123");
      expect(store.getState().cloudPhase).toBe("running");
      expect(store.getState().executionId).toBe("exec-123");

      // Execution fails
      store.getState().failCloud("Server returned 500");
      expect(store.getState().cloudPhase).toBe("failed");
      expect(store.getState().clientError).toBe("Server returned 500");

      // User resets to try again
      store.getState().reset();
      expect(store.getState().cloudPhase).toBe("idle");
      expect(store.getState().files).toEqual([]);
      expect(store.getState().config).toEqual({ quality: 80 });
    });

    it("tracks idle → uploading → running (happy path — completed is set externally)", () => {
      store.getState().setFiles([mockFile("data.csv")]);
      store.getState().startUpload();
      store.getState().startExecution("exec-456");

      // The "completed" phase is typically set by the cloud execution
      // polling/subscription path, not by the store directly.
      // The store's cloudPhase tracks client-initiated transitions only.
      expect(store.getState().cloudPhase).toBe("running");
      expect(store.getState().executionId).toBe("exec-456");
    });
  });
});
