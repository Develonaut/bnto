import { describe, it, expect, vi, beforeEach } from "vitest";

// ── Mocks ──────────────────────────────────────────────────────────────────
let mockReady = true;
const mockFuncRef = { __type: "funcRef" };
const mockArgs = { someArg: true };
const mockTransform = vi.fn((doc: Record<string, unknown>) => ({
  id: doc._id,
  transformed: true,
}));

const mockUsePaginatedQuery = vi.fn();

vi.mock("convex/react", () => ({
  usePaginatedQuery: (...args: unknown[]) => mockUsePaginatedQuery(...args),
}));

vi.mock("./useReady", () => ({
  useReady: vi.fn(() => mockReady),
}));

vi.mock("../core", () => ({
  core: {
    executions: {
      historyRefMethod: vi.fn(() => ({
        funcRef: mockFuncRef,
        args: mockArgs,
        transform: mockTransform,
      })),
    },
  },
}));

vi.mock("react", () => ({
  useMemo: vi.fn((fn: () => unknown) => fn()),
}));

import { useServerExecutionHistory } from "./useServerExecutionHistory";

// ── Tests ──────────────────────────────────────────────────────────────────

describe("useServerExecutionHistory — routing", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockReady = true;
    mockUsePaginatedQuery.mockReturnValue({
      results: [],
      status: "LoadingFirstPage",
      loadMore: vi.fn(),
      isLoading: true,
    });
  });

  describe("enabled: true + ready: true → active Convex subscription", () => {
    it("passes real args to usePaginatedQuery", () => {
      useServerExecutionHistory({ enabled: true });

      expect(mockUsePaginatedQuery).toHaveBeenCalledWith(
        mockFuncRef,
        mockArgs,
        { initialNumItems: 20 },
      );
    });

    it("forwards custom pageSize", () => {
      useServerExecutionHistory({ pageSize: 50, enabled: true });

      expect(mockUsePaginatedQuery).toHaveBeenCalledWith(
        mockFuncRef,
        mockArgs,
        { initialNumItems: 50 },
      );
    });

    it("transforms results through the transform function", () => {
      const rawDocs = [{ _id: "exec_1" }, { _id: "exec_2" }];
      mockUsePaginatedQuery.mockReturnValue({
        results: rawDocs,
        status: "Exhausted",
        loadMore: vi.fn(),
        isLoading: false,
      });

      const result = useServerExecutionHistory({ enabled: true });

      expect(result.items).toHaveLength(2);
      expect(mockTransform).toHaveBeenCalledTimes(2);
    });
  });

  describe("enabled: false → skipped (no Convex subscription)", () => {
    it("passes 'skip' to usePaginatedQuery", () => {
      useServerExecutionHistory({ enabled: false });

      expect(mockUsePaginatedQuery).toHaveBeenCalledWith(
        mockFuncRef,
        "skip",
        { initialNumItems: 20 },
      );
    });

    it("returns empty items", () => {
      const result = useServerExecutionHistory({ enabled: false });

      expect(result.items).toEqual([]);
    });

    it("does not call transform", () => {
      useServerExecutionHistory({ enabled: false });

      expect(mockTransform).not.toHaveBeenCalled();
    });
  });

  describe("ready: false → skipped (provider not mounted)", () => {
    beforeEach(() => {
      mockReady = false;
    });

    it("passes 'skip' even when enabled: true", () => {
      useServerExecutionHistory({ enabled: true });

      expect(mockUsePaginatedQuery).toHaveBeenCalledWith(
        mockFuncRef,
        "skip",
        { initialNumItems: 20 },
      );
    });

    it("returns isLoading: true", () => {
      mockUsePaginatedQuery.mockReturnValue({
        results: [],
        status: "LoadingFirstPage",
        loadMore: vi.fn(),
        isLoading: false,
      });

      const result = useServerExecutionHistory({ enabled: true });

      expect(result.isLoading).toBe(true);
    });
  });

  describe("defaults", () => {
    it("enabled defaults to true — creates subscription when ready", () => {
      useServerExecutionHistory();

      expect(mockUsePaginatedQuery).toHaveBeenCalledWith(
        mockFuncRef,
        mockArgs,
        { initialNumItems: 20 },
      );
    });

    it("pageSize defaults to 20", () => {
      useServerExecutionHistory();

      expect(mockUsePaginatedQuery).toHaveBeenCalledWith(
        mockFuncRef,
        mockArgs,
        { initialNumItems: 20 },
      );
    });
  });
});
