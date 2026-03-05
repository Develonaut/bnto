"use client";

import type { ReactNode } from "react";

import { cn } from "../utils/cn";

import { BentoItemContext } from "./bentoGridContext";
import { Grid } from "./Grid";
import { partitionBentoChildren } from "./partitionBentoChildren";
import type { GapSize } from "../utils/layoutTypes";

/* ── Re-exports ──────────────────────────────────────────────── */

export { assignCellLayouts } from "./assignCellLayouts";
export { useBentoItem } from "./useBentoItem";
export type { CellLayout } from "./bentoGridContext";

/* ── Pinned marker ──────────────────────────────────────────── */

interface PinnedProps {
  colSpan: 1 | 2 | 3;
  rowSpan: 1 | 2;
  featured?: boolean;
  children: ReactNode;
}

function BentoPinned({ children }: PinnedProps) {
  return <>{children}</>;
}
BentoPinned.displayName = "BentoGrid.Pinned";

/* ── Component ───────────────────────────────────────────────── */

type BentoGridProps = {
  cols?: 1 | 2 | 3;
  gap?: GapSize;
  minRowHeight?: string;
  uniform?: boolean;
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
  const { entries, rows } = partitionBentoChildren(children, cols, uniform);

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
      {entries.map((entry) => (
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
