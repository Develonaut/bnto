import { describe, it, expect } from "vitest";
import { filterPlaceholderChanges } from "./filterPlaceholderChanges";
import { PLACEHOLDER_ID } from "./injectPlaceholder";
import type { NodeChange } from "@xyflow/react";
import type { BentoNode } from "../adapters/types";

describe("filterPlaceholderChanges", () => {
  it("removes changes targeting the placeholder node", () => {
    const changes: NodeChange<BentoNode>[] = [
      {
        type: "dimensions",
        id: PLACEHOLDER_ID,
        dimensions: { width: 200, height: 200 },
        resizing: false,
      },
      {
        type: "dimensions",
        id: "real-node",
        dimensions: { width: 200, height: 200 },
        resizing: false,
      },
    ];
    const result = filterPlaceholderChanges(changes);
    expect(result).toHaveLength(1);
    expect((result[0] as { id: string }).id).toBe("real-node");
  });

  it("passes through all changes when no placeholder changes", () => {
    const changes: NodeChange<BentoNode>[] = [
      { type: "dimensions", id: "a", dimensions: { width: 100, height: 100 }, resizing: false },
      { type: "dimensions", id: "b", dimensions: { width: 100, height: 100 }, resizing: false },
    ];
    expect(filterPlaceholderChanges(changes)).toHaveLength(2);
  });

  it("returns empty array when all changes are placeholder", () => {
    const changes: NodeChange<BentoNode>[] = [
      {
        type: "dimensions",
        id: PLACEHOLDER_ID,
        dimensions: { width: 200, height: 200 },
        resizing: false,
      },
    ];
    expect(filterPlaceholderChanges(changes)).toHaveLength(0);
  });
});
