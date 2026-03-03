import { describe, it, expect, vi, beforeEach } from "vitest";

// Mock adapters and client before imports
const mockFuncRef = { __type: "funcRef" };

vi.mock("../adapters/convex/executionAdapter", () => ({
  getExecutionHistoryRef: vi.fn(() => ({
    funcRef: mockFuncRef,
    args: {},
  })),
}));

vi.mock("../adapters/local/localHistoryAdapter", () => ({
  addEntry: vi.fn(),
  getEntries: vi.fn(() => []),
  clearEntries: vi.fn(),
}));

vi.mock("../client", () => ({
  getQueryClient: vi.fn(() => ({
    invalidateQueries: vi.fn(),
  })),
}));

import { createHistoryService } from "./historyService";
import { addEntry, clearEntries } from "../adapters/local/localHistoryAdapter";
import { getQueryClient } from "../client";
import type { LocalHistoryEntry } from "../types/localHistory";

const mockAddEntry = vi.mocked(addEntry);
const mockClearEntries = vi.mocked(clearEntries);
const mockGetQueryClient = vi.mocked(getQueryClient);

describe("createHistoryService", () => {
  let service: ReturnType<typeof createHistoryService>;

  beforeEach(() => {
    vi.clearAllMocks();
    service = createHistoryService();
  });

  describe("serverRef", () => {
    it("returns funcRef, args, and transform", () => {
      const result = service.serverRef();

      expect(result.funcRef).toBe(mockFuncRef);
      expect(result.args).toEqual({});
      expect(typeof result.transform).toBe("function");
    });

    it("transform converts raw execution doc to Execution type", () => {
      const { transform } = service.serverRef();

      const rawDoc = {
        _id: "exec_123",
        userId: "user_456",
        workflowId: "wf_789",
        status: "completed" as const,
        progress: [{ nodeId: "n1", status: "completed" }],
        startedAt: 1000,
        completedAt: 2000,
      };

      const result = transform(rawDoc);

      expect(result.id).toBe("exec_123");
      expect(result.userId).toBe("user_456");
      expect(result.workflowId).toBe("wf_789");
      expect(result.status).toBe("completed");
      expect(result.startedAt).toBe(1000);
      expect(result.completedAt).toBe(2000);
    });

    it("transform handles nullable fields", () => {
      const { transform } = service.serverRef();

      const rawDoc = {
        _id: "exec_456",
        userId: "user_789",
        status: "pending" as const,
        progress: [],
        startedAt: 3000,
        workflowId: null,
        error: null,
        result: null,
        outputFiles: null,
        sessionId: null,
        completedAt: null,
      };

      const result = transform(rawDoc);

      expect(result.workflowId).toBeUndefined();
      expect(result.error).toBeUndefined();
      expect(result.result).toBeUndefined();
      expect(result.outputFiles).toBeUndefined();
      expect(result.sessionId).toBeUndefined();
      expect(result.completedAt).toBeUndefined();
    });
  });

  describe("localQueryOptions", () => {
    it("returns query key and staleTime", () => {
      const options = service.localQueryOptions();

      expect(options.queryKey).toEqual(["local-history", "executions"]);
      expect(options.staleTime).toBe(Infinity);
      expect(typeof options.queryFn).toBe("function");
    });
  });

  describe("record", () => {
    it("adds entry to IndexedDB and invalidates cache", async () => {
      const mockInvalidate = vi.fn();
      mockGetQueryClient.mockReturnValue({
        invalidateQueries: mockInvalidate,
      } as never);

      const entry: LocalHistoryEntry = {
        id: "test-1",
        slug: "compress-images",
        status: "completed",
        timestamp: Date.now(),
        durationMs: 500,
        inputFileCount: 1,
        outputFileCount: 1,
      };

      await service.record(entry);

      expect(mockAddEntry).toHaveBeenCalledWith(entry);
      expect(mockInvalidate).toHaveBeenCalledWith({
        queryKey: ["local-history", "executions"],
      });
    });
  });

  describe("clear", () => {
    it("clears IndexedDB and invalidates cache", async () => {
      const mockInvalidate = vi.fn();
      mockGetQueryClient.mockReturnValue({
        invalidateQueries: mockInvalidate,
      } as never);

      await service.clear();

      expect(mockClearEntries).toHaveBeenCalled();
      expect(mockInvalidate).toHaveBeenCalledWith({
        queryKey: ["local-history", "executions"],
      });
    });
  });
});
