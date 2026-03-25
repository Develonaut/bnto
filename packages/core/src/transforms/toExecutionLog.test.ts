import { describe, it, expect } from "vitest";
import { toExecutionLog } from "./toExecutionLog";

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
