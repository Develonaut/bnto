import { describe, it, expect } from "vitest";

import { assignWithPinned } from "./assignWithPinned";

describe("assignWithPinned", () => {
  it("returns empty layouts and 0 rows when no flow items", () => {
    const result = assignWithPinned([{ colSpan: 2, rowSpan: 2, featured: true }], 0, 3);
    expect(result.flowLayouts).toEqual([]);
    expect(result.rows).toBe(2);
  });

  it("places flow items in the sidebar beside pinned cells", () => {
    // 3-col grid, pinned takes 2 cols × 2 rows → sidebar is 1 col × 2 rows = 2 slots
    const result = assignWithPinned([{ colSpan: 2, rowSpan: 2, featured: true }], 2, 3);
    expect(result.flowLayouts).toHaveLength(2);
    // 2 items in 2 sidebar slots → each 1×1
    expect(result.flowLayouts[0]).toEqual({
      colSpan: 1,
      rowSpan: 1,
      featured: false,
    });
    expect(result.flowLayouts[1]).toEqual({
      colSpan: 1,
      rowSpan: 1,
      featured: false,
    });
    expect(result.rows).toBe(2);
  });

  it("stretches a single sidebar item to fill 2 vertical slots", () => {
    const result = assignWithPinned([{ colSpan: 2, rowSpan: 2, featured: true }], 1, 3);
    expect(result.flowLayouts).toHaveLength(1);
    expect(result.flowLayouts[0]).toEqual({
      colSpan: 1,
      rowSpan: 2,
      featured: false,
    });
    expect(result.rows).toBe(2);
  });

  it("overflows into bottom rows when sidebar is full", () => {
    // 3-col grid, pinned 2×2 → sidebar has 2 slots, but we have 4 flow items
    const result = assignWithPinned([{ colSpan: 2, rowSpan: 2, featured: true }], 4, 3);
    // 2 sidebar + 2 bottom
    expect(result.flowLayouts).toHaveLength(4);
    // Bottom items fill a 3-col row → 2 items in 3 cells → one gets span 2
    expect(result.rows).toBe(3); // 2 pinned rows + 1 bottom row
  });

  it("handles pinned cells that fill the full width (no sidebar)", () => {
    // 2-col grid, pinned takes 2 cols → sidebar width is 0
    const result = assignWithPinned([{ colSpan: 2, rowSpan: 2, featured: true }], 2, 2);
    // All flow items go to bottom
    expect(result.flowLayouts).toHaveLength(2);
    expect(result.rows).toBe(3); // 2 pinned + 1 bottom
  });

  it("computes rows from pinned height when no flow items exist", () => {
    const result = assignWithPinned([{ colSpan: 1, rowSpan: 1, featured: false }], 0, 3);
    expect(result.rows).toBe(1);
  });
});
