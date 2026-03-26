/** Gap resolution — maps t-shirt sizes to Tailwind gap utilities with responsive support. */

import { cn } from "./cn";

export type GapSize = "xs" | "sm" | "md" | "lg" | "xl";
export type ResponsiveGap = GapSize | { mobile?: GapSize; tablet?: GapSize; desktop?: GapSize };

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
