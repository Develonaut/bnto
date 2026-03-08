import { describe, it, expect } from "vitest";
import {
  applyPipelineEvent,
  buildPendingState,
  buildFinalState,
  buildFailedState,
} from "./executionState";
import type { PipelineDefinition } from "@bnto/core";
import type { ExecutionState } from "../store/types";

const mockDefinition: PipelineDefinition = {
  nodes: [
    { id: "input-1", type: "input", params: {} },
    { id: "compress-1", type: "compress-image", params: { quality: 80 } },
    { id: "resize-1", type: "resize-image", params: { width: 800 } },
    { id: "output-1", type: "output", params: {} },
  ],
};

describe("applyPipelineEvent", () => {
  it("sets node to active on NodeStarted", () => {
    const current: ExecutionState = { "compress-1": "pending" };
    const next = applyPipelineEvent(current, {
      type: "NodeStarted",
      nodeId: "compress-1",
      nodeType: "compress-image",
      nodeIndex: 0,
      totalNodes: 2,
    });
    expect(next["compress-1"]).toBe("active");
  });

  it("sets node to completed on NodeCompleted", () => {
    const current: ExecutionState = { "compress-1": "active" };
    const next = applyPipelineEvent(current, {
      type: "NodeCompleted",
      nodeId: "compress-1",
      filesProcessed: 3,
      durationMs: 500,
    });
    expect(next["compress-1"]).toBe("completed");
  });

  it("sets node to failed on NodeFailed", () => {
    const current: ExecutionState = { "compress-1": "active" };
    const next = applyPipelineEvent(current, {
      type: "NodeFailed",
      nodeId: "compress-1",
      error: "out of memory",
    });
    expect(next["compress-1"]).toBe("failed");
  });

  it("returns same state for informational events", () => {
    const current: ExecutionState = { "compress-1": "active" };
    const next = applyPipelineEvent(current, {
      type: "FileProgress",
      nodeId: "compress-1",
      fileIndex: 0,
      totalFiles: 1,
      percent: 50,
      message: "compressing",
    });
    expect(next).toBe(current);
  });

  it("does not mutate the original state", () => {
    const current: ExecutionState = { "compress-1": "pending" };
    applyPipelineEvent(current, {
      type: "NodeStarted",
      nodeId: "compress-1",
      nodeType: "compress-image",
      nodeIndex: 0,
      totalNodes: 1,
    });
    expect(current["compress-1"]).toBe("pending");
  });
});

describe("buildPendingState", () => {
  it("marks processing nodes as pending, I/O as idle", () => {
    const state = buildPendingState(mockDefinition, {});
    expect(state["compress-1"]).toBe("pending");
    expect(state["resize-1"]).toBe("pending");
    expect(state["input-1"]).toBeUndefined();
    expect(state["output-1"]).toBeUndefined();
  });

  it("preserves base state for non-processing nodes", () => {
    const base: ExecutionState = { "input-1": "idle" };
    const state = buildPendingState(mockDefinition, base);
    expect(state["input-1"]).toBe("idle");
    expect(state["compress-1"]).toBe("pending");
  });
});

describe("buildFinalState", () => {
  it("marks input as idle, everything else as completed", () => {
    const state = buildFinalState(mockDefinition);
    expect(state["input-1"]).toBe("idle");
    expect(state["compress-1"]).toBe("completed");
    expect(state["resize-1"]).toBe("completed");
    expect(state["output-1"]).toBe("completed");
  });
});

describe("buildFailedState", () => {
  it("marks active nodes as failed", () => {
    const state = buildFailedState({ "compress-1": "active", "resize-1": "pending" });
    expect(state["compress-1"]).toBe("failed");
  });

  it("marks pending nodes as idle", () => {
    const state = buildFailedState({ "compress-1": "active", "resize-1": "pending" });
    expect(state["resize-1"]).toBe("idle");
  });

  it("preserves completed and failed nodes", () => {
    const state = buildFailedState({
      "compress-1": "completed",
      "resize-1": "failed",
      "output-1": "active",
    });
    expect(state["compress-1"]).toBe("completed");
    expect(state["resize-1"]).toBe("failed");
    expect(state["output-1"]).toBe("failed");
  });

  it("does not mutate the original state", () => {
    const current: ExecutionState = { "compress-1": "active" };
    buildFailedState(current);
    expect(current["compress-1"]).toBe("active");
  });
});
