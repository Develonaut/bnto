import { describe, it, expect, vi, beforeEach } from "vitest";

// Mock adapters and client before imports
const mockFuncRef = { __type: "funcRef" };

vi.mock("../adapters/convex/executionAdapter", () => ({
  getExecutionHistoryRef: vi.fn(() => ({
    funcRef: mockFuncRef,
    args: {},
  })),
  logExecutionEventStart: vi.fn(),
  completeExecutionEvent: vi.fn(),
  migrateLocalHistory: vi.fn(),
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

    it("transform converts raw execution event doc to Execution type", () => {
      const { transform } = service.serverRef();

      const rawDoc = {
        _id: "evt_123",
        userId: "user_456",
        slug: "compress-images",
        status: "completed" as const,
        timestamp: 1000,
        durationMs: 1000,
      };

      const result = transform(rawDoc);

      expect(result.id).toBe("evt_123");
      expect(result.userId).toBe("user_456");
      expect(result.slug).toBe("compress-images");
      expect(result.status).toBe("completed");
      expect(result.startedAt).toBe(1000);
      expect(result.completedAt).toBe(2000);
    });

    it("transform handles started events (no duration)", () => {
      const { transform } = service.serverRef();

      const rawDoc = {
        _id: "evt_456",
        userId: "user_789",
        slug: "clean-csv",
        status: "started" as const,
        timestamp: 3000,
        durationMs: null,
      };

      const result = transform(rawDoc);

      expect(result.slug).toBe("clean-csv");
      expect(result.status).toBe("running");
      expect(result.startedAt).toBe(3000);
      expect(result.completedAt).toBeUndefined();
      expect(result.error).toBeUndefined();
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
