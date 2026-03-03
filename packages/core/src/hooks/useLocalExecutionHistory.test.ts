import { describe, it, expect, vi, beforeEach } from "vitest";

// ── Mocks ──────────────────────────────────────────────────────────────────
let mockReady = true;
const mockUseQuery = vi.fn();

vi.mock("@tanstack/react-query", () => ({
  useQuery: (...args: unknown[]) => mockUseQuery(...args),
}));

vi.mock("./useReady", () => ({
  useReady: vi.fn(() => mockReady),
}));

vi.mock("../core", () => ({
  core: {
    executions: {
      historyQueryOptions: vi.fn(() => ({
        queryKey: ["local-history", "executions"],
        queryFn: vi.fn(),
        staleTime: Infinity,
      })),
    },
  },
}));

vi.mock("../transforms/localHistory", () => ({
  localEntryToExecution: vi.fn((entry: Record<string, unknown>) => ({
    id: entry.id,
    source: "local",
  })),
}));

import { useLocalExecutionHistory } from "./useLocalExecutionHistory";

// ── Tests ──────────────────────────────────────────────────────────────────

describe("useLocalExecutionHistory — routing", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockReady = true;
    mockUseQuery.mockReturnValue({
      data: undefined,
      isLoading: true,
    });
  });

  describe("enabled: true + ready: true → active IndexedDB query", () => {
    it("passes enabled: true to useQuery", () => {
      useLocalExecutionHistory({ enabled: true });

      expect(mockUseQuery).toHaveBeenCalledWith(
        expect.objectContaining({ enabled: true }),
      );
    });

    it("includes select transform in query options", () => {
      useLocalExecutionHistory({ enabled: true });

      const queryOpts = mockUseQuery.mock.calls[0][0];
      expect(typeof queryOpts.select).toBe("function");
    });

    it("includes query key for cache identity", () => {
      useLocalExecutionHistory({ enabled: true });

      const queryOpts = mockUseQuery.mock.calls[0][0];
      expect(queryOpts.queryKey).toEqual(["local-history", "executions"]);
    });
  });

  describe("enabled: false → skipped (no IndexedDB reads)", () => {
    it("passes enabled: false to useQuery", () => {
      useLocalExecutionHistory({ enabled: false });

      expect(mockUseQuery).toHaveBeenCalledWith(
        expect.objectContaining({ enabled: false }),
      );
    });

    it("returns empty items when disabled", () => {
      const result = useLocalExecutionHistory({ enabled: false });

      expect(result.items).toEqual([]);
    });
  });

  describe("ready: false → skipped (provider not mounted)", () => {
    beforeEach(() => {
      mockReady = false;
    });

    it("passes enabled: false even when enabled: true", () => {
      useLocalExecutionHistory({ enabled: true });

      expect(mockUseQuery).toHaveBeenCalledWith(
        expect.objectContaining({ enabled: false }),
      );
    });

    it("returns isLoading: true", () => {
      mockUseQuery.mockReturnValue({
        data: undefined,
        isLoading: false,
      });

      const result = useLocalExecutionHistory({ enabled: true });

      expect(result.isLoading).toBe(true);
    });
  });

  describe("return shape contract", () => {
    it("status is always 'Exhausted' — no pagination for IndexedDB", () => {
      const result = useLocalExecutionHistory();

      expect(result.status).toBe("Exhausted");
    });

    it("loadMore is a no-op function", () => {
      const result = useLocalExecutionHistory();

      expect(typeof result.loadMore).toBe("function");
      expect(() => result.loadMore(10)).not.toThrow();
    });

    it("returns data from useQuery when available", () => {
      const mockEntries = [
        { id: "entry-1", source: "local" },
        { id: "entry-2", source: "local" },
      ];
      mockUseQuery.mockReturnValue({
        data: mockEntries,
        isLoading: false,
      });

      const result = useLocalExecutionHistory({ enabled: true });

      expect(result.items).toBe(mockEntries);
      expect(result.isLoading).toBe(false);
    });
  });

  describe("defaults", () => {
    it("enabled defaults to true — queries IndexedDB when ready", () => {
      useLocalExecutionHistory();

      expect(mockUseQuery).toHaveBeenCalledWith(
        expect.objectContaining({ enabled: true }),
      );
    });
  });
});
