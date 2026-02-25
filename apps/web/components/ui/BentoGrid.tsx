"use client";

import * as React from "react";

import { Grid } from "./Grid";
import type { ResponsiveGap } from "./layoutTypes";

/* ── Types ───────────────────────────────────────────────────── */

export interface CellLayout {
  colSpan: 1 | 2 | 3;
  rowSpan: 1 | 2;
  featured: boolean;
}

/* ── Context ─────────────────────────────────────────────────── */

const BentoItemContext = React.createContext<CellLayout | null>(null);

/** Access the current item's bento layout inside a `<BentoGrid>` child. */
export function useBentoItem(): CellLayout {
  const ctx = React.useContext(BentoItemContext);
  if (!ctx) throw new Error("useBentoItem must be used inside <BentoGrid>");
  return ctx;
}

/* ── Layout algorithm ────────────────────────────────────────── */

/**
 * Sidebar-aware layout algorithm. The grid has two zones:
 *
 *  1. **Top zone** — featured card (2×2) + sidebar column(s) to its right
 *  2. **Bottom zone** — full-width rows below the featured card
 *
 * Items in the sidebar must be 1-col wide (they only have 1 column).
 * Items in the bottom zone are widened evenly to fill complete rows.
 */
export function assignCellLayouts(
  itemCount: number,
  cols: number,
): { layouts: CellLayout[]; rows: number } {
  if (itemCount === 0) return { layouts: [], rows: 0 };

  // Single item → full-width hero
  if (itemCount === 1) {
    const c = Math.min(cols, 3) as 1 | 2 | 3;
    return { layouts: [{ colSpan: c, rowSpan: 2, featured: true }], rows: 2 };
  }

  // For 1-col grid, everything stacks
  if (cols === 1) {
    return {
      rows: itemCount,
      layouts: Array.from({ length: itemCount }, (_, i) => ({
        colSpan: 1 as const,
        rowSpan: 1 as const,
        featured: i === 0,
      })),
    };
  }

  // Featured card: span min(2, cols) columns × 2 rows
  const featColSpan = Math.min(2, cols) as 1 | 2 | 3;
  const featRowSpan = 2;

  // Sidebar: columns to the right of featured (0 for 2-col, 1 for 3-col)
  const sidebarWidth = cols - featColSpan;
  const sidebarSlots = sidebarWidth * featRowSpan;
  const rest = itemCount - 1;
  const sidebarCount = Math.min(rest, sidebarSlots);
  const bottomCount = rest - sidebarCount;

  const layouts: CellLayout[] = [
    { colSpan: featColSpan, rowSpan: featRowSpan, featured: true },
  ];

  // Sidebar items: 1-col wide. If only 1 item with 2 vertical
  // slots, stretch it tall (1×2) so the sidebar isn't half-empty.
  if (sidebarCount === 1 && sidebarSlots >= 2) {
    layouts.push({ colSpan: 1, rowSpan: 2, featured: false });
  } else {
    for (let i = 0; i < sidebarCount; i++) {
      layouts.push({ colSpan: 1, rowSpan: 1, featured: false });
    }
  }

  // Bottom items: fill complete rows with even span distribution
  if (bottomCount > 0) {
    const bottomRows = Math.ceil(bottomCount / cols);
    const totalCells = bottomRows * cols;
    const baseSpan = Math.floor(totalCells / bottomCount);
    const wider = totalCells % bottomCount;

    for (let i = 0; i < bottomCount; i++) {
      const span = Math.min(i < wider ? baseSpan + 1 : baseSpan, 3) as 1 | 2 | 3;
      layouts.push({ colSpan: span, rowSpan: 1, featured: false });
    }
  }

  const rows = featRowSpan + (bottomCount > 0 ? Math.ceil(bottomCount / cols) : 0);
  return { layouts, rows };
}

/* ── Component ───────────────────────────────────────────────── */

type BentoGridProps = {
  /** Number of columns (auto-mapped to responsive breakpoints). Default `3`. */
  cols?: 1 | 2 | 3;
  /** Gap between cells. Default `"md"`. */
  gap?: ResponsiveGap;
  /** Minimum row height. Default `"10rem"`. */
  minRowHeight?: string;
  /** Additional className for the grid container. */
  className?: string;
  children: React.ReactNode;
};

function BentoGridRoot({
  cols = 3,
  gap = "md",
  minRowHeight = "10rem",
  className,
  children,
}: BentoGridProps) {
  const childArray = React.Children.toArray(children);
  const { layouts, rows } = assignCellLayouts(childArray.length, cols);

  const colsResponsive =
    cols === 1 ? (1 as const)
    : cols === 2 ? { mobile: 1 as const, tablet: 2 as const }
    : { mobile: 1 as const, tablet: 2 as const, desktop: 3 as const };

  return (
    <Grid
      cols={colsResponsive}
      gap={gap}
      flow="dense"
      rows={`repeat(${rows}, minmax(${minRowHeight}, auto))`}
      className={className}
    >
      {childArray.map((child, i) => (
        <BentoItemContext.Provider key={React.isValidElement(child) ? child.key : i} value={layouts[i]}>
          <Grid.Item colSpan={layouts[i].colSpan} rowSpan={layouts[i].rowSpan} className="min-h-0">
            {child}
          </Grid.Item>
        </BentoItemContext.Provider>
      ))}
    </Grid>
  );
}

/* ── Namespace ───────────────────────────────────────────────── */

export const BentoGrid = Object.assign(BentoGridRoot, {
  Root: BentoGridRoot,
});

export type { BentoGridProps };
