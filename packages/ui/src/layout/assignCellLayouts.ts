import type { CellLayout } from "./bentoGridContext";

/**
 * Sidebar-aware layout algorithm. The grid has two zones:
 *
 *  1. **Top zone** -- featured card (2x2) + sidebar column(s) to its right
 *  2. **Bottom zone** -- full-width rows below the featured card
 *
 * Items in the sidebar must be 1-col wide (they only have 1 column).
 * Items in the bottom zone are widened evenly to fill complete rows.
 */
export function assignCellLayouts(
  itemCount: number,
  cols: number,
): { layouts: CellLayout[]; rows: number } {
  if (itemCount === 0) return { layouts: [], rows: 0 };
  if (itemCount === 1) return singleItemLayout(cols);
  if (cols === 1) return stackedLayout(itemCount);
  return sidebarLayout(itemCount, cols);
}

function singleItemLayout(cols: number): { layouts: CellLayout[]; rows: number } {
  const c = Math.min(cols, 3) as 1 | 2 | 3;
  return { layouts: [{ colSpan: c, rowSpan: 2, featured: true }], rows: 2 };
}

function stackedLayout(itemCount: number): { layouts: CellLayout[]; rows: number } {
  return {
    rows: itemCount,
    layouts: Array.from({ length: itemCount }, (_, i) => ({
      colSpan: 1 as const,
      rowSpan: 1 as const,
      featured: i === 0,
    })),
  };
}

function buildSidebarItems(sidebarCount: number, sidebarSlots: number): CellLayout[] {
  if (sidebarCount === 1 && sidebarSlots >= 2) {
    return [{ colSpan: 1, rowSpan: 2, featured: false }];
  }
  return Array.from({ length: sidebarCount }, () => ({
    colSpan: 1 as const,
    rowSpan: 1 as const,
    featured: false,
  }));
}

function buildBottomItems(bottomCount: number, cols: number): CellLayout[] {
  const bottomRows = Math.ceil(bottomCount / cols);
  const totalCells = bottomRows * cols;
  const baseSpan = Math.floor(totalCells / bottomCount);
  const wider = totalCells % bottomCount;

  return Array.from({ length: bottomCount }, (_, i) => {
    const span = Math.min(i < wider ? baseSpan + 1 : baseSpan, 3) as 1 | 2 | 3;
    return { colSpan: span, rowSpan: 1 as const, featured: false };
  });
}

function sidebarLayout(itemCount: number, cols: number): { layouts: CellLayout[]; rows: number } {
  const featColSpan = Math.min(2, cols) as 1 | 2 | 3;
  const featRowSpan = 2;
  const sidebarWidth = cols - featColSpan;
  const sidebarSlots = sidebarWidth * featRowSpan;
  const rest = itemCount - 1;
  const sidebarCount = Math.min(rest, sidebarSlots);
  const bottomCount = rest - sidebarCount;

  const layouts: CellLayout[] = [
    { colSpan: featColSpan, rowSpan: featRowSpan, featured: true },
    ...buildSidebarItems(sidebarCount, sidebarSlots),
    ...buildBottomItems(bottomCount, cols),
  ];

  const rows = featRowSpan + (bottomCount > 0 ? Math.ceil(bottomCount / cols) : 0);
  return { layouts, rows };
}
