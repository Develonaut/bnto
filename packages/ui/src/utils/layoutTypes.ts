import { cn } from "./cn";

/* ── Gap ──────────────────────────────────────────────────────── */

export type GapSize = "xs" | "sm" | "md" | "lg" | "xl";
export type ResponsiveGap =
  | GapSize
  | { mobile?: GapSize; tablet?: GapSize; desktop?: GapSize };

const gapMap: Record<GapSize, string> = {
  xs: "gap-1.5",
  sm: "gap-2",
  md: "gap-4",
  lg: "gap-8",
  xl: "gap-12",
};

const mdGapMap: Record<GapSize, string> = {
  xs: "md:gap-1.5",
  sm: "md:gap-2",
  md: "md:gap-4",
  lg: "md:gap-8",
  xl: "md:gap-12",
};

const lgGapMap: Record<GapSize, string> = {
  xs: "lg:gap-1.5",
  sm: "lg:gap-2",
  md: "lg:gap-4",
  lg: "lg:gap-8",
  xl: "lg:gap-12",
};

export function resolveGap(gap: ResponsiveGap | undefined): string {
  if (!gap) return "";
  if (typeof gap === "string") return gapMap[gap];
  return cn(
    gap.mobile && gapMap[gap.mobile],
    gap.tablet && mdGapMap[gap.tablet],
    gap.desktop && lgGapMap[gap.desktop],
  );
}

/* ── Align / Justify ──────────────────────────────────────────── */

export type Align = "start" | "center" | "end" | "stretch" | "baseline";
export type Justify = "start" | "center" | "end" | "between" | "around" | "evenly";

export const alignMap: Record<Align, string> = {
  start: "items-start",
  center: "items-center",
  end: "items-end",
  stretch: "items-stretch",
  baseline: "items-baseline",
};

export const justifyMap: Record<Justify, string> = {
  start: "justify-start",
  center: "justify-center",
  end: "justify-end",
  between: "justify-between",
  around: "justify-around",
  evenly: "justify-evenly",
};

/* ── Padding (Inset) ─────────────────────────────────────────── */

export const paddingMap: Record<GapSize, string> = {
  xs: "p-1.5",
  sm: "p-2",
  md: "p-4",
  lg: "p-8",
  xl: "p-12",
};

export const paddingXMap: Record<GapSize, string> = {
  xs: "px-1.5",
  sm: "px-2",
  md: "px-4",
  lg: "px-8",
  xl: "px-12",
};

export const paddingYMap: Record<GapSize, string> = {
  xs: "py-1.5",
  sm: "py-2",
  md: "py-4",
  lg: "py-8",
  xl: "py-12",
};

/* ── Polymorphic ──────────────────────────────────────────────── */

export type LayoutElement =
  | "div"
  | "section"
  | "nav"
  | "main"
  | "footer"
  | "header"
  | "aside"
  | "ul"
  | "ol"
  | "li"
  | "form"
  | "fieldset";
