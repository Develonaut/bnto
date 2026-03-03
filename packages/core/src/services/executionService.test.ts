import { describe, it, expect, vi, beforeEach } from "vitest";

// Mock the adapter and client before imports
const mockFuncRef = { __type: "funcRef" };
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
  getExecutionHistoryRef: vi.fn(() => ({
    funcRef: mockFuncRef,
    args: {},
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

  describe("historyRefMethod", () => {
    it("returns funcRef, args, and transform", () => {
      const result = service.historyRefMethod();

      expect(result.funcRef).toBe(mockFuncRef);
      expect(result.args).toEqual({});
      expect(typeof result.transform).toBe("function");
    });

    it("transform converts raw execution doc to Execution type", () => {
      const { transform } = service.historyRefMethod();

      const rawDoc = {
        _id: "exec_123",
        userId: "user_456",
        recipeId: "wf_789",
        status: "completed" as const,
        progress: [{ nodeId: "n1", status: "completed" }],
        startedAt: 1000,
        completedAt: 2000,
      };

      const result = transform(rawDoc);

      expect(result.id).toBe("exec_123");
      expect(result.userId).toBe("user_456");
      expect(result.recipeId).toBe("wf_789");
      expect(result.status).toBe("completed");
      expect(result.startedAt).toBe(1000);
      expect(result.completedAt).toBe(2000);
    });

    it("transform handles nullable fields", () => {
      const { transform } = service.historyRefMethod();

      const rawDoc = {
        _id: "exec_456",
        userId: "user_789",
        status: "pending" as const,
        progress: [],
        startedAt: 3000,
        recipeId: null,
        error: null,
        result: null,
        outputFiles: null,
        sessionId: null,
        completedAt: null,
      };

      const result = transform(rawDoc);

      expect(result.recipeId).toBeUndefined();
      expect(result.error).toBeUndefined();
      expect(result.result).toBeUndefined();
      expect(result.outputFiles).toBeUndefined();
      expect(result.sessionId).toBeUndefined();
      expect(result.completedAt).toBeUndefined();
    });
  });
});
