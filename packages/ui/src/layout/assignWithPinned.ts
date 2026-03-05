import type { CellLayout } from "./bentoGridContext";

/**
 * Compute flow layouts for children when some cells are pinned.
 * Fills sidebar slots first (beside pinned cells), then bottom rows.
 */
export function assignWithPinned(
  pinnedLayouts: CellLayout[],
  flowCount: number,
  cols: number,
): { flowLayouts: CellLayout[]; rows: number } {
  if (flowCount === 0) {
    const pinnedRows = pinnedLayouts.reduce((max, l) => Math.max(max, l.rowSpan), 0);
    return { flowLayouts: [], rows: pinnedRows };
  }

  const pinnedMaxRow = pinnedLayouts.reduce((max, l) => Math.max(max, l.rowSpan), 0);
  const pinnedCols = pinnedLayouts.reduce((sum, l) => sum + l.colSpan, 0);

  const sidebarWidth = Math.max(0, cols - pinnedCols);
  const sidebarSlots = sidebarWidth * pinnedMaxRow;
  const sidebarCount = Math.min(flowCount, sidebarSlots);
  const bottomCount = flowCount - sidebarCount;

  const flowLayouts = buildSidebarLayouts(sidebarCount, sidebarSlots);
  appendBottomLayouts(flowLayouts, bottomCount, cols);

  const rows = pinnedMaxRow + (bottomCount > 0 ? Math.ceil(bottomCount / cols) : 0);
  return { flowLayouts, rows };
}

function buildSidebarLayouts(count: number, slots: number): CellLayout[] {
  const layouts: CellLayout[] = [];
  if (count === 1 && slots >= 2) {
    layouts.push({ colSpan: 1, rowSpan: 2, featured: false });
  } else {
    for (let i = 0; i < count; i++) {
      layouts.push({ colSpan: 1, rowSpan: 1, featured: false });
    }
  }
  return layouts;
}

function appendBottomLayouts(layouts: CellLayout[], count: number, cols: number) {
  if (count <= 0) return;
  const bottomRows = Math.ceil(count / cols);
  const totalCells = bottomRows * cols;
  const baseSpan = Math.floor(totalCells / count);
  const wider = totalCells % count;

  for (let i = 0; i < count; i++) {
    const span = Math.min(i < wider ? baseSpan + 1 : baseSpan, 3) as 1 | 2 | 3;
    layouts.push({ colSpan: span, rowSpan: 1, featured: false });
  }
}
