import type { HTMLAttributes } from "react";
import * as React from "react";

import { cn } from "@/lib/cn";

import {
  type Align,
  type Justify,
  type LayoutElement,
  type ResponsiveGap,
  alignMap,
  justifyMap,
  resolveGap,
} from "./layoutTypes";

/* ── Types ───────────────────────────────────────────────────── */

type Cols = 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12;
type Span = 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | "full";
type ResponsiveCols = Cols | { mobile?: Cols; tablet?: Cols; desktop?: Cols };
type ResponsiveSpan = Span | { mobile?: Span; tablet?: Span; desktop?: Span };

/* ── Class maps (static strings for Tailwind scanner) ────────── */
/* prettier-ignore */
const colsClasses = {
  "": { 1:"grid-cols-1",2:"grid-cols-2",3:"grid-cols-3",4:"grid-cols-4",5:"grid-cols-5",6:"grid-cols-6",7:"grid-cols-7",8:"grid-cols-8",9:"grid-cols-9",10:"grid-cols-10",11:"grid-cols-11",12:"grid-cols-12" },
  "md:": { 1:"md:grid-cols-1",2:"md:grid-cols-2",3:"md:grid-cols-3",4:"md:grid-cols-4",5:"md:grid-cols-5",6:"md:grid-cols-6",7:"md:grid-cols-7",8:"md:grid-cols-8",9:"md:grid-cols-9",10:"md:grid-cols-10",11:"md:grid-cols-11",12:"md:grid-cols-12" },
  "lg:": { 1:"lg:grid-cols-1",2:"lg:grid-cols-2",3:"lg:grid-cols-3",4:"lg:grid-cols-4",5:"lg:grid-cols-5",6:"lg:grid-cols-6",7:"lg:grid-cols-7",8:"lg:grid-cols-8",9:"lg:grid-cols-9",10:"lg:grid-cols-10",11:"lg:grid-cols-11",12:"lg:grid-cols-12" },
} as const;

/* prettier-ignore */
const colSpanClasses = {
  "": { 1:"col-span-1",2:"col-span-2",3:"col-span-3",4:"col-span-4",5:"col-span-5",6:"col-span-6",7:"col-span-7",8:"col-span-8",9:"col-span-9",10:"col-span-10",11:"col-span-11",12:"col-span-12",full:"col-span-full" },
  "md:": { 1:"md:col-span-1",2:"md:col-span-2",3:"md:col-span-3",4:"md:col-span-4",5:"md:col-span-5",6:"md:col-span-6",7:"md:col-span-7",8:"md:col-span-8",9:"md:col-span-9",10:"md:col-span-10",11:"md:col-span-11",12:"md:col-span-12",full:"md:col-span-full" },
  "lg:": { 1:"lg:col-span-1",2:"lg:col-span-2",3:"lg:col-span-3",4:"lg:col-span-4",5:"lg:col-span-5",6:"lg:col-span-6",7:"lg:col-span-7",8:"lg:col-span-8",9:"lg:col-span-9",10:"lg:col-span-10",11:"lg:col-span-11",12:"lg:col-span-12",full:"lg:col-span-full" },
} as const;

/* prettier-ignore */
const rowSpanClasses = {
  "": { 1:"row-span-1",2:"row-span-2",3:"row-span-3",4:"row-span-4",5:"row-span-5",6:"row-span-6",7:"row-span-7",8:"row-span-8",9:"row-span-9",10:"row-span-10",11:"row-span-11",12:"row-span-12",full:"row-span-full" },
  "md:": { 1:"md:row-span-1",2:"md:row-span-2",3:"md:row-span-3",4:"md:row-span-4",5:"md:row-span-5",6:"md:row-span-6",7:"md:row-span-7",8:"md:row-span-8",9:"md:row-span-9",10:"md:row-span-10",11:"md:row-span-11",12:"md:row-span-12",full:"md:row-span-full" },
  "lg:": { 1:"lg:row-span-1",2:"lg:row-span-2",3:"lg:row-span-3",4:"lg:row-span-4",5:"lg:row-span-5",6:"lg:row-span-6",7:"lg:row-span-7",8:"lg:row-span-8",9:"lg:row-span-9",10:"lg:row-span-10",11:"lg:row-span-11",12:"lg:row-span-12",full:"lg:row-span-full" },
} as const;

/* ── Resolvers ───────────────────────────────────────────────── */

function resolveCols(cols: ResponsiveCols | undefined): string {
  if (!cols) return "";
  if (typeof cols === "number") return colsClasses[""][cols];
  return cn(
    cols.mobile && colsClasses[""][cols.mobile],
    cols.tablet && colsClasses["md:"][cols.tablet],
    cols.desktop && colsClasses["lg:"][cols.desktop],
  );
}

type SpanMaps = { "": Record<Span, string>; "md:": Record<Span, string>; "lg:": Record<Span, string> };

function resolveSpan(
  span: ResponsiveSpan | undefined,
  maps: SpanMaps,
): string {
  if (!span) return "";
  if (typeof span === "number" || typeof span === "string") return maps[""][span];
  return cn(
    span.mobile && maps[""][span.mobile],
    span.tablet && maps["md:"][span.tablet],
    span.desktop && maps["lg:"][span.desktop],
  );
}

/* ── Grid Root ───────────────────────────────────────────────── */

type GridFlow = "row" | "column" | "dense" | "row-dense" | "column-dense";

const flowMap: Record<GridFlow, string> = {
  "row": "grid-flow-row",
  "column": "grid-flow-col",
  "dense": "grid-flow-dense",
  "row-dense": "grid-flow-row-dense",
  "column-dense": "grid-flow-col-dense",
};

type GridRootProps = HTMLAttributes<HTMLElement> & {
  /** Number of columns. Responsive object or single value. Default `1`. */
  cols?: ResponsiveCols;
  /** Spacing between cells. T-shirt size or responsive object. Default `"md"`. */
  gap?: ResponsiveGap;
  /** Auto-placement flow. `"dense"` backfills gaps left by spanning items. */
  flow?: GridFlow;
  /** Align grid items on the cross axis (`items-*`). */
  align?: Align;
  /** Justify grid items on the main axis (`justify-*`). */
  justify?: Justify;
  /** CSS grid-template-areas for named area layouts. */
  areas?: string;
  /** CSS grid-template-rows (pairs with `areas`). */
  rows?: string;
  /** Render as a different element. Default `"div"`. */
  as?: LayoutElement;
};

function GridRoot({
  cols = 1,
  gap = "md",
  flow,
  align,
  justify,
  areas,
  rows,
  as: Tag = "div",
  className,
  style,
  ...props
}: GridRootProps) {
  return (
    <Tag
      className={cn(
        "grid",
        resolveCols(cols),
        resolveGap(gap),
        flow && flowMap[flow],
        align && alignMap[align],
        justify && justifyMap[justify],
        className,
      )}
      style={{
        ...style,
        ...(areas ? { gridTemplateAreas: areas } : undefined),
        ...(rows ? { gridTemplateRows: rows } : undefined),
      } as React.CSSProperties}
      {...props}
    />
  );
}

/* ── Grid Item ───────────────────────────────────────────────── */

type GridItemProps = HTMLAttributes<HTMLDivElement> & {
  /** Column span. Responsive object or single value. */
  colSpan?: ResponsiveSpan;
  /** Row span. Responsive object or single value. */
  rowSpan?: ResponsiveSpan;
  /** CSS grid-area name (for use with `Grid areas` prop). */
  area?: string;
};

const GridItem = React.forwardRef<HTMLDivElement, GridItemProps>(
  ({ colSpan, rowSpan, area, className, style, ...props }, ref) => (
    <div
      ref={ref}
      className={cn(
        resolveSpan(colSpan, colSpanClasses),
        resolveSpan(rowSpan, rowSpanClasses),
        className,
      )}
      style={{
        ...style,
        ...(area ? { gridArea: area } : undefined),
      }}
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

export type { GridRootProps, GridItemProps, ResponsiveCols, ResponsiveSpan, Cols, Span };
