import { describe, it, expect } from "vitest";
import { localEntryToExecution } from "./localHistory";
import type { LocalHistoryEntry } from "../types/localHistory";

describe("localEntryToExecution", () => {
  const entry: LocalHistoryEntry = {
    id: "abc-123",
    slug: "compress-images",
    status: "completed",
    timestamp: 1709300000000,
    durationMs: 1500,
    inputFileCount: 3,
    outputFileCount: 3,
  };

  it("maps id directly", () => {
    expect(localEntryToExecution(entry).id).toBe("abc-123");
  });

  it('sets userId to "local"', () => {
    expect(localEntryToExecution(entry).userId).toBe("local");
  });

  it("maps slug field", () => {
    expect(localEntryToExecution(entry).slug).toBe("compress-images");
  });

  it("maps status field", () => {
    expect(localEntryToExecution(entry).status).toBe("completed");
  });

  it('sets source to "local"', () => {
    expect(localEntryToExecution(entry).source).toBe("local");
  });

  it("computes completedAt from timestamp + durationMs", () => {
    const result = localEntryToExecution(entry);
    expect(result.startedAt).toBe(1709300000000);
    expect(result.completedAt).toBe(1709300000000 + 1500);
  });

  it("includes duration in result", () => {
    expect(localEntryToExecution(entry).result?.duration).toBe(1500);
  });

  it("maps error for failed entries", () => {
    const failed: LocalHistoryEntry = {
      ...entry,
      status: "failed",
      error: "WASM processing failed",
    };
    const result = localEntryToExecution(failed);
    expect(result.status).toBe("failed");
    expect(result.error).toBe("WASM processing failed");
    expect(result.result?.error).toBe("WASM processing failed");
  });

  it("leaves workflowId undefined", () => {
    expect(localEntryToExecution(entry).workflowId).toBeUndefined();
  });

  it("returns empty progress array", () => {
    expect(localEntryToExecution(entry).progress).toEqual([]);
  });
});
