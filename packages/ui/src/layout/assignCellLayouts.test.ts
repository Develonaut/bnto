import { describe, it, expect } from "vitest";

import { assignCellLayouts } from "./assignCellLayouts";

describe("assignCellLayouts", () => {
  it("returns empty layouts for 0 items", () => {
    const result = assignCellLayouts(0, 3);
    expect(result.layouts).toEqual([]);
    expect(result.rows).toBe(0);
  });

  it("returns a single full-width hero for 1 item", () => {
    const result = assignCellLayouts(1, 3);
    expect(result.layouts).toHaveLength(1);
    expect(result.layouts[0]).toEqual({
      colSpan: 3,
      rowSpan: 2,
      featured: true,
    });
    expect(result.rows).toBe(2);
  });

  it("caps single item colSpan at 3 for wider grids", () => {
    const result = assignCellLayouts(1, 5);
    expect(result.layouts[0].colSpan).toBe(3);
  });

  it("stacks items in a 1-col grid", () => {
    const result = assignCellLayouts(3, 1);
    expect(result.layouts).toHaveLength(3);
    expect(result.rows).toBe(3);
    expect(result.layouts[0].featured).toBe(true);
    expect(result.layouts[1].featured).toBe(false);
  });

  it("creates a featured 2x2 card with sidebar items in a 3-col grid", () => {
    const result = assignCellLayouts(3, 3);
    expect(result.layouts[0]).toEqual({
      colSpan: 2,
      rowSpan: 2,
      featured: true,
    });
    // 2 remaining items, 1 sidebar col × 2 rows = 2 slots
    expect(result.layouts).toHaveLength(3);
  });

  it("stretches a single sidebar item to fill 2 rows", () => {
    const result = assignCellLayouts(2, 3);
    expect(result.layouts[0]).toEqual({
      colSpan: 2,
      rowSpan: 2,
      featured: true,
    });
    // Only 1 sidebar item with 2 slots → stretched to 1×2
    expect(result.layouts[1]).toEqual({
      colSpan: 1,
      rowSpan: 2,
      featured: false,
    });
    expect(result.rows).toBe(2);
  });

  it("overflows items into bottom rows", () => {
    // 3-col grid, 6 items: 1 featured (2×2) + 2 sidebar + 3 bottom
    const result = assignCellLayouts(6, 3);
    expect(result.layouts).toHaveLength(6);
    expect(result.layouts[0].featured).toBe(true);
    expect(result.rows).toBe(3); // 2 featured rows + 1 bottom row
  });

  it("handles 2-col grid (no sidebar)", () => {
    const result = assignCellLayouts(3, 2);
    expect(result.layouts[0]).toEqual({
      colSpan: 2,
      rowSpan: 2,
      featured: true,
    });
    // No sidebar (cols - featColSpan = 0), 2 items go to bottom
    expect(result.rows).toBe(3); // 2 featured + 1 bottom
  });
});
