import { describe, it, expect } from "vitest";
import { createElement } from "react";

import { partitionBentoChildren } from "./partitionBentoChildren";

// Create a mock Pinned component with the expected displayName
function MockPinned(props: {
  colSpan: 1 | 2 | 3;
  rowSpan: 1 | 2;
  featured?: boolean;
  children?: React.ReactNode;
}) {
  return createElement("div", null, props.children);
}
MockPinned.displayName = "BentoGrid.Pinned";

describe("partitionBentoChildren", () => {
  it("assigns uniform 1x1 layouts when uniform is true", () => {
    const children = [
      createElement("div", { key: "a" }, "A"),
      createElement("div", { key: "b" }, "B"),
      createElement("div", { key: "c" }, "C"),
    ];

    const result = partitionBentoChildren(children, 3, true);

    expect(result.entries).toHaveLength(3);
    for (const entry of result.entries) {
      expect(entry.layout).toEqual({ colSpan: 1, rowSpan: 1, featured: false });
    }
    expect(result.rows).toBe(1);
  });

  it("computes auto layouts when uniform is false and no pinned children", () => {
    const children = [
      createElement("div", { key: "a" }, "A"),
      createElement("div", { key: "b" }, "B"),
      createElement("div", { key: "c" }, "C"),
    ];

    const result = partitionBentoChildren(children, 3, false);

    expect(result.entries).toHaveLength(3);
    // First item should be featured in auto mode
    expect(result.entries[0].layout.featured).toBe(true);
  });

  it("separates pinned and flow children", () => {
    const children = [
      createElement(MockPinned, { key: "p", colSpan: 2, rowSpan: 2 }, "Pinned"),
      createElement("div", { key: "a" }, "A"),
      createElement("div", { key: "b" }, "B"),
    ];

    const result = partitionBentoChildren(children, 3, false);

    expect(result.entries).toHaveLength(3);
    // First entry should be the pinned one
    expect(result.entries[0].layout).toEqual({
      colSpan: 2,
      rowSpan: 2,
      featured: true,
    });
  });

  it("handles empty children", () => {
    const result = partitionBentoChildren([], 3, false);

    expect(result.entries).toHaveLength(0);
    expect(result.rows).toBe(0);
  });

  it("defaults featured to true for pinned children without explicit prop", () => {
    const children = [createElement(MockPinned, { key: "p", colSpan: 1, rowSpan: 1 }, "Pinned")];

    const result = partitionBentoChildren(children, 3, false);

    // Pinned defaults featured to true when not specified
    expect(result.entries[0].layout.featured).toBe(true);
  });
});
