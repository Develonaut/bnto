import type { HTMLAttributes } from "react";
import { forwardRef } from "react";

import { cn } from "@/lib/cn";

import { BouncyStagger } from "./Animate";
import {
  type Align,
  type GapSize,
  type Justify,
  type LayoutElement,
  alignMap,
  justifyMap,
} from "./layoutTypes";

/* ── Types ───────────────────────────────────────────────────── */

type Cols = 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12;
type Rows = 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12;
type Span = 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | "full";
type Start = 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13;
type Flow = "row" | "col" | "dense" | "row-dense" | "col-dense";

/* ── Static class maps (Tailwind scanner compliant) ──────────── */

/* prettier-ignore */
const colsMap: Record<Cols, string> = { 1:"grid-cols-1",2:"grid-cols-2",3:"grid-cols-3",4:"grid-cols-4",5:"grid-cols-5",6:"grid-cols-6",7:"grid-cols-7",8:"grid-cols-8",9:"grid-cols-9",10:"grid-cols-10",11:"grid-cols-11",12:"grid-cols-12" };

/* prettier-ignore */
const rowsMap: Record<Rows, string> = { 1:"grid-rows-1",2:"grid-rows-2",3:"grid-rows-3",4:"grid-rows-4",5:"grid-rows-5",6:"grid-rows-6",7:"grid-rows-7",8:"grid-rows-8",9:"grid-rows-9",10:"grid-rows-10",11:"grid-rows-11",12:"grid-rows-12" };

/* prettier-ignore */
const gapMap: Record<GapSize, string> = { xs:"gap-1.5",sm:"gap-2",md:"gap-4",lg:"gap-8",xl:"gap-12" };

/* prettier-ignore */
const flowMap: Record<Flow, string> = { "row":"grid-flow-row","col":"grid-flow-col","dense":"grid-flow-dense","row-dense":"grid-flow-row-dense","col-dense":"grid-flow-col-dense" };

/* prettier-ignore */
const colSpanMap: Record<Span, string> = { 1:"col-span-1",2:"col-span-2",3:"col-span-3",4:"col-span-4",5:"col-span-5",6:"col-span-6",7:"col-span-7",8:"col-span-8",9:"col-span-9",10:"col-span-10",11:"col-span-11",12:"col-span-12",full:"col-span-full" };

/* prettier-ignore */
const rowSpanMap: Record<Span, string> = { 1:"row-span-1",2:"row-span-2",3:"row-span-3",4:"row-span-4",5:"row-span-5",6:"row-span-6",7:"row-span-7",8:"row-span-8",9:"row-span-9",10:"row-span-10",11:"row-span-11",12:"row-span-12",full:"row-span-full" };

/* prettier-ignore */
const colStartMap: Record<Start, string> = { 1:"col-start-1",2:"col-start-2",3:"col-start-3",4:"col-start-4",5:"col-start-5",6:"col-start-6",7:"col-start-7",8:"col-start-8",9:"col-start-9",10:"col-start-10",11:"col-start-11",12:"col-start-12",13:"col-start-13" };

/* prettier-ignore */
const rowStartMap: Record<Start, string> = { 1:"row-start-1",2:"row-start-2",3:"row-start-3",4:"row-start-4",5:"row-start-5",6:"row-start-6",7:"row-start-7",8:"row-start-8",9:"row-start-9",10:"row-start-10",11:"row-start-11",12:"row-start-12",13:"row-start-13" };

/* ── Grid Root ───────────────────────────────────────────────── */

type GridRootProps = HTMLAttributes<HTMLElement> & {
  /** Number of columns (1-12). Maps to `grid-cols-{n}`. Default `1`. */
  cols?: Cols;
  /** Number of rows (1-12). Maps to `grid-rows-{n}`. */
  rows?: Rows;
  /** Gap between cells. T-shirt size. Default `"md"`. */
  gap?: GapSize;
  /** Auto-placement flow direction. */
  flow?: Flow;
  /** Align items on the cross axis (`items-*`). */
  align?: Align;
  /** Justify items on the main axis (`justify-*`). */
  justify?: Justify;
  /** Render as a different element. Default `"div"`. */
  as?: LayoutElement;
  /** Animate items with bouncy stagger entrance. */
  animated?: boolean;
};

function GridRoot({
  cols = 1,
  rows,
  gap = "md",
  flow,
  align,
  justify,
  animated,
  as: Tag = "div",
  className,
  children,
  ...props
}: GridRootProps) {
  const grid = (
    <Tag
      className={cn(
        "grid",
        colsMap[cols],
        rows && rowsMap[rows],
        gap && gapMap[gap],
        flow && flowMap[flow],
        align && alignMap[align],
        justify && justifyMap[justify],
        className,
      )}
      {...props}
    >
      {children}
    </Tag>
  );

  if (animated) {
    return <BouncyStagger asChild>{grid}</BouncyStagger>;
  }

  return grid;
}

/* ── Grid Item ───────────────────────────────────────────────── */

type GridItemProps = HTMLAttributes<HTMLDivElement> & {
  /** Column span (1-12 or "full"). */
  colSpan?: Span;
  /** Row span (1-12 or "full"). */
  rowSpan?: Span;
  /** Column start position (1-13). */
  colStart?: Start;
  /** Row start position (1-13). */
  rowStart?: Start;
};

const GridItem = forwardRef<HTMLDivElement, GridItemProps>(
  ({ colSpan, rowSpan, colStart, rowStart, className, ...props }, ref) => (
    <div
      ref={ref}
      className={cn(
        colSpan && colSpanMap[colSpan],
        rowSpan && rowSpanMap[rowSpan],
        colStart && colStartMap[colStart],
        rowStart && rowStartMap[rowStart],
        className,
      )}
      {...props}
    />
  ),
);
GridItem.displayName = "Grid.Item";

/* ── Namespace ───────────────────────────────────────────────── */

export const Grid = Object.assign(GridRoot, {
  Root: GridRoot,
  Item: GridItem,
});

export type { GridRootProps, GridItemProps, Cols, Rows, Span, Start, Flow };
