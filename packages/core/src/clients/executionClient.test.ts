import { describe, it, expect, vi, beforeEach } from "vitest";
import { createExecutionClient } from "./executionClient";
import { EXECUTION_STORE } from "../services/executionInstance";
import type { AuthClient } from "./authClient";
import type { ExecutionService } from "../services/executionService";
import type { BrowserExecutionService } from "../services/browserExecutionService";
import type { HistoryService } from "../services/historyService";
import type { ExecutionInstance } from "../services/executionInstance";
import type { BrowserRunResult } from "../types/browser";
import type { PipelineDefinition } from "../types/pipeline";

// ── Shared definition ──────────────────────────────────────────────────────

const COMPRESS_DEFINITION: PipelineDefinition = {
  nodes: [
    { id: "input", type: "input", params: {} },
    { id: "process", type: "image-compress", params: {} },
    { id: "output", type: "output", params: {} },
  ],
};

// ── Helpers ──────────────────────────────────────────────────────────────────

function mockExecutionService(): ExecutionService {
  return {
    getQueryOptions: vi.fn(),
    listQueryOptions: vi.fn(),
    logsQueryOptions: vi.fn(),
    start: vi.fn(),
    startPredefined: vi.fn(),
    invalidateExecution: vi.fn(),
    invalidateExecutions: vi.fn(),
  } as unknown as ExecutionService;
}

function mockHistoryService(): HistoryService {
  return {
    serverRef: vi.fn(),
    localQueryOptions: vi.fn(),
    record: vi.fn().mockResolvedValue(undefined),
    recordServerStart: vi.fn().mockResolvedValue("server-event-1"),
    recordServerComplete: vi.fn().mockResolvedValue(undefined),
    clear: vi.fn(),
    queryKey: ["local-history", "executions"],
  } as unknown as HistoryService;
}

const mockAuthClient = {
  isAuthenticated: () => false,
} as unknown as AuthClient;

const authedAuthClient = {
  isAuthenticated: () => true,
} as unknown as AuthClient;

const fakeStore = {} as ReturnType<
  typeof import("../stores/executionInstanceStore").createExecutionInstanceStore
>;

function mockInstance(runResult: BrowserRunResult): ExecutionInstance {
  return {
    run: vi.fn().mockResolvedValue(runResult),
    reset: vi.fn(),
    [EXECUTION_STORE]: fakeStore,
  };
}

function mockBrowserService(instance: ExecutionInstance): BrowserExecutionService {
  return {
    createExecution: vi.fn(() => instance),
    runPipeline: vi.fn(),
    downloadResult: vi.fn(),
    downloadAllResults: vi.fn(),
  } as unknown as BrowserExecutionService;
}

const completedResult: BrowserRunResult = {
  status: "completed",
  results: [{ blob: new Blob(), filename: "out.jpg", mimeType: "image/jpeg", metadata: {} }],
  durationMs: 150,
};

const failedResult: BrowserRunResult = {
  status: "failed",
  results: [],
  durationMs: 50,
  error: "Engine crashed",
};

const abortedResult: BrowserRunResult = {
  status: "aborted",
  results: [],
  durationMs: 10,
};

// ── Tests ────────────────────────────────────────────────────────────────────

describe("createExecutionClient — auto-recording", () => {
  let execService: ExecutionService;
  let historyService: HistoryService;

  beforeEach(() => {
    vi.clearAllMocks();
    vi.stubGlobal("crypto", { randomUUID: () => "test-uuid" });
    execService = mockExecutionService();
    historyService = mockHistoryService();
  });

  /** Create a wrapped execution instance for testing. */
  function createWrapped(result: BrowserRunResult, auth: AuthClient = mockAuthClient) {
    const instance = mockInstance(result);
    const browser = mockBrowserService(instance);
    const client = createExecutionClient(execService, browser, historyService, auth);
    return { wrapped: client.createExecution(), instance, client };
  }

  describe("completed execution", () => {
    it("records to history with correct fields", async () => {
      const { wrapped } = createWrapped(completedResult);
      const files = [new File(["a"], "a.jpg"), new File(["b"], "b.jpg")];
      await wrapped.run(COMPRESS_DEFINITION, files);

      expect(historyService.record).toHaveBeenCalledWith(
        expect.objectContaining({
          id: "test-uuid",
          slug: "image-compress",
          status: "completed",
          durationMs: 150,
          inputFileCount: 2,
          outputFileCount: 1,
        }),
      );
    });

    it("returns the original result unchanged", async () => {
      const { wrapped } = createWrapped(completedResult);
      const result = await wrapped.run(COMPRESS_DEFINITION, []);
      expect(result).toBe(completedResult);
    });
  });

  describe("failed execution", () => {
    it("records to history with error field", async () => {
      const { wrapped } = createWrapped(failedResult);
      await wrapped.run(COMPRESS_DEFINITION, []);

      expect(historyService.record).toHaveBeenCalledWith(
        expect.objectContaining({
          status: "failed",
          error: "Engine crashed",
          durationMs: 50,
          outputFileCount: 0,
        }),
      );
    });
  });

  describe("aborted execution", () => {
    it("does NOT record to history", async () => {
      const { wrapped } = createWrapped(abortedResult);
      await wrapped.run(COMPRESS_DEFINITION, []);
      expect(historyService.record).not.toHaveBeenCalled();
    });

    it("returns the aborted result unchanged", async () => {
      const { wrapped } = createWrapped(abortedResult);
      const result = await wrapped.run(COMPRESS_DEFINITION, []);
      expect(result).toBe(abortedResult);
    });
  });

  describe("recording failure is silent", () => {
    it("does not throw when history.record rejects", async () => {
      vi.mocked(historyService.record).mockRejectedValue(new Error("IndexedDB full"));
      const { wrapped } = createWrapped(completedResult);
      const result = await wrapped.run(COMPRESS_DEFINITION, []);
      expect(result).toBe(completedResult);
    });
  });

  describe("wrapped instance preserves shape", () => {
    it("preserves reset method", () => {
      const { wrapped, instance } = createWrapped(completedResult);
      wrapped.reset();
      expect(instance.reset).toHaveBeenCalled();
    });

    it("preserves EXECUTION_STORE symbol", () => {
      const { wrapped } = createWrapped(completedResult);
      expect(wrapped[EXECUTION_STORE]).toBe(fakeStore);
    });
  });

  describe("history slug derivation", () => {
    it("uses first processing node type as slug for history", async () => {
      const multiNodeDef: PipelineDefinition = {
        nodes: [
          { id: "in", type: "input", params: {} },
          { id: "resize", type: "image-resize", params: {} },
          { id: "compress", type: "image-compress", params: {} },
          { id: "out", type: "output", params: {} },
        ],
      };

      const { wrapped } = createWrapped(completedResult);
      await wrapped.run(multiNodeDef, [new File(["a"], "a.jpg")]);

      expect(historyService.record).toHaveBeenCalledWith(
        expect.objectContaining({ slug: "image-resize" }),
      );
    });
  });

  describe("server history — auth guard", () => {
    it("skips server history when unauthenticated", async () => {
      const { wrapped } = createWrapped(completedResult);
      await wrapped.run(COMPRESS_DEFINITION, [new File(["a"], "a.jpg")]);
      expect(historyService.recordServerStart).not.toHaveBeenCalled();
    });

    it("records server history when authenticated", async () => {
      const { wrapped } = createWrapped(completedResult, authedAuthClient);
      await wrapped.run(COMPRESS_DEFINITION, [new File(["a"], "a.jpg")]);

      expect(historyService.recordServerStart).toHaveBeenCalledWith("image-compress");
      expect(historyService.recordServerComplete).toHaveBeenCalled();
    });
  });
});
