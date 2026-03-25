import { describe, it, expect } from "vitest";
import { shouldAutoDownload } from "./runExecution";
import type { EditorStore } from "../store/types";

/** Minimal store shape for shouldAutoDownload tests. */
function mockStore(
  overrides: {
    outputNodeId?: string;
    outputParams?: Record<string, unknown>;
  } = {},
): EditorStore {
  const { outputNodeId = "out-1", outputParams = { autoDownload: true } } = overrides;
  return {
    nodes: [],
    configs: {
      [outputNodeId]: {
        nodeType: "output",
        name: "Output",
        parameters: outputParams,
      },
    },
  } as unknown as EditorStore;
}

describe("shouldAutoDownload", () => {
  it("returns true when autoDownload is true", () => {
    expect(shouldAutoDownload(mockStore())).toBe(true);
  });

  it("returns true when autoDownload is not set (defaults to true)", () => {
    expect(shouldAutoDownload(mockStore({ outputParams: {} }))).toBe(true);
  });

  it("returns false when autoDownload is false", () => {
    expect(shouldAutoDownload(mockStore({ outputParams: { autoDownload: false } }))).toBe(false);
  });

  it("returns true when no output node exists", () => {
    const store = { nodes: [], configs: {} } as unknown as EditorStore;
    expect(shouldAutoDownload(store)).toBe(true);
  });
});
