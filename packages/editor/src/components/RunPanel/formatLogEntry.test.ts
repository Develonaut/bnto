import { describe, it, expect } from "vitest";
import { formatLogEntry } from "./formatLogEntry";
import type { RunLogEntry } from "./types";
import type { PipelineEvent } from "@bnto/core";

const ts = new Date("2026-03-08T14:30:05.123Z").getTime();

function entry(event: PipelineEvent): RunLogEntry {
  return { timestamp: ts, event };
}

describe("formatLogEntry", () => {
  it("formats PipelineStarted", () => {
    const result = formatLogEntry(entry({ type: "PipelineStarted", totalNodes: 3, totalFiles: 5 }));
    expect(result.icon).toBe(">>");
    expect(result.message).toContain("3 node(s)");
    expect(result.message).toContain("5 file(s)");
  });

  it("formats NodeStarted", () => {
    const result = formatLogEntry(
      entry({
        type: "NodeStarted",
        nodeId: "n1",
        nodeType: "compress-image",
        nodeIndex: 0,
        totalNodes: 2,
      }),
    );
    expect(result.icon).toBe("->");
    expect(result.message).toContain("[compress-image]");
    expect(result.message).toContain("1/2");
  });

  it("formats FileProgress", () => {
    const result = formatLogEntry(
      entry({
        type: "FileProgress",
        nodeId: "n1",
        fileIndex: 2,
        totalFiles: 4,
        percent: 75,
        message: "compressing",
      }),
    );
    expect(result.icon).toBe("  ");
    expect(result.message).toContain("3/4");
    expect(result.message).toContain("75%");
    expect(result.message).toContain("compressing");
  });

  it("formats NodeCompleted", () => {
    const result = formatLogEntry(
      entry({
        type: "NodeCompleted",
        nodeId: "n1",
        filesProcessed: 3,
        durationMs: 1200,
      }),
    );
    expect(result.icon).toBe("ok");
    expect(result.message).toContain("3 file(s)");
    expect(result.message).toContain("1200ms");
  });

  it("formats NodeFailed", () => {
    const result = formatLogEntry(
      entry({ type: "NodeFailed", nodeId: "n1", error: "out of memory" }),
    );
    expect(result.icon).toBe("!!");
    expect(result.message).toContain("out of memory");
  });

  it("formats PipelineCompleted", () => {
    const result = formatLogEntry(
      entry({ type: "PipelineCompleted", totalFilesProcessed: 10, durationMs: 5000 }),
    );
    expect(result.icon).toBe("<<");
    expect(result.message).toContain("10 file(s)");
    expect(result.message).toContain("5000ms");
  });

  it("formats PipelineFailed", () => {
    const result = formatLogEntry(
      entry({ type: "PipelineFailed", nodeId: "n1", error: "timeout" }),
    );
    expect(result.icon).toBe("!!");
    expect(result.message).toContain("timeout");
  });

  it("includes formatted time", () => {
    const result = formatLogEntry(entry({ type: "PipelineFailed", nodeId: "n1", error: "x" }));
    // Time should be HH:MM:SS.mmm format
    expect(result.time).toMatch(/^\d{2}:\d{2}:\d{2}\.\d{3}$/);
  });
});
