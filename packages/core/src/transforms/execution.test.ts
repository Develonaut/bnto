import { describe, it, expect } from "vitest";
import { toExecution, toExecutionLog } from "./execution";

describe("toExecution", () => {
  const baseDoc = {
    _id: "exec_123" as never,
    _creationTime: 1000,
    userId: "user_456" as never,
    workflowId: "wf_789" as never,
    status: "running" as const,
    progress: [{ nodeId: "node-1", status: "completed" }],
    startedAt: 1000,
  };

  it("maps core fields from Convex doc to Execution type", () => {
    const result = toExecution(baseDoc);

    expect(result).toEqual({
      id: "exec_123",
      userId: "user_456",
      workflowId: "wf_789",
      status: "running",
      progress: [{ nodeId: "node-1", status: "completed" }],
      result: undefined,
      error: undefined,
      outputFiles: undefined,
      sessionId: undefined,
      startedAt: 1000,
      completedAt: undefined,
    });
  });

  it("maps outputFiles when present", () => {
    const doc = {
      ...baseDoc,
      outputFiles: [
        {
          key: "executions/exec_123/output/result.zip",
          name: "result.zip",
          sizeBytes: 2048,
          contentType: "application/zip",
        },
      ],
    };

    const result = toExecution(doc);

    expect(result.outputFiles).toEqual([
      {
        key: "executions/exec_123/output/result.zip",
        name: "result.zip",
        sizeBytes: 2048,
        contentType: "application/zip",
      },
    ]);
  });

  it("maps sessionId when present", () => {
    const doc = { ...baseDoc, sessionId: "session-abc" };
    const result = toExecution(doc);
    expect(result.sessionId).toBe("session-abc");
  });

  it("maps completed execution with result", () => {
    const doc = {
      ...baseDoc,
      status: "completed" as const,
      result: { status: "success", nodesExecuted: 3, nodeOutputs: {}, duration: 1200 },
      completedAt: 2200,
    };

    const result = toExecution(doc);

    expect(result.status).toBe("completed");
    expect(result.result).toEqual({
      status: "success",
      nodesExecuted: 3,
      nodeOutputs: {},
      duration: 1200,
    });
    expect(result.completedAt).toBe(2200);
  });

  it("maps failed execution with error", () => {
    const doc = {
      ...baseDoc,
      status: "failed" as const,
      error: "Go API returned 500",
      completedAt: 1500,
    };

    const result = toExecution(doc);

    expect(result.status).toBe("failed");
    expect(result.error).toBe("Go API returned 500");
  });
});

describe("toExecutionLog", () => {
  it("maps core fields from Convex doc to ExecutionLog type", () => {
    const doc = {
      _id: "log_001" as never,
      _creationTime: 1000,
      executionId: "exec_123" as never,
      nodeId: "image-resize-1",
      level: "info" as const,
      message: "Resized image to 800x600",
      timestamp: 1500,
    };

    const result = toExecutionLog(doc);

    expect(result).toEqual({
      id: "log_001",
      executionId: "exec_123",
      nodeId: "image-resize-1",
      level: "info",
      message: "Resized image to 800x600",
      timestamp: 1500,
    });
  });
});
