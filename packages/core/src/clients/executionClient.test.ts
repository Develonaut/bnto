import { describe, it, expect, vi, beforeEach } from "vitest";
import { createExecutionClient } from "./executionClient";
import { EXECUTION_STORE } from "../services/executionInstance";
import type { ExecutionService } from "../services/executionService";
import type { BrowserExecutionService } from "../services/browserExecutionService";
import type { HistoryService } from "../services/historyService";
import type { ExecutionInstance } from "../services/executionInstance";
import type { BrowserRunResult } from "../types/browser";
import type { PipelineDefinition } from "../engine/types";

// ── Shared definition ──────────────────────────────────────────────────────

const COMPRESS_DEFINITION: PipelineDefinition = {
  nodes: [
    { id: "input", type: "input", params: {} },
    { id: "process", type: "compress-images", params: {} },
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
    clear: vi.fn(),
    queryKey: ["local-history", "executions"],
  } as unknown as HistoryService;
}

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
    isCapable: vi.fn(),
    hasImplementation: vi.fn(),
    getCapableSlugs: vi.fn(),
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

  describe("completed execution", () => {
    it("records to history with correct fields", async () => {
      const instance = mockInstance(completedResult);
      const browser = mockBrowserService(instance);
      const client = createExecutionClient(execService, browser, historyService);
      const wrapped = client.createExecution();

      const files = [new File(["a"], "a.jpg"), new File(["b"], "b.jpg")];
      await wrapped.run(COMPRESS_DEFINITION, files);

      expect(historyService.record).toHaveBeenCalledWith(
        expect.objectContaining({
          id: "test-uuid",
          slug: "compress-images",
          status: "completed",
          durationMs: 150,
          inputFileCount: 2,
          outputFileCount: 1,
        }),
      );
    });

    it("returns the original result unchanged", async () => {
      const instance = mockInstance(completedResult);
      const browser = mockBrowserService(instance);
      const client = createExecutionClient(execService, browser, historyService);
      const wrapped = client.createExecution();

      const result = await wrapped.run(COMPRESS_DEFINITION, []);
      expect(result).toBe(completedResult);
    });
  });

  describe("failed execution", () => {
    it("records to history with error field", async () => {
      const instance = mockInstance(failedResult);
      const browser = mockBrowserService(instance);
      const client = createExecutionClient(execService, browser, historyService);
      const wrapped = client.createExecution();

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
      const instance = mockInstance(abortedResult);
      const browser = mockBrowserService(instance);
      const client = createExecutionClient(execService, browser, historyService);
      const wrapped = client.createExecution();

      await wrapped.run(COMPRESS_DEFINITION, []);
      expect(historyService.record).not.toHaveBeenCalled();
    });

    it("returns the aborted result unchanged", async () => {
      const instance = mockInstance(abortedResult);
      const browser = mockBrowserService(instance);
      const client = createExecutionClient(execService, browser, historyService);
      const wrapped = client.createExecution();

      const result = await wrapped.run(COMPRESS_DEFINITION, []);
      expect(result).toBe(abortedResult);
    });
  });

  describe("recording failure is silent", () => {
    it("does not throw when history.record rejects", async () => {
      const instance = mockInstance(completedResult);
      const browser = mockBrowserService(instance);
      vi.mocked(historyService.record).mockRejectedValue(new Error("IndexedDB full"));
      const client = createExecutionClient(execService, browser, historyService);
      const wrapped = client.createExecution();

      const result = await wrapped.run(COMPRESS_DEFINITION, []);
      expect(result).toBe(completedResult);
    });
  });

  describe("wrapped instance preserves shape", () => {
    it("preserves reset method", () => {
      const instance = mockInstance(completedResult);
      const browser = mockBrowserService(instance);
      const client = createExecutionClient(execService, browser, historyService);
      const wrapped = client.createExecution();

      wrapped.reset();
      expect(instance.reset).toHaveBeenCalled();
    });

    it("preserves EXECUTION_STORE symbol", () => {
      const instance = mockInstance(completedResult);
      const browser = mockBrowserService(instance);
      const client = createExecutionClient(execService, browser, historyService);
      const wrapped = client.createExecution();

      expect(wrapped[EXECUTION_STORE]).toBe(fakeStore);
    });
  });

  describe("history slug derivation", () => {
    it("uses first processing node type as slug for history", async () => {
      const multiNodeDef: PipelineDefinition = {
        nodes: [
          { id: "in", type: "input", params: {} },
          { id: "resize", type: "resize-images", params: {} },
          { id: "compress", type: "compress-images", params: {} },
          { id: "out", type: "output", params: {} },
        ],
      };

      const instance = mockInstance(completedResult);
      const browser = mockBrowserService(instance);
      const client = createExecutionClient(execService, browser, historyService);
      const wrapped = client.createExecution();

      await wrapped.run(multiNodeDef, [new File(["a"], "a.jpg")]);

      expect(historyService.record).toHaveBeenCalledWith(
        expect.objectContaining({ slug: "resize-images" }),
      );
    });
  });
});
