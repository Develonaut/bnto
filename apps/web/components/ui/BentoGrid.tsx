"use client";

import { Children, isValidElement } from "react";
import type { ReactNode, ReactElement, Key } from "react";

import { cn } from "@/lib/cn";

import { assignCellLayouts } from "./assignCellLayouts";
import { BentoItemContext } from "./bentoGridContext";
import type { CellLayout } from "./bentoGridContext";
import { Grid } from "./Grid";
import type { GapSize } from "./layoutTypes";

/* ── Re-exports ──────────────────────────────────────────────── */

export { assignCellLayouts } from "./assignCellLayouts";
export { useBentoItem } from "./useBentoItem";
export type { CellLayout } from "./bentoGridContext";

/* ── Pinned marker ──────────────────────────────────────────── */

interface PinnedProps {
  /** Fixed column span. */
  colSpan: 1 | 2 | 3;
  /** Fixed row span. */
  rowSpan: 1 | 2;
  /** Treat as the featured cell (exposed via `useBentoItem`). Default `true`. */
  featured?: boolean;
  children: ReactNode;
}

function BentoPinned({ children }: PinnedProps) {
  return <>{children}</>;
}
BentoPinned.displayName = "BentoGrid.Pinned";

function isPinned(child: ReactNode): child is ReactElement<PinnedProps> {
  return isValidElement(child) && (child.type as { displayName?: string }).displayName === "BentoGrid.Pinned";
}

/* ── Pinned layout helper ────────────────────────────────────── */

function assignWithPinned(
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

  const flowLayouts: CellLayout[] = [];

  if (sidebarCount === 1 && sidebarSlots >= 2) {
    flowLayouts.push({ colSpan: 1, rowSpan: 2, featured: false });
  } else {
    for (let i = 0; i < sidebarCount; i++) {
      flowLayouts.push({ colSpan: 1, rowSpan: 1, featured: false });
    }
  }

  if (bottomCount > 0) {
    const bottomRows = Math.ceil(bottomCount / cols);
    const totalCells = bottomRows * cols;
    const baseSpan = Math.floor(totalCells / bottomCount);
    const wider = totalCells % bottomCount;

    for (let i = 0; i < bottomCount; i++) {
      const span = Math.min(i < wider ? baseSpan + 1 : baseSpan, 3) as 1 | 2 | 3;
      flowLayouts.push({ colSpan: span, rowSpan: 1, featured: false });
    }
  }

  const rows = pinnedMaxRow + (bottomCount > 0 ? Math.ceil(bottomCount / cols) : 0);
  return { flowLayouts, rows };
}

/* ── Component ───────────────────────────────────────────────── */

type BentoGridProps = {
  /** Number of columns (auto-mapped to responsive breakpoints). Default `3`. */
  cols?: 1 | 2 | 3;
  /** Gap between cells. Default `"md"`. */
  gap?: GapSize;
  /** Minimum row height. Default `"10rem"`. */
  minRowHeight?: string;
  /** Skip featured layout — all cells are equal 1×1. Default `false`. */
  uniform?: boolean;
  /** Additional className for the grid container. */
  className?: string;
  children: ReactNode;
};

function BentoGridRoot({
  cols = 3,
  gap = "md",
  minRowHeight = "10rem",
  uniform = false,
  className,
  children,
}: BentoGridProps) {
  const childArray = Children.toArray(children);

  const pinned: { layout: CellLayout; element: ReactNode; key: Key }[] = [];
  const flow: { element: ReactNode; key: Key }[] = [];

  childArray.forEach((child, i) => {
    const key = isValidElement(child) ? child.key ?? i : i;
    if (isPinned(child)) {
      pinned.push({
        layout: {
          colSpan: child.props.colSpan,
          rowSpan: child.props.rowSpan,
          featured: child.props.featured ?? true,
        },
        element: child.props.children,
        key,
      });
    } else {
      flow.push({ element: child, key });
    }
  });

  let allEntries: { layout: CellLayout; element: ReactNode; key: Key }[];
  let rows: number;

  if (pinned.length > 0) {
    const { flowLayouts, rows: r } = assignWithPinned(
      pinned.map((p) => p.layout),
      flow.length,
      cols,
    );
    rows = r;
    allEntries = [
      ...pinned,
      ...flow.map((f, i) => ({ ...f, layout: flowLayouts[i] })),
    ];
  } else if (uniform) {
    const uniformLayout: CellLayout = { colSpan: 1, rowSpan: 1, featured: false };
    rows = Math.ceil(childArray.length / cols);
    allEntries = childArray.map((child, i) => ({
      layout: uniformLayout,
      element: child,
      key: isValidElement(child) ? child.key ?? i : i,
    }));
  } else {
    const result = assignCellLayouts(childArray.length, cols);
    rows = result.rows;
    allEntries = childArray.map((child, i) => ({
      layout: result.layouts[i],
      element: child,
      key: isValidElement(child) ? child.key ?? i : i,
    }));
  }

  return (
    <Grid
      cols={1}
      gap={gap}
      flow="dense"
      className={cn(
        cols >= 2 && "md:grid-cols-2",
        cols >= 3 && "lg:grid-cols-3",
        className,
      )}
      style={{ gridTemplateRows: `repeat(${rows}, minmax(${minRowHeight}, auto))` }}
    >
      {allEntries.map((entry) => (
        <BentoItemContext.Provider key={entry.key} value={entry.layout}>
          <Grid.Item colSpan={entry.layout.colSpan} rowSpan={entry.layout.rowSpan} className="min-h-0">
            {entry.element}
          </Grid.Item>
        </BentoItemContext.Provider>
      ))}
    </Grid>
  );
}

/* ── Namespace ───────────────────────────────────────────────── */

export const BentoGrid = Object.assign(BentoGridRoot, {
  Root: BentoGridRoot,
  Pinned: BentoPinned,
});

export type { BentoGridProps };
