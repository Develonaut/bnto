import { describe, it, expect, vi, beforeEach } from "vitest";

// Mock the adapter and client before imports
vi.mock("../adapters/convex/executionAdapter", () => ({
  getExecutionQuery: vi.fn((id: string) => ({
    queryKey: ["executions", "get", id],
    queryFn: vi.fn(),
  })),
  getExecutionsQuery: vi.fn((recipeId: string) => ({
    queryKey: ["executions", "listByRecipe", recipeId],
    queryFn: vi.fn(),
  })),
  getExecutionLogsQuery: vi.fn((executionId: string) => ({
    queryKey: ["executionLogs", "list", executionId],
    queryFn: vi.fn(),
  })),
  startExecution: vi.fn(),
}));

vi.mock("../client", () => ({
  getQueryClient: vi.fn(() => ({
    invalidateQueries: vi.fn(),
  })),
}));

import { createExecutionService } from "./executionService";
import { startExecution } from "../adapters/convex/executionAdapter";

const mockStartExecution = vi.mocked(startExecution);

describe("createExecutionService", () => {
  let service: ReturnType<typeof createExecutionService>;

  beforeEach(() => {
    vi.clearAllMocks();
    service = createExecutionService();
  });

  describe("start", () => {
    it("passes recipeId to the adapter", async () => {
      mockStartExecution.mockResolvedValueOnce("exec_new" as never);

      await service.start({ recipeId: "wf_123" });

      expect(mockStartExecution).toHaveBeenCalledWith({
        recipeId: "wf_123",
      });
    });

    it("passes slug and sessionId to adapter when provided", async () => {
      mockStartExecution.mockResolvedValueOnce("exec_new" as never);

      await service.start({
        recipeId: "wf_456",
        slug: "compress-images",
        sessionId: "session-abc",
      });

      expect(mockStartExecution).toHaveBeenCalledWith({
        recipeId: "wf_456",
        slug: "compress-images",
        sessionId: "session-abc",
      });
    });

    it("returns the execution ID from the adapter", async () => {
      mockStartExecution.mockResolvedValueOnce("exec_789" as never);

      const result = await service.start({ recipeId: "wf_789" });

      expect(result).toBe("exec_789");
    });
  });
});
