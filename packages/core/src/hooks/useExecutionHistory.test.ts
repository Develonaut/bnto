import { describe, it, expect, vi, beforeEach } from "vitest";

// ── Mocks ──────────────────────────────────────────────────────────────────
let mockIsAuthenticated = false;

vi.mock("./useIsAuthenticated", () => ({
  useIsAuthenticated: vi.fn(() => mockIsAuthenticated),
}));

const mockServerResult = {
  items: [{ id: "server-1", source: "server" }],
  isLoading: false,
  status: "Exhausted",
  loadMore: vi.fn(),
};

const mockLocalResult = {
  items: [{ id: "local-1", source: "local" }],
  isLoading: false,
  status: "Exhausted" as const,
  loadMore: vi.fn(),
};

vi.mock("./useServerExecutionHistory", () => ({
  useServerExecutionHistory: vi.fn(() => mockServerResult),
}));

vi.mock("./useLocalExecutionHistory", () => ({
  useLocalExecutionHistory: vi.fn(() => mockLocalResult),
}));

import { useExecutionHistory } from "./useExecutionHistory";
import { useServerExecutionHistory } from "./useServerExecutionHistory";
import { useLocalExecutionHistory } from "./useLocalExecutionHistory";

const mockServer = vi.mocked(useServerExecutionHistory);
const mockLocal = vi.mocked(useLocalExecutionHistory);

// ── Tests ──────────────────────────────────────────────────────────────────

describe("useExecutionHistory — routing", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockIsAuthenticated = false;
  });

  describe("authenticated → server path", () => {
    beforeEach(() => {
      mockIsAuthenticated = true;
    });

    it("enables server hook", () => {
      useExecutionHistory();

      expect(mockServer).toHaveBeenCalledWith(
        expect.objectContaining({ enabled: true }),
      );
    });

    it("disables local hook", () => {
      useExecutionHistory();

      expect(mockLocal).toHaveBeenCalledWith({ enabled: false });
    });

    it("returns server result", () => {
      const result = useExecutionHistory();

      expect(result).toBe(mockServerResult);
    });

    it("forwards pageSize to server hook", () => {
      useExecutionHistory({ pageSize: 50 });

      expect(mockServer).toHaveBeenCalledWith({
        pageSize: 50,
        enabled: true,
      });
    });
  });

  describe("unauthenticated → local path", () => {
    beforeEach(() => {
      mockIsAuthenticated = false;
    });

    it("disables server hook", () => {
      useExecutionHistory();

      expect(mockServer).toHaveBeenCalledWith(
        expect.objectContaining({ enabled: false }),
      );
    });

    it("enables local hook", () => {
      useExecutionHistory();

      expect(mockLocal).toHaveBeenCalledWith({ enabled: true });
    });

    it("returns local result", () => {
      const result = useExecutionHistory();

      expect(result).toBe(mockLocalResult);
    });

    it("does not forward pageSize to local hook", () => {
      useExecutionHistory({ pageSize: 50 });

      // Local hook only receives enabled — no pageSize (IndexedDB has no pagination).
      expect(mockLocal).toHaveBeenCalledWith({ enabled: true });
    });
  });
});
